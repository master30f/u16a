use crate::parser::Parser;

struct Compiler<'a> {
    parser: Parser<'a>
}

impl<'a> Compiler<'a> {
    fn new(parser: Parser<'a>) -> Self {
        Self { parser }
    }

    fn compile(&mut self) -> Result<(), String> {
        let statements = self.parser.parse()?;

        Ok(())
    }
}