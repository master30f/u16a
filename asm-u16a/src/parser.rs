use crate::{environment::{Token, Statement, Expression}, lexer::Lexer};

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

    fn end_statement(&mut self, statement: Statement) -> Result<Statement, String> {
        match &self.current {
            Some(Ok(Token::NewLine)) => self.advance_and(Ok(statement)),
            Some(_) => Err(String::from("Expected NL of EOF")),
            None => Ok(statement)
        }
    }

    fn make_expression(&mut self) -> Expression {
        match self.expect_current().expect("FATAL: Err variant") {
            Token::Word(value) => Expression::Identifier(value),
            Token::Integer(value) => Expression::Integer(value),
            _ => unreachable!()
        }
    }

    fn make_operation(&mut self, name: String) -> Result<Statement, String> {
        let mut arguments: Vec<Expression> = vec![];

        let mut expect_comma = false;
        let mut expect_argument = true;
        while let Some(token) = self.advance() {
            match token? {
                Token::Word(_) | Token::Integer(_) => {
                    if !expect_argument {
                        return Err(String::from("Arguments must be separated by commas"))
                    }
                    
                    arguments.push(self.make_expression());

                    expect_comma = true;
                    expect_argument = false;
                },
                Token::Comma => {
                    if !expect_comma {
                        return self.advance_and(Err(String::from("Commas can only separate arguments")))
                    }
                    expect_argument = true;
                    expect_comma = false;
                }
                _ => break
            }
        }

        self.end_statement(Statement::Operation { name, arguments })
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Result<Statement, String>;

    fn next(&mut self) -> Option<Result<Statement, String>> {
        if let None = self.current {
            self.advance();
        }

        while let Ok(Token::NewLine) = self.current.clone()? {
            self.advance();
        }

        let current = match self.current.clone()? {
            Ok(current) => current,
            Err(error) => return Some(Err(error))
        };

        Some(
            match current {
                Token::Label(name) => {
                    let statement = self.advance_and(Statement::Label(name));
                    self.end_statement(statement)
                },
                Token::Word(name)  => self.make_operation(name),
                _ => self.advance_and(Err(format!("Unexpected token: {:?}", current)))
            }
        )
    }
}