pub fn write_line(msg: String, w: &mut impl std::io::Write) {
    if writeln!(w, "{}", msg).is_err() {
        println!("Err");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_out;

    #[test]
    fn test_write() {
        let msg = "the answer is 42";
        let res = [msg, ""].join("\n");
        test_out!(res.as_bytes(), write_line, String::from(msg));
    }
}
