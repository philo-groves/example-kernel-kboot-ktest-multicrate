//! A simple kernel that prints "Hello World!" to the serial port and 
//! exits QEMU.
//! 
//! A main.rs is file required because the `bootloader` will look for a binary crate.

#![no_std]                                    // do not link the Rust standard library
#![no_main]                                   // disable default Rust-level entry points
#![feature(abi_x86_interrupt)]                // enable the x86-interrupt calling convention
#![cfg_attr(test, feature(custom_test_frameworks))]           // test setup: enable custom test frameworks
#![cfg_attr(test, test_runner(ktest::runner))]                // test setup: use the custom test runner only in test mode
#![cfg_attr(test, reexport_test_harness_main = "test_main")]  // test setup: rename the test harness entry point

// This is the entry point for the kernel, as required by the bootloader crate. The 
// `boot_info` argument provides information about the boot process and system.
//
// The `build.rs` script will scan for this macro to find the kernel crate.
bootloader_api::entry_point!(kernel_main, config = &kernel_core::BOOTLOADER_CONFIG);

/// Entry point for the kernel after the bootloader. This function is called 
/// by the bootloader with a reference to the inflated `BootInfo` structure.
/// 
/// This function is supplied to the `bootloader::entry_point!` macro.
fn kernel_main(_boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    // any kernel init can happen here

    // run tests if we are in test mode
    #[cfg(test)]
    {
        ktest::init_harness("binary");
        test_main();
    }

    // continue running the kernel in a HLT loop
    kernel_core::hlt_loop();
}

#[cfg(test)]
mod tests {
    use ktest::ktest;

    #[ktest]
    fn basic_main_assertion() {
        assert_eq!(1, 1);
    }
}
