use crate::action::Action;
use crate::action::ActionErr;
use crate::cli;
use crate::util::fs_util;
use crate::util::printer;

pub struct LsAction<Out: std::io::Write, ErrOut: std::io::Write> {
    path: std::path::PathBuf,
    printer: printer::Printer<Out, ErrOut>,
}

impl<Out: std::io::Write, ErrOut: std::io::Write> LsAction<Out, ErrOut> {
    fn ls(&mut self) {
        match fs_util::get_read_dir(&self.path) {
            Ok(rd) => {
                for entry in rd.flatten() {
                    self.ls_entry(&entry.path())
                }
            }
            Err(e) => self.printer.write_err_line(format!("{}", e)),
        }
    }

    fn ls_entry(&mut self, path: &std::path::Path) {
        match fs_util::get_file_or_dir_name(&path) {
            Ok(name) => self.printer.write_line(name),
            Err(e) => self.printer.write_err_line(format!("{}", e)),
        }
    }
}

impl<Out: std::io::Write, ErrOut: std::io::Write> Action<Out, ErrOut> for LsAction<Out, ErrOut> {
    fn new(args: cli::Args, mut printer: printer::Printer<Out, ErrOut>) -> Result<Self, ActionErr> {
        match fs_util::get_path_or_current_dir(args.path) {
            Ok(path) => Ok(LsAction { path, printer }),
            Err(e) => {
                printer.write_err_line(format!("{}", e));
                Err(ActionErr::CreateErr)
            }
        }
    }

    fn run(&mut self) {
        self.ls();
    }
}

#[cfg(test)]
pub mod test_util {
    use super::LsAction;
    use crate::util::printer;

    pub fn get_mock_ls_action(path: std::path::PathBuf) -> LsAction<Vec<u8>, Vec<u8>> {
        LsAction {
            path,
            printer: printer::test_util::get_mock_printer(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::test_util::get_mock_ls_action;
    use crate::assert_printer_outs;
    use crate::util::fs_util;

    #[test]
    fn test_ls() {
        _test_ls(
            fs_util::test_util::get_mock_dir_path(),
            "mock-dir-1\nmock-dir-2\nmock-file-1.txt\nmock-file-2.txt\n",
            "",
        );
        _test_ls(
            fs_util::test_util::get_mock_dir_1_path(),
            "mock-dir-1-1\nmock-file-1-1.txt\n",
            "",
        );
        _test_ls(fs_util::test_util::get_mock_dir_2_path(), "", "");
        _test_ls(fs_util::test_util::get_mock_dir_1_1_path(), "", "");
    }

    fn _test_ls(path: std::path::PathBuf, expected_out: &str, expected_err_out: &str) {
        let mut ls_action = get_mock_ls_action(path);
        ls_action.ls();
        assert_printer_outs!(ls_action.printer, expected_out, expected_err_out);
    }

    #[test]
    fn test_ls_entry() {
        _test_ls_entry(
            &fs_util::test_util::get_mock_dir_1_path(),
            "mock-dir-1\n",
            "",
        );
        _test_ls_entry(
            &fs_util::test_util::get_mock_file_1_path(),
            "mock-file-1.txt\n",
            "",
        );
        _test_ls_entry(
            &fs_util::test_util::get_mock_file_2_path(),
            "mock-file-2.txt\n",
            "",
        );
        _test_ls_entry(
            &fs_util::test_util::get_mock_file_1_1_path(),
            "mock-file-1-1.txt\n",
            "",
        );
    }

    fn _test_ls_entry(path: &std::path::PathBuf, expected_out: &str, expected_err_out: &str) {
        let mut ls_action = get_mock_ls_action(fs_util::test_util::get_mock_dir_path());
        ls_action.ls_entry(&path);
        assert_printer_outs!(ls_action.printer, expected_out, expected_err_out);
    }
}
