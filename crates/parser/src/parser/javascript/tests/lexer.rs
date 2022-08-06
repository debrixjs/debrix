use super::super::*;

fn tokenize(input: &str) -> Token {
	let mut iter = ChIter::new(input.to_owned());
	let token = lexer::scan(&mut iter).unwrap();
	assert!(iter.peek_next().is_none());
	token
}

#[test]
fn test_scan_identifier() {
	assert_eq!(tokenize("abc"), Token::Identifier("abc".to_string()));
}

#[test]
fn test_scan_string_literal() {
	assert_eq!(tokenize("\"abc\""), Token::StringLiteral("\"abc\"".to_string()));
}

#[test]
fn test_scan_number_literal() {
	assert_eq!(tokenize("123"), Token::NumberLiteral(123_f64));
}

#[test]
fn test_scan_template_literal() {
	assert_eq!(tokenize("`abc`"), Token::TemplateLiteral("abc".to_string()));
}

#[test]
fn test_scan_keyword_true() {
	assert_eq!(tokenize("true"), Token::True)
}

#[test]
fn test_scan_keyword_false() {
	assert_eq!(tokenize("false"), Token::False)
}

#[test]
fn test_scan_keyword_null() {
	assert_eq!(tokenize("null"), Token::Null)
}

#[test]
fn test_scan_keyword_delete() {
	assert_eq!(tokenize("delete"), Token::Delete)
}

#[test]
fn test_scan_keyword_in() {
	assert_eq!(tokenize("in"), Token::In)
}

#[test]
fn test_scan_keyword_instanceof() {
	assert_eq!(tokenize("instanceof"), Token::Instanceof)
}

#[test]
fn test_scan_keyword_new() {
	assert_eq!(tokenize("new"), Token::New)
}

#[test]
fn test_scan_keyword_return() {
	assert_eq!(tokenize("return"), Token::Return)
}

#[test]
fn test_scan_keyword_this() {
	assert_eq!(tokenize("this"), Token::This)
}

#[test]
fn test_scan_keyword_typeof() {
	assert_eq!(tokenize("typeof"), Token::Typeof)
}

#[test]
fn test_scan_keyword_void() {
	assert_eq!(tokenize("void"), Token::Void)
}

#[test]
fn test_scan_plus() {
	assert_eq!(tokenize("+"), Token::Plus);
}

#[test]
fn test_scan_minus() {
	assert_eq!(tokenize("-"), Token::Minus);
}

#[test]
fn test_scan_increment() {
	assert_eq!(tokenize("++"), Token::Increment);
}

#[test]
fn test_scan_decrement() {
	assert_eq!(tokenize("--"), Token::Decrement);
}

#[test]
fn test_scan_not() {
	assert_eq!(tokenize("!"), Token::Not);
}

#[test]
fn test_scan_bit_not() {
	assert_eq!(tokenize("~"), Token::BitNot);
}

#[test]
fn test_scan_multiply() {
	assert_eq!(tokenize("*"), Token::Multiply);
}

#[test]
fn test_scan_exponentiate() {
	assert_eq!(tokenize("**"), Token::Exponentiate);
}

#[test]
fn test_scan_divide() {
	assert_eq!(tokenize("/"), Token::Divide);
}

#[test]
fn test_scan_modulo() {
	assert_eq!(tokenize("%"), Token::Modulo);
}

#[test]
fn test_scan_less_than() {
	assert_eq!(tokenize("<"), Token::LessThan);
}

#[test]
fn test_scan_greater_than() {
	assert_eq!(tokenize(">"), Token::GreaterThan);
}

#[test]
fn test_scan_less_than_equal() {
	assert_eq!(tokenize("<="), Token::LessThanEqual);
}

#[test]
fn test_scan_greater_than_equal() {
	assert_eq!(tokenize(">="), Token::GreaterThanEqual);
}

#[test]
fn test_scan_left_shift() {
	assert_eq!(tokenize("<<"), Token::LeftShift);
}

#[test]
fn test_scan_right_shift() {
	assert_eq!(tokenize(">>"), Token::RightShift);
}

#[test]
fn test_scan_unsigned_right_shift() {
	assert_eq!(tokenize(">>>"), Token::UnsignedRightShift);
}

