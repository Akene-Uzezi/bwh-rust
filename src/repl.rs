use std::io::{self, BufRead, Write};

use crate::{lexer, token};
const PROMPT: &str = ">>";

pub fn start<R: BufRead, W: Write>(mut reader: R, mut writer: W) -> io::Result<()> {
    let mut line = String::new();
    loop {
        write!(writer, "{}", PROMPT)?;
        writer.flush()?;

        line.clear();
        let bytes_read = reader.read_line(&mut line)?;
        if bytes_read == 0 {
            return Ok(());
        }
        let input = &line.as_bytes();
        let mut l = lexer::Lexer::new(input.to_vec());
        loop {
            let tok = l.next_token();
            if tok.token_type == token::EOF {
                break;
            }
            writeln!(writer, "{:?}", tok)?;
        }
    }
}
