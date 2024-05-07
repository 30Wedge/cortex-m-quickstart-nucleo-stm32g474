//! Changing the panicking behavior
//!
//! The easiest way to change the panicking behavior is to use a different [panic handler crate][0].
//!
//! [0]: https://crates.io/keywords/panic-impl

#![no_main]
#![no_std]

// Pick one of these panic handlers:

/* `panic!` halts execution by dropping into an infinite loop.
 * The panic message is ignored. If a debugger is connected, then you still
 * get to see a backtrace of where the panic happened.
 */
use panic_halt as _;

/* Reports panic messages to the host stderr using semihosting
 * It gives you a nicely formatted error message like this:
 * """
 * panicked at examples/panic.rs:33:5:
 * Oops
 * """
 * and the debugger stops at the 'panic' call with a neat backtrace
 *
 * NOTE to use this you need to import `panic-semihosting` in Cargo.toml
 */
//use panic_semihosting as _;

/* Logs panic messages using the ITM (Instrumentation Trace Macrocell)
 * NOTE to use this you need to uncomment the `panic-itm` dependency in Cargo.toml
 */
// use panic_itm as _;

use cortex_m_rt::entry;

/* Include stm32g474 library to link in exception handlers */
#[allow(unused_imports)]
use stm32g4::stm32g474;

#[entry]
fn main() -> ! {
    panic!("Oops")
}
