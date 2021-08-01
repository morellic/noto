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
mod tests {
    use crate::assert_eq_bytes;
    use crate::util::test_util::mock;

    #[test]
    fn test_write_line() {
        let mut printer = mock::get_mock_printer();
        let line = "the answer is 42";
        let res = [line, ""].join("\n");
        printer.write_line(String::from(line));
        assert_eq_bytes!(printer.out, res.as_bytes());
        assert_eq_bytes!(printer.err_out, b"");
    }

    #[test]
    fn test_write_err_line() {
        let mut printer = mock::get_mock_printer();
        let err_line = "the err is 42";
        let res = [err_line, ""].join("\n");
        printer.write_err_line(String::from(err_line));
        assert_eq_bytes!(printer.out, b"");
        assert_eq_bytes!(printer.err_out, res.as_bytes());
    }
}
