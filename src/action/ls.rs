use crate::util::fs;
use crate::util::out;

pub fn ls(p: &std::path::Path, w: &mut impl std::io::Write) {
    if let Some(rd) = fs::get_read_dir(p, w) {
        for entry in rd.flatten() {
            ls_entry(&entry.path(), w);
        }
    }
}

fn ls_entry(p: &std::path::Path, w: &mut impl std::io::Write) {
    let entry_name = p.file_name().unwrap().to_str().unwrap();
    out::write_line(entry_name.to_string(), w);
}

#[cfg(test)]
mod tests {
    use super::ls;
    use super::ls_entry;

    use crate::test_out;
    use crate::util::test_util::mock;

    #[test]
    fn test_ls() {
        let mock_dir_out = b"mock-dir-1\nmock-dir-2\nmock-file-1.txt\nmock-file-2.txt\n";
        test_out!(mock_dir_out, ls, &mock::get_mock_dir_path());

        let mock_dir_1_out = b"mock-dir-1-1\nmock-file-1-1.txt\n";
        test_out!(mock_dir_1_out, ls, &mock::get_mock_dir_1_path());

        let mock_dir_2_out = b"";
        test_out!(mock_dir_2_out, ls, &mock::get_mock_dir_2_path());

        let mock_dir_1_1_out = b"";
        test_out!(mock_dir_1_1_out, ls, &mock::get_mock_dir_1_1_path());
    }

    #[test]
    fn test_ls_entry() {
        let mock_dir_1_out = b"mock-dir-1\n";
        test_out!(mock_dir_1_out, ls_entry, &mock::get_mock_dir_1_path());

        let mock_file_1_out = b"mock-file-1.txt\n";
        test_out!(mock_file_1_out, ls_entry, &mock::get_mock_file_1_path());

        let mock_file_2_out = b"mock-file-2.txt\n";
        test_out!(mock_file_2_out, ls_entry, &mock::get_mock_file_2_path());

        let mock_file_1_1_out = b"mock-file-1-1.txt\n";
        test_out!(mock_file_1_1_out, ls_entry, &mock::get_mock_file_1_1_path());
    }
}
