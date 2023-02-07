use crate::assembler::Token;
use nom::digit;
use nom::types::CompleteStr;

named!(pub register<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("$") >> register_number: digit >>
            (
                Token::Register {register_number: register_number.parse::<u8>().unwrap()}
            )
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_register() {
        let result = register(CompleteStr("$0"));
        assert!(result.is_ok());
        let result = register(CompleteStr("0"));
        assert!(result.is_err());
        let result = register(CompleteStr("$a"));
        assert!(result.is_err());
        let result = register(CompleteStr("$"));
        assert!(result.is_err());
    }
}
