use super::super::{PestParser, R::*};
use crate::pest::Parser;

#[test]
fn test_new_expression() {
	PestParser::parse(expression, "new Test()").unwrap();
}

#[test]
fn test_new_expression_on_member() {
	PestParser::parse(expression, "new test.Test()").unwrap();
}

#[test]
fn test_new_expression_on_literal() {
	PestParser::parse(expression, "new 1234()").unwrap();
}

#[test]
fn test_new_expression_without_parens() {
	PestParser::parse(expression, "new Test").unwrap();
}

#[test]
fn test_call_expression() {
	PestParser::parse(expression, "test()").unwrap();
}

#[test]
fn test_call_expression_on_member() {
	PestParser::parse(expression, "test.test()").unwrap();
}

#[test]
fn test_call_expression_on_literal() {
	PestParser::parse(expression, "1234()").unwrap();
}

#[test]
fn test_call_expression_with_args() {
	PestParser::parse(expression, "test(1, 2, 3)").unwrap();
}

#[test]
fn test_call_expression_with_trailing_comma() {
	PestParser::parse(expression, "test(1, 2, 3,)").unwrap();
}

#[test]
fn test_unary_expression() {
	PestParser::parse(expression, "-test").unwrap();
}

#[test]
#[ignore]
fn test_unary_expression_with_parens() {
	PestParser::parse(expression, "-(test)").unwrap();
}

#[test]
#[ignore]
fn test_ident_starting_with_unary_text() {
	let mut pairs = PestParser::parse(expression, "void2").unwrap();
	assert_eq!(pairs.next().unwrap().as_rule(), ident)
}

#[test]
fn test_binary_expression() {
	PestParser::parse(expression, "1 + 2").unwrap();
}

#[test]
#[ignore]
fn test_binary_expression_with_parens() {
	PestParser::parse(expression, "(1 + 2)").unwrap();
}

#[test]
fn test_binary_expression_with_unary() {
	PestParser::parse(expression, "1 + -2").unwrap();
}

#[test]
fn test_binary_expression_with_idents() {
	PestParser::parse(expression, "test + test2").unwrap();
}

#[test]
fn test_function() {
	PestParser::parse(expression, "() => undefined").unwrap();
}

#[test]
fn test_function_shorthand() {
	PestParser::parse(expression, "=> undefined").unwrap();
}

#[test]
fn test_function_with_args() {
	PestParser::parse(expression, "(arg1, arg2) => undefined").unwrap();
}

#[test]
fn test_function_with_trailing_comma() {
	PestParser::parse(expression, "(arg1, arg2,) => undefined").unwrap();
}

#[test]
fn test_array() {
	PestParser::parse(expression, "[]").unwrap();
}

#[test]
fn test_array_with_items() {
	PestParser::parse(expression, "[1, 2, 3]").unwrap();
}

#[test]
fn test_array_with_trailing_comma() {
	PestParser::parse(expression, "[1, 2, 3,]").unwrap();
}

#[test]
fn test_object() {
	PestParser::parse(expression, "{}").unwrap();
}

#[test]
#[ignore]
fn test_object_with_items() {
	PestParser::parse(expression, "{test: 1, test2: 2, test3: 3}").unwrap();
}

#[test]
#[ignore]
fn test_object_shorthand() {
	PestParser::parse(expression, "{test, test2, test3}").unwrap();
}

#[test]
#[ignore]
fn test_object_mixed_shorthand() {
	PestParser::parse(expression, "{test, test2: 2, test3}").unwrap();
}

#[test]
#[ignore]
fn test_object_with_trailing_comma() {
	PestParser::parse(expression, "{test: 1, test2: 2, test3: 3,}").unwrap();
}
