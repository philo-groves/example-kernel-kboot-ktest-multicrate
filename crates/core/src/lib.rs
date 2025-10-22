//! A simple bare-bones kernel.
//!
//! A main.rs file exists, but a kernel should primarily be a library crate.

#![no_std]                     // do not link the Rust standard library
#![cfg_attr(test, no_main)]    // disable default Rust-level entry points
#![feature(abi_x86_interrupt)] // enable the x86-interrupt calling convention
#![cfg_attr(test, feature(custom_test_frameworks))]          // test setup: enable custom test frameworks
#![cfg_attr(test, test_runner(ktest::runner))]               // test setup: use the custom test runner only in test mode
#![cfg_attr(test, reexport_test_harness_main = "test_main")] // test setup: rename the test harness entry point

#[cfg(test)]
ktest::klib!("library");

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop { x86_64::instructions::nop(); }
}

/// Halts the CPU until the next interrupt.
pub fn hlt_loop() -> ! {
    loop { x86_64::instructions::hlt(); }
}

#[cfg(test)]
mod tests {
    use ktest::ktest;

    #[ktest]
    #[ignore]
    fn when_ignore_then_do_not_test() {
        assert_eq!(1, 1);
    }

    #[ktest]
    #[should_panic]
    fn when_should_panic_then_pass() -> () {
        panic!("This panic should not cause a fail");
    }

    #[ktest]
    fn when_should_not_pass_then_fail() -> () {
        panic!("Make sure tests fail correctly");
    }
}
