//! Debugging a crash (exception):
//! Note: For this example to trigger a crash, it must be built with the
//! --release flag like `cargo build --example crash --release`
//!
//! Most crash conditions trigger a hard fault exception, whose handler is defined via
//! `exception!(HardFault, ..)`. The `HardFault` handler has access to the exception frame, a
//! snapshot of the CPU registers at the moment of the exception.
//!
//! This program crashes and the custom `HardFault` handler prints to the
//! console the contents of the `ExceptionFrame` and then triggers a
//! breakpoint. From that breakpoint one can see the backtrace that led to the
//! exception.
//!
//! ``` text
//! (gdb) continue
//! Breakpoint 2, cortex_m_rt::HardFault_ (ef=0x2001ffd8) at src/lib.rs:560
//! 560	    loop {
//! (gdb) backtrace
//! #0  cortex_m_rt::HardFault_ (ef=0x2001ffd8) at src/lib.rs:560
//! #1  <signal handler called>
//! #2  core::ptr::read_volatile<u32> (src=<optimized out>) at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mod.rs:1665
//! #3  crash::__cortex_m_rt_main () at /home/amacgregor/Projects/rust/embedded_stm32g4/hello_g4/examples/crash.rs:103
//! #4  crash::__cortex_m_rt_main_trampoline () at /home/amacgregor/Projects/rust/embedded_stm32g4/hello_g4/examples/crash.rs:94
//! #5  cortex_m_rt::Reset::trampoline () at src/lib.rs:547
//! #6  0x0800025c in cortex_m_rt::Reset () at src/lib.rs:550
//! ```
//!
//! From gdb, you can walk up the stack frame with `up` to see the state of the
//! stack on panic.
//! ``` text
//! (gdb) up
//! #1  <signal handler called>
//! (gdb) up
//! #2  core::ptr::read_volatile<u32> (src=<optimized out>) at /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mod.rs:1665
//! warning: 1665	/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/mod.rs: No such file or directory
//! (gdb)
//! #3  crash::__cortex_m_rt_main () at /home/amacgregor/Projects/rust/embedded_stm32g4/hello_g4/examples/crash.rs:106
//! 106	        ptr::read_volatile(0x2FFF_FFFF as *const u32);
//! ```
//!
//! I didn't have any success looking at the disassembly at the address stored
//! in the PC or LR registers to find the instruction that caused the HardFault
//! like the https://github.com/rust-embedded/cortex-m-quickstart project did.
//! ```
//! ExceptionFrame {
//! r0: 0x2fffffff,
//! r1: 0x00f00000,
//! r2: 0x00000000,
//! r3: 0x00000000,
//! r12: 0x00000000,
//! lr: 0x0800025d,
//! pc: 0x08000dbc,
//! xpsr: 0x61000000,
//! }
//! ```
//! ```
//! (gdb) disassemble/m 0x08000dbc
//! Dump of assembler code for function cortex_m_rt::Reset::trampoline:
//! 546	            fn trampoline() -> ! {
//!   0x08000db8 <+0>:	mvn.w	r0, #3489660928	@ 0xd0000000
//! ```
//!
//!
//! ---

#![no_main]
#![no_std]

use panic_halt as _;

use core::fmt::Write;
use core::ptr;

use cortex_m_rt::{entry, exception, ExceptionFrame};
use cortex_m_semihosting::hio;

/* Include stm32g474 library to link in exception handlers */
#[allow(unused_imports)]
use stm32g4::stm32g474;

#[entry]
fn main() -> ! {
    unsafe {
        /*
         * read an address outside of the RAM region; this causes a HardFault exception
         * on --release builds only.
         * On debug builds, the `assert_unsafe_precondition!` checks in
         * ptr::read_volatile() will instead generate a panic
         */
        ptr::read_volatile(0x2FFF_FFFF as *const u32);
    }

    loop {}
}


#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    if let Ok(mut hstdout) = hio::hstdout() {
        writeln!(hstdout, "{:#?}", ef).ok();
    }

    loop {}
}
