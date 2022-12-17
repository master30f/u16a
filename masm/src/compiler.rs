use std::collections::HashMap;

use crate::{parser::Parser, environment::{ByteSource, ByteSink, FlagMap, Statement, Action, WordIndex, Mode}};

#[derive(Debug)]
struct Bus {
    source: ByteSource,
    sink: ByteSink
}

#[derive(Debug)]
struct Microinstruction {
    bus0: Option<Bus>,
    bus1: Option<Bus>,
    flags: FlagMap
}

impl From<(Option<Action>, FlagMap)> for Microinstruction {
    fn from(other: (Option<Action>, FlagMap)) -> Self {
        let action = other.0;
        let flags = other.1;

        let mut bus0: Option<Bus> = None;
        let mut bus1: Option<Bus> = None;

        match action {
            Some(Action::OpenByteStream(source, sink)) => {
                bus0 = Some(Bus { source, sink });
            },
            Some(Action::OpenWordStream(source, sink)) => {
                let source0 = ByteSource::WordSlice(source.clone(), WordIndex::X);
                let source1 = ByteSource::WordSlice(source.clone(), WordIndex::Y);

                let sink0 = ByteSink::WordSlice(sink.clone(), WordIndex::X);
                let sink1 = ByteSink::WordSlice(sink.clone(), WordIndex::Y);

                bus0 = Some(Bus { source: source0, sink: sink0 });
                bus1 = Some(Bus { source: source1, sink: sink1 });
            },
            None => ()
        }

        Self { bus0, bus1, flags }
    }
}

type Channel = Vec<Microinstruction>;

#[derive(Debug)]
struct Instruction {
    channels: Vec<Channel>
}

pub struct Compiler {
    statements: Vec<Statement>
}

impl Compiler {
    pub fn new(statements: Vec<Statement>) -> Self {
        Self { statements }
    }

    fn compile_definition(byte_channel: &mut Channel, word_channel: &mut Channel, mode: &Mode, statements: &Vec<Statement>) {
        for statement in statements {
            match statement {
                Statement::Action(action, flags) => {
                    if mode.is_byte() {
                        byte_channel.push(Microinstruction::from((action.clone(), flags.clone())));
                    }
                    if mode.is_word() {
                        word_channel.push(Microinstruction::from((action.clone(), flags.clone())));
                    }
                },
                Statement::Comment(_) => (),
                Statement::Definition { instruction, mode, statements } => {
                    Self::compile_definition(byte_channel, word_channel, mode, statements)
                }
            }
        }
    }

    pub fn compile(&mut self) -> Result<(), String> {
        let mut instructions: HashMap<u8, Instruction> = HashMap::new();

        for statement in &self.statements {
            match statement {
                Statement::Definition { instruction: Some(instruction_code), mode, statements } => {
                    let mut byte_channel = Channel::new();
                    let mut word_channel = Channel::new();

                    Self::compile_definition(&mut byte_channel, &mut word_channel, mode, statements);

                    let instruction = Instruction {
                        channels: vec![byte_channel, word_channel]
                    };

                    instructions.insert(instruction_code.clone(), instruction);
                },
                Statement::Definition { instruction: _, mode: _, statements: _ } => {
                    return Err(String::from("Definitions in the global scope must have an instruction defined."))
                },
                Statement::Comment(_) => (),
                _ => return Err(String::from("The global scope may only include definitions and comments."))
            }
        }

        println!("{:?}", instructions);

        Ok(())
    }
}