#[test]
fn test_scan_equal() {
	assert_eq!(tokenize("=="), Token::Equal);
}

#[test]
fn test_scan_not_equal() {
	assert_eq!(tokenize("!="), Token::NotEqual);
}

#[test]
fn test_scan_strict_equal() {
	assert_eq!(tokenize("==="), Token::StrictEqual);
}

#[test]
fn test_scan_strict_not_equal() {
	assert_eq!(tokenize("!=="), Token::StrictNotEqual);
}

#[test]
fn test_scan_bit_and() {
	assert_eq!(tokenize("&"), Token::BitAnd);
}

#[test]
fn test_scan_bit_xor() {
	assert_eq!(tokenize("^"), Token::BitXor);
}

#[test]
fn test_scan_bit_or() {
	assert_eq!(tokenize("|"), Token::BitOr);
}

#[test]
fn test_scan_logical_and() {
	assert_eq!(tokenize("&&"), Token::LogicalAnd);
}

#[test]
fn test_scan_logical_or() {
	assert_eq!(tokenize("||"), Token::LogicalOr);
}

#[test]
fn test_scan_question_mark() {
	assert_eq!(tokenize("?"), Token::QuestionMark);
}

#[test]
fn test_scan_colon() {
	assert_eq!(tokenize(":"), Token::Colon);
}

#[test]
fn test_scan_assign() {
	assert_eq!(tokenize("="), Token::Assign);
}

#[test]
fn test_scan_plus_assign() {
	assert_eq!(tokenize("+="), Token::PlusAssign);
}

#[test]
fn test_scan_minus_assign() {
	assert_eq!(tokenize("-="), Token::MinusAssign);
}

#[test]
fn test_scan_multiply_assign() {
	assert_eq!(tokenize("*="), Token::MultiplyAssign);
}

#[test]
fn test_scan_exponentiate_assign() {
	assert_eq!(tokenize("**="), Token::ExponentiateAssign);
}

#[test]
fn test_scan_divide_assign() {
	assert_eq!(tokenize("/="), Token::DivideAssign);
}

#[test]
fn test_scan_modulo_assign() {
	assert_eq!(tokenize("%="), Token::ModuloAssign);
}

#[test]
fn test_scan_left_shift_assign() {
	assert_eq!(tokenize("<<="), Token::LeftShiftAssign);
}

#[test]
fn test_scan_right_shift_assign() {
	assert_eq!(tokenize(">>="), Token::RightShiftAssign);
}

#[test]
fn test_scan_unsigned_right_shift_assign() {
	assert_eq!(tokenize(">>>="), Token::UnsignedRightShiftAssign);
}

#[test]
fn test_scan_bit_and_assign() {
	assert_eq!(tokenize("&="), Token::BitAndAssign);
}

#[test]
fn test_scan_bit_xor_assign() {
	assert_eq!(tokenize("^="), Token::BitXorAssign);
}

#[test]
fn test_scan_bit_or_assign() {
	assert_eq!(tokenize("|="), Token::BitOrAssign);
}

#[test]
fn test_scan_comma() {
	assert_eq!(tokenize(","), Token::Comma);
}

#[test]
fn test_scan_arrow() {
	assert_eq!(tokenize("=>"), Token::Arrow);
}

#[test]
fn test_scan_open_paren() {
	assert_eq!(tokenize("("), Token::OpenParen);
}

#[test]
fn test_scan_close_paren() {
	assert_eq!(tokenize(")"), Token::CloseParen);
}

#[test]
fn test_scan_open_bracket() {
	assert_eq!(tokenize("["), Token::OpenBracket);
}

#[test]
fn test_scan_close_bracket() {
	assert_eq!(tokenize("]"), Token::CloseBracket);
}

#[test]
fn test_scan_open_brace() {
	assert_eq!(tokenize("{"), Token::OpenBrace);
}

#[test]
fn test_scan_close_brace() {
	assert_eq!(tokenize("}"), Token::CloseBrace);
}

#[test]
fn test_scan_dot() {
	assert_eq!(tokenize("."), Token::Dot);
}

#[test]
fn test_scan_ellipsis() {
	assert_eq!(tokenize("..."), Token::Ellipsis);
}

#[test]
fn test_scan_semicolon() {
	assert_eq!(tokenize(";"), Token::Semicolon);
}
