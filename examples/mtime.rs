//! Set up an mtime() function with the systick timer
//!
//! ---

#![deny(unsafe_code)]
#![no_main]
#![no_std]

use panic_halt as _;

use core::cell::RefCell;
use core::ops::Deref;
use cortex_m::{
    self,
    interrupt::{self, Mutex},
    peripheral::syst::SystClkSource,
};
use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::{dbg, hprintln};

/* Include stm32g474 library to link in exception handlers */
use stm32g4::stm32g474;

#[entry]
fn main() -> ! {
    let p = cortex_m::Peripherals::take().unwrap();
    let syst = p.SYST;
    start_mtime_counter(syst);

    let peri = stm32g474::Peripherals::take().unwrap();
    let rcc = peri.RCC;
    /* Enable both GPIO clocks in the same go */
    rcc.ahb2enr
        .write(|w| w.gpioaen().enabled().gpiocen().enabled());

    /* Poll PC13 for button press */
    let gpioc = peri.GPIOC;
    gpioc.moder.write(|w| w.moder13().input());

    // Configure PA5 (User LED) as output
    let gpioa = peri.GPIOA;
    gpioa.odr.write(|w| w.odr5().low());
    gpioa.moder.write(|w| w.moder5().output());

    struct Debouncer {
        state: stm32g474::gpioc::idr::IDR0_A,
        count: u32,
    }
    let mut d = Debouncer {
        state: stm32g474::gpioc::idr::IDR0_A::High,
        count: 0,
    };

    hprintln!("Press USER button to show mtime").unwrap();
    loop {
        /* Do some rough 'n ready debouncing by only recognizing a button
         * press if there are 50 consecutive samples at the same state
         * This is a good idea in general, but its probably not needed for
         * this example because `dbg!()` output takes so long in real time
         * that the button is likely to settle in a stable state after call.
         */
        while d.count < 50 {
            if gpioc.idr.read().idr13().variant() == d.state {
                d.count += 1;
            } else {
                d.count = 0;
            }
        }
        /* Reset the debouncer. Seems like there should be a nicer way to toggle
         * expected state
         */
        d.count = 0;
        if d.state == stm32g474::gpioc::idr::IDR0_A::Low {
            gpioa.odr.write(|w| w.odr5().low());
            d.state = stm32g474::gpioc::idr::IDR0_A::High;
        } else {
            // Print mtime on rising edge
            gpioa.odr.write(|w| w.odr5().high());
            dbg!(mtime().unwrap());
            d.state = stm32g474::gpioc::idr::IDR0_A::Low;
        };
    }
}

struct MTime {
    /* u32 mtime counter wraps around after ~50 days;
     * u64 mtime counter effectively will never wrap
     */
    count: u64,
    /* I think this is the right thing to do to take ownership of the SYST
     * object. I'm not sure if its a better idea to keep ownership here or
     * to just drop it in start_mtime_counter() */
    #[allow(dead_code)]
    syst: cortex_m::peripheral::SYST,
}

/* I'm not sure about the type of the MTime counter. Is this really the best way to
 * wrap it like this then lazy initialize it?
 * We pay for the abstraction by requiring this to be wrapped in an interrupt::free
 * critical section instead of using an atomic, or polling read strategy to
 * access the u64 count.
 * Also note that the cortex_m rust crate only supports the 'big hammer' mutex
 * option in interrupt::free that disables *all* interrupts, where some cores,
 * like Cortex-M4, have hardware that allows selectively disabling lower priority
 * interrupts only.
 */
static G_MTIME: Mutex<RefCell<Option<MTime>>> = Mutex::new(RefCell::new(None));

fn mtime() -> Option<u64> {
    interrupt::free(|cs| {
        if let Some(ref mtime) = G_MTIME.borrow(cs).borrow().deref() {
            Some(mtime.count)
        } else {
            /* I think this case is the only argument for storing MTime count
             * in a Mutex<RefCell<Option<>>> type instead of a Mutex<MTime>. This lets the app handle what happens
             * if it calls mtime() before the SysTick counter gets started, instead
             * of perhaps returning an invalid timestamp.
             */
            None
        }
    })
}

fn start_mtime_counter(mut syst: cortex_m::peripheral::SYST) {
    interrupt::free(|cs| {
        let mut maybe_mtime = G_MTIME.borrow(cs).borrow_mut();
        if let None = maybe_mtime.deref() {
            // configures the system timer to trigger a SysTick exception every 1ms
            // Assuming Core clock defaults to 16MHz (HSI oscillator)
            // See STM32G4x Reference manual 7.2
            syst.set_clock_source(SystClkSource::Core);
            syst.set_reload(16_000);
            syst.enable_counter();
            syst.enable_interrupt();
            // Move syst ownership into G_MTIME
            maybe_mtime.replace(MTime { count: 0, syst });
        } else {
            panic!();
            // double init, but I think it shouldn't be possible to do this because
            // main() would need another mutable syst object to pass in
        }
    });
}

#[exception]
fn SysTick() {
    /*
     * Increment the mtime counter on Systick interrupt.
     */
    interrupt::free(|cs| {
        /* Use unwrap because we should never see an interrupt here unless
         * G_MTIME has been initialized by start_mtime_counter() */
        let mut mtime = G_MTIME.borrow(cs).borrow_mut();
        mtime.as_mut().unwrap().count += 1;
    });
}
