use std::collections::HashMap;

use parser::ast::NodeCollection;

use crate::{render::RESERVED_JAVASCRIPT_KEYWORDS, *};

const INTERNAL_MOD: &str = "@debrixjs/internal";

fn serialize_string_literal(lit: &ast::StringLiteral) -> String {
	lit.quote.to_string() + &lit.value + &lit.quote.to_string()
}

fn in_string(value: &str) -> String {
	"\"".to_owned() + &value.replace('"', "\\\"") + "\""
}

fn is_ident(char: &char) -> bool {
	char.is_alphanumeric() || char == &'$' || char == &'_'
}
fn to_valid_ident(value: &str) -> String {
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

fn join_spaces(value: &str) -> String {
	let mut string = String::new();
	let mut is_space = false;

	for char in value.chars() {
		if char.is_whitespace() {
			if !is_space {
				string.push(' ');
			}

			is_space = true;
			continue;
		}

		string.push(char);
	}

	string
}

fn serialize_javascript(expr: &ast::javascript::Expression) -> Chunk {
	let mut chunk = Chunk::new();

	match expr {
		ast::javascript::Expression::Identifier(expr) => {
			return serialize_javascript_identifier(expr, true);
		}
		ast::javascript::Expression::Literal(expr) => {
			chunk.map(expr.start()).write(&expr.raw()).map(expr.end());
		}
		ast::javascript::Expression::Unary(expr) => {
			let operator = expr.operator.to_string();

			chunk
				.map(expr.start)
				.write(&operator)
				.map(expr.start + operator.len())
				.write(" ")
				.append(&serialize_javascript(&expr.operand));
		}
		ast::javascript::Expression::Binary(expr) => {
			chunk
				.append(&serialize_javascript(&expr.left))
				.write(" ")
				.write(&expr.operator.to_string())
				.write(" ")
				.append(&serialize_javascript(&expr.right));
		}
		ast::javascript::Expression::Conditional(expr) => {
			chunk
				.append(&serialize_javascript(&expr.condition))
				.write(" ? ")
				.append(&serialize_javascript(&expr.consequent))
				.write(" : ")
				.append(&serialize_javascript(&expr.alternate));
		}
		ast::javascript::Expression::Call(expr) => {
			chunk.append(&serialize_javascript(&expr.callee)).write("(");

			let mut arguments = expr.arguments.iter().peekable();
			while let Some(arg) = arguments.next() {
				chunk.append(&serialize_javascript(arg));

				if arguments.peek().is_some() {
					chunk.write(", ");
				}
			}

			chunk.write(")");
		}
		ast::javascript::Expression::New(expr) => {
			chunk
				.map(expr.start)
				.write("new ")
				.append(&serialize_javascript(&expr.callee));

			if expr.arguments.len() > 0 {
				chunk.write("(");

				let mut arguments = expr.arguments.iter().peekable();
				while let Some(arg) = arguments.next() {
					chunk.append(&serialize_javascript(arg));

					if arguments.peek().is_some() {
						chunk.write(", ");
					}
				}

				chunk.write(")");
			}
		}
		ast::javascript::Expression::Member(expr) => {
			chunk.append(&serialize_javascript(&expr.object));

			if !expr.optional && !expr.computed {
				chunk.write(".");
			} else if expr.optional && !expr.computed {
				chunk.write("?.");
			} else if !expr.optional && expr.computed {
				chunk.write("[");
			} else if expr.optional && expr.computed {
				chunk.write("?.[");
			}

			chunk.append(&serialize_javascript(&expr.property));

			if expr.computed {
				chunk.write("]");
			}
		}
		ast::javascript::Expression::Function(expr) => {
			chunk.map(expr.start).write("(");

			let mut params = expr.parameters.iter().peekable();
			while let Some(param) = params.next() {
				chunk.append(&serialize_javascript(param));

				if params.peek().is_some() {
					chunk.write(", ");
				}
			}

			chunk
				.write(") => ")
				.append(&serialize_javascript(&expr.body));
		}
		ast::javascript::Expression::Assignment(expr) => {
			chunk
				.append(&serialize_javascript(&expr.left))
				.write(" ")
				.write(&expr.operator.to_string())
				.write(" ")
				.append(&serialize_javascript(&expr.right));
		}
		ast::javascript::Expression::Spread(expr) => {
			chunk
				.map(expr.start)
				.write("...")
				.append(&serialize_javascript(&expr.argument));
		}
		ast::javascript::Expression::Template(expr) => {
			return serialize_javascript_template(expr);
		}
		ast::javascript::Expression::TaggedTemplate(expr) => {
			chunk
				.append(&serialize_javascript(&expr.tag))
				.append(&serialize_javascript_template(&expr.quasi));
		}
		ast::javascript::Expression::Object(expr) => {
			chunk.map(expr.start).write("{");

			let mut props = expr.properties.iter().peekable();
			while let Some(prop) = props.next() {
				match prop {
					ast::javascript::ObjectProperty::Keyed(prop) => {
						chunk.append(&serialize_javascript_identifier(&prop.key, false));

						if let Some(value) = &prop.value {
							chunk.write(": ").append(&serialize_javascript(value));
						}
					}
					ast::javascript::ObjectProperty::Computed(prop) => {
						chunk
							.map(prop.start)
							.write("[")
							.append(&serialize_javascript(&prop.key))
							.write("]: ")
							.append(&serialize_javascript(&prop.value));
					}
					ast::javascript::ObjectProperty::Spread(prop) => {
						chunk
							.map(prop.start)
							.write("...")
							.append(&serialize_javascript(&prop.argument));
					}
				}

				if props.peek().is_some() {
					chunk.write(", ");
				}
			}

			chunk.write("}").map(expr.end);
		}
		ast::javascript::Expression::Array(expr) => {
			chunk.map(expr.start).write("[");

			let mut elements = expr.elements.iter().peekable();
			while let Some(expr) = elements.next() {
				chunk.append(&serialize_javascript(&expr));

				if elements.peek().is_some() {
					chunk.write(", ");
				}
			}

			chunk.write("]").map(expr.end);
		}
		ast::javascript::Expression::Parenthesized(expr) => {
			chunk
				.map(expr.start)
				.write("(")
				.append(&serialize_javascript(&expr.expression))
				.write(")")
				.map(expr.end);
		}
		ast::javascript::Expression::Empty(expr) => {
			chunk.map(expr.start).write(";").map(expr.end);
		}
	}

	chunk
}

fn serialize_javascript_identifier(
	expr: &ast::javascript::IdentifierExpression,
	translate: bool,
) -> Chunk {
	let mut chunk = Chunk::new();

	if translate {
		chunk.map(expr.start).write("this.");
	}

	chunk.write(&expr.name).map(expr.end);
	chunk
}

fn serialize_javascript_template(expr: &ast::javascript::TemplateLiteral) -> Chunk {
	// TODO: serialize all inline expressions in the template
	let mut chunk = Chunk::new();
	chunk.map(expr.start).write(&expr.raw).map(expr.end);
	chunk
}

pub struct Renderer {
	head: Chunk,
	decl: Chunk,
	init: Chunk,
	binding: Chunk,
	append: Chunk,
	idents: HashMap<String, usize>,
	usage: HashMap<String, String>,
	helpers: Vec<String>,
	imports: HashMap<String, String>,
}

// list of identifiers which cannot be used as a temporary variable
const STATIC_NAMES: [&str; 9] = [
	"comment",
	"element",
	"attr",
	"bind_attr",
	"bind",
	"space",
	"text",
	"bind_text",
	"append",
];

impl Renderer {
	pub fn new() -> Self {
		Self {
			head: Chunk::new(),
			decl: Chunk::new(),
			init: Chunk::new(),
			binding: Chunk::new(),
			append: Chunk::new(),
			idents: HashMap::new(),
			usage: HashMap::new(),
			helpers: Vec::new(),
			imports: HashMap::new(),
		}
	}

	fn unique(&mut self, name: &str) -> String {
		// generate unqiue identifer
		let mut ident = name.to_owned();

		if self.idents.contains_key(&ident) {
			let index = self.idents.get_mut(&ident).unwrap();
			*index += 1;
			ident.push_str("_");
			ident.push_str(&index.to_string());
		} else {
			self.idents.insert(ident.clone(), 0);

			if STATIC_NAMES.contains(&ident.as_ref()) {
				return self.unique(name);
			}
		}

		if RESERVED_JAVASCRIPT_KEYWORDS.contains(&ident.as_ref()) {
			ident.push_str("_");
		}

		ident
	}

	fn helper(&mut self, name: &str) -> String {
		let name = name.to_owned();

		if !self.helpers.contains(&name) {
			self.helpers.push(name.clone());
		}

		name
	}

	fn append(&mut self, parent: &str, children: &str) {
		let helper = self.helper("append");

		self.append
			.write(&helper)
			.write("(")
			.write(parent)
			.write(", [")
			.write(children)
			.write("]);\n");
	}

	pub fn render(mut self, document: ast::Document) -> Result<Chunk, Error> {
		let mut root_element = None;

		for node in document.children {
			match node {
				ast::Node::Comment(_)
				| ast::Node::Text(_)
				| ast::Node::TextBinding(_)
				| ast::Node::FlowControl(_) => {
					return Err(Error::compiler(
						node.start(),
						node.end(),
						"node is not allowed here",
					));
				}

				ast::Node::Element(_) => {
					if root_element.is_some() {
						return Err(Error::compiler(
							node.start(),
							node.end(),
							"node is not allowed here",
						));
					}

					root_element = Some(self.render_child(node)?);
				}

				_ => self.render_node(node)?,
			};
		}

		let root_element = if let Some(root_node) = root_element {
			root_node
		} else {
			return Err(Error::compiler(0, 0, "expected root element"));
		};

		if self.helpers.len() > 0 {
			let mut helpers = self.helpers.iter().peekable();

			self.head.write("import { ");

			while let Some(helper) = helpers.next() {
				self.head.write(helper);

				if helpers.peek().is_some() {
					self.head.write(", ");
				}
			}

			self.head
				.write(" } from \"")
				.write(INTERNAL_MOD)
				.write("\";\n");
		}

		let mut document = Chunk::new();
		let append_helper = self.helper("append");
		let render_ident = self.unique("render");

		document
			.append(&format_chunk(self.head, 0))
			.write("\n\nfunction ")
			.write(&render_ident)
			.write("() {\n")
			.write("\t/* declarations */\n")
			.append(&format_chunk(self.decl, 1))
			.write("\n\n\t/* initialization */\n")
			.append(&format_chunk(self.init, 1))
			.write("\n\n\t/* binding */\n")
			.append(&format_chunk(self.binding, 1))
			.write("\n\n\t/* appending */\n")
			.append(&format_chunk(self.append, 1))
			.write("\n\n\treturn ")
			.write(&root_element)
			.write(";\n")
			.write("}\n\n")
			.write("/* implements Component */\n")
			.write("export default class Greeting {\n");

		if self.usage.contains_key("model") {
			let local = self.usage.get("model").unwrap();

			document
				.write("\tconstructor({ props = {} } = {}) {\n")
				.write("\t\tlet data = new ")
				.write(local)
				.write("({ props });\n")
				.write("\t\tthis.__node = ")
				.write(&render_ident)
				.write(".apply(data);\n");
		} else {
			document
				.write("\tconstructor() {\n")
				.write("\t\tthis.__node = ")
				.write(&render_ident)
				.write(".apply(undefined);\n");
		}

		document
			.write("\t}\n\n")
			.write("\tattach(target, anchor) {\n")
			.write("\t\t")
			.write(&append_helper)
			.write("(target, [this.__node], anchor);\n")
			.write("\t}\n")
			.write("}\n");

		Ok(document)
	}

	fn render_node(&mut self, node: ast::Node) -> Result<(), Error> {
		match node {
			ast::Node::DependencyStatement(node) => self.render_dependency_statement(node),
			_ => {
				self.render_child(node)?;
				Ok(())
			}
		}
	}

	fn render_child(&mut self, node: ast::Node) -> Result<String, Error> {
		match node {
			ast::Node::Comment(node) => self.render_comment(node),
			ast::Node::Element(node) => self.render_element(node),
			ast::Node::Text(node) => self.render_text(node),
			ast::Node::TextBinding(node) => self.render_text_binding(node),
			ast::Node::FlowControl(node) => self.render_flow_control(node),

			_ => panic!(""),
		}
	}

	fn render_dependency_statement(&mut self, node: ast::DependencyStatement) -> Result<(), Error> {
		self.head.map(node.start).write("import ");

		if let Some(default) = node.default {
			let local = default
				.local
				.as_ref()
				.unwrap_or_else(|| default.usage.as_ref().unwrap());

			let local_name = self.unique(&local.name);

			if let Some(usage) = &default.usage {
				if self.usage.contains_key(&usage.name) {
					return Err(Error::compiler(
						usage.start,
						usage.end,
						&format!("{} has already been defined", usage.name),
					));
				}

				self.usage.insert(usage.name.clone(), local_name.clone());
			} else {
				self.imports.insert(local.name.clone(), local_name.clone());
			}

			// `default` may start with a '#'. The imported identifier should only be mapped to the
			// source identifier, excluding the '#'. Therefore, `default` location will not be used.
			self.head.map(local.start).write(&local_name).map(local.end);

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
				let imported = specifier
					.imported
					.as_ref()
					.unwrap_or_else(|| &specifier.usage.as_ref().unwrap());

				self.head
					.map(specifier.start)
					.write(&imported.name)
					.map(specifier.end);

				let mut imported_local_name = &imported.name;

				if let Some(local) = &specifier.local {
					imported_local_name = &local.name;

					self.head
						.write(" as ")
						.map(local.start)
						.write(&local.name)
						.map(local.end);
				}

				let local_name = self.unique(imported_local_name);

				if let Some(usage) = &specifier.usage {
					if self.usage.contains_key(&usage.name) {
						return Err(Error::compiler(
							usage.start,
							usage.end,
							&format!("{} has already been defined", usage.name),
						));
					}

					self.usage.insert(usage.name.clone(), local_name);
				} else {
					self.imports
						.insert(imported_local_name.clone(), local_name.clone());
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

	fn render_comment(&mut self, node: ast::Comment) -> Result<String, Error> {
		let helper = self.helper("comment");
		let name = self.unique("comment");

		self.decl
			.write("let ")
			.write(&name)
			.write(" = ")
			.map(node.start)
			.write(&helper)
			.write("(")
			.write(&in_string(&node.comment))
			.write(");\n")
			.map(node.end);

		Ok(name)
	}

	fn render_element(&mut self, node: ast::Element) -> Result<String, Error> {
		if let Some(local) = self.imports.get(&node.tag_name.name).cloned() {
			let name = self.unique(&to_valid_ident(&node.tag_name.name));

			self.decl
				.write("let ")
				.write(&name)
				.write(" = ")
				.map(node.start)
				.write("new ")
				.write(&local);

			if node.children.len() > 0 {
				let mut children = Vec::new();

				for child in node.children {
					children.push(self.render_child(child)?);
				}

				self.decl
					.write("({ children: [")
					.write(&children.join(", "))
					.write("] });\n")
					.map(node.end);
			} else {
				self.decl.write("(").write(");\n").map(node.end);
			}

			self.render_attributes(&name, node.attributes)?;

			if let Some(bindings) = node.bindings {
				self.render_bindings(&name, bindings)?;
			}

			Ok(name)
		} else {
			let helper = self.helper("element");
			let name = self.unique(&to_valid_ident(&node.tag_name.name));

			self.decl
				.write("let ")
				.write(&name)
				.write(" = ")
				.map(node.start)
				.write(&helper)
				.write("(")
				.map(node.tag_name.start)
				.write(&in_string(&node.tag_name.name))
				.map(node.tag_name.end)
				.write(");\n")
				.map(node.end);

			self.render_attributes(&name, node.attributes)?;

			if let Some(bindings) = node.bindings {
				self.render_bindings(&name, bindings)?;
			}

			let mut children = Vec::new();

			for child in node.children {
				children.push(self.render_child(child)?);
			}

			self.append(&name, &children.join(", "));

			Ok(name)
		}
	}

	fn render_attributes(
		&mut self,
		parent: &str,
		attributes: Vec<ast::Attribute>,
	) -> Result<(), Error> {
		for attr in attributes {
			match attr {
				ast::Attribute::Static(attr) => {
					let helper = self.helper("attr");

					self.init
						.map(attr.start)
						.write(&helper)
						.write("(")
						.write(parent)
						.write(", ")
						.map(attr.name.start)
						.write(&in_string(&attr.name.name))
						.map(attr.name.end);

					if let Some(value) = attr.value {
						self.init
							.write(", ")
							.map(value.start)
							.write(&serialize_string_literal(&value))
							.map(value.end);
					}

					self.init.write(");\n").map(attr.end);
				}
				ast::Attribute::Binding(binding) => {
					let helper = self.helper("bind_attr");

					self.init
						.map(binding.start)
						.write(&helper)
						.write("(")
						.write(parent)
						.write(", ")
						.write(&in_string(&binding.name.name))
						.write(", ")
						.append(&serialize_javascript(&binding.value))
						.write(");\n");
				}
			}
		}

		Ok(())
	}

	fn render_bindings(
		&mut self,
		parent: &str,
		bindings: NodeCollection<ast::Binding>,
	) -> Result<(), Error> {
		let helper = self.helper("bind");

		for binding in bindings.nodes {
			self.binding
				.write(&helper)
				.write("(")
				.write(parent)
				.write(", ")
				.write(&to_valid_ident(&binding.name.name))
				.write(", this.$computed(() => ")
				.append(&serialize_javascript(&binding.value))
				.write("));\n");
		}

		Ok(())
	}

	fn render_text(&mut self, node: ast::Text) -> Result<String, Error> {
		let text = join_spaces(&node.content);

		if text == " " {
			let helper = self.helper("space");
			let name = self.unique("space");

			self.decl
				.write("let ")
				.write(&name)
				.write(" = ")
				.map(node.start)
				.write(&helper)
				.write("();\n")
				.map(node.end);

			Ok(name)
		} else {
			let helper = self.helper("text");
			let name = self.unique("text");

			self.decl
				.write("let ")
				.write(&name)
				.write(" = ")
				.map(node.start)
				.write(&helper)
				.write("(")
				.write(&in_string(&text))
				.write(");\n")
				.map(node.end);

			Ok(name)
		}
	}

	fn render_text_binding(&mut self, node: ast::TextBinding) -> Result<String, Error> {
		let helper = self.helper("text");
		let name = self.unique("text");

		self.decl
			.write("let ")
			.write(&name)
			.write(" = ")
			.map(node.start)
			.write(&helper)
			.write("();\n")
			.map(node.end);

		let helper = self.helper("bind_text");

		self.binding
			.write(&helper)
			.write("(")
			.write(&name)
			.write(", this.$computed(() => ")
			.append(&serialize_javascript(&node.expression))
			.write("));\n");

		Ok(name)
	}

	fn render_flow_control(&mut self, _node: ast::FlowControl) -> Result<String, Error> {
		Ok("undefined".to_owned())
	}
}
