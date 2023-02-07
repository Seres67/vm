use crate::assembler::instruction_parser::AssemblerInstruction;
use crate::assembler::operand_parser::operand;
use crate::assembler::Token;
use nom::alpha1;
use nom::types::CompleteStr;

named!(directive_declaration<CompleteStr, Token>,
    do_parse!(
        tag!(".") >>
        name: alpha1 >>
        (
            Token::Directive {name: name.to_string()}
        )
    )
);

named!(directive_combined<CompleteStr, AssemblerInstruction>,
    ws!(
        do_parse!(
            tag!(".") >>
            name: directive_declaration >>
            operand1: opt!(operand) >>
            operand2: opt!(operand) >>
            operand3: opt!(operand) >>
            (
                AssemblerInstruction {
                    opcode: None,
                    directive: Some(name),
                    label: None,
                    operand1,
                    operand2,
                    operand3
                }
            )
        )
    )
);

named!(pub directive<CompleteStr, AssemblerInstruction>,
    do_parse!(
        ins:alt!(
            directive_combined
        ) >>
        (
            ins
        )
    )
);
