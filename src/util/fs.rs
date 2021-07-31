use crate::util::out;

pub fn get_current_dir(w: &mut impl std::io::Write) -> Option<std::path::PathBuf> {
    match std::env::current_dir() {
        Ok(dir) => Some(dir),
        Err(e) => {
            let msg = format!("Could not access the current directory!\n{}", e);
            out::write_line(msg, w);
            None
        }
    }
}

pub fn get_read_dir(p: &std::path::Path, w: &mut impl std::io::Write) -> Option<std::fs::ReadDir> {
    if p.is_file() {
        let msg = format!("{} is a file!", p.file_name().unwrap().to_str().unwrap());
        out::write_line(msg, w);
        return None;
    }

    match std::fs::read_dir(&p) {
        Ok(rd) => Some(rd),
        Err(e) => {
            let s = format!("Could not access the directory {}!\n{}", p.display(), e);
            out::write_line(s, w);
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::test_out_get_ret;
    use crate::util::test_util::mock;

    #[test]
    fn test_get_current_dir() {
        let pb = test_out_get_ret!(b"", get_current_dir);
        assert_eq!(pb.is_some(), true);
    }

    #[test]
    fn test_get_read_dir() {
        _test_get_read_dir_with_file_path();
        _test_get_read_dir_with_dir_path();
    }

    fn _test_get_read_dir_with_file_path() {
        let mock_file_1_out = b"mock-file-1.txt is a file!\n";
        let mock_file_1_ret =
            test_out_get_ret!(mock_file_1_out, get_read_dir, &mock::get_mock_file_1_path());
        assert_eq!(mock_file_1_ret.is_some(), false);
    }

    fn _test_get_read_dir_with_dir_path() {
        let mock_dir_1_out = b"";
        let mock_dir_1_ret =
            test_out_get_ret!(mock_dir_1_out, get_read_dir, &mock::get_mock_dir_1_path());
        assert_eq!(mock_dir_1_ret.is_some(), true);
        assert_eq!(mock_dir_1_ret.unwrap().count(), 2);

        let mock_dir_2_out = b"";
        let mock_dir_2_ret =
            test_out_get_ret!(mock_dir_2_out, get_read_dir, &mock::get_mock_dir_2_path());
        assert_eq!(mock_dir_2_ret.is_some(), true);
        assert_eq!(mock_dir_2_ret.unwrap().count(), 0);
    }
}
