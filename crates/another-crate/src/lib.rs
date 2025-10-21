#![no_std]

// ============================== Test Setup ===================================
// The following attributes are necessary to enable `ktest` in a library crate. They 
// set up the custom test framework and entrypoint for tests.

#![cfg_attr(test, feature(custom_test_frameworks))]          // enable no_std tests
#![cfg_attr(test, test_runner(ktest::runner))]               // set `ktest` custom test framework runner
#![cfg_attr(test, no_main)]                                  // disable normal main entrypoint
#![cfg_attr(test, reexport_test_harness_main = "test_main")] // set test main entrypoint (should always be "test_main")

// The following line is the real key to enabling `ktest` in this library crate. It will inject 
// the necessary boilerplate to set up the test environment when tests are run. This 
// includes an entrypoint, panic handler, and basic bootloader configuration.
//
// This macro does not include test attributes, such as "feature(custom_test_frameworks)" and 
// "no_main" (above), because their inclusion would require several additional unstable features: 
// "custom_inner_attributes", "include_prelude", and potentially others.
//
// "another_crate" is the name of this crate, and should be replaced with the actual crate name 
// for proper log outputs and test classification from the test runner.
#[cfg_attr(test, ktest::klib("another_crate"))]

// In this case, no allocator is required for the library crate. However, if any crate 
// does require an allocator (e.g., for String, Vec, etc.), it can be set up for tests 
// with a separate macro, `ktest::klib_requires_alloc`.
//
// The allocator that is configured in the main kernel crate is not available to kernel libraries 
// during tests; therefore, this macro will include a basic linked-list allocator with the test runner.
//
// #[cfg_attr(test, ktest::klib_requires_alloc("another_crate"))]

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

    #[ktest]
    #[should_panic]
    fn when_should_panic_then_pass() -> () {
        panic!("This panic should not cause a fail");
    }

    #[ktest]
    fn when_should_not_pass_then_fail() -> () {
        panic!("This fail was intentional");
    }
}
