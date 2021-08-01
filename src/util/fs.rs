use crate::util::printer;

pub fn get_current_dir<Out: std::io::Write, ErrOut: std::io::Write>(
    printer: &mut printer::Printer<Out, ErrOut>,
) -> Option<std::path::PathBuf> {
    match std::env::current_dir() {
        Ok(dir) => Some(dir),
        Err(e) => {
            let err_msg = format!("Could not access the current directory!\n{}", e);
            printer.write_err_line(err_msg);
            None
        }
    }
}

pub fn get_read_dir<Out: std::io::Write, ErrOut: std::io::Write>(
    path: &std::path::Path,
    printer: &mut printer::Printer<Out, ErrOut>,
) -> Option<std::fs::ReadDir> {
    if path.is_file() {
        let err_msg = format!("{} is a file!", path.file_name().unwrap().to_str().unwrap());
        printer.write_err_line(err_msg);
        return None;
    }

    match std::fs::read_dir(&path) {
        Ok(rd) => Some(rd),
        Err(e) => {
            let err_msg = format!("Could not access the directory {}!\n{}", path.display(), e);
            printer.write_err_line(err_msg);
            None
        }
    }
}

pub fn get_file_or_dir_name<Out: std::io::Write, ErrOut: std::io::Write>(
    path: &std::path::Path,
    printer: &mut printer::Printer<Out, ErrOut>,
) -> Option<String> {
    if let Some(name) = path.file_name() {
        if let Some(s) = name.to_str() {
            return Some(s.to_string());
        }
    }

    let err_msg = format!(
        "Could not get the file or directory name of {}",
        path.display()
    );
    printer.write_err_line(err_msg);

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::assert_outs_get_ret;
    use crate::util::test_util::mock;

    #[test]
    fn test_get_current_dir() {
        let ret = assert_outs_get_ret!(b"", b"", get_current_dir);
        assert_eq!(ret.is_some(), true);
    }

    #[test]
    fn test_get_read_dir() {
        _test_get_read_dir_with_file_path();
        _test_get_read_dir_with_dir_path();
    }

    fn _test_get_read_dir_with_file_path() {
        let mock_file_1_ret = assert_outs_get_ret!(
            b"",
            b"mock-file-1.txt is a file!\n",
            get_read_dir,
            &mock::get_mock_file_1_path()
        );
        assert_eq!(mock_file_1_ret.is_some(), false);
    }

    fn _test_get_read_dir_with_dir_path() {
        let mock_dir_1_ret =
            assert_outs_get_ret!(b"", b"", get_read_dir, &mock::get_mock_dir_1_path());
        assert_eq!(mock_dir_1_ret.is_some(), true);
        assert_eq!(mock_dir_1_ret.unwrap().count(), 2);

        let mock_dir_2_ret =
            assert_outs_get_ret!(b"", b"", get_read_dir, &mock::get_mock_dir_2_path());
        assert_eq!(mock_dir_2_ret.is_some(), true);
        assert_eq!(mock_dir_2_ret.unwrap().count(), 0);
    }

    #[test]
    fn test_get_file_or_dir_name() {
        _test_get_file_or_dir_name_with_file_path();
        _test_get_file_or_dir_name_with_dir_path();
    }

    fn _test_get_file_or_dir_name_with_file_path() {
        let mock_file_1_name = assert_outs_get_ret!(
            b"",
            b"",
            get_file_or_dir_name,
            &mock::get_mock_file_1_path()
        );
        assert_eq!(mock_file_1_name.is_some(), true);
        assert_eq!(mock_file_1_name.unwrap(), "mock-file-1.txt");
    }

    fn _test_get_file_or_dir_name_with_dir_path() {
        let mock_dir_1_name =
            assert_outs_get_ret!(b"", b"", get_file_or_dir_name, &mock::get_mock_dir_1_path());
        assert_eq!(mock_dir_1_name.is_some(), true);
        assert_eq!(mock_dir_1_name.unwrap(), "mock-dir-1");
    }

    fn _test_get_file_or_dir_name_with_non_existing_path() {
        let non_existing_name = assert_outs_get_ret!(
            b"",
            b"",
            get_file_or_dir_name,
            &mock::get_non_existing_path()
        );
        assert_eq!(non_existing_name.is_some(), true);
        assert_eq!(non_existing_name.unwrap(), "42");
    }
}
