use crate::action::ActionParams;
use crate::util::fs;
use crate::util::printer;

pub fn ls(mut params: ActionParams) {
    ls_path(&params.path, &mut params.printer);
}

fn ls_path<Out: std::io::Write, ErrOut: std::io::Write>(
    path: &std::path::Path,
    mut printer: &mut printer::Printer<Out, ErrOut>,
) {
    if let Some(rd) = fs::get_read_dir(&path, &mut printer) {
        rd.flatten()
            .for_each(|entry| ls_entry(&entry.path(), &mut printer));
    }
}

fn ls_entry<Out: std::io::Write, ErrOut: std::io::Write>(
    path: &std::path::Path,
    mut printer: &mut printer::Printer<Out, ErrOut>,
) {
    if let Some(name) = fs::get_file_or_dir_name(&path, &mut printer) {
        printer.write_line(name);
    }
}

#[cfg(test)]
mod tests {
    use super::ls_entry;
    use super::ls_path;

    use crate::assert_outs;
    use crate::util::test_util::mock;

    #[test]
    fn test_ls() {
        assert_outs!(
            b"mock-dir-1\nmock-dir-2\nmock-file-1.txt\nmock-file-2.txt\n",
            b"",
            ls_path,
            &mock::get_mock_dir_path()
        );

        assert_outs!(
            b"mock-dir-1-1\nmock-file-1-1.txt\n",
            b"",
            ls_path,
            &mock::get_mock_dir_1_path()
        );

        assert_outs!(b"", b"", ls_path, &mock::get_mock_dir_2_path());

        assert_outs!(b"", b"", ls_path, &mock::get_mock_dir_1_1_path());
    }

    #[test]
    fn test_ls_entry() {
        assert_outs!(b"mock-dir-1\n", b"", ls_entry, &mock::get_mock_dir_1_path());

        assert_outs!(
            b"mock-file-1.txt\n",
            b"",
            ls_entry,
            &mock::get_mock_file_1_path()
        );

        assert_outs!(
            b"mock-file-2.txt\n",
            b"",
            ls_entry,
            &mock::get_mock_file_2_path()
        );

        assert_outs!(
            b"mock-file-1-1.txt\n",
            b"",
            ls_entry,
            &mock::get_mock_file_1_1_path()
        );
    }
}
