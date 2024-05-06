//! Prints "Hello, world!" on the host console using semihosting
//! Identical to examples/hello.rs because I don't know how to make
//! a cargo project without anything under src/

#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

/* Include stm32g474 library to link in exception handlers */
#[allow(unused_imports)]
use stm32g4::stm32g474;

#[entry]
fn main() -> ! {
    hprintln!("Hello, world!").unwrap();
    loop {}
}
