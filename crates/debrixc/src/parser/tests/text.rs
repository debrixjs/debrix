use super::super::{PestParser, R::*};
use crate::pest::Parser;

#[test]
fn test_text() {
	PestParser::parse(text, "text ().").unwrap();
}

#[test]
fn test_escape() {
	PestParser::parse(text, "\\{\\<\\\\").unwrap();
}

#[test]
fn test_binding() {
	PestParser::parse(text_binding, "{test}").unwrap();
}

#[test]
#[ignore]
fn test_binding_with_curly() {
	PestParser::parse(text_binding, "{{test}.test}").unwrap();
}
