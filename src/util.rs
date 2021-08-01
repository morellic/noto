pub mod fs;
pub mod printer;

/// Helper modules, functions and macros to use in unit tests
#[cfg(test)]
pub mod test_util {
    /// Helpers to create mock objects, including paths to example files and directories under the
    /// example mock directory $project_dir/tests/mock_dir
    pub mod mock {

        // -----------------------------
        // Functions to get mock objects
        // -----------------------------

        pub fn get_mock_printer() -> crate::util::printer::Printer<Vec<u8>, Vec<u8>> {
            crate::util::printer::Printer {
                out: Vec::new(),
                err_out: Vec::new(),
            }
        }

        // -------------------------------------------------------------------------
        // Functions to get paths of mock examples under project_dir>/tests/mock_dir
        // -------------------------------------------------------------------------

        // mock-dir/
        // └── mock-dir-1/
        //     ├── mock-dir-1-1/
        //     └── mock-file-1-1.txt
        // ├── mock-dir-2/
        // ├── mock-file-1.txt
        // └── mock-file-2.txt

        pub fn get_mock_dir_path() -> std::path::PathBuf {
            std::path::PathBuf::from(r"./tests/mock-dir")
        }

        pub fn get_mock_dir_1_path() -> std::path::PathBuf {
            std::path::PathBuf::from(r"./tests/mock-dir/mock-dir-1")
        }

        pub fn get_mock_dir_1_1_path() -> std::path::PathBuf {
            std::path::PathBuf::from(r"./tests/mock-dir/mock-dir-1/mock-dir-1-1")
        }

        pub fn get_mock_file_1_path() -> std::path::PathBuf {
            std::path::PathBuf::from(r"./tests/mock-dir/mock-file-1.txt")
        }

        pub fn get_mock_file_1_1_path() -> std::path::PathBuf {
            std::path::PathBuf::from(r"./tests/mock-dir/mock-dir-1/mock-file-1-1.txt")
        }

        pub fn get_mock_file_2_path() -> std::path::PathBuf {
            std::path::PathBuf::from(r"./tests/mock-dir/mock-file-2.txt")
        }

        pub fn get_mock_dir_2_path() -> std::path::PathBuf {
            std::path::PathBuf::from(r"./tests/mock-dir/mock-dir-2")
        }

        pub fn get_non_existing_path() -> std::path::PathBuf {
            std::path::PathBuf::from(r"./this/does/not/exist/42")
        }

        fn _get_path(args: &[&str]) -> std::path::PathBuf {
            args.iter().collect()
        }
    }

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

    #[macro_export]
    macro_rules! assert_outs {
        ($expected_out:expr, $expected_err_out:expr, $func:expr) => {
            let mut printer = crate::util::test_util::mock::get_mock_printer();
            $func(&mut printer);
            crate::assert_eq_bytes!(printer.out, $expected_out);
            crate::assert_eq_bytes!(printer.err_out, $expected_err_out);
        };
        ($expected_out:expr, $expected_err_out:expr, $func:expr, $($args:expr),*) => {
            $(
                let mut printer = crate::util::test_util::mock::get_mock_printer();
                $func($args, &mut printer);
                crate::assert_eq_bytes!(printer.out, $expected_out);
                crate::assert_eq_bytes!(printer.err_out, $expected_err_out);
            )*
        }
    }

    #[macro_export]
    macro_rules! assert_outs_get_ret {
        ($expected_out:expr, $expected_err_out:expr, $func:expr) => {{
            let mut printer = crate::util::test_util::mock::get_mock_printer();
            let ret = $func(&mut printer);
            crate::assert_eq_bytes!(printer.out, $expected_out);
            crate::assert_eq_bytes!(printer.err_out, $expected_err_out);
            ret
        }};
        ($expected_out:expr, $expected_err_out:expr, $func:expr, $($args:expr),*) => {{
            $(
                let mut printer = crate::util::test_util::mock::get_mock_printer();
                let ret = $func($args, &mut printer);
                crate::assert_eq_bytes!(printer.out, $expected_out);
                crate::assert_eq_bytes!(printer.err_out, $expected_err_out);
                ret
            )*
        }}
    }
}
