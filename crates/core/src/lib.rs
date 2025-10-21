//! A simple kernel that prints "Hello World!" to the serial port and
//! exits QEMU.
//!
//! While a main.rs file exists, a kernel should primarily be a library crate.

#![no_std]                     // do not link the Rust standard library
#![cfg_attr(test, no_main)]    // disable default Rust-level entry points
#![feature(abi_x86_interrupt)] // enable the x86-interrupt calling convention
#![cfg_attr(test, feature(custom_test_frameworks))]          // test setup: enable custom test frameworks
#![cfg_attr(test, test_runner(ktest::runner))]               // test setup: use the custom test runner only in test mode
#![cfg_attr(test, reexport_test_harness_main = "test_main")] // test setup: rename the test harness entry point

// Note: ktest::klib macro invocation would normally go here, but since this is
// the core crate of the kernel, full control of the entrypoint should be maintained.

/// Bootloader configuration that maps the kernel to the higher half of the
/// address space.
pub const BOOTLOADER_CONFIG: bootloader_api::BootloaderConfig = {
    let config = bootloader_api::BootloaderConfig::new_default();
    config
};

/// A simple panic handler that prints the panic information to the serial
/// port and exits QEMU with a failure code.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    use core::fmt::Write;

    let _ = writeln!(serial(), "PANIC: {info}");
    exit_qemu(QemuExitCode::Failed);
}

/// A simple panic handler for tests, delegating to `ktest` for smoothly
/// handling failures (panics) and continuing with other tests.
#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    ktest::panic(info)
}

/// Exit QEMU with the given exit code. This function will not return.
/// 
/// The exit code is written to the I/O port `0xf4`, which is monitored
/// by QEMU to exit with the given code.
pub fn exit_qemu(exit_code: QemuExitCode) -> ! {
    use x86_64::instructions::{nop, port::Port};

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }

    loop {
        nop();
    }
}

/// Returns a serial port interface for printing to the host machine. QEMU
/// emulates a 16550 UART, which is a common serial port interface.
pub fn serial() -> uart_16550::SerialPort {
    let mut port = unsafe { uart_16550::SerialPort::new(0x3F8) };
    port.init();
    port
}

/// Halts the CPU until the next interrupt. This is useful for preventing
/// the CPU from running in a busy loop when there is nothing to do.
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

/// Exit codes for QEMU. These codes are written to the I/O port `0xf4`
/// to signal QEMU to exit with the given code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

// The entry point for `cargo test`. This function is called by the bootloader
// after the kernel is loaded. Only relevant to library tests.
#[cfg(test)]
fn kernel_test_main(_boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    // any kernel init can happen here

    ktest::init_harness("library");
    test_main();
    
    hlt_loop();
}

// Register the test entry point with the bootloader. Only relevant to library tests.
#[cfg(test)]
bootloader_api::entry_point!(kernel_test_main, config = &BOOTLOADER_CONFIG);

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
