use crate::assembler::Token;
use crate::instruction::Opcode;
use nom::alpha1;
use nom::types::CompleteStr;

named!(pub opcode<CompleteStr, Token>,
    do_parse!(
        opcode: alpha1 >>
        (
            {
                Token::Op{code: Opcode::from(opcode)}
            }
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_load() {
        let result = opcode(CompleteStr("load"));
        assert!(result.is_ok());
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Op { code: Opcode::LOAD });
        assert_eq!(rest, CompleteStr(""));
        let result = opcode(CompleteStr("aold"));
        let (_, token) = result.unwrap();
        assert_eq!(
            token,
            Token::Op {
                code: Opcode::ILLEGAL
            }
        );
    }
}
