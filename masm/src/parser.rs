use std::collections::HashSet;

use crate::{environment::{Token, Statement, ModeFlag, Mode, Action, Flag, WordSource, WordSink, ByteSource, ByteSink, FlagMap}, lexer::Lexer};

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

    fn make_definition(&mut self, instruction: Option<u8>) -> Result<Statement, String> {
        let current = self.expect_current()?;
        let mut mode_flags: HashSet<ModeFlag> = HashSet::new();
        
        if let Token::LeftSquareBrace = current {
            while let Some(token) = self.advance() {
                if let Token::Word(value) = token? {
                    let flag = ModeFlag::try_from(value).map_err(|_| String::from("Unknown mode flag"))?;
                    
                    if mode_flags.contains(&flag) {
                        return Err(String::from("Duplicate mode flag"))
                    }
                    
                    mode_flags.insert(flag);
                    
                    let token = self.advance().ok_or(String::from("Expected mode closure"))??;
                    
                    if let Token::Ampersand = token {
                        continue;
                    }
                    
                    if let Token::RightSquareBrace = token {
                        self.advance();
                        break;
                    }
                    
                    return Err(String::from("Expected mode closure"))
                } else {
                    return Err(String::from("Expected mode flag"))
                }
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

        let mode = if mode_flags.len() == 0 {
            Mode::full()
        } else {
            Mode::from(mode_flags)
        };

        Ok(Statement::Definition { instruction, mode, statements })
    }
    
    fn make_action_compound(&mut self, action: &mut Option<Action>, flags: &mut Vec<Flag>) -> Result<(), String> {
        let token = self.current.clone().ok_or(String::from("Invalid syntax"))??;
        if let Token::Word(first) = token {
            match self.advance() {
                Some(Ok(Token::Arrow)) => {
                    match self.advance() {
                        Some(Ok(Token::Word(second))) => {
                            //println!("OpenStream\n Source: {:?}\n Sink: {:?}", &first, &second);
                            match WordSource::try_from(first.clone()) {
                                Ok(source) => {
                                    let sink = WordSink::try_from(second).map_err(|_| String::from("Invalid sink"))?;
                                    *action = Some(Action::OpenWordStream(source, sink));
                                },
                                Err(_) => {
                                    let source = ByteSource::try_from(first).map_err(|_| String::from("Invalid source"))?;
                                    let sink = ByteSink::try_from(second).map_err(|_| String::from("Invalid sink"))?;
                                    *action = Some(Action::OpenByteStream(source, sink));
                                }
                            }
                            self.advance();
                            Ok(())
                        }
                        _ => {
                            Err(String::from("Expected sink"))
                        }
                    }
                }
                None | Some(Ok(Token::NewLine | Token::Ampersand)) => {
                    flags.push(Flag::try_from(first).map_err(|_| String::from("Invalid flag"))?);
                    Ok(())
                }
                Some(Err(error)) => Err(error),
                Some(Ok(token)) => {
                    //println!("{:?}", token);
                    Err(String::from("Invalid syntax"))
                }
            }
        } else {
            Err(String::from("Invalid syntax"))
        }
    }

    fn make_action(&mut self) -> Result<Statement, String> {
        let mut action: Option<Action> = None;
        let mut flags: Vec<Flag> = Vec::new();

        loop {
            self.make_action_compound(&mut action, &mut flags)?;
            //println!("In make_action loop\n Current: {:?}", &self.current);
            match &self.current {
                Some(Ok(Token::NewLine)) | None => return Ok(Statement::Action(action, FlagMap::from(flags))),
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
            Token::Word(_) => self.make_action(),
            _ => {
                //println!("{:?}", &token);
                Err(String::from("Expected statement"))
            }
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