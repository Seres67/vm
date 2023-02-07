use std::fs::File;
use crate::assembler::program_parser::program;
use crate::vm::VM;
use std::io;
use std::io::{Read, Write};
use std::path::Path;
use nom::types::CompleteStr;

#[derive(Default)]
pub struct REPL {
    command_buffer: Vec<String>,
    vm: VM,
}

impl REPL {
    pub fn new() -> REPL {
        REPL {
            command_buffer: vec![],
            vm: VM::new(),
        }
    }

    pub fn run(&mut self) {
        println!("Back at it again.");
        loop {
            let mut buffer = String::new();
            let stdin = io::stdin();
            print!(">>> ");
            io::stdout().flush().expect("Unable to flush stdout");
            stdin.read_line(&mut buffer).expect("Unable to read line");
            let buffer = buffer.trim();
            self.command_buffer.push(buffer.to_string());
            match buffer {
                ".quit" => {
                    println!("Already leavin?");
                    std::process::exit(0);
                }
                ".history" => {
                    for command in &self.command_buffer {
                        println!("{command}");
                    }
                }
                ".program" => {
                    for instruction in &self.vm.program {
                        println!("{instruction}");
                    }
                }
                ".registers" => {
                    println!("{:#?}", self.vm.registers);
                }
                ".clear" => {
                    self.vm.program.clear();
                }
                ".load_file" => {
                    print!("File path:");
                    io::stdout().flush().expect("unable to flush stdout");
                    let mut tmp = String::new();
                    stdin.read_line(&mut tmp).expect("unable to read user input");
                    let tmp = tmp.trim();
                    let filename = Path::new(&tmp);
                    let mut f = File::open(filename).expect("unable to open file");
                    let mut contents = String::new();
                    f.read_to_string(&mut contents).expect("unable to read file");
                    let program = match program(CompleteStr(&contents)) {
                        Ok((_, program)) => {
                            program
                        },
                        Err(e) => {
                            println!("Unable to parse input: {e:?}");
                            continue;
                        }
                    };
                    self.vm.program.append(&mut program.to_bytes());
                }
                _ => {
                    let program = match program(buffer.into()) {
                        Ok((_, program)) => program,
                        Err(_) => {
                            println!("Unable to parse input");
                            continue;
                        }
                    };
                    self.vm.program.append(&mut program.to_bytes());
                    self.vm.run_once();
                }
            }
        }
    }
}
