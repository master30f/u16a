use crate::{environment::{Token, Statement, Stream}, lexer::Lexer};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Option<Result<Token, String>>
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self {
            lexer,
            current: None
        }
    }

    fn advance(&mut self) -> Option<Result<Token, String>> {
        self.current = self.lexer.next();
        self.current.clone()
    }

    fn advance_and<T>(&mut self, item: T) -> T {
        self.advance();
        item
    }

    fn expect_current(&self) -> Result<Token, String> {
        self.current.clone().expect("FATAL: Current not present")
    }

    /*fn advance_newline(&mut self) -> Option<Result<Token, String>> {
        match self.advance() {
            Some(Ok(Token::NewLine)) => self.advance(),
            Some(Ok(token)) => {
                println!("{:?}", token);
                Some(Err(String::from("Expected newline")))
            },
            token => token
        }
    }*/

    fn newline(&mut self) -> Result<(), String> {
        match self.advance() {
            Some(Ok(Token::NewLine)) | None => Ok(()),
            _ => Err(String::from("Expected newline")),
        }
    }

    fn make_definition(&mut self, instruction: Option<u16>) -> Result<Statement, String> {
        let current = self.expect_current()?;
        let mut alu_zero: Option<bool> = None;
        
        if let Token::LeftSquareBrace = current {
            let negate = if let Token::Bang = self.advance().ok_or(String::from("fdf"))?? {
                self.advance();
                false
            } else {
                true
            };

            if let Token::AluZero = self.expect_current()? {
                if let Token::RightSquareBrace = self.advance().ok_or(String::from("AAA"))?? {
                    alu_zero = Some(negate);
                    self.advance();
                } else {
                    return Err(String::from("Expected mode closure"))
                }
            } else {
                return Err(String::from("Expected 'alu_zero'"))
            }
        }
        
        match &self.current {
            Some(Ok(Token::LeftCurlyBrace)) => {},
            Some(Ok(_)) | None => return Err(String::from("Expected definition block")),
            Some(Err(error)) => return Err(error.clone())
        }

        self.newline()?;

        let mut statements: Vec<Statement> = vec![];

        let mut brace_found: bool = false;

        self.advance();

        while let Some(token) = self.current.clone() {
            if let Token::RightCurlyBrace = token? {
                brace_found = true;
                self.advance();
                break;
            }
            statements.push(self.make_statement()?);
        }

        if !brace_found { return Err(String::from("Expected definition closure")) }

        Ok(Statement::Definition { instruction, alu_zero, statements })
    }
    
    fn make_action_compound(&mut self, stream: &mut Option<Stream>, flags: &mut Vec<Token>) -> Result<(), String> {
        let first = self.current.clone().ok_or(String::from("Invalid syntax"))??;

        match self.advance() {
            Some(Ok(Token::Arrow)) => {
                self.advance();

                let second = self.current.clone().ok_or(String::from("Invalid syntax"))??;
                        
                if !first.is_source() {
                    return Err(String::from("Invalid syntax"))
                }

                if !second.is_sink() {
                    return Err(String::from("Expected sink"))
                }

                *stream = Some(Stream { from: first, to: second });

                self.advance();
                Ok(())
            }
            None | Some(Ok(Token::NewLine | Token::Ampersand)) => {
                flags.push(first);
                Ok(())
            }
            Some(Err(error)) => Err(error),
            Some(Ok(token)) => {
                Err(String::from("Invalid syntax"))
            }
        }
    }

    fn make_action(&mut self) -> Result<Statement, String> {
        let mut stream: Option<Stream> = None;
        let mut flags: Vec<Token> = Vec::new();

        loop {
            self.make_action_compound(&mut stream, &mut flags)?;
            //println!("In make_action loop\n Current: {:?}", &self.current);
            match &self.current {
                Some(Ok(Token::NewLine)) | None => return Ok(Statement::Action { stream, flags }),
                Some(Ok(Token::Ampersand)) => { self.advance(); },
                Some(Err(error)) => return Err(error.clone()),
                _token => {
                    //println!("{:?}", token);
                    return Err(String::from("Invalid syntax"));
                }
            }
        }
    }

    fn make_statement(&mut self) -> Result<Statement, String> {
        let token = self.expect_current()?;

        let out = match token {
            Token::Instruction(instruction) => {
                self.advance();
                self.make_definition(Some(instruction))
            },
            Token::Comment(comment) => self.advance_and(Ok(Statement::Comment(comment))),
            Token::LeftSquareBrace => self.make_definition(None),
            _ => self.make_action()
        }?;

        //println!("{:?}", &self.current);

        match &self.current {
            Some(Ok(Token::NewLine)) | None => {
                //println!("Advancing newline");
                self.advance();
            },
            Some(Err(error)) => { return Err(error.clone()); },
            token => {
                //println!("{:?}", token);
                return Err(String::from("Statement: Expected newline"));
            }
        }

        //println!("Statement made.\nCurrent: {:?}", &self.current);

        Ok(out)
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements: Vec<Statement> = vec![];

        self.advance();

        while let Some(Ok(Token::NewLine)) = &self.current {
            self.advance();
        }

        while let Some(token) = self.current.clone() {
            //println!("in loop: {:?}", &token);
            statements.push(self.make_statement()?);

            while let Some(Ok(Token::NewLine)) = &self.current {
                self.advance();
            }
        }

        Ok(statements)
    }
}