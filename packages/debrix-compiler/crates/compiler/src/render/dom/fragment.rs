use super::*;

pub struct Fragment {
	pub js: JavascriptSerializer,
	c_attr: Chunk,
	c_decl: Chunk,
	c_init: Chunk,
	c_bind: Chunk,
	c_insert: Chunk,
}

impl Fragment {
	pub fn new() -> Self {
		Self {
			js: JavascriptSerializer::new(),
			c_attr: Chunk::new(),
			c_decl: Chunk::new(),
			c_init: Chunk::new(),
			c_bind: Chunk::new(),
			c_insert: Chunk::new(),
		}
	}

	pub fn render_single(
		self,
		doc: &mut Document,
		name: String,
		node: ast::Node,
	) -> Result<(), Error> {
		self.render(doc, name, vec![node])
	}

	pub fn render(
		mut self,
		doc: &mut Document,
		name: String,
		nodes: Vec<ast::Node>,
	) -> Result<(), Error> {
		self.js.local_vars.push("$self".to_owned());

		let mut elements = Vec::new();

		for node in nodes {
			elements.append(&mut self.render_node(doc, node)?);
		}

		doc.c_fragments
			.write("function ")
			.write(&name)
			.write("(")
			.write("$self");

		if self.c_attr.source.len() > 0 {
			doc.c_fragments.write(", ").append(&self.c_attr);
		}

		doc.c_fragments.write(") {\n");

		if self.c_decl.source.len() > 0 {
			doc.c_fragments
				.write("\t/* declarations */\n")
				.append(&format_chunk(self.c_decl, 1));
		}

		if self.c_init.source.len() > 0 {
			doc.c_fragments
				.write("\n\n\t/* initialization */\n")
				.append(&format_chunk(self.c_init, 1));
		}

		if self.c_bind.source.len() > 0 {
			doc.c_fragments
				.write("\n\n\t/* binding */\n")
				.append(&format_chunk(self.c_bind, 1));
		}

		if self.c_insert.source.len() > 0 {
			doc.c_fragments
				.write("\n\n\t/* appending */\n")
				.append(&format_chunk(self.c_insert, 1));
		}

		doc.c_fragments
			.write("\n\n\treturn [")
			.write(&elements.join(", "))
			.write("];\n")
			.write("}\n\n");

		Ok(())
	}

	fn render_node(&mut self, doc: &mut Document, node: ast::Node) -> Result<Vec<String>, Error> {
		match node {
			ast::Node::Comment(node) => Ok(vec![self.render_comment(doc, node)?]),
			ast::Node::Element(node) => Ok(vec![self.render_element(doc, node)?]),
			ast::Node::Text(node) => Ok(vec![self.render_text(doc, node)?]),
			ast::Node::TextBinding(node) => Ok(vec![self.render_text_binding(doc, node)?]),
			ast::Node::FlowControl(node) => self.render_flow_control(doc, node),

			_ => Err(Error::compiler(
				node.start(),
				node.end(),
				"Node is not allowed here.",
			)),
		}
	}

	fn insert(&mut self, doc: &mut Document, target: &str, previous: &str, nodes: &str) {
		let insert = doc.import("insert", None, INTERNAL_MODULE);
		self.c_insert
			.write(&insert)
			.write("(")
			.write(target)
			.write(", ")
			.write(&previous)
			.write(", ")
			.write(&nodes)
			.write(");\n");
	}

