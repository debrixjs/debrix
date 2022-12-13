use super::*;

pub struct Fragment<'a> {
	pub doc: &'a mut Document,
	pub unique: Unique,
	pub js: JavascriptSerializer,

	attr: Chunk,
	decl: Chunk,
	init: Chunk,
	bind: Chunk,
	insert: Chunk,
}

impl<'a> Fragment<'a> {
	pub fn new(doc: &'a mut Document) -> Self {
		Self {
			doc,
			unique: Unique::new(),
			js: JavascriptSerializer::new(),

			attr: Chunk::new(),
			decl: Chunk::new(),
			init: Chunk::new(),
			bind: Chunk::new(),
			insert: Chunk::new(),
		}
	}

	pub fn render(mut self, nodes: Vec<ast::Node>) -> Result<String, Error> {
		let render_fragment = self.doc.unique.from("render_fragment");
		let mut elements = Vec::new();

		for node in nodes {
			elements.append(&mut self.render_node(node)?);
		}

		self.doc
			.frags
			.write("function ")
			.write(&render_fragment)
			.write("(")
			.append(&self.attr)
			.write(") {\n");

		if self.decl.source.len() > 0 {
			self.doc
				.frags
				.write("\t/* declarations */\n")
				.append(&format_chunk(self.decl, 1));
		}

		if self.init.source.len() > 0 {
			self.doc
				.frags
				.write("\n\n\t/* initialization */\n")
				.append(&format_chunk(self.init, 1));
		}

		if self.bind.source.len() > 0 {
			self.doc
				.frags
				.write("\n\n\t/* binding */\n")
				.append(&format_chunk(self.bind, 1));
		}

		if self.insert.source.len() > 0 {
			self.doc
				.frags
				.write("\n\n\t/* appending */\n")
				.append(&format_chunk(self.insert, 1));
		}

		self.doc
			.frags
			.write("\n\n\treturn [")
			.write(&elements.join(", "))
			.write("];\n")
			.write("}\n\n");

		Ok(render_fragment)
	}

	fn render_node(&mut self, node: ast::Node) -> Result<Vec<String>, Error> {
		match node {
			ast::Node::Comment(node) => Ok(vec![self.render_comment(node)?]),
			ast::Node::Element(node) => Ok(vec![self.render_element(node)?]),
			ast::Node::Text(node) => Ok(vec![self.render_text(node)?]),
			ast::Node::TextBinding(node) => Ok(vec![self.render_text_binding(node)?]),
			ast::Node::FlowControl(node) => self.render_flow_control(node),

			_ => Err(Error::compiler(
				node.start(),
				node.end(),
				"Node is not allowed here.",
			)),
		}
	}

	fn insert(&mut self, target: &str, previous: &str, nodes: &str) {
		let insert = self.doc.helper("insert");
		self.insert
			.write(&insert)
			.write("(")
			.write(target)
			.write(", ")
			.write(&previous)
			.write(", ")
			.write(&nodes)
			.write(");");
	}

