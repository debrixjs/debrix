use super::super::{PestParser, R::*};
use crate::pest::Parser;

#[test]
fn test_self_closing() {
	PestParser::parse(element, "<div />").unwrap();
}

#[test]
fn test_element() {
	PestParser::parse(element, "<div></div>").unwrap();
}

#[test]
fn test_children() {
	PestParser::parse(element, "<div><div></div></div>").unwrap();
}

#[test]
fn test_child_text() {
	PestParser::parse(element, "<div>text</div>").unwrap();
}

#[test]
fn test_child_comment() {
	PestParser::parse(element, "<div><!-- comment --></div>").unwrap();
}
