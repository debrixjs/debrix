use super::*;
use std::collections::HashMap;

#[rustfmt::skip]
pub const RESERVED_JAVASCRIPT_KEYWORDS: [&str; 64] = [
	"abstract", "arguments", "await", "boolean", "break", "byte", "case", "catch", "char",
	"class", "const", "continue", "debugger", "default", "delete", "do", "double", "else", "enum",
	"eval", "export", "extends", "false", "final", "finally", "float", "for", "function", "goto",
	"if", "implements", "import", "in", "instanceof", "int", "interface", "let", "long", "native",
	"new", "null", "package", "private", "protected", "public", "return", "short", "static",
	"super", "switch", "synchronized", "this", "throw", "throws", "transient", "true", "try",
	"typeof", "var", "void", "volatile", "while", "with", "yield"
];

pub(crate) const INTERNAL_MODULE: &str = "@debrix/internal";

pub(crate) const MODEL_KIND: &str = "model";
pub(crate) const COMPONENT_KIND: &str = "component";
pub(crate) const BINDER_KIND: &str = "binder";

pub struct Unique {
	map: HashMap<String, usize>,
}

impl Unique {
	pub fn new() -> Self {
		Self {
			map: HashMap::new(),
		}
	}

	fn _unique(&mut self, name: &str, force_num: bool) -> String {
		let mut ident = name.to_owned();
		let index: usize;

		if self.map.contains_key(&ident) {
			let index_mut = self.map.get_mut(&ident).unwrap();
			*index_mut += 1;
			index = index_mut.clone();
		} else {
			index = if force_num { 1 } else { 0 };
			self.map.insert(ident.clone(), index);
		}

		if index > 0 || force_num {
			ident.push_str("_");
			ident.push_str(&index.to_string());
		}

		if RESERVED_JAVASCRIPT_KEYWORDS.contains(&ident.as_ref()) {
			ident.push_str("_");
		}

		ident
	}

	pub fn ensure(&mut self, name: &str) -> String {
		self._unique(name, false)
	}

	pub fn from(&mut self, name: &str) -> String {
		self._unique(name, true)
	}
}

pub fn in_string(value: &str) -> String {
	"\"".to_owned() + &value.replace('"', "\\\"") + "\""
}

pub fn is_ident(char: &char) -> bool {
	char.is_alphanumeric() || char == &'$' || char == &'_'
}

pub fn to_valid_ident(value: &str) -> String {
	let mut ident = String::new();
	let mut chars = value.chars().into_iter();

	if let Some(char) = chars.next() {
		if is_ident(&char) {
			if char.is_numeric() {
				ident.push('_');
			}

			ident.push(char);
		} else {
			ident.push('_');
		}
	} else {
		return "_".to_owned();
	}

	while let Some(char) = chars.next() {
		if is_ident(&char) {
			ident.push(char);
		} else {
			ident.push('_');
		}
	}

	ident
}

pub fn serialize_string_literal(lit: &ast::StringLiteral) -> String {
	lit.quote.to_string() + &lit.value + &lit.quote.to_string()
}

pub fn join_spaces(value: &str) -> String {
	let mut string = String::new();
	let mut is_space = false;

	for char in value.chars() {
		if char.is_whitespace() {
			if !is_space {
				string.push(' ');
			}

			is_space = true;
		} else {
			is_space = false;
			string.push(char);
		}
	}

	string
}
