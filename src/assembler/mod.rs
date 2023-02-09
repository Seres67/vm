use crate::assembler::program_parser::{program, Program};
use crate::instruction::Opcode;
use nom::types::CompleteStr;

pub mod directive_parser;
pub mod instruction_parser;
pub mod label_parser;
pub mod opcode_parser;
pub mod operand_parser;
pub mod program_parser;
pub mod register_parser;

#[derive(Debug, PartialEq)]
pub enum Token {
    Op { code: Opcode },
    Register { register_number: u8 },
    Number { value: i32 },
    LabelDeclaration { name: String },
    LabelUsage { name: String },
    Directive { name: String },
}

#[derive(Debug)]
pub enum AssemblerPhase {
    First,
    Second,
}

#[derive(Debug)]
pub struct Assembler {
    pub phase: AssemblerPhase,
    pub symbols: SymbolTable,
}

#[derive(Debug)]
pub struct Symbol {
    name: String,
    offset: u32,
    symbol_type: SymbolType,
}

impl Symbol {
    pub fn new(name: String, symbol_type: SymbolType, offset: u32) -> Symbol {
        Symbol {
            name,
            symbol_type,
            offset,
        }
    }
}

#[derive(Debug)]
pub struct SymbolTable {
    symbols: Vec<Symbol>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable { symbols: vec![] }
    }

    pub fn add_symbol(&mut self, symbol: Symbol) {
        self.symbols.push(symbol);
    }

    pub fn symbol_value(&self, symbol_name: &str) -> Option<u32> {
        for symbol in &self.symbols {
            if symbol.name == symbol_name {
                return Option::from(symbol.offset);
            }
        }
        None
    }
}

#[derive(Debug)]
pub enum SymbolType {
    Label,
}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler {
            phase: AssemblerPhase::First,
            symbols: SymbolTable::new(),
        }
    }

    pub fn assemble(&mut self, raw: &str) -> Option<Vec<u8>> {
        match program(CompleteStr(raw)) {
            Ok((_remainder, program)) => {
                self.process_first_phase(&program);
                Some(self.process_second_phase(&program))
            }
            Err(e) => {
                println!("Error while assembling: {e:?}");
                None
            }
        }
    }

    fn process_first_phase(&mut self, p: &Program) {
        self.extract_labels(p);
        self.phase = AssemblerPhase::Second;
    }

    fn process_second_phase(&mut self, p: &Program) -> Vec<u8> {
        let mut program = vec![];
        for i in &p.instructions {
            let mut bytes = i.to_bytes(&self.symbols);
            program.append(&mut bytes);
        }
        program
    }

    fn extract_labels(&mut self, program: &Program) {
        let mut c = 0;
        for i in &program.instructions {
            if i.is_label() {
                if let Some(name) = i.label_name() {
                    let symbol = Symbol::new(name, SymbolType::Label, c);
                    self.symbols.add_symbol(symbol);
                };
            }
            c += 4;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_table() {
        let mut sym = SymbolTable::new();
        let new_symbol = Symbol::new("test".to_string(), SymbolType::Label, 12);
        sym.add_symbol(new_symbol);
        assert_eq!(sym.symbols.len(), 1);
        let v = sym.symbol_value("test");
        assert!(v.is_some());
        let v = v.unwrap();
        assert_eq!(v, 12);
        let v = sym.symbol_value("does_not_exist");
        assert!(v.is_none());
    }

    #[test]
    fn test_assemble_program() {
        let mut asm = Assembler::new();
        let test_string = "load $0 #100\nload $1 #1\nload $2 #0\ntest: inc $0\nneq $0 $2\njeq @test\nhlt";
        /*1 0 0 100
          1 1 0 1
          1 2 0 0
          17 0 0 0
          10 0 2 0
          15 0 0 0
          0
         */
       // let program = asm.assemble(test_string).unwrap();
       //  let mut vm = VM::new();
       //  assert_eq!(program.len(), 28);
       //  vm.add_bytes(program);
       //  assert_eq!(vm.program.len(), 28);
    }
}
