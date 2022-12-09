use asm_u16a::{lexer::Lexer, parser::Parser, compiler::Compiler};

fn main() {
    let lexer = Lexer::new("jmp\njpz\njpn\nhlt\nadd\nsub\ninc\ndec\nneg\nor\nand\nxor\nnot\nmov\nldr\nstr".chars());

    let parser = Parser::new(lexer);

    let mut statements = vec![];
    let mut failed = false;
    for statement in parser {
        match statement {
            Ok(statement) => statements.push(statement),
            Err(error) => {
                failed = true;
                println!("{:?}", error)
            }
        }
    }
    if failed {
        return
    }

    let mut compiler = Compiler::new(statements, 1);
    compiler.compile();
}
