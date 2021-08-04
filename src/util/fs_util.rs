use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum FsErr {
    #[error("Cannot read file name")]
    FileNameReadErr,

    #[error("Cannot read directory contents")]
    DirReadErr,

    #[error("Cannot read directory name")]
    DirNameReadErr,

    #[error("Cannot read current working directory")]
    CurrentDirReadErr,

    #[error("Path is not a file")]
    PathIsNotFileErr,

    #[error("Path is not a directory")]
    PathIsNotDirErr,
}

pub fn get_current_dir() -> Result<std::path::PathBuf, FsErr> {
    match std::env::current_dir() {
        Ok(path) => Ok(path),
        Err(_) => Err(FsErr::CurrentDirReadErr),
    }
}

pub fn get_read_dir(path: &std::path::Path) -> Result<std::fs::ReadDir, FsErr> {
    if path.is_file() {
        return Err(FsErr::PathIsNotDirErr);
    }

    match std::fs::read_dir(&path) {
        Ok(path) => Ok(path),
        Err(_) => Err(FsErr::DirReadErr),
    }
}

pub fn get_file_or_dir_name(path: &std::path::Path) -> Result<String, FsErr> {
    if path.is_file() {
        get_file_name(path)
    } else {
        get_dir_name(path)
    }
}

pub fn get_file_name(path: &std::path::Path) -> Result<String, FsErr> {
    if !path.is_file() {
        return Err(FsErr::PathIsNotFileErr);
    }

    match path.file_name() {
        Some(name) => Ok(name.to_str().unwrap().to_owned()),
        None => Err(FsErr::FileNameReadErr),
    }
}

pub fn get_dir_name(path: &std::path::Path) -> Result<String, FsErr> {
    if path.is_file() {
        return Err(FsErr::PathIsNotDirErr);
    }

    match path.file_name() {
        Some(name) => Ok(name.to_str().unwrap().to_owned()),
        None => Err(FsErr::DirNameReadErr),
    }
}

pub fn get_path_or_current_dir(
    path: Option<std::path::PathBuf>,
) -> Result<std::path::PathBuf, FsErr> {
    if path.is_none() {
        return get_current_dir();
    }
    Ok(path.unwrap())
}

#[cfg(test)]
pub mod test_util {
    // ----------------------------------------------------------------------------
    // Functions to get paths of mock examples under project_dir>/examples/mock_dir
    // ----------------------------------------------------------------------------

    // mock-dir/
    // └── mock-dir-1/
    //     ├── mock-dir-1-1/
    //     └── mock-file-1-1.txt
    // ├── mock-dir-2/
    // ├── mock-file-1.txt
    // └── mock-file-2.txt

    pub fn get_mock_dir_path() -> std::path::PathBuf {
        std::path::PathBuf::from(r"./examples/mock-dir")
    }

    pub fn get_mock_dir_1_path() -> std::path::PathBuf {
        std::path::PathBuf::from(r"./examples/mock-dir/mock-dir-1")
    }

    pub fn get_mock_file_1_path() -> std::path::PathBuf {
        std::path::PathBuf::from(r"./examples/mock-dir/mock-file-1.txt")
    }

    pub fn get_mock_file_1_1_path() -> std::path::PathBuf {
        std::path::PathBuf::from(r"./examples/mock-dir/mock-dir-1/mock-file-1-1.txt")
    }

    pub fn get_mock_file_2_path() -> std::path::PathBuf {
        std::path::PathBuf::from(r"./examples/mock-dir/mock-file-2.txt")
    }

    pub fn get_non_existing_path() -> std::path::PathBuf {
        std::path::PathBuf::from(r"./this/does/not/exist/42")
    }

    fn _get_path(args: &[&str]) -> std::path::PathBuf {
        args.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::test_util;
    use super::*;

    #[test]
    fn test_get_current_dir() {
        assert_eq!(get_current_dir().is_ok(), true);
    }

    #[test]
    fn test_get_read_dir() {
        _test_get_read_dir_with_file_path();
        _test_get_read_dir_with_dir_path();
    }

    fn _test_get_read_dir_with_file_path() {
        let mock_file_1_res = get_read_dir(&test_util::get_mock_file_1_path());
        assert_eq!(mock_file_1_res.is_err(), true);
        assert_eq!(mock_file_1_res.unwrap_err(), FsErr::PathIsNotDirErr)
    }

    fn _test_get_read_dir_with_dir_path() {
        let mock_dir_1_res = get_read_dir(&test_util::get_mock_dir_path());
        assert_eq!(mock_dir_1_res.is_ok(), true);
        assert_eq!(mock_dir_1_res.unwrap().count(), 3);

        let mock_dir_2_res = get_read_dir(&test_util::get_mock_dir_1_path());
        assert_eq!(mock_dir_2_res.is_ok(), true);
        assert_eq!(mock_dir_2_res.unwrap().count(), 1);
    }

    fn _test_get_read_dir_with_non_existing_path() {
        let mock_dir_1_res = get_read_dir(&test_util::get_non_existing_path());
        assert_eq!(mock_dir_1_res.is_err(), true);
        assert_eq!(mock_dir_1_res.unwrap_err(), FsErr::DirReadErr);
    }

    #[test]
    fn test_get_file_or_dir_name() {
        _test_get_file_or_dir_name_with_file_path();
        _test_get_file_or_dir_name_with_dir_path();
        _test_get_file_or_dir_name_with_non_existing_path();
    }

    fn _test_get_file_or_dir_name_with_file_path() {
        let mock_file_1_res = get_file_or_dir_name(&test_util::get_mock_file_1_path());
        assert_eq!(mock_file_1_res.is_ok(), true);
        assert_eq!(mock_file_1_res.unwrap(), "mock-file-1.txt");
    }

    fn _test_get_file_or_dir_name_with_dir_path() {
        let mock_dir_1_res = get_file_or_dir_name(&test_util::get_mock_dir_1_path());
        assert_eq!(mock_dir_1_res.is_ok(), true);
        assert_eq!(mock_dir_1_res.unwrap(), "mock-dir-1");
    }

    fn _test_get_file_or_dir_name_with_non_existing_path() {
        let non_existing_path_res = get_file_or_dir_name(&test_util::get_non_existing_path());
        assert_eq!(non_existing_path_res.is_ok(), true);
        assert_eq!(non_existing_path_res.unwrap(), "42");
    }
}
