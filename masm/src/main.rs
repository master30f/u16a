use masm::{lexer::Lexer, parser::Parser, compiler::Compiler};
use std::{fs, path::Path};

fn main() {
    let path = Path::new("./instructions.masm");
    let content = fs::read_to_string(path).unwrap();
    //println!("{}", content);

    let lexer = Lexer::new(content.chars());

    /*for token in lexer {
        println!("{:?}", token);
    }*/

    let mut parser = Parser::new(lexer);

    let statements = parser.parse();

    match statements {
        Ok(statements) => {
            //println!("{:?}", statements)
            let mut compiler = Compiler::new();

            compiler.compile(statements);
        },
        Err(error) => println!("{}", error)
    }
}
