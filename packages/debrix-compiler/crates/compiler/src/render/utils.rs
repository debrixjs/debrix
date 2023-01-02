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
pub(crate) const DEFAULT_SLOT_NAME: &str = "main";

pub struct Unique {
	map: HashMap<String, usize>,
}

impl Unique {
	pub fn new() -> Self {
		Self {
			map: HashMap::new(),
		}
	}

	fn set(&mut self, name: &str, index: usize) {
		if self.map.contains_key(name) {
			*self.map.get_mut(name).unwrap() = index;
		} else {
			self.map.insert(name.to_owned(), index);
		}
	}

	fn next(&self, name: &str, start: usize) -> usize {
		if self.map.contains_key(name) {
			self.map.get(name).unwrap().to_owned() + 1
		} else {
			start
		}
	}

	fn join(name: &str, index: usize) -> String {
		let mut name = name.to_owned();

		if index > 0 {
			name.push_str("_");
			name.push_str(&index.to_string());
		}

		if RESERVED_JAVASCRIPT_KEYWORDS.contains(&name.as_ref()) {
			name.push_str("_");
		}

		name
	}

	pub fn ensure(&mut self, name: &str) -> String {
		let index = self.next(name, 0);
		self.set(name, index);
		Unique::join(name, index)
	}

	pub fn from(&mut self, name: &str) -> String {
		let index = self.next(name, 1);
		self.set(name, index);
		Unique::join(name, index)
	}
}

pub fn find_static_attr(name: &str) -> impl Fn(&&ast::Attribute) -> bool {
	let name = name.to_owned();
	move |attr: &&ast::Attribute| -> bool {
		match attr {
			ast::Attribute::Static(attr) => &attr.name.name == &name,
			_ => false,
		}
	}
}

pub fn in_string(value: &str) -> String {
	"\"".to_owned() + &value.replace('"', "\\\"") + "\""
}

fn is_ident_char(char: &char) -> bool {
	char.is_alphanumeric() || char == &'$' || char == &'_'
}

pub fn is_valid_identifier(s: &str) -> bool {
	s.chars().find(|c| !is_ident_char(c)).is_none()
}

pub fn to_valid_identifier(value: &str) -> String {
	let mut ident = String::new();
	let mut chars = value.chars().into_iter();

	if let Some(char) = chars.next() {
		if is_ident_char(&char) {
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
		if is_ident_char(&char) {
			ident.push(char);
		} else {
			ident.push('_');
		}
	}

	ident
}

pub fn to_valid_property(s: &str) -> String {
	if is_valid_identifier(s) {
		s.to_owned()
	} else {
		in_string(s)
	}
}

pub fn snake_case(s: &str) -> String {
	let mut chars = s.chars();

	if let Some(ch) = chars.next() {
		let mut s = ch.to_string().to_lowercase();

		while let Some(ch) = chars.next() {
			if ch.is_uppercase() {
				s.push('_');
				s.push_str(&ch.to_string().to_lowercase());
			} else {
				s.push(ch);
			}
		}

		s
	} else {
		String::new()
	}
}

pub fn title_case(s: &str) -> String {
	let mut chars = s.chars();

	if let Some(ch) = chars.next() {
		let mut s = ch.to_string().to_uppercase();

		while let Some(ch) = chars.next() {
			match ch {
				'-' | '_' => {
					if let Some(ch) = chars.next() {
						s.push_str(&ch.to_string().to_uppercase());
					}
				}
				_ => s.push_str(&ch.to_string().to_lowercase()),
			}
		}

		s
	} else {
		String::new()
	}
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

pub struct Map<K, V> {
	keys: Vec<K>,
	values: Vec<V>,
}

#[allow(dead_code)]
impl<K, V> Map<K, V> {
	pub fn new() -> Self {
		Self {
			keys: Vec::new(),
			values: Vec::new(),
		}
	}

	pub fn clear(&mut self) {
		self.keys.clear();
		self.values.clear();
	}

	pub fn set(&mut self, key: K, value: V) {
		self.keys.push(key);
		self.values.push(value);
	}

	pub fn keys(&mut self) -> std::slice::Iter<K> {
		self.keys.iter()
	}

	pub fn values(&mut self) -> std::slice::Iter<V> {
		self.values.iter()
	}

	pub fn entries(&mut self) -> std::vec::IntoIter<(&K, &V)> {
		let mut entries = Vec::new();
		let mut index = 0;
		let len = self.keys.len();

		while index < len {
			entries.push((
				self.keys.get(index).unwrap(),
				self.values.get(index).unwrap(),
			));
			index += 1;
		}

		entries.into_iter()
	}

	pub fn into_keys(self) -> std::vec::IntoIter<K> {
		self.keys.into_iter()
	}

	pub fn into_values(self) -> std::vec::IntoIter<V> {
		self.values.into_iter()
	}

	pub fn into_entries(self) -> std::vec::IntoIter<(K, V)> {
		let mut entries = Vec::new();
		let mut keys = self.keys.into_iter();
		let mut values = self.values.into_iter();

		while let Some(key) = keys.next() {
			let value = values.next().unwrap();
			entries.push((key, value));
		}

		entries.into_iter()
	}

	pub fn has<Q: ?Sized>(&self, key: &Q) -> bool
	where
		K: std::borrow::Borrow<Q> + PartialEq<Q>,
	{
		self.get(key).is_some()
	}

	pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
	where
		K: std::borrow::Borrow<Q> + PartialEq<Q>,
	{
		self.keys
			.iter()
			.position(|k| k == key)
			.map(|index| self.values.get(index).unwrap())
	}

	pub fn get_mut<Q: ?Sized>(&mut self, key: &Q) -> Option<&mut V>
	where
		K: std::borrow::Borrow<Q> + PartialEq<Q>,
	{
		self.keys
			.iter()
			.position(|k| k == key)
			.map(|index| self.values.get_mut(index).unwrap())
	}

	pub fn delete<Q: ?Sized>(&mut self, key: &Q) -> Option<V>
	where
		K: std::borrow::Borrow<Q> + PartialEq<Q>,
	{
		self.keys
			.iter()
			.position(|k| k == key)
			.map(|index| self.values.remove(index))
	}

	pub fn size(&self) -> usize {
		self.keys.len()
	}

	pub fn is_empty(&self) -> bool {
		self.keys.is_empty()
	}
}

impl<K, V> From<Vec<(K, V)>> for Map<K, V> {
	fn from(vec: Vec<(K, V)>) -> Self {
		let mut map = Self::new();

		for (key, value) in vec {
			map.set(key, value);
		}

		map
	}
}
