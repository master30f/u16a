use crate::environment::{Statement, Expression};

pub struct Compiler {
    statements: Vec<Statement>,
    index: usize,
    version: u8
}

fn interpret_expression(expression: &Expression) -> Vec<u8> {
    match expression {
        Expression::Identifier(name) => todo!(),
        Expression::Integer(value) => todo!()
    }
}

impl Compiler {
    pub fn new(statements: Vec<Statement>, version: u8) -> Self {
        Self {
            statements,
            index: 0,
            version
        }
    }

    fn current(&self) -> Option<&Statement> {
        Some(self.statements.get(self.index)?)
    }

    fn advance(&mut self) -> Option<&Statement> {
        self.index += 1;
        self.current()
    }

    fn expect_current(&self) -> &Statement {
        self.current().expect("FATAL: CURRENT NOT PRESENT")
    }

    fn compile_statement(&mut self) -> Result<Vec<u8>, String> {
        match self.expect_current() {
            Statement::Operation { name, arguments } => {
                let id: u8 = match name.as_str() {
                    "jmp" => 0b0000_0000,
                    "jpz" => 0b0000_0001,
                    "jpn" => 0b0000_0010,
                    "hlt" => 0b0000_0011,

                    "add" => 0b0001_0000,
                    "sub" => 0b0001_0001,
                    "inc" => 0b0001_0010,
                    "dec" => 0b0001_0011,
                    "neg" => 0b0001_0100,

                    "or"  => 0b0010_0000,
                    "and" => 0b0010_0001,
                    "xor" => 0b0010_0010,
                    "not" => 0b0010_0011,

                    "mov" => 0b0011_0000,

                    "ldr" => 0b0100_0000,
                    "str" => 0b0100_0001,

                    _     => return Err(String::from("Invalid operation"))
                };

                for argument in arguments {
                    interpret_expression(argument);
                }

                self.advance();
                println!("{}", id);
            },
            _ => todo!()
        }

        Ok(vec![])
    }

    pub fn compile(&mut self) -> Vec<u8> {
        while let Some(_) = self.current() {
            self.compile_statement();
        }
        vec![]
    }
}