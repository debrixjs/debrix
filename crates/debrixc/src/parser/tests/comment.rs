use super::super::{PestParser, R::*};
use crate::pest::Parser;

#[test]
fn test_comment() {
	PestParser::parse(comment, "<!-- comment -->").unwrap();
}

#[test]
fn test_double_start() {
	PestParser::parse(comment, "<!-- <!-- -->").unwrap();
}
