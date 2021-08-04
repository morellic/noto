pub struct Printer<Out: std::io::Write, ErrOut: std::io::Write> {
    pub out: Out,
    pub err_out: ErrOut,
}

impl<Out: std::io::Write, ErrOut: std::io::Write> Printer<Out, ErrOut> {
    pub fn write_line(&mut self, line: String) {
        if let Err(e) = writeln!(self.out, "{}", line) {
            self.write_err_line(format!("Could not write a line!\n{}", e));
        }
    }

    pub fn write_err_line(&mut self, line: String) {
        if let Err(e) = writeln!(self.err_out, "{}", line) {
            panic!("{}", e);
        }
    }
}

#[cfg(test)]
pub mod test_util {
    use super::Printer;

    pub fn get_mock_printer() -> Printer<Vec<u8>, Vec<u8>> {
        Printer {
            out: Vec::new(),
            err_out: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::test_util;
    use crate::assert_printer_outs;

    #[test]
    fn test_write_line() {
        let mut printer = test_util::get_mock_printer();
        let line = "answer 42";
        let out = [line, ""].join("\n");
        let err_out = "";
        printer.write_line(line.to_string());
        assert_printer_outs!(printer, out, err_out);
    }

    #[test]
    fn test_write_err_line() {
        let mut printer = test_util::get_mock_printer();
        let line = "err 42";
        let out = "";
        let err_out = [line, ""].join("\n");
        printer.write_err_line(line.to_string());
        assert_printer_outs!(printer, out, err_out);
    }
}
