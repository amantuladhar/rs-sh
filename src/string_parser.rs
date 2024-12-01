use thiserror::Error;

pub struct StringParser<'a> {
    source: &'a str,
    cur_pos: usize,
}

impl<'a> StringParser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source, cur_pos: 0 }
    }
    pub fn get_processed_args(&mut self) -> Result<Vec<String>, StringParseError> {
        let mut args = Vec::new();
        while !self.is_at_end(0) {
            let cur = self.peek();
            let arg = match cur {
                "'" => self.single_quote_arg()?,
                "\"" => self.double_quote_arg()?,
                " " | "\t" => {
                    let _ = self.consume();
                    continue;
                }
                _ => self.unquoted_arg()?,
            };
            args.push(arg);
        }
        Ok(args)
    }

    fn consume(&mut self) -> &str {
        let cur = &self.source[self.cur_pos..self.cur_pos + 1];
        self.cur_pos += 1;
        cur
    }

    fn unquoted_arg(&mut self) -> Result<String, StringParseError> {
        let mut arg = String::new();
        loop {
            let p = self.peek();
            if self.is_at_end(0) {
                break;
            }
            match p {
                " " => break,
                "\\" => {
                    let _ = self.consume();
                    continue;
                }
                _ => arg.push_str(self.consume()),
            }
        }
        Ok(arg)
    }

    fn double_quote_arg(&mut self) -> Result<String, StringParseError> {
        let _ = self.consume(); // consume start "
        let mut arg = String::new();
        loop {
            let p = self.peek();
            if self.is_at_end(0) {
                break;
            }
            match p {
                "\"" => {
                    let _ = self.consume();
                    // If we don't see space yet, we need to continue parsing current arg
                    if self.peek() == " " {
                        break;
                    }
                }
                // TODO: Implement all special escape sequences
                // https://www.gnu.org/software/bash/manual/bash.html#Double-Quotes
                "\\" => {
                    let _ = self.consume();
                    match self.consume() {
                        "n" => arg.push_str("\\n"),
                        "\\" => arg.push_str("\\"),
                        "\"" => arg.push_str("\""),
                        e => {
                            arg.push_str("\\");
                            arg.push_str(e);
                        }
                    }
                    continue;
                }
                _ => arg.push_str(self.consume()),
            }
        }
        Ok(arg)
    }

    fn single_quote_arg(&mut self) -> Result<String, StringParseError> {
        let _ = self.consume();
        let start = self.cur_pos;
        while self.peek() != "'" {
            if self.is_at_end(0) {
                return Err(StringParseError::UnterminatedQuote);
            }
            let _ = self.consume();
        }
        let _ = self.consume();
        Ok(self.source[start..self.cur_pos - 1].to_owned())
    }

    fn peek(&self) -> &str {
        if self.is_at_end(0) {
            return "\0";
        }
        &self.source[self.cur_pos..self.cur_pos + 1]
    }

    fn is_at_end(&self, offset: usize) -> bool {
        (self.cur_pos + offset) >= self.source.len()
    }
}

#[derive(Error, Debug)]
pub enum StringParseError {
    #[error("unterminated quote")]
    UnterminatedQuote,
}
