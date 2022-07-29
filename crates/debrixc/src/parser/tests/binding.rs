use super::super::{PestParser, R::*};
use crate::pest::Parser;

#[test]
fn test_binding() {
	PestParser::parse(element, "<div (test: value)></div>").unwrap();
}

#[test]
#[ignore]
fn test_with_parens() {
	PestParser::parse(element, "<div (test: fn())></div>").unwrap();
}
