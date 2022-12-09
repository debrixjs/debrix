use std::collections::HashMap;

use super::*;

pub struct Document {
	pub head: Chunk,
	pub frags: Chunk,

	pub unique: Unique,
	helpers: Vec<(String, String)>,
	helpers_map: HashMap<String, String>,
	declaration_map: HashMap<String, HashMap<String, String>>,
}

impl Document {
	pub fn new() -> Self {
		Self {
			frags: Chunk::new(),
			head: Chunk::new(),

			unique: Unique::new(),
			helpers: Vec::new(),
			helpers_map: HashMap::new(),
			declaration_map: HashMap::new(),
		}
	}

	pub fn render(mut self, document: ast::Document) -> Result<Chunk, Error> {
		let mut html_nodes = Vec::new();

		for node in document.children {
			match node {
				ast::Node::Comment(_)
				| ast::Node::Text(_)
				| ast::Node::TextBinding(_)
				| ast::Node::FlowControl(_)
				| ast::Node::Element(_) => {
					html_nodes.push(node);
				}

				ast::Node::DependencyStatement(node) => {
					self.render_dependency(node)?;
				}
			};
		}

		let render_fragment = self.render_fragment(html_nodes)?;

		if self.helpers.len() > 0 {
			let mut helpers = self.helpers.iter().peekable();

			self.head.write("import { ");

			while let Some(helper) = helpers.next() {
				self.head.write(&helper.0);

				if helper.0 != helper.1 {
					self.head.write(" as ").write(&helper.1);
				}

				if helpers.peek().is_some() {
					self.head.write(", ");
				}
			}

			self.head
				.write(" } from \"")
				.write(INTERNAL_MODULE)
				.write("\";\n");
		}

		let mut document = Chunk::new();
		let constructor = self.find_declaration(MODEL_KIND, MODEL_KIND);
		let insert_helper = self.helper("insert");
		let destroy_helper = self.helper("destroy");

		document
			.append(&format_chunk(self.head, 0))
			.write("\n\n")
			.append(&format_chunk(self.frags, 0))
			.write("\n\n/* implements Component */\n")
			.write("export default class Greeting {\n");

		if let Some(constructor) = constructor {
			document
				.write("\tconstructor({ props = {} } = {}) {\n")
				.write("\t\tthis.__data = new ")
				.write(&constructor)
				.write("({ props });\n")
				.write("\t\tthis.__nodes = ")
				.write(&render_fragment)
				.write(".apply(this.__data);\n");
		} else {
			document
				.write("\tconstructor() {\n")
				.write("\t\tthis.__nodes = ")
				.write(&render_fragment)
				.write(".apply(undefined);\n");
		}

		document
			.write("\t}\n\n")
			.write("\tinsert(target, anchor) {\n")
			.write("\t\t")
			.write(&insert_helper)
			.write("(target, anchor, ...this.__nodes);\n")
			.write("\t}\n\n")
			.write("\tdestroy() {\n")
			.write("\t\tthis.__data && this.__data.dispose();\n")
			.write("\t\t")
			.write(&destroy_helper)
			.write("(this.__nodes);\n")
			.write("\t}\n")
			.write("}\n");

		Ok(document)
	}

	pub(crate) fn render_fragment(&mut self, nodes: Vec<ast::Node>) -> Result<String, Error> {
		let frag = Fragment::new(self);
		frag.render(nodes)
	}

	fn render_dependency(&mut self, node: ast::DependencyStatement) -> Result<(), Error> {
		self.head.map(node.start).write("import ");

		if let Some(default) = node.default {
			let name_owner = default.local.as_ref().unwrap_or_else(|| &default.usage);
			let unique = self.unique.from(&name_owner.name);

			self.head
				.map(default.usage.start)
				.write(&unique)
				.map(name_owner.end);

			if !self.declare(&default.usage.name, &name_owner.name, &unique) {
				return Err(Error::compiler(
					default.start,
					default.end,
					&format!(
						"Variable '{:?}' for '{:?}' is already defined.",
						&name_owner.name, &default.usage
					),
				));
			}

			if node.named.is_some() {
				self.head.write(", ");
			} else {
				self.head.write(" ");
			}
		}

		if let Some(named) = node.named {
			self.head.map(named.start).write("{ ");

			let mut specifiers = named.nodes.iter().peekable();

			while let Some(specifier) = specifiers.next() {
				let name_owner = specifier
					.local
					.as_ref()
					.unwrap_or_else(|| &specifier.imported);
				let unique = self.unique.from(&name_owner.name);

				self.head
					.map(specifier.start)
					.write(&specifier.imported.name)
					.map(specifier.end);

				if &specifier.imported.name != &unique {
					self.head
						.write(" as ")
						.map(name_owner.start)
						.write(&unique)
						.map(name_owner.end);
				}

				if !self.declare(&specifier.usage.name, &name_owner.name, &unique) {
					return Err(Error::compiler(
						specifier.start,
						specifier.end,
						&format!(
							"Variable '{:?}' for '{:?}' is already defined.",
							&name_owner.name, &specifier.usage
						),
					));
				}

				if specifiers.peek().is_some() {
					self.head.write(", ");
				}
			}

			self.head.map(named.end).write(" } ");
		}

		self.head
			.write("from ")
			.map(node.source.start)
			.write(&serialize_string_literal(&node.source))
			.map(node.source.end)
			.write(";\n");

		Ok(())
	}

	pub(crate) fn helper(&mut self, name: &str) -> String {
		if self.helpers_map.contains_key(name) {
			self.helpers_map.get(name).unwrap().to_owned()
		} else {
			let unqiue = self.unique.ensure(name);
			self.helpers_map.insert(name.to_owned(), unqiue.clone());
			self.helpers.push((name.to_owned(), unqiue.clone()));
			unqiue
		}
	}

	pub(crate) fn find_declaration(&self, kind: &str, name: &str) -> Option<String> {
		if let Some(kind) = self.declaration_map.get(kind) {
			Some(kind.get(name).unwrap().to_owned())
		} else {
			None
		}
	}

	pub(crate) fn declare(&mut self, kind: &str, name: &str, unqiue: &str) -> bool {
		let kind = self
			.declaration_map
			.entry(kind.to_owned())
			.or_insert(HashMap::new());

		if kind.contains_key(name) {
			return false;
		}

		kind.insert(name.to_owned(), unqiue.to_owned());
		true
	}
}
