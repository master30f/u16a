use std::str::Chars;

use crate::environment::Token;

pub struct Lexer<'a> {
    chars: Chars<'a>,
    current: Option<char>
}

impl<'a> Lexer<'a> {
    pub fn new(chars: Chars<'a>) -> Self {
        Self {
            chars,
            current: None
        }
    }

    fn advance(&mut self) -> Option<char> {
        self.current = self.chars.next();
        self.current
    }

    fn advance_and<T>(&mut self, item: T) -> T {
        self.advance();
        item
    }

    fn expect_current(&self) -> char {
        self.current.expect("FATAL: Current not present")
    }

    fn skip_whitespace(&mut self) -> Option<()> {
        while let Some(' ' | '\t' | '\r') = self.current {
            self.advance()?;
        }
        Some(())
    }

    fn skip_comment(&mut self) -> Option<()> {
        self.skip_whitespace()?;
        if self.current? == ';' {
            while match self.advance()? {
                '\n' => false,
                _ => true
            } { }
        }
        Some(())
    }

    fn make_word_or_label(&mut self) -> Token {
        let mut out = String::from(self.expect_current());
        
        while let Some(current) = self.advance() {
            if current.is_alphanumeric() {
                out.push(current);
            } else if current == ':' {
                self.advance();
                return Token::Label(out)
            } else {
                break;
            }
        }

        Token::Word(out)
    }

    fn make_number(&mut self, base: u32) -> Result<Token, String> {
        let mut out = String::from(self.expect_current());

        while let Some(current) = self.advance() {
            if current.is_numeric() {
                out.push(current);
            } else {
                break;
            }
        }

        let int = u16::from_str_radix(&out, base).map_err(|_| format!("Not a number: {}", &out))?;
        Ok(Token::Integer(int))
    }

    fn make_number_with_base(&mut self) -> Result<Token, String> {
        if let Some(current) = self.advance() {
            if current.is_alphabetic() {
                // FIXME: Convert to json-based solution
                self.make_number(
                    match current {
                        'b' => 2,
                        'o' |
                        'q' => 8,
                        'd' => 10,
                        'x' |
                        'h' => 16,
                        _ => return Err(format!("Unknown base identifier '{}'", current))
                    }
                )
            } else {
                Ok(Token::Integer(0))
            }
        } else {
            Ok(Token::Integer(0))
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, String>;

    fn next(&mut self) -> Option<Result<Token, String>> {
        if let None = self.current {
            self.advance();
        }

        self.skip_comment()?;
        self.skip_whitespace()?;

        let current = self.current?;

        Some(Ok(
            match current {
                '0'  => match self.make_number_with_base() {
                    Ok(token) => token,
                    Err(error) => return Some(Err(error))
                },
                ','  => self.advance_and(Token::Comma),
                '\n' => self.advance_and(Token::NewLine),
                _    => {
                    if current.is_alphabetic() {
                        self.make_word_or_label()
                    } else if current.is_numeric() {
                        match self.make_number(10) {
                            Ok(token) => token,
                            Err(error) => return Some(Err(error))
                        }
                    } else {
                        return self.advance_and(Some(Err(format!("Unknown symbol '{}'", current))))
                    }
                }
            }
        ))
    }
}