use crate::instruction::Opcode;

#[derive(Default)]
pub struct VM {
    pub registers: [i32; 32],
    pc: usize,
    pub program: Vec<u8>,
    heap: Vec<u8>,
    remainder: u32,
    equal_flag: bool,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            pc: 0,
            program: vec![],
            heap: vec![],
            remainder: 0,
            equal_flag: false,
        }
    }

    pub fn add_byte(&mut self, byte: u8) {
        self.program.push(byte);
    }

    pub fn run(&mut self) {
        'running: loop {
            if !self.execute_instruction() {
                break 'running;
            }
        }
    }

    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return false;
        }
        match self.decode_opcode() {
            Opcode::HLT => {
                println!("Stopping VM...");
                return false;
            }
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits();
                self.registers[register] = number as i32;
            }
            Opcode::ADD => {
                let lhs = self.registers[self.next_8_bits() as usize];
                let rhs = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = lhs + rhs;
            }
            Opcode::SUB => {
                let lhs = self.registers[self.next_8_bits() as usize];
                let rhs = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = lhs - rhs;
            }
            Opcode::MUL => {
                let lhs = self.registers[self.next_8_bits() as usize];
                let rhs = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = lhs * rhs;
            }
            Opcode::DIV => {
                let lhs = self.registers[self.next_8_bits() as usize];
                let rhs = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = lhs / rhs;
                self.remainder = (lhs % rhs) as u32;
            }
            Opcode::JMP => {
                let target = self.registers[self.next_8_bits() as usize];
                self.pc = target as usize;
            }
            Opcode::JMPF => {
                let target = self.registers[self.next_8_bits() as usize];
                self.pc += target as usize;
            }
            Opcode::JMPB => {
                let target = self.registers[self.next_8_bits() as usize];
                self.pc -= target as usize;
            }
            Opcode::EQ => {
                let lhs = self.registers[self.next_8_bits() as usize];
                let rhs = self.registers[self.next_8_bits() as usize];
                if lhs == rhs {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }
                self.next_8_bits();
            }
            Opcode::NEQ => {
                let lhs = self.registers[self.next_8_bits() as usize];
                let rhs = self.registers[self.next_8_bits() as usize];
                if lhs != rhs {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }
                self.next_8_bits();
            }
            Opcode::GT => {
                let lhs = self.registers[self.next_8_bits() as usize];
                let rhs = self.registers[self.next_8_bits() as usize];
                if lhs > rhs {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }
                self.next_8_bits();
            }
            Opcode::LT => {
                let lhs = self.registers[self.next_8_bits() as usize];
                let rhs = self.registers[self.next_8_bits() as usize];
                if lhs < rhs {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }
                self.next_8_bits();
            }
            Opcode::GE => {
                let lhs = self.registers[self.next_8_bits() as usize];
                let rhs = self.registers[self.next_8_bits() as usize];
                if lhs >= rhs {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }
                self.next_8_bits();
            }
            Opcode::LE => {
                let lhs = self.registers[self.next_8_bits() as usize];
                let rhs = self.registers[self.next_8_bits() as usize];
                if lhs <= rhs {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }
                self.next_8_bits();
            }
            Opcode::JEQ => {
                let target = self.registers[self.next_8_bits() as usize];
                if self.equal_flag {
                    self.pc = target as usize;
                }
            }
            Opcode::ALLOC => {
                let register = self.next_8_bits() as usize;
                let bytes = self.registers[register];
                let new_end = self.heap.len() as i32 + bytes;
                self.heap.resize(new_end as usize, 0);
            }
            Opcode::INC => {
                let register = self.next_8_bits() as usize;
                self.registers[register] += 1;
            }
            Opcode::DEC => {
                let register = self.next_8_bits() as usize;
                self.registers[register] += 1;
            }
            opcode => {
                println!("What are you trying to do? (Invalid Opcode: {opcode:?})");
                return false;
            }
        }
        true
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        opcode
    }

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        result
    }

    fn next_16_bits(&mut self) -> u16 {
        let result: u16 = (self.program[self.pc] as u16) << 8 | self.program[self.pc + 1] as u16;
        self.pc += 2;
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0);
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        let test_bytes = vec![0, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_load() {
        let mut test_vm = VM::new();
        test_vm.program = vec![1, 0, 1, 244];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_opcode_add() {
        let mut test_vm = VM::new();
        // LOAD 0 1
        // LOAD 1 1
        // ADD 0 1 2
        test_vm.program = vec![1, 0, 0, 1, 1, 1, 0, 1, 2, 0, 1, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 1);
        assert_eq!(test_vm.registers[1], 1);
        assert_eq!(test_vm.registers[2], 2);
    }

    #[test]
    fn test_opcode_sub() {
        let mut test_vm = VM::new();
        // LOAD 0 1
        // LOAD 1 1
        // SUB 0 1 2
        test_vm.program = vec![1, 0, 0, 1, 1, 1, 0, 1, 3, 0, 1, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 1);
        assert_eq!(test_vm.registers[1], 1);
        assert_eq!(test_vm.registers[2], 0);
    }

    #[test]
    fn test_opcode_mul() {
        let mut test_vm = VM::new();
        // LOAD 0 1
        // LOAD 1 1
        // MUL 0 1 2
        test_vm.program = vec![1, 0, 0, 1, 1, 1, 0, 1, 4, 0, 1, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 1);
        assert_eq!(test_vm.registers[1], 1);
        assert_eq!(test_vm.registers[2], 1);
    }

    #[test]
    fn test_opcode_div() {
        let mut test_vm = VM::new();
        // LOAD 0 1
        // LOAD 1 1
        // DIV 0 1 2
        test_vm.program = vec![1, 0, 0, 1, 1, 1, 0, 1, 5, 0, 1, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 1);
        assert_eq!(test_vm.registers[1], 1);
        assert_eq!(test_vm.registers[2], 1);
        assert_eq!(test_vm.remainder, 0);
    }

    #[test]
    fn test_opcode_div_remainder() {
        let mut test_vm = VM::new();
        // LOAD 0 3
        // LOAD 1 2
        // DIV 0 1 2 | Q = 1, R = 1
        test_vm.program = vec![1, 0, 0, 3, 1, 1, 0, 2, 5, 0, 1, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 3);
        assert_eq!(test_vm.registers[1], 2);
        assert_eq!(test_vm.registers[2], 1);
        assert_eq!(test_vm.remainder, 1);
    }

    #[test]
    fn test_jmp_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 1;
        test_vm.program = vec![6, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_jmpf_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 2;
        test_vm.program = vec![7, 0, 0, 0, 6, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_eq_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![9, 0, 1, 0, 9, 0, 1, 0];
        test_vm.run_once();
        assert!(test_vm.equal_flag);
        test_vm.registers[1] = 20;
        test_vm.run_once();
        assert!(!test_vm.equal_flag);
    }

    #[test]
    fn test_jeq_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 7;
        test_vm.equal_flag = true;
        test_vm.program = vec![15, 0, 0, 0, 16, 0, 0, 0, 16, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 7);
    }

    #[test]
    fn test_aloc_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 1024;
        test_vm.program = vec![16, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.heap.len(), 1024);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        let test_bytes = vec![200, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }
}
