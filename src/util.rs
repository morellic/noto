pub mod fs;
pub mod out;

pub use fs::get_current_dir;
pub use fs::get_read_dir;
pub use out::write_line;

/// Helper modules, functions and macros to use in unit tests
#[cfg(test)]
pub mod test_util {
    /// Helpers to create mock objects, including paths to example files and directories under the
    /// example mock directory $project_dir/tests/mock_dir
    pub mod mock {
        // --------------------------------------------------------------------
        // Functions to get paths of examples under project_dir>/tests/mock_dir
        // --------------------------------------------------------------------

        // mock-dir/
        // └── mock-dir-1/
        //     ├── mock-dir-1-1/
        //     └── mock-file-1-1.txt
        // ├── mock-dir-2/
        // ├── mock-file-1.txt
        // └── mock-file-2.txt

        pub fn get_mock_dir_path() -> std::path::PathBuf {
            _get_path(&[".", "tests", "mock-dir"])
        }

        pub fn get_mock_dir_1_path() -> std::path::PathBuf {
            _get_path(&[".", "tests", "mock-dir", "mock-dir-1"])
        }

        pub fn get_mock_dir_1_1_path() -> std::path::PathBuf {
            _get_path(&[".", "tests", "mock-dir", "mock-dir-1", "mock-dir-1-1"])
        }

        pub fn get_mock_file_1_path() -> std::path::PathBuf {
            _get_path(&[".", "tests", "mock-dir", "mock-file-1.txt"])
        }

        pub fn get_mock_file_1_1_path() -> std::path::PathBuf {
            _get_path(&[".", "tests", "mock-dir", "mock-dir-1", "mock-file-1-1.txt"])
        }

        pub fn get_mock_file_2_path() -> std::path::PathBuf {
            _get_path(&[".", "tests", "mock-dir", "mock-file-2.txt"])
        }

        pub fn get_mock_dir_2_path() -> std::path::PathBuf {
            _get_path(&[".", "tests", "mock-dir", "mock-dir-2"])
        }

        fn _get_path(args: &[&str]) -> std::path::PathBuf {
            args.iter().collect()
        }
    }

    // -----------------------
    // Unit test helper macros
    // -----------------------

    #[macro_export]
    macro_rules! assert_eq_bytes {
        ($bytes:expr, $expected_bytes:expr) => {
            // compare as str for a better fail message
            assert_eq!(
                std::str::from_utf8(&$bytes).unwrap(),
                std::str::from_utf8($expected_bytes).unwrap(),
                "\ngot right but expected left"
            );
        };
    }

    #[macro_export]
    macro_rules! test_out {
        ($expected_out:expr, $func:expr) => {
            let mut out = Vec::new();
            $func(&mut out);
            crate::assert_eq_bytes!(out, $expected_out);
        };
        ($expected_out:expr, $func:expr, $($args:expr),*) => {
            $(
                let mut out = Vec::new();
                $func($args, &mut out);
                crate::assert_eq_bytes!(out, $expected_out);
            )*
        }
    }

    #[macro_export]
    macro_rules! test_out_get_ret {
        ($expected_out:expr, $func:expr) => {{
            let mut out = Vec::new();
            let ret = $func(&mut out);
            crate::assert_eq_bytes!(out, $expected_out);
            ret
        }};
        ($expected_out:expr, $func:expr, $($args:expr),*) => {{
            $(
                let mut out = Vec::new();
                let ret = $func($args, &mut out);
                crate::assert_eq_bytes!(out, $expected_out);
                ret
            )*
        }}
    }
}
