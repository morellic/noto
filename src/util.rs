pub mod fs_util;
pub mod printer;

#[cfg(test)]
pub mod test_util {
    // ----------------------------
    // Helper macros for unit tests
    // ----------------------------

    #[macro_export]
    macro_rules! assert_eq_bytes {
        ($bytes:expr, $expected_bytes:expr) => {
            // compare as str for a better fail message
            assert_eq!(
                std::str::from_utf8(&$bytes).unwrap(),
                std::str::from_utf8($expected_bytes).unwrap(),
                "\ngot left but expected right"
            );
        };
    }
}
