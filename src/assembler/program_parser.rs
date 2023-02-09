use nom::types::CompleteStr;
use crate::assembler::directive_parser::directive;

use crate::assembler::instruction_parser::{instruction, AssemblerInstruction};
use crate::assembler::SymbolTable;

#[derive(Debug, PartialEq)]
pub struct Program {
    pub(crate) instructions: Vec<AssemblerInstruction>,
}

named!(pub program<CompleteStr, Program>,
    do_parse!(
        instructions: many1!(alt!(instruction | directive)) >>
        (
            Program {
                instructions
            }
        )
    )
);

impl Program {
    pub fn to_bytes(&self, symbols: &SymbolTable) -> Vec<u8> {
        let mut program = vec![];
        for instruction in &self.instructions {
            program.append(&mut instruction.to_bytes(symbols));
        }
        program
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_program() {
        let result = program(CompleteStr("load $0 #100\n"));
        assert!(result.is_ok());
        let (leftover, p) = result.unwrap();
        assert_eq!(leftover, CompleteStr(""));
        assert_eq!(1, p.instructions.len());
    }

    #[test]
    fn test_program_to_bytes() {
        let result = program(CompleteStr("load $0 #100\n"));
        assert!(result.is_ok());
        let (_, program) = result.unwrap();
        let bytecode = program.to_bytes(&SymbolTable { symbols: vec![] });
        assert_eq!(bytecode.len(), 4);
        println!("{bytecode:?}");
    }
}
