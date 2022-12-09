use std::collections::HashSet;

#[derive(Debug, Clone)]
pub enum Token {
    Word(String),
    Instruction(u8),
    Comment(String),
    LeftSquareBrace,
    RightSquareBrace,
    LeftCurlyBrace,
    RightCurlyBrace,
    Arrow,
    Ampersand,
    NewLine
}

#[derive(Debug, Clone)]
pub enum WordIndex {
    X,
    Y
}

#[derive(Debug, Clone)]
pub enum WordSource {
    Arg0,
    Arg1,
    Ip
}

impl TryFrom<String> for WordSource {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(match value.as_str() {
            "arg0" => Self::Arg0,
            "arg1" => Self::Arg1,
            "ip" => Self::Ip,
            _ => return Err(())
        })
    }
}

#[derive(Debug, Clone)]
pub enum ByteSource {
    WordSlice(WordSource, WordIndex),
    Ram
}

impl TryFrom<String> for ByteSource {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(match value.as_str() {
            "ram" => Self::Ram,
            _ => return Err(())
        })
    }
}

#[derive(Debug, Clone)]
pub enum WordSink {
    Addr,
    Arg0,
    Ip
}

impl TryFrom<String> for WordSink {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(match value.as_str() {
            "addr" => Self::Addr,
            "arg0" => Self::Arg0,
            "ip" => Self::Ip,
            _ => return Err(())
        })
    }
}

#[derive(Debug, Clone)]
pub enum ByteSink {
    WordSlice(WordSink, WordIndex),
    Inst,
    Args
}

impl TryFrom<String> for ByteSink {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(match value.as_str() {
            "inst" => Self::Inst,
            "args" => Self::Args,
            _ => return Err(())
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Flag {
    IC,
    EI
}

impl TryFrom<String> for Flag {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(match value.as_str() {
            "ic" => Self::IC,
            "ei" => Self::EI,
            _ => return Err(())
        })
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum ModeFlag {
    Byte,
    Word,
}

impl TryFrom<String> for ModeFlag {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(match value.as_str() {
            "byte" => Self::Byte,
            "word" => Self::Word,
            _ => return Err(())
        })
    }
}

#[derive(Debug, Clone)]
pub struct Mode {
    byte: bool,
    word: bool,
}

impl Mode {
    pub fn full() -> Self {
        Self { byte: true, word: true }
    }

    fn is_byte(&self) -> bool { self.byte }
    fn is_word(&self) -> bool { self.word }
}

impl From<HashSet<ModeFlag>> for Mode {
    fn from(flags: HashSet<ModeFlag>) -> Self {
        let mut byte = false;
        let mut word = false;

        for flag in flags {
            match flag {
                ModeFlag::Byte => byte = true,
                ModeFlag::Word => word = true,
            }
        }

        return Self { byte, word }
    }
}

#[derive(Debug, Clone)]
pub enum Action {
    OpenWordStream(WordSource, WordSink),
    OpenByteStream(ByteSource, ByteSink)
}

#[derive(Debug, Clone)]
pub enum Statement {
    Action(Option<Action>, HashSet<Flag>),
    Definition {
        instruction: Option<u8>,
        mode: Mode,
        statements: Vec<Statement>
    },
    Comment(String)
}