	fn render_comment(&mut self, doc: &mut Document, node: ast::Comment) -> Result<String, Error> {
		let helper = doc.import("comment", None, INTERNAL_MODULE);
		let name = doc.unique.from("comment");

		self.c_decl
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

	fn render_element(&mut self, doc: &mut Document, node: ast::Element) -> Result<String, Error> {
		let mut first_spread_attribute: Option<ast::Range> = None;
		for attr in &node.attributes {
			match attr {
				ast::Attribute::Spread(attr) => {
					if let Some(first) = first_spread_attribute {
						return Err(Error::compiler(
							attr.start,
							attr.end,
							&format!(
								"Attributes can only be expanded once. Occured at {:?}.",
								first
							),
						));
					} else {
						first_spread_attribute = Some(attr.range());
					}
				}
				_ => continue,
			}
		}

		if let Some(constructor) =
			doc.declaration(DeclarationKind::Component, Some(&node.tag_name.name))
		{
			let constructor = constructor.to_owned();

			let name = doc.unique.from(&to_valid_identifier(&node.tag_name.name));
			let mut slots: Map<String, Vec<ast::Node>> = Map::new();
			let mut c_attrs = Chunk::new();

			for node in node.children {
				let mut slot_name = None;
				match &node {
					ast::Node::Element(node) => {
						for attr in node.attributes.iter() {
							match attr {
								ast::Attribute::Static(attr) => {
									if attr.name.name == "slot" {
										if slot_name.is_some() {
											return Err(Error::compiler(
												attr.start,
												attr.end,
												"Special attribute cannot be defined twice.",
											));
										}

										if let Some(literal) = &attr.value {
											slot_name = Some(literal.value.to_owned());
										} else {
											return Err(Error::compiler(
												attr.start,
												attr.end,
												"Special attribute must have value.",
											));
										}
									}
								}
								ast::Attribute::Binding(attr) => {
									if attr.name.name == "slot" {
										return Err(Error::compiler(
											attr.start,
											attr.end,
											"Special attribute must be static.",
										));
									}
								}
								ast::Attribute::ShortBinding(attr) => {
									if attr.name.name == "slot" {
										return Err(Error::compiler(
											attr.start,
											attr.end,
											"Special attribute must be static.",
										));
									}
								}
								ast::Attribute::Spread(_) => continue,
							}
						}
					}
					_ => {}
				}

				let slot_name = slot_name.unwrap_or(DEFAULT_SLOT_NAME.to_owned());
				if let Some(vec) = slots.get_mut(&slot_name) {
					vec.push(node);
				} else {
					slots.set(slot_name, vec![node]);
				}
			}

			for attribute in node.attributes {
				match attribute {
					ast::Attribute::Static(attribute) => {
						if &attribute.name.name == "slot" {
							// TODO:
							continue;
						}

						c_attrs
							.write(&to_valid_property(&attribute.name.name))
							.write(": ")
							.write(&in_string(
								&attribute.value.map(|v| v.value).unwrap_or("".to_owned()),
							))
							.write(",\n");
					}

					ast::Attribute::Binding(attribute) => {
						c_attrs
							.write(&to_valid_property(&attribute.name.name))
							.write(": this.$computed(() => ")
							.append(&self.js.serialize(&attribute.value))
							.write("),\n");
					}

					ast::Attribute::ShortBinding(attribute) => {
						c_attrs
							.write(&to_valid_property(&attribute.name.name))
							.write(": this.$computed(() => ")
							.append(&self.js.serialize(&attribute.name.into()))
							.write("),\n");
					}

					ast::Attribute::Spread(attribute) => {
						c_attrs
							.write("_: ")
							.append(&self.js.serialize(&attribute.value))
							.write(",\n");
					}
				}
			}

			self.c_decl
				.write("let ")
				.write(&name)
				.write(" = ")
				.map(node.start)
				.write("new ")
				.write(&constructor);

			self.c_decl
				.write("({\n")
				.write("\t__family: ")
				.write(&in_string("__family"))
				.write(" in ")
				.write(&constructor)
				.write(" && $self[")
				.write(&constructor)
				.write(".__family")
				.write("],\n");

			if !slots.is_empty() {
				self.c_decl.write("\tslots: {\n");

				let mut slots = slots.into_entries().peekable();
				while let Some((name, nodes)) = slots.next() {
					let fragment_name = doc.unique.from("render_fragment");
					let fragment = Fragment::new();
					fragment.render(doc, fragment_name.clone(), nodes)?;

					self.c_decl
						.write("\t\t")
						.write(&to_valid_property(&name))
						.write(": ")
						.write(&fragment_name)
						.write(".bind(this)");

					if slots.peek().is_some() {
						self.c_decl.write(",\n");
					} else {
						self.c_decl.write("\n");
					}
				}

				self.c_decl.write("\t},\n");
			}

			if !c_attrs.is_empty() {
				self.c_decl
					.write("\tattrs: {\n")
					.append(&format_chunk(c_attrs, 2))
					.write("\n\t},\n");
			}

			self.c_decl.write("});\n");

			self.c_decl.map(node.end);

			return Ok(name);
		}

		if &node.tag_name.name == "slot" {
			let slot_name =
				if let Some(attr) = node.attributes.iter().find(find_static_attr("name")) {
					match attr {
						ast::Attribute::Static(attr) => {
							if let Some(literal) = &attr.value {
								literal.value.clone()
							} else {
								return Err(Error::compiler(
									attr.start,
									attr.end,
									"Attribute must have value.",
								));
							}
						}
						_ => {
							return Err(Error::compiler(
								attr.start(),
								attr.end(),
								"Attribute must be static.",
							))
						}
					}
				} else {
					DEFAULT_SLOT_NAME.to_owned()
				};

			let instance_name = doc.unique.from("fragment");

			self.c_decl
				.write("let ")
				.write(&instance_name)
				.write(" = $self.slots");

			if is_valid_identifier(&slot_name) {
				self.c_decl.write(".").write(&slot_name);
			} else {
				self.c_decl
					.write("[")
					.write(&in_string(&slot_name))
					.write("]");
			}

			self.c_decl.write(" && $self.slots");

			if is_valid_identifier(&slot_name) {
				self.c_decl.write(".").write(&slot_name);
			} else {
				self.c_decl
					.write("[")
					.write(&in_string(&slot_name))
					.write("]");
			}

			self.c_decl.write("($self);\n");

			return Ok(instance_name);
		}

		let tag_name = if node.tag_name.name.starts_with("html:") {
			&node.tag_name.name[5..]
		} else {
			&node.tag_name.name
		};

		let helper = doc.import("element", None, INTERNAL_MODULE);
		let name = doc.unique.from(&to_valid_identifier(tag_name));

		self.c_decl
			.write("let ")
			.write(&name)
			.write(" = ")
			.map(node.start)
			.write(&helper)
			.write("(")
			.map(node.tag_name.start)
			.write(&in_string(tag_name))
			.map(node.tag_name.end)
			.write(");\n")
			.map(node.end);

		for attribute in node.attributes.into_iter() {
			match &attribute {
				ast::Attribute::Static(attribute) => {
					if &attribute.name.name == "slot" {
						continue;
					}

					if &attribute.name.name == "as" {
						continue;
					}
				}
				_ => {}
			}

			self.render_attribute(doc, &name, attribute)?;
		}

		if node.children.len() > 0 {
			let mut args = Vec::new();

			for child in node.children {
				args.append(&mut self.render_node(doc, child)?);
			}

			self.insert(doc, &name, "null", &args.join(", "));
		}

		Ok(name)
	}

	fn render_attribute(
		&mut self,
		doc: &mut Document,
		parent: &str,
		attribute: ast::Attribute,
	) -> Result<(), Error> {
		match attribute {
			ast::Attribute::Static(attr) => {
				let helper = doc.import("attr", None, INTERNAL_MODULE);

				self.c_init
					.map(attr.start)
					.write(&helper)
					.write("(")
					.write(parent)
					.write(", ")
					.map(attr.name.start)
					.write(&in_string(&attr.name.name))
					.map(attr.name.end);

				if let Some(value) = attr.value {
					self.c_init
						.write(", ")
						.map(value.start)
						.write(&serialize_string_literal(&value))
						.map(value.end);
				}

				self.c_init.write(");\n").map(attr.end);
			}
			ast::Attribute::Binding(attr) => {
				if attr.name.name.starts_with("bind:") {
					let name = &attr.name.name[5..];
					let helper = doc.import("bind", None, INTERNAL_MODULE);

					let decl = doc.declaration(DeclarationKind::Binder, Some(name));

					if decl.is_none() {
						return Err(Error::compiler(
							attr.name.start,
							attr.name.end,
							&format!("Undefined binder '{}'.", name),
						));
					}

					let decl = decl.unwrap();

					self.c_bind
						.write(&helper)
						.write("(")
						.write(parent)
						.write(", ")
						.write(&decl)
						.write(", this.$computed(() => ")
						.append(&self.js.serialize(&attr.value))
						.write("));\n");
				} else {
					let helper = doc.import("bind_attr", None, INTERNAL_MODULE);

					self.c_bind
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
				let helper = doc.import("bind_attr_spread", None, INTERNAL_MODULE);

				self.c_bind
					.map(attr.start)
					.write(&helper)
					.write("(")
					.write(parent)
					.write(", this.$computed(() => ")
					.append(&self.js.serialize(&attr.value))
					.write("));\n");
			}
			ast::Attribute::ShortBinding(attr) => {
				let helper = doc.import("bind_attr", None, INTERNAL_MODULE);

				self.c_bind
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

		Ok(())
	}

	fn render_text(&mut self, doc: &mut Document, node: ast::Text) -> Result<String, Error> {
		let text = join_spaces(&node.content);

		if text == " " {
			let helper = doc.import("space", None, INTERNAL_MODULE);
			let name = doc.unique.from("space");

			self.c_decl
				.write("let ")
				.write(&name)
				.write(" = ")
				.map(node.start)
				.write(&helper)
				.write("();\n")
				.map(node.end);

			Ok(name)
		} else {
			let helper = doc.import("text", None, INTERNAL_MODULE);
			let name = doc.unique.from("text");

			self.c_decl
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

	fn render_text_binding(
		&mut self,
		doc: &mut Document,
		node: ast::TextBinding,
	) -> Result<String, Error> {
		let helper = doc.import("text", None, INTERNAL_MODULE);
		let name = doc.unique.from("text");

		self.c_decl
			.write("let ")
			.write(&name)
			.write(" = ")
			.map(node.start)
			.write(&helper)
			.write("();\n")
			.map(node.end);

		let helper = doc.import("bind_text", None, INTERNAL_MODULE);

		self.c_bind
			.write(&helper)
			.write("(")
			.write(&name)
			.write(", this.$computed(() => ")
			.append(&self.js.serialize(&node.expression))
			.write("));\n");

		Ok(name)
	}

	fn render_flow_control(
		&mut self,
		doc: &mut Document,
		node: ast::FlowControl,
	) -> Result<Vec<String>, Error> {
		match node {
			ast::FlowControl::When(node) => {
				let mut names = Vec::new();

				let instance_name = doc.unique.from("fragment");
				let fragment_name = doc.unique.from("render_fragment");
				let fragment = Fragment::new();
				fragment.render(doc, fragment_name.clone(), node.children)?;

				self.c_decl
					.write("let ")
					.write(&instance_name)
					.write(" = ")
					.write(&fragment_name)
					.write(".call(this);\n");

				let bind_when = doc.import("bind_when", None, INTERNAL_MODULE);
				let binding_name = doc.unique.from("flow");
				names.push(binding_name.clone());

				let mut accessor = Chunk::new();
				accessor
					.write("this.$computed(() => ")
					.append(&self.js.serialize(&node.condition))
					.write("))");

				if node.chain.len() > 0 {
					let accessor_name = doc.unique.from("accessor");

					self.c_bind
						.write("\nlet ")
						.write(&accessor_name)
						.write(" = ")
						.append(&accessor)
						.write(";\n");

					self.c_bind
						.write("let ")
						.write(&binding_name)
						.write(" = ")
						.write(&bind_when)
						.write("(")
						.write(&instance_name)
						.write(", ")
						.write(&accessor_name)
						.write("\n");

					for node in node.chain {
						let instance_name = doc.unique.from("fragment");
						let fragment_name = doc.unique.from("render_fragment");
						let fragment = Fragment::new();
						fragment.render(doc, fragment_name.clone(), node.children)?;

						self.c_decl
							.write("let ")
							.write(&instance_name)
							.write(" = ")
							.write(&fragment_name)
							.write(".call(this);\n");

						let computed_not = doc.import("computed_not", None, INTERNAL_MODULE);
						let binding_name = doc.unique.from("flow");
						names.push(binding_name.clone());

						self.c_bind
							.write("let ")
							.write(&binding_name)
							.write(" = ")
							.write(&bind_when)
							.write("(")
							.write(&instance_name)
							.write(", ")
							.write(&computed_not)
							.write("(")
							.write(&accessor_name)
							.write(");\n");
					}
				} else {
					self.c_bind
						.write("let ")
						.write(&binding_name)
						.write(" = ")
						.write(&bind_when)
						.write("(")
						.write(&instance_name)
						.write(", ")
						.append(&accessor)
						.write(";\n");
				}

				Ok(names)
			}

			ast::FlowControl::Each(node) => {
				let bind_each = doc.import("bind_each", None, INTERNAL_MODULE);
				let name = doc.unique.from("flow");

				let fragment_name = doc.unique.from("render_fragment");
				let mut fragment = Fragment::new();

				let iterator = doc.unique.ensure(&node.iterator.name);
				fragment.js.local_vars.push(iterator.clone());

				let mut attr = Chunk::new();
				attr.map(node.iterator.start)
					.write(&iterator)
					.map(node.iterator.end);

				fragment.c_attr.append(&attr);
				fragment.render(doc, fragment_name.clone(), node.children)?;

				self.c_bind
					.write("let ")
					.write(&name)
					.write(" = ")
					.write(&bind_each)
					.write("(")
					.write(&fragment_name)
					.write(".bind(this), this.$computed(() => ")
					.append(&self.js.serialize(&node.iterable))
					.write("));\n");

				Ok(vec![name])
			}
		}
	}
}
