use super::*;

struct Import {
	name: String,
	alias: Option<String>,
	from: String,
}

impl Import {
	fn new(name: String, alias: Option<String>, from: String) -> Self {
		Self { name, alias, from }
	}
}

struct Export {
	name: String,
	alias: Option<String>,
	from: Option<String>,
}

impl Export {
	fn new(name: String, alias: Option<String>, from: Option<String>) -> Self {
		Self { name, alias, from }
	}
}

#[derive(PartialEq, Clone)]
pub enum DeclarationKind {
	Model,
	Component,
	Binder,
	Unknown(String),
}

impl DeclarationKind {
	pub fn from(s: &str) -> Self {
		match s {
			"model" => DeclarationKind::Model,
			"component" => DeclarationKind::Component,
			"binder" => DeclarationKind::Binder,
			_ => DeclarationKind::Unknown(s.to_owned()),
		}
	}
}

struct Declaration {
	kind: DeclarationKind,
	name: Option<String>,
	unqiue: String,
}

pub struct Document {
	pub c_imports: Chunk,
	pub c_exports: Chunk,
	pub c_fragments: Chunk,
	pub c_components: Chunk,
	pub unique: Unique,
	imports: Vec<Import>,
	exports: Vec<Export>,
	declarations: Vec<Declaration>,
}

impl Document {
	pub fn new() -> Self {
		Self {
			c_imports: Chunk::new(),
			c_exports: Chunk::new(),
			c_fragments: Chunk::new(),
			c_components: Chunk::new(),
			unique: Unique::new(),
			imports: Vec::new(),
			exports: Vec::new(),
			declarations: Vec::new(),
		}
	}

	pub fn import(&mut self, name: &str, alias: Option<&str>, from: &str) -> String {
		if let Some(import) = self
			.imports
			.iter()
			.find(|i| i.name == name && i.from == from)
		{
			return import
				.alias
				.as_ref()
				.map(|s| s.to_owned())
				.unwrap_or(import.name.clone());
		}

		let alias = self.unique.ensure(alias.unwrap_or(name));
		let alias_opt = if alias == name {
			None
		} else {
			Some(alias.clone())
		};

		self.imports.push(Import::new(
			name.to_owned(),
			alias_opt.clone(),
			from.to_owned(),
		));

		alias
	}

	pub fn export(&mut self, name: &str, alias: Option<&str>, from: Option<&str>) {
		if self
			.imports
			.iter()
			.find(|i| i.alias.as_ref().unwrap_or(&i.name) == alias.unwrap_or(name))
			.is_some()
		{
			if let Some(alias) = alias {
				panic!("{} (as {}) is already exported", name, alias);
			} else {
				panic!("{} is already exported", name);
			}
		}

		self.exports.push(Export::new(
			name.to_owned(),
			alias.map(|s| s.to_owned()),
			from.map(|s| s.to_owned()),
		));
	}

