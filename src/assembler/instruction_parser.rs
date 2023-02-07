use crate::assembler::label_parser::label_declaration;
use crate::assembler::opcode_parser::opcode;
use crate::assembler::operand_parser::integer_operand;
use crate::assembler::operand_parser::operand;
use crate::assembler::register_parser::register;
use crate::assembler::Token;
use nom::multispace;
use nom::types::CompleteStr;

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    pub opcode: Option<Token>,
    pub label: Option<Token>,
    pub directive: Option<Token>,
    pub operand1: Option<Token>,
    pub operand2: Option<Token>,
    pub operand3: Option<Token>,
}

impl AssemblerInstruction {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut results: Vec<u8> = vec![];
        if let Some(ref token) = self.opcode {
            match token {
                Token::Op { code } => {
                    let b: u8 = *code as u8;
                    results.push(b);
                },
                _ => {
                    println!("Non-opcode found in opcode field");
                }
            }
        }
        for operand in [&self.operand1, &self.operand2, &self.operand3].iter().copied().flatten() {
                AssemblerInstruction::extract_operand(operand, &mut results);
        }
        while results.len() < 4 {
            results.push(0);
        }
        results
    }

    fn extract_operand(t: &Token, results: &mut Vec<u8>) {
        match t {
            Token::Register { register_number } => results.push(*register_number),
            Token::Number { value } => {
                let converted = *value as u16;
                let byte1 = converted;
                let byte2 = converted >> 8;
                results.push(byte2 as u8);
                results.push(byte1 as u8);
            }
            _ => {
                println!("Opcode found in operand field");
                std::process::exit(1);
            }
        }
    }
}

named!(pub instruction_one<CompleteStr, AssemblerInstruction>,
    do_parse!(
        opcode: opcode >>
        register1: register >>
        register2: register >>
        register3: register >>
        (
            AssemblerInstruction {
                opcode: Some(opcode),
                directive: None,
                label: None,
                operand1: Some(register1),
                operand2: Some(register2),
                operand3: Some(register3)
            }
        )
    )
);

named!(pub instruction_two<CompleteStr, AssemblerInstruction>,
    do_parse!(
        opcode: opcode >>
        register: register >>
        integer: integer_operand >>
        (
            AssemblerInstruction{
                opcode: Some(opcode),
                directive: None,
                label: None,
                operand1: Some(register),
                operand2: Some(integer),
                operand3: None
            }
        )
    )
);

named!(pub instruction_three<CompleteStr, AssemblerInstruction>,
    do_parse!(
        opcode: opcode >>
        register: register >>
        (
            AssemblerInstruction{
                opcode: Some(opcode),
                directive: None,
                label: None,
                operand1: Some(register),
                operand2: None,
                operand3: None
            }
        )
    )
);

named!(pub instruction_four<CompleteStr, AssemblerInstruction>,
    do_parse!(
        opcode: opcode >>
        opt!(multispace) >>
        (
            AssemblerInstruction {
                opcode: Some(opcode),
                directive: None,
                label: None,
                operand1: None,
                operand2: None,
                operand3: None
            }
        )
    )
);

named!(pub instruction<CompleteStr, AssemblerInstruction>,
    do_parse!(
        instruction: alt!(
            instruction_one |
            instruction_two |
            instruction_three |
            instruction_four
        ) >>
        (
            instruction
        )
    )
);

named!(instruction_combined<CompleteStr, AssemblerInstruction>,
    do_parse!(
        label: opt!(label_declaration) >>
        opcode: opcode >>
        operand1: opt!(operand) >>
        operand2: opt!(operand) >>
        operand3: opt!(operand) >>
        (
            AssemblerInstruction {
                opcode: Some(opcode),
                label,
                directive: None,
                operand1,
                operand2,
                operand3,
            }
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::Opcode;

    #[test]
    fn test_parse_instruction_form_one() {
        let result = instruction(CompleteStr("load $0 #100\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Option::from(Token::Op { code: Opcode::LOAD }),
                    directive: None,
                    label: None,
                    operand1: Some(Token::Register { register_number: 0 }),
                    operand2: Some(Token::Number { value: 100 }),
                    operand3: None
                }
            ))
        );
    }
}
