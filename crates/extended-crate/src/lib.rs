#![no_std]

// ============================== Test Setup ===================================
// The following attributes are necessary to enable `ktest` in a library crate. They 
// set up the custom test framework and entrypoint for tests.

#![cfg_attr(test, feature(custom_test_frameworks))]          // enable no_std tests
#![cfg_attr(test, test_runner(ktest::runner))]               // set `ktest` custom test framework runner
#![cfg_attr(test, no_main)]                                  // disable normal main entrypoint
#![cfg_attr(test, reexport_test_harness_main = "test_main")] // set test main entrypoint (MUST BE "test_main")

// The following line is the real key to enabling `ktest` in this library crate. It will inject 
// the necessary boilerplate to set up the test environment when tests are run. This 
// includes an entrypoint, panic handler, and basic bootloader configuration.
//
// This macro does not include test attributes, such as "feature(custom_test_frameworks)" and 
// "no_main" (above), because their inclusion would require several additional unstable features: 
// "custom_inner_attributes", "include_prelude", and potentially others.
//
// "extended_crate" is the name of this crate, and should be replaced with the actual crate name 
// for proper log outputs and test classification from the test runner.

extern crate alloc;

mod other_module;

#[cfg(test)] // klib_config and boot_config are optional
ktest::klib!("extended_crate", klib_config = &KLIB_CONFIG, boot_config = &BOOTLOADER_CONFIG);

#[cfg(test)] // this config is optional
pub const KLIB_CONFIG: ktest::KlibConfig = ktest::KlibConfigBuilder::new_default()
    .before_tests(|boot_info| init(boot_info))
    .after_tests(|| teardown())
    .build();

#[cfg(test)] // this config is optional
pub const BOOTLOADER_CONFIG: bootloader_api::BootloaderConfig = {
    const HIGHER_HALF_START: u64 = 0xffff_8000_0000_0000;
    const PHYSICAL_MEMORY_OFFSET: u64 = 0x0000_0880_0000_0000;

    let mut config = bootloader_api::BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(bootloader_api::config::Mapping::FixedAddress(PHYSICAL_MEMORY_OFFSET));
    config.mappings.dynamic_range_start = Some(HIGHER_HALF_START);
    config
};

#[cfg(test)] // this function is optional
pub fn init(_boot_info: &'static bootloader_api::BootInfo) {
    // Setup code to run before tests, e.g. kernel initialization
}

#[cfg(test)] // this function is optional
pub fn teardown() {
    // Teardown code to run after tests
}

// =========================== End of Test Setup ===============================

#[cfg(test)]
mod tests {
    use ktest::ktest;

    #[ktest]
    fn when_alloc_enabled_then_verify_heap_initialized() {
        let mut vec = alloc::vec::Vec::from([1, 2, 3, 4, 5]);
        vec.push(6); // test that we can dynamically allocate memory

        assert_eq!(vec.len(), 6);
    }

    #[ktest]
    #[ignore]
    fn when_ignore_then_do_not_test() {
        assert_eq!(1, 1);
    }

    #[ktest]
    #[should_panic]
    fn when_should_panic_then_do_not_fail() -> () {
        panic!("This panic should not cause a fail");
    }

    // #[ktest]
    // fn when_should_not_pass_then_fail() -> () {
    //     panic!("This fail was intentional");
    // }
}
