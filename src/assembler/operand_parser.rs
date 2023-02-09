use crate::assembler;
use crate::assembler::label_parser::label_usage;
use assembler::register_parser::register;
use assembler::Token;
use nom::digit;
use nom::types::CompleteStr;

named!(pub integer_operand<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("#") >> register_number: digit >>
            (
                Token::Number{value: register_number.parse::<i32>().unwrap()}
            )
        )
    )
);

named!(pub operand<CompleteStr, Token>,
    alt!(
        integer_operand |
        label_usage |
        register
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_integer_operand() {
        let result = integer_operand(CompleteStr("#10"));
        assert!(result.is_ok());
        let (rest, value) = result.unwrap();
        assert_eq!(rest, CompleteStr(""));
        assert_eq!(value, Token::Number { value: 10 });

        let result = integer_operand(CompleteStr("10"));
        assert!(result.is_err());
    }
}
