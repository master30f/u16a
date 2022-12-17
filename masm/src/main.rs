use masm::{lexer::Lexer, parser::Parser, compiler::Compiler};
use std::fs;

fn main() {
    let content = fs::read_to_string("/home/zorby/src/u16a/masm/i.masm").unwrap();
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
            let mut compiler = Compiler::new(statements);

            compiler.compile();
        },
        Err(error) => println!("{}", error)
    }
}
