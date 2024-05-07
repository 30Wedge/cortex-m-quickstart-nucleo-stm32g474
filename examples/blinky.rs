#![no_std]
#![no_main]
/*
 * blinky.rs - Runs on NUCLEO_STM32G474 board. Blinks UserLED (PA5) at 1Hz
 */

// Halt on panic; you can put a breakpoint on `rust_begin_unwind` to catch panics
use panic_halt as _; //

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

/* Fun fact; you have to explicitly include something from the stm32g071 package
 * otherwise interrupt handlers won't be linked in and the compiler will complain
 * with a somewhat obscure error message
 */
use stm32g0::stm32g071;

#[entry]
fn main() -> ! {
    /* Blinky demo */

    /* Copy the Systick setup from the embedded Rust book */
    let core_p = cortex_m::peripheral::Peripherals::take().unwrap();
    let mut systick = core_p.SYST;
    systick.set_clock_source(cortex_m::peripheral::syst::SystClkSource::Core);
    /* We assume that there has been no other clock tree configuration.
     * That should mean that the "Core" system clock is configured to run
     * at 8MHz by default, which directly feeds the Systick counter.
     * Setting the reload period to 8MHz configures a 1s period
     */
    systick.set_reload(8_000_000);
    systick.clear_current();
    systick.enable_counter();

    /* Enabling the RCC registers is the secret sauce that's not included in
     * the embedded rust book.
     * If you don't enable the RCC then writes to memory mapped peripherals
     * will fail without an indication and make you question the correctness of
     * the svd2rust crate you're using.
     * I got deep in the assembly single-stepping weeds before coming back here.
     */
    let peri = stm32g071::Peripherals::take().unwrap();
    let rcc = peri.RCC;
    rcc.iopenr.write(|w| w.iopaen().set_bit());

    let gpioa = peri.GPIOA;

    // Configure PA5 (User LED) as output
    gpioa.odr.write(|w| w.odr5().low());
    gpioa.moder.write(|w| w.moder5().output());

    loop {
        gpioa.odr.write(|w| w.odr5().high());
        hprintln!("blink-1").unwrap();
        while !systick.has_wrapped() {
            // Loop to delay 1s
        }

        gpioa.odr.write(|w| w.odr5().low());
        hprintln!("blink-0").unwrap();
        while !systick.has_wrapped() {
            // Loop to delay 1s
        }
    }
}