	fn render_comment(&mut self, node: ast::Comment) -> Result<String, Error> {
		let helper = self.doc.helper("comment");
		let name = self.unique.from("comment");

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
		let decl = self
			.doc
			.find_declaration(COMPONENT_KIND, &node.tag_name.name);

		if let Some(decl) = decl {
			let name = self.unique.from(&to_valid_ident(&node.tag_name.name));

			self.decl
				.write("let ")
				.write(&name)
				.write(" = ")
				.map(node.start)
				.write("new ")
				.write(&decl);

			if node.children.len() > 0 {
				let mut children = Vec::new();

				for child in node.children {
					children.append(&mut self.render_node(child)?);
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

			return Ok(name);
		}

		let helper = self.doc.helper("element");
		let name = self.unique.from(&to_valid_ident(&node.tag_name.name));

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

		let mut children = Vec::new();

		for child in node.children {
			children.append(&mut self.render_node(child)?);
		}

		self.insert(&name, "null", &children.join(", "));

		Ok(name)
	}

	fn render_attributes(
		&mut self,
		parent: &str,
		attributes: Vec<ast::Attribute>,
	) -> Result<(), Error> {
		for attr in attributes {
			match attr {
				ast::Attribute::Static(attr) => {
					let helper = self.doc.helper("attr");

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
				ast::Attribute::Binding(attr) => {
					if attr.name.name.starts_with("bind:") {
						let name = &attr.name.name[5..];
						let helper = self.doc.helper("bind");

						let decl = self.doc.find_declaration(BINDER_KIND, name);

						if decl.is_none() {
							return Err(Error::compiler(
								attr.name.start,
								attr.name.end,
								&format!("Undefined binder '{}'.", name),
							));
						}

						let decl = decl.unwrap();

						self.bind
							.write(&helper)
							.write("(")
							.write(parent)
							.write(", ")
							.write(&decl)
							.write(", this.$computed(() => ")
							.append(&self.js.serialize(&attr.value))
							.write("));\n");
					} else {
						let helper = self.doc.helper("bind_attr");

						self.bind
							.map(attr.start)
							.write(&helper)
							.write("(")
							.write(parent)
							.write(", ")
							.write(&in_string(&attr.name.name))
							.write(", this.$computed(() => ")
							.append(&self.js.serialize(&attr.value))
							.write("));\n");
					}
				}
				ast::Attribute::Spread(attr) => {
					let helper = self.doc.helper("bind_attr_spread");

					self.bind
						.map(attr.start)
						.write(&helper)
						.write("(")
						.write(parent)
						.write(", this.$computed(() => ")
						.append(&self.js.serialize(&attr.value))
						.write("));\n");
				}
				ast::Attribute::ShortBinding(attr) => {
					let helper = self.doc.helper("bind_attr");

					self.bind
						.map(attr.start)
						.write(&helper)
						.write("(")
						.write(parent)
						.write(", ")
						.write(&in_string(&attr.name.name))
						.write(", this.$computed(() => ")
						.append(&self.js.serialize(&attr.name.into()))
						.write("));\n");
				}
			}
		}

		Ok(())
	}

	fn render_text(&mut self, node: ast::Text) -> Result<String, Error> {
		let text = join_spaces(&node.content);

		if text == " " {
			let helper = self.doc.helper("space");
			let name = self.unique.from("space");

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
			let helper = self.doc.helper("text");
			let name = self.unique.from("text");

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
		let helper = self.doc.helper("text");
		let name = self.unique.from("text");

		self.decl
			.write("let ")
			.write(&name)
			.write(" = ")
			.map(node.start)
			.write(&helper)
			.write("();\n")
			.map(node.end);

		let helper = self.doc.helper("bind_text");

		self.bind
			.write(&helper)
			.write("(")
			.write(&name)
			.write(", this.$computed(() => ")
			.append(&self.js.serialize(&node.expression))
			.write("));\n");

		Ok(name)
	}

	fn render_flow_control(&mut self, node: ast::FlowControl) -> Result<Vec<String>, Error> {
		match node {
			ast::FlowControl::When(node) => {
				let mut names = Vec::new();

				let fragment_name = self.unique.from("fragment");
				let render_fragment = self.doc.render_fragment(node.children)?;

				self.decl
					.write("let ")
					.write(&fragment_name)
					.write(" = ")
					.write(&render_fragment)
					.write(".apply(this);\n");

				let bind_when = self.doc.helper("bind_when");
				let binding_name = self.unique.from("flow");
				names.push(binding_name.clone());

				let mut accessor = Chunk::new();
				accessor
					.write("this.$computed(() => ")
					.append(&self.js.serialize(&node.condition))
					.write("))");

				if node.chain.len() > 0 {
					let accessor_name = self.unique.from("accessor");

					self.bind
						.write("\nlet ")
						.write(&accessor_name)
						.write(" = ")
						.append(&accessor)
						.write(";\n");

					self.bind
						.write("let ")
						.write(&binding_name)
						.write(" = ")
						.write(&bind_when)
						.write("(")
						.write(&fragment_name)
						.write(", ")
						.write(&accessor_name)
						.write("\n");

					for node in node.chain {
						let fragment_name = self.unique.from("fragment");
						let render_fragment = self.doc.render_fragment(node.children)?;

						self.decl
							.write("let ")
							.write(&fragment_name)
							.write(" = ")
							.write(&render_fragment)
							.write(".apply(this);\n");

						let computed_not = self.doc.helper("computed_not");
						let binding_name = self.unique.from("flow");
						names.push(binding_name.clone());

						self.bind
							.write("let ")
							.write(&binding_name)
							.write(" = ")
							.write(&bind_when)
							.write("(")
							.write(&fragment_name)
							.write(", ")
							.write(&computed_not)
							.write("(")
							.write(&accessor_name)
							.write(");\n");
					}
				} else {
					self.bind
						.write("let ")
						.write(&binding_name)
						.write(" = ")
						.write(&bind_when)
						.write("(")
						.write(&fragment_name)
						.write(", ")
						.append(&accessor)
						.write(";\n");
				}

				Ok(names)
			}

			ast::FlowControl::Each(node) => {
				let bind_each = self.doc.helper("bind_each");
				let name = self.unique.from("flow");

				let mut frag = Fragment::new(self.doc);

				let iterator = frag.unique.ensure(&node.iterator.name);
				frag.js.local_vars.push(iterator.clone());

				let mut attr = Chunk::new();
				attr.map(node.iterator.start)
					.write(&iterator)
					.map(node.iterator.end);

				frag.attr.append(&attr);
				let render_fragment = frag.render(node.children)?;

				self.bind
					.write("let ")
					.write(&name)
					.write(" = ")
					.write(&bind_each)
					.write("(")
					.write(&render_fragment)
					.write(".bind(this), this.$computed(() => ")
					.append(&self.js.serialize(&node.iterable))
					.write("));\n");

				Ok(vec![name])
			}
		}
	}
}
