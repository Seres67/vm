use nom::types::CompleteStr;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Opcode {
    HLT,
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
    JMP,
    JMPF,
    JMPB,
    EQ,
    NEQ,
    GT,
    LT,
    GE, // greater or equal
    LE,
    JEQ,
    ALLOC,
    INC,
    DEC,
    DJEQ,
    ILLEGAL, // Illegal
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0 => Opcode::HLT,
            1 => Opcode::LOAD,
            2 => Opcode::ADD,
            3 => Opcode::SUB,
            4 => Opcode::MUL,
            5 => Opcode::DIV,
            6 => Opcode::JMP,
            7 => Opcode::JMPF,
            8 => Opcode::JMPB,
            9 => Opcode::EQ,
            10 => Opcode::NEQ,
            11 => Opcode::GT,
            12 => Opcode::LT,
            13 => Opcode::GE,
            14 => Opcode::LE,
            15 => Opcode::JEQ,
            16 => Opcode::ALLOC,
            17 => Opcode::INC,
            18 => Opcode::DEC,
            19 => Opcode::DJEQ,
            _ => Opcode::ILLEGAL,
        }
    }
}

impl<'a> From<CompleteStr<'a>> for Opcode {
    fn from(value: CompleteStr<'a>) -> Self {
        let lowercase_value = value.to_lowercase();
        match CompleteStr(&lowercase_value) {
            CompleteStr("hlt") => Opcode::HLT,
            CompleteStr("load") => Opcode::LOAD,
            CompleteStr("add") => Opcode::ADD,
            CompleteStr("sub") => Opcode::SUB,
            CompleteStr("mul") => Opcode::MUL,
            CompleteStr("div") => Opcode::DIV,
            CompleteStr("jmp") => Opcode::JMP,
            CompleteStr("jmpf") => Opcode::JMPF,
            CompleteStr("jmpb") => Opcode::JMPB,
            CompleteStr("eq") => Opcode::EQ,
            CompleteStr("neq") => Opcode::NEQ,
            CompleteStr("gt") => Opcode::GT,
            CompleteStr("lt") => Opcode::LT,
            CompleteStr("ge") => Opcode::GE,
            CompleteStr("le") => Opcode::LE,
            CompleteStr("jeq") => Opcode::JEQ,
            CompleteStr("alloc") => Opcode::ALLOC,
            CompleteStr("inc") => Opcode::INC,
            CompleteStr("dec") => Opcode::DEC,
            CompleteStr("djeq") => Opcode::DJEQ,
            _ => Opcode::ILLEGAL,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction { opcode }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hlt() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode, Opcode::HLT);
    }

    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::new(Opcode::HLT);
        assert_eq!(instruction.opcode, Opcode::HLT);
    }
}