	pub fn render(mut self, document: ast::Document) -> Result<Chunk, Error> {
		let mut exports: Map<String, ast::Element> = Map::new();
		for node in document.children {
			match node {
				ast::Node::Element(node) => {
					let is_attr = if let Some(attr) =
						node.attributes.iter().find(find_static_attr("as"))
					{
						match attr {
							ast::Attribute::Static(attr) => {
								if let Some(literal) = &attr.value {
									if !is_valid_identifier(&literal.value) {
										return Err(Error::compiler(
												literal.start + 1,
												literal.end - 1,
												&format!("Component cannot be named non-valid javascript identifiers. \"{}\" is not a valid identifier.", literal.value),
											));
									}

									Some(attr)
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
						None
					};

					let is = if let Some(attr) = is_attr {
						&attr.value.as_ref().unwrap().value
					} else {
						"default"
					};

					if exports.has(is) {
						let range = if let Some(attr) = is_attr {
							attr.range()
						} else {
							node.start_tag
						};

						return Err(Error::compiler(
							range.start,
							range.end,
							&if is == "default" {
								"Component is already defined!".to_owned()
							} else {
								format!("Component {} is already defined!", is)
							},
						));
					}

					exports.set(is.to_owned(), node);
				}

				ast::Node::DependencyStatement(node) => {
					self.render_dependency(node)?;
				}

				_ => {
					return Err(Error::compiler(
						node.start(),
						node.end(),
						"Node is not allowed here.",
					))
				}
			};
		}

		let mut component_names: Map<String, String> = Map::new();
		for (name, _node) in exports.entries() {
			let component_name = self.unique.ensure(&title_case(&name));
			if !self.declare(
				DeclarationKind::Component,
				Some(name.clone()),
				component_name.clone(),
			) {
				// TODO: position for error
				return Err(Error::compiler(
					0,
					0,
					&format!("Variable '{:?}' for 'component' is already defined.", &name),
				));
			}
			component_names.set(name.to_owned(), component_name);
		}

		let family_symbol = if exports.size() > 1 || !exports.has("default") {
			let symbol = self.unique.ensure("FAMILY");
			self.c_imports
				.write("const ")
				.write(&symbol)
				.write(" = Symbol();\n");
			Some(symbol)
		} else {
			None
		};

		for (name, node) in exports.into_entries() {
			let component_name = component_names.get(&name).unwrap().to_owned();
			let model_constructor = if name == "default" {
				self.declaration(DeclarationKind::Model, None)
					.map(|s| s.to_owned())
			} else {
				None
			};

			Component::new().render(
				&mut self,
				component_name.clone(),
				node,
				family_symbol.clone(),
				model_constructor,
			)?;
			self.export(&component_name, Some(&name), None);
		}

		self.render_imports();
		self.render_exports();

		let mut document = Chunk::new();
		document
			.append(&format_chunk(self.c_imports, 0))
			.write("\n\n")
			.append(&format_chunk(self.c_fragments, 0))
			.write("\n\n")
			.append(&format_chunk(self.c_components, 0))
			.write("\n\n")
			.append(&format_chunk(self.c_exports, 0))
			.write("\n");

		Ok(document)
	}

	fn render_dependency(&mut self, node: ast::DependencyStatement) -> Result<(), Error> {
		self.c_imports.map(node.start).write("import ");

		if let Some(specifier) = node.default {
			let name_owner = specifier.local.as_ref().unwrap_or_else(|| &specifier.usage);
			let name = name_owner.name.clone();
			let unique = self.unique.ensure(&name_owner.name);
			let range = name_owner.range();

			self.c_imports
				.map(specifier.usage.start)
				.write(&unique)
				.map(range.end);

			let kind = DeclarationKind::from(&specifier.usage.name);
			let local = specifier.local.map(|i| i.name);

			if !self.declare(
				kind.clone(),
				match kind {
					DeclarationKind::Model => None,
					_ => local,
				},
				unique,
			) {
				return Err(Error::compiler(
					specifier.start,
					specifier.end,
					&format!(
						"Variable '{:?}' for '{:?}' is already defined.",
						&name, &specifier.usage
					),
				));
			}

			if node.named.is_some() {
				self.c_imports.write(", ");
			} else {
				self.c_imports.write(" ");
			}
		}

		if let Some(named) = node.named {
			self.c_imports.map(named.start).write("{ ");

			let mut specifiers = named.nodes.iter().peekable();

			while let Some(specifier) = specifiers.next() {
				let name_owner = specifier
					.local
					.as_ref()
					.unwrap_or_else(|| &specifier.imported);
				let unique = self.unique.ensure(&name_owner.name);

				self.c_imports
					.map(specifier.start)
					.write(&specifier.imported.name)
					.map(specifier.end);

				if &specifier.imported.name != &unique {
					self.c_imports
						.write(" as ")
						.map(name_owner.start)
						.write(&unique)
						.map(name_owner.end);
				}

				if !self.declare(
					DeclarationKind::from(&specifier.usage.name),
					Some(name_owner.name.clone()),
					unique,
				) {
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
					self.c_imports.write(", ");
				}
			}

			self.c_imports.map(named.end).write(" } ");
		}

		self.c_imports
			.write("from ")
			.map(node.source.start)
			.write(&serialize_string_literal(&node.source))
			.map(node.source.end)
			.write(";\n");

		Ok(())
	}

	fn render_imports(&mut self) {
		let mut imports: Map<String, Map<String, Option<String>>> = Map::new();

		for import in &self.imports {
			if let Some(map) = imports.get_mut(&import.from) {
				if map.has(&import.name) {
					panic!("duplicate import");
				} else {
					map.set(import.name.clone(), import.alias.clone());
				}
			} else {
				let mut map = Map::new();
				map.set(import.name.clone(), import.alias.clone());
				imports.set(import.from.clone(), map);
			}
		}

		for (from, mut specifiers) in imports.into_entries() {
			self.c_imports.write("import ");

			if let Some(alias) = specifiers.delete("default") {
				let alias = alias.expect("default is unnamed");
				self.c_imports.write(&alias);

				if specifiers.size() > 0 {
					self.c_imports.write(", ");
				}
			}

			if specifiers.size() > 0 {
				let mut specifiers = specifiers.into_entries().peekable();

				self.c_imports.write("{ ");
				while let Some((name, alias)) = specifiers.next() {
					self.c_imports.write(&name);

					if let Some(alias) = alias {
						self.c_imports.write(" as ").write(&alias);
					}

					if specifiers.peek().is_some() {
						self.c_imports.write(",");
					}

					self.c_imports.write(" ");
				}
				self.c_imports.write("} ");
			}

			self.c_imports
				.write("from ")
				.write(&in_string(&from))
				.write(";\n");
		}
	}

	fn render_exports(&mut self) {
		let mut exports: Map<Option<String>, Map<String, Option<String>>> = Map::new();

		for export in &self.exports {
			if let Some(map) = exports.get_mut(&export.from) {
				if map.has(&export.name) {
					panic!("duplicate import");
				} else {
					map.set(export.name.clone(), export.alias.clone());
				}
			} else {
				let mut map = Map::new();
				map.set(export.name.clone(), export.alias.clone());
				exports.set(export.from.clone(), map);
			}
		}

		for (from, mut specifiers) in exports.into_entries() {
			self.c_exports.write("export ");

			if let Some(alias) = specifiers.delete("default") {
				let alias = alias.expect("default is unnamed");
				self.c_exports.write(&alias);

				if specifiers.size() > 0 {
					self.c_exports.write(", ");
				}
			}

			if specifiers.size() > 0 {
				let mut specifiers = specifiers.into_entries().peekable();

				self.c_exports.write("{ ");
				while let Some((name, alias)) = specifiers.next() {
					self.c_exports.write(&name);

					if let Some(alias) = alias {
						self.c_exports.write(" as ").write(&alias);
					}

					if specifiers.peek().is_some() {
						self.c_exports.write(",");
					}

					self.c_exports.write(" ");
				}
				self.c_exports.write("} ");
			}

			if let Some(from) = from {
				self.c_exports.write("from ").write(&in_string(&from));
			}

			self.c_exports.write(";\n");
		}
	}

	pub(crate) fn declaration(&self, kind: DeclarationKind, name: Option<&str>) -> Option<&str> {
		self.declarations
			.iter()
			.find(|declaration| {
				declaration.kind == kind && declaration.name == name.map(|s| s.to_owned())
			})
			.map(|declaration| declaration.unqiue.as_ref())
	}

	pub(crate) fn declare(
		&mut self,
		kind: DeclarationKind,
		name: Option<String>,
		unqiue: String,
	) -> bool {
		if self
			.declaration(kind.clone(), name.as_ref().map(|x| &**x))
			.is_some()
		{
			return false;
		}

		self.declarations.push(Declaration {
			kind,
			name: name.map(|s| s.to_owned()),
			unqiue: unqiue.to_owned(),
		});

		true
	}
}
