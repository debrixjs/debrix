use super::super::lexer::*;
use crate::*;

fn tokenize(input: &str) -> Token {
	let mut scanner = Scanner::new(input);
	let mut lexer = Lexer::new(&mut scanner);
	let token = lexer.scan().unwrap();
	assert!(scanner.is_done());
	token
}

#[test]
fn test_scan_identifier() {
	let token = tokenize("abc");
	assert_eq!(token.kind, TokenKind::Identifier);
	assert_eq!(token.start, 0);
}

#[test]
fn test_scan_string_literal() {
	let token = tokenize("\"abc\"");
	assert_eq!(token.kind, TokenKind::String);
	assert_eq!(token.start, 0);
}

#[test]
fn test_scan_number_literal() {
	let token = tokenize("123");
	assert_eq!(token.kind, TokenKind::Numeric);
	assert_eq!(token.start, 0);
}

#[test]
fn test_scan_number_literal_with_decimal() {
	let token = tokenize("12.3");
	assert_eq!(token.kind, TokenKind::Numeric);
	assert_eq!(token.start, 0);
}

#[test]
fn test_scan_template_literal() {
	let token = tokenize("`abc`");
	assert_eq!(token.kind, TokenKind::Template);
	assert_eq!(token.start, 0);
}

#[test]
fn test_scan_keyword_true() {
	assert_eq!(tokenize("true").kind, TokenKind::True)
}

#[test]
fn test_scan_keyword_false() {
	assert_eq!(tokenize("false").kind, TokenKind::False)
}

#[test]
fn test_scan_keyword_null() {
	assert_eq!(tokenize("null").kind, TokenKind::Null)
}

#[test]
fn test_scan_keyword_delete() {
	assert_eq!(tokenize("delete").kind, TokenKind::Delete)
}

#[test]
fn test_scan_keyword_in() {
	assert_eq!(tokenize("in").kind, TokenKind::In)
}

#[test]
fn test_scan_keyword_instanceof() {
	assert_eq!(tokenize("instanceof").kind, TokenKind::Instanceof)
}

#[test]
fn test_scan_keyword_new() {
	assert_eq!(tokenize("new").kind, TokenKind::New)
}

#[test]
fn test_scan_keyword_return() {
	assert_eq!(tokenize("return").kind, TokenKind::Return)
}

#[test]
fn test_scan_keyword_this() {
	assert_eq!(tokenize("this").kind, TokenKind::This)
}

#[test]
fn test_scan_keyword_typeof() {
	assert_eq!(tokenize("typeof").kind, TokenKind::Typeof)
}

#[test]
fn test_scan_keyword_void() {
	assert_eq!(tokenize("void").kind, TokenKind::Void)
}

#[test]
fn test_scan_plus() {
	assert_eq!(tokenize("+").kind, TokenKind::Plus);
}

#[test]
fn test_scan_minus() {
	assert_eq!(tokenize("-").kind, TokenKind::Minus);
}

#[test]
fn test_scan_increment() {
	assert_eq!(tokenize("++").kind, TokenKind::Increment);
}

#[test]
fn test_scan_decrement() {
	assert_eq!(tokenize("--").kind, TokenKind::Decrement);
}

#[test]
fn test_scan_not() {
	assert_eq!(tokenize("!").kind, TokenKind::Not);
}

#[test]
fn test_scan_bit_not() {
	assert_eq!(tokenize("~").kind, TokenKind::BitNot);
}

#[test]
fn test_scan_multiply() {
	assert_eq!(tokenize("*").kind, TokenKind::Multiply);
}

#[test]
fn test_scan_exponentiate() {
	assert_eq!(tokenize("**").kind, TokenKind::Exponentiate);
}

#[test]
fn test_scan_divide() {
	assert_eq!(tokenize("/").kind, TokenKind::Divide);
}

#[test]
fn test_scan_modulo() {
	assert_eq!(tokenize("%").kind, TokenKind::Modulo);
}

#[test]
fn test_scan_less_than() {
	assert_eq!(tokenize("<").kind, TokenKind::LessThan);
}

#[test]
fn test_scan_greater_than() {
	assert_eq!(tokenize(">").kind, TokenKind::GreaterThan);
}

#[test]
fn test_scan_less_than_equal() {
	assert_eq!(tokenize("<=").kind, TokenKind::LessThanEqual);
}

#[test]
fn test_scan_greater_than_equal() {
	assert_eq!(tokenize(">=").kind, TokenKind::GreaterThanEqual);
}

#[test]
fn test_scan_left_shift() {
	assert_eq!(tokenize("<<").kind, TokenKind::LeftShift);
}

#[test]
fn test_scan_right_shift() {
	assert_eq!(tokenize(">>").kind, TokenKind::RightShift);
}

#[test]
fn test_scan_unsigned_right_shift() {
	assert_eq!(tokenize(">>>").kind, TokenKind::UnsignedRightShift);
}

#[test]
fn test_scan_equal() {
	assert_eq!(tokenize("==").kind, TokenKind::Equal);
}

