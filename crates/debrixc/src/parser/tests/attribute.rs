use super::super::{PestParser, R::*};
use crate::pest::Parser;

#[test]
pub fn test_static() {
	PestParser::parse(element, "<div class=\"test\"></div>").unwrap();
}

#[test]
pub fn test_binding() {
	PestParser::parse(element, "<div class=(test)></div>").unwrap();
}

#[test]
#[ignore]
pub fn test_binding_with_parens() {
	PestParser::parse(element, "<div class=(test())></div>").unwrap();
}
