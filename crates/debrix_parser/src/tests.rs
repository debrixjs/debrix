// These tests only tests parsing whole documents. You can find the tests
// for parsing nodes in the bottom of the files in ./parser/...

// NOTE! Use the naming convention test_parse_xxx_document when defining tests.

use super::*;

fn parse(document: &str) -> ast::Document {
	parse_document(document.to_owned()).unwrap()
}

#[test]
pub fn test_parse_empty_document() {
	parse("");
}

#[test]
pub fn test_parse_simple_document() {
	parse(r#"
		using model from 'self.model.js'

		<p>Hello {$props.name}!</p>
	"#);
}
