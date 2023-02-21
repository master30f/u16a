#[derive(Debug, Clone)]
pub enum Token {
    ZX,
    NX,
    ZY,
    NY,
    F,
    NO,
    IC,
    EI,
    RAM,
    IP,
    RX,
    RY,
    RZ,
    ALU,
    ADDR,
    INST,
    AluZero,
    Instruction(u16),
    Comment(String),
    LeftSquareBrace,
    RightSquareBrace,
    LeftCurlyBrace,
    RightCurlyBrace,
    Arrow,
    Ampersand,
    NewLine,
    Bang
}

impl Token {
    pub fn try_from_word_string(word: &str) -> Result<Self, String> {
        Ok(match word {
            "zx"   => Token::ZX,
            "nx"   => Token::NX,
            "zy"   => Token::ZY,
            "ny"   => Token::NY,
            "f"    => Token::F,
            "no"   => Token::NO,
            "ic"   => Token::IC,
            "ei"   => Token::EI,
            "ram"  => Token::RAM,
            "ip"   => Token::IP,
            "rx"   => Token::RX,
            "ry"   => Token::RY,
            "rz"   => Token::RZ,
            "alu"  => Token::ALU,
            "addr" => Token::ADDR,
            "inst" => Token::INST,
            "alu_zero" => Token::AluZero,
            _ => {
                return Err(String::from("Invalid word"))
            }
        })
    }

    pub fn is_source(&self) -> bool {
        match self {
            | Self::RAM
            | Self::IP
            | Self::RX
            | Self::RY
            | Self::RZ
            | Self::ALU => true,
            _ => false
        }
    }

    pub fn is_sink(&self) -> bool {
        match self {
            | Self::RAM
            | Self::IP
            | Self::RX
            | Self::RY
            | Self::RZ
            | Self::INST
            | Self::ADDR => true,
            _ => false
        }
    }
}

#[derive(Debug, Clone)]
pub struct Stream {
    pub from: Token,
    pub to: Token
}

#[derive(Debug, Clone)]
pub enum Statement {
    Action {
        stream: Option<Stream>,
        flags: Vec<Token>
    },
    Definition {
        instruction: Option<u16>,
        alu_zero: Option<bool>,
        statements: Vec<Statement>
    },
    Comment(String)
}