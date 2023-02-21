use std::{collections::HashMap, hash::Hash};

use crate::{environment::{Statement, Stream, Token}};

type Channel = Vec<u16>;

#[derive(Debug)]
struct Instruction {
    channels: Vec<Channel>
}

pub struct Compiler {
    alu_zero_ptr: u8,
    alu_non_zero_ptr: u8,
    out: HashMap<u16, u16>
}

fn make_micro_index(instruction_code: &u16, ptr: u8, alu_zero: bool) -> Result<u16, String> {
    if ptr > 8 {
        return Err(String::from("Pointer out of bounds"))
    }

    Ok((instruction_code << 4) | ((ptr as u16) << 1) | (alu_zero as u16))
}

fn action_to_microinstructions(stream: &Option<Stream>, flags: &Vec<Token>) -> u16 {
    let mut out: u16 = 0;

    for flag in flags {
        out |= match flag {
            Token::ZX => 0b00_0000_0000_000001,
            Token::NX => 0b00_0000_0000_000010,
            Token::ZY => 0b00_0000_0000_000100,
            Token::NY => 0b00_0000_0000_001000,
            Token::F  => 0b00_0000_0000_010000,
            Token::NO => 0b00_0000_0000_100000,
            Token::IC => 0b01_0000_0000_000000,
            Token::EI => 0b10_0000_0000_000000,
            _         => unreachable!()
        }
    }

    if let Some(stream) = stream {
        out |= (match stream.from {
            Token::RAM => 1,
            Token::IP  => 2,
            Token::RX  => 3,
            Token::RY  => 4,
            Token::RZ  => 5,
            Token::ALU => 6,
            _          => unreachable!()
        } << 6);

        out |= (match stream.to {
            Token::RAM  => 1,
            Token::ADDR => 2,
            Token::INST => 3,
            Token::IP   => 4,
            Token::RX   => 5,
            Token::RY   => 6,
            Token::RZ   => 7,
            _           => unreachable!()
        } << 10);
    }

    return out;
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            alu_zero_ptr: 0,
            alu_non_zero_ptr: 0,
            out: HashMap::new()
        }
    }

    fn compile_definition(&mut self, instruction_code: &u16, alu_zero: &Option<bool>, statements: &Vec<Statement>) -> Result<(), String> {
        for statement in statements {
            match statement {
                Statement::Action { stream, flags } => {
                    let inst = action_to_microinstructions(stream, flags);

                    match alu_zero {
                        Some(true) => {
                            self.out.insert(make_micro_index(instruction_code, self.alu_zero_ptr, true)?, inst);
                            self.alu_zero_ptr += 1;
                        }
                        Some(false) => {
                            self.out.insert(make_micro_index(instruction_code, self.alu_non_zero_ptr, false)?, inst);
                            self.alu_non_zero_ptr += 1;
                        }
                        None => {
                            self.out.insert(make_micro_index(instruction_code, self.alu_zero_ptr, true)?, inst);
                            self.alu_zero_ptr += 1;
                            self.out.insert(make_micro_index(instruction_code, self.alu_non_zero_ptr, false)?, inst);
                            self.alu_non_zero_ptr += 1;
                        }
                    }
                }
                Statement::Definition { instruction: _, alu_zero, statements } => {
                    self.compile_definition(instruction_code, alu_zero, statements)?;
                },
                Statement::Comment(_) => {}
            }
        }

        Ok(())
    }

    pub fn compile(&mut self, statements: Vec<Statement>) -> Result<(), String> {
        for statement in &statements {
            match statement {
                Statement::Definition { instruction: Some(instruction_code), alu_zero: None, statements } => {
                    let inst0 = action_to_microinstructions(&Some(Stream { from: Token::IP,  to: Token::ADDR }), &vec![]);
                    let inst1 = action_to_microinstructions(&Some(Stream { from: Token::RAM, to: Token::INST }), &vec![Token::IC]);

                    self.out.insert(make_micro_index(instruction_code, 0, false)?, inst0);
                    self.out.insert(make_micro_index(instruction_code, 0, true)?, inst0);
                    self.out.insert(make_micro_index(instruction_code, 1, false)?, inst1);
                    self.out.insert(make_micro_index(instruction_code, 1, true)?, inst1);

                    self.alu_zero_ptr = 2;
                    self.alu_non_zero_ptr = 2;

                    self.compile_definition(instruction_code, &None, statements)?;
                },
                Statement::Definition { instruction: _, alu_zero: _, statements: _ } => {
                    return Err(String::from("Definitions in the global scope must have an instruction defined."))
                },
                Statement::Comment(_) => (),
                _ => return Err(String::from("The global scope may only include definitions and comments."))
            }
        }

        let mut array: Vec<u16> = vec![0; 65536];

        for (key, value) in &self.out {
            array[key.clone() as usize] = value.clone();
        }

        let mut actual_out = String::from("addr/data: 16 16\n");

        let mut amount: u32 = 0;
        let mut prev_item: u16 = array[0];
        for item in array {
            if prev_item != item {
                let mut add = if amount > 1 {
                    format!("{}*", amount)
                } else { String::new() };
                add += &format!("{:x} ", prev_item);
                actual_out += &add;
                prev_item = item;
                amount = 0;
            }
            amount += 1;
        }
        let mut add = if amount > 1 {
            format!("{}*", amount)
        } else { String::new() };
        add += &format!("{:x}", prev_item);

        println!("{}", actual_out);

        Ok(())
    }
}