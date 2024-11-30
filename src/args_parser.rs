use thiserror::Error;

pub struct ArgsParser<'a> {
    source: &'a str,
    cur_pos: usize,
}

impl<'a> ArgsParser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source, cur_pos: 0 }
    }
    pub fn get_processed_args(&mut self) -> Result<Vec<String>, ArgsParseError> {
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
        return cur;
    }

    fn unquoted_arg(&mut self) -> Result<String, ArgsParseError> {
        let start = self.cur_pos;
        while self.peek() != " " {
            if self.is_at_end(0) {
                break;
            }
            let _ = self.consume();
        }
        return Ok(self.source[start..self.cur_pos].to_owned());
    }

    fn double_quote_arg(&mut self) -> Result<String, ArgsParseError> {
        todo!()
    }

    fn single_quote_arg(&mut self) -> Result<String, ArgsParseError> {
        let _ = self.consume();
        let start = self.cur_pos;
        while self.peek() != "'" {
            if self.is_at_end(0) {
                return Err(ArgsParseError::UnterminatedQuote);
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
pub enum ArgsParseError {
    #[error("unterminated quote")]
    UnterminatedQuote,
}
