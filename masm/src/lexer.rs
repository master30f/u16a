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

    fn make_comment(&mut self) -> Result<Token, String> {
        if self.advance().ok_or("Invalid syntax".to_string())? != '/' {
            return Err("Invalid syntax".to_string());
        }
        let mut out = String::new();
        while match self.advance() {
            Some('\n') | None => false,
            Some(character) => {
                out.push(character);
                true
            }
        } { }
        Ok(Token::Comment(out))
    }

    fn make_word(&mut self) -> Result<Token, String> {
        let mut out = String::from(self.expect_current());
        
        while let Some(current) = self.advance() {
            if current.is_alphanumeric() || current == '_' {
                out.push(current);
            } else {
                break;
            }
        }

        Token::try_from_word_string(&out)
    }

    fn make_instruction(&mut self) -> Result<Token, String> {
        let mut out = String::from(self.expect_current());
        
        while let Some(current) = self.advance() {
            if current == '_' {
                continue;
            }
            if current.is_numeric() {
                out.push(current);
            } else {
                break;
            }
        }

        Ok(Token::Instruction(u16::from_str_radix(out.as_str(), 2).map_err(|_| String::from("Invalid instruction"))?))
    }

    fn make_arrow(&mut self) -> Result<Token, String> {
        match self.advance() {
            Some('>') => Ok(self.advance_and(Token::Arrow)),
            _ => Err(String::from("Invalid token"))
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, String>;

    fn next(&mut self) -> Option<Result<Token, String>> {
        if let None = self.current {
            self.advance();
        }

        self.skip_whitespace()?;

        let current = self.current?;

        Some(Ok(
            match current {
                '\n' => self.advance_and(Token::NewLine),
                '[' => self.advance_and(Token::LeftSquareBrace),
                ']' => self.advance_and(Token::RightSquareBrace),
                '{' => self.advance_and(Token::LeftCurlyBrace),
                '}' => self.advance_and(Token::RightCurlyBrace),
                '&' => self.advance_and(Token::Ampersand),
                '!' => self.advance_and(Token::Bang),
                '/' => return Some(self.make_comment()),
                '-' => return Some(self.make_arrow()),
                _   => {
                    if current.is_numeric() {
                        let instruction = self.make_instruction();
                        match instruction {
                            Ok(instruction) => instruction,
                            Err(error) => return Some(Err(error))
                        }
                    } else if current.is_alphabetic() {
                        return Some(self.make_word())
                    } else {
                        return self.advance_and(Some(Err(format!("Unknown symbol '{}'", current))))
                    }
                }
            }
        ))
    }
}