#[test]
fn test_scan_not_equal() {
	assert_eq!(tokenize("!=").kind, TokenKind::NotEqual);
}

#[test]
fn test_scan_strict_equal() {
	assert_eq!(tokenize("===").kind, TokenKind::StrictEqual);
}

#[test]
fn test_scan_strict_not_equal() {
	assert_eq!(tokenize("!==").kind, TokenKind::StrictNotEqual);
}

#[test]
fn test_scan_bit_and() {
	assert_eq!(tokenize("&").kind, TokenKind::BitAnd);
}

#[test]
fn test_scan_bit_xor() {
	assert_eq!(tokenize("^").kind, TokenKind::BitXor);
}

#[test]
fn test_scan_bit_or() {
	assert_eq!(tokenize("|").kind, TokenKind::BitOr);
}

#[test]
fn test_scan_logical_and() {
	assert_eq!(tokenize("&&").kind, TokenKind::LogicalAnd);
}

#[test]
fn test_scan_logical_or() {
	assert_eq!(tokenize("||").kind, TokenKind::LogicalOr);
}

#[test]
fn test_scan_question_mark() {
	assert_eq!(tokenize("?").kind, TokenKind::QuestionMark);
}

#[test]
fn test_scan_colon() {
	assert_eq!(tokenize(":").kind, TokenKind::Colon);
}

#[test]
fn test_scan_assign() {
	assert_eq!(tokenize("=").kind, TokenKind::Assign);
}

#[test]
fn test_scan_plus_assign() {
	assert_eq!(tokenize("+=").kind, TokenKind::PlusAssign);
}

#[test]
fn test_scan_minus_assign() {
	assert_eq!(tokenize("-=").kind, TokenKind::MinusAssign);
}

#[test]
fn test_scan_multiply_assign() {
	assert_eq!(tokenize("*=").kind, TokenKind::MultiplyAssign);
}

#[test]
fn test_scan_exponentiate_assign() {
	assert_eq!(tokenize("**=").kind, TokenKind::ExponentiateAssign);
}

#[test]
fn test_scan_divide_assign() {
	assert_eq!(tokenize("/=").kind, TokenKind::DivideAssign);
}

#[test]
fn test_scan_modulo_assign() {
	assert_eq!(tokenize("%=").kind, TokenKind::ModuloAssign);
}

#[test]
fn test_scan_left_shift_assign() {
	assert_eq!(tokenize("<<=").kind, TokenKind::LeftShiftAssign);
}

#[test]
fn test_scan_right_shift_assign() {
	assert_eq!(tokenize(">>=").kind, TokenKind::RightShiftAssign);
}

#[test]
fn test_scan_unsigned_right_shift_assign() {
	assert_eq!(tokenize(">>>=").kind, TokenKind::UnsignedRightShiftAssign);
}

#[test]
fn test_scan_bit_and_assign() {
	assert_eq!(tokenize("&=").kind, TokenKind::BitAndAssign);
}

#[test]
fn test_scan_bit_xor_assign() {
	assert_eq!(tokenize("^=").kind, TokenKind::BitXorAssign);
}

#[test]
fn test_scan_bit_or_assign() {
	assert_eq!(tokenize("|=").kind, TokenKind::BitOrAssign);
}

#[test]
fn test_scan_comma() {
	assert_eq!(tokenize(",").kind, TokenKind::Comma);
}

#[test]
fn test_scan_arrow() {
	assert_eq!(tokenize("=>").kind, TokenKind::Arrow);
}

#[test]
fn test_scan_open_paren() {
	assert_eq!(tokenize("(").kind, TokenKind::OpenParen);
}

#[test]
fn test_scan_close_paren() {
	assert_eq!(tokenize(")").kind, TokenKind::CloseParen);
}

#[test]
fn test_scan_open_bracket() {
	assert_eq!(tokenize("[").kind, TokenKind::OpenBracket);
}

#[test]
fn test_scan_close_bracket() {
	assert_eq!(tokenize("]").kind, TokenKind::CloseBracket);
}

#[test]
fn test_scan_open_brace() {
	assert_eq!(tokenize("{").kind, TokenKind::OpenBrace);
}

#[test]
fn test_scan_close_brace() {
	assert_eq!(tokenize("}").kind, TokenKind::CloseBrace);
}

#[test]
fn test_scan_dot() {
	assert_eq!(tokenize(".").kind, TokenKind::Dot);
}

#[test]
fn test_scan_ellipsis() {
	assert_eq!(tokenize("...").kind, TokenKind::Ellipsis);
}

#[test]
fn test_scan_semicolon() {
	assert_eq!(tokenize(";").kind, TokenKind::Semicolon);
}

#[test]
fn test_skip_whitespace() {
	assert_eq!(tokenize(" foo").kind, TokenKind::Identifier);
}

#[test]
fn test_skip_comment() {
	assert_eq!(tokenize("//foo\nbar").kind, TokenKind::Identifier);
}

#[test]
fn test_skip_multiline_comment() {
	assert_eq!(tokenize("/*\nfoo\n*/bar").kind, TokenKind::Identifier);
}
