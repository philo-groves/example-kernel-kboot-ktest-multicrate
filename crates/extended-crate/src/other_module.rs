#[cfg(test)]
mod tests {
    use ktest::ktest;

    #[ktest]
    fn basic_other_module_assert() {
        assert_eq!(1, 1);
    }
}
