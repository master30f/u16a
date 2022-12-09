#[derive(Debug, Clone)]
pub enum Token {
    Word(String),
    Label(String),
    Integer(u16),
    Comma,
    NewLine
}

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(String),
    Integer(u16)
}

#[derive(Debug, Clone)]
pub enum Statement {
    Label(String),
    Operation{name: String, arguments: Vec<Expression>}
}