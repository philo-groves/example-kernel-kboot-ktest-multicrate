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
// "basic_crate" is the name of this crate, and should be replaced with the actual crate name 
// for proper log outputs and test classification from the test runner.

#[cfg(test)] // klib_config and boot_config are optional
ktest::klib!("basic_crate");

// =========================== End of Test Setup ===============================

#[cfg(test)]
mod tests {
    use ktest::ktest;

    #[ktest]
    #[ignore]
    fn when_ignore_then_do_not_test() {
        assert_eq!(1, 1);
    }

    #[ktest]
    fn when_not_ignored_then_test() {
        assert_eq!(1, 1);
    }

    // #[ktest]
    // fn when_should_not_pass_then_fail() -> () {
    //     panic!("This fail was intentional");
    // }
}
