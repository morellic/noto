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
    #[macro_export]
    macro_rules! assert_printer_outs {
        ($printer:expr, $out:expr, $err_out:expr) => {
            crate::util::printer::test_util::assert_eq_bytes(&$printer.out, &$out);
            crate::util::printer::test_util::assert_eq_bytes(&$printer.err_out, &$err_out);
        };
    }

    pub fn assert_eq_bytes(out: &[u8], expected_out: &[u8]) {
        // compare as str for a better fail message
        assert_eq!(
            std::str::from_utf8(&out).unwrap(),
            std::str::from_utf8(&expected_out).unwrap(),
            "\ngot left but expected right"
        );
    }

    pub fn get_mock_printer() -> crate::util::printer::Printer<Vec<u8>, Vec<u8>> {
        crate::util::printer::Printer {
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
        assert_printer_outs!(printer, out.as_bytes(), err_out.as_bytes());
    }

    #[test]
    fn test_write_err_line() {
        let mut printer = test_util::get_mock_printer();
        let line = "err 42";
        let out = "";
        let err_out = [line, ""].join("\n");
        printer.write_err_line(line.to_string());
        assert_printer_outs!(printer, out.as_bytes(), err_out.as_bytes());
    }
}
