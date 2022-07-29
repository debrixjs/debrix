use std::collections::HashMap;

use pest::iterators::{Pair, Pairs};

use crate::{debug::debug_pair, error::Error, parser::R, render::Chunk, utils};

const DEBRIX: &str = "debrix";
const DEBRIX_INTERNAL: &str = "@debrix/internal";

#[rustfmt::skip]
const RESERVED_JAVASCRIPT_KEYWORDS: [&str; 64] = [
	"abstract", "arguments", "await", "boolean", "break", "byte", "case", "catch", "char",
	"class", "const", "continue", "debugger", "default", "delete", "do", "double", "else", "enum",
	"eval", "export", "extends", "false", "final", "finally", "float", "for", "function", "goto",
	"if", "implements", "import", "in", "instanceof", "int", "interface", "let", "long", "native",
	"new", "null", "package", "private", "protected", "public", "return", "short", "static",
	"super", "switch", "synchronized", "this", "throw", "throws", "transient", "true", "try",
	"typeof", "var", "void", "volatile", "while", "with", "yield"
];

const DEBRIX_DEFAULT_BINDERS: [&str; 3] = ["input", "event", "click"];

struct Chunks {
	global: Chunk,
	init: Chunk,
	init_bind: Chunk,
	init_insert: Chunk,
	insert: Chunk,
	data: Chunk,
}

impl Chunks {
	fn new() -> Self {
		Self {
			global: Chunk::new(),
			init: Chunk::new(),
			init_bind: Chunk::new(),
			init_insert: Chunk::new(),
			insert: Chunk::new(),
			data: Chunk::new(),
		}
	}
}

pub struct Renderer {
	c: Chunks,
	idents: HashMap<String, usize>,
	imports: HashMap<String, HashMap<String, String>>,
}

impl<'a> Renderer {
	pub fn new() -> Self {
		Self {
			c: Chunks::new(),
			idents: HashMap::new(),
			imports: HashMap::new(),
		}
	}

	fn get_unique(&mut self, ident: &str) -> String {
		// generate unqiue identifer
		let mut ident = ident.to_owned();

		if self.idents.contains_key(&ident) {
			let index = self.idents.get_mut(&ident).unwrap();
			*index += 1;
			ident.push_str("_");
			ident.push_str(&index.to_string());
		} else {
			self.idents.insert(ident.clone(), 0);
		}

		if RESERVED_JAVASCRIPT_KEYWORDS.contains(&ident.as_ref()) {
			ident.push_str("_");
		}

		ident
	}

	fn lazy_import(&mut self, ident: &str, local: Option<&str>, source: &str) -> String {
		let entry = self
			.imports
			.entry(source.to_owned())
			.or_insert(HashMap::new());

		if entry.contains_key(ident) {
			entry.get(ident).unwrap().to_owned()
		} else {
			let unique = self.get_unique(if let Some(alias) = local {
				alias
			} else {
				ident
			});
			self.imports
				.get_mut(source)
				.unwrap()
				.insert(ident.to_owned(), unique.to_owned());
			unique
		}
	}

	fn render_import(
		&mut self,
		entries: &HashMap<String, String>,
		source: &str,
	) -> Result<(), Error<'a, R>> {
		self.c.global.write("import ");

		if entries.contains_key("default") {
			self.c
				.global
				.write(entries.get("default").unwrap())
				.write(", { ");
		} else {
			self.c.global.write("{ ");
		}

		for (ident, local) in entries {
			if ident == "default" {
				continue;
			}

			self.c.global.write(&ident);

			if ident != local {
				self.c.global.write(" as ").write(&local);
			}

			if entries.keys().last().unwrap() != ident {
				self.c.global.write(", ");
			}
		}

		self.c
			.global
			.write(" } from \"")
			.write(&source)
			.write("\";\n");

		Ok(())
	}

	fn render_expression_raw(&mut self, expression: Pair<R>) -> Chunk {
		let mut chunk = Chunk::new();
		let mut inner = expression.into_inner();

		while let Some(pair) = inner.next() {
			match pair.as_rule() {
				R::binary_expression => {
					let mut inner = pair.into_inner();
					let operator = inner.next().unwrap();
					let right = inner.next().unwrap();

					chunk
						.write(" ")
						.write(operator.as_str())
						.write(" ")
						.append(&self.render_expression_raw(right), 0);
				}

				R::new_expression => {
					let mut inner = pair.into_inner();
					let target = inner.next().unwrap();
					let arguments = inner.next();

					chunk
						.write("new ")
						.append(&self.render_expression_raw(target), 0);

					if let Some(arguments) = arguments {
						chunk.write(" (");

						let mut inner = arguments.into_inner();
						while let Some(argument) = inner.next() {
							chunk.append(&self.render_expression_raw(argument), 0);
							if inner.peek().is_some() {
								chunk.write(", ");
							}
						}

						chunk.write(")");
					}
				}

				R::call_expression => {
					let mut inner = pair.into_inner();
					let target = inner.next().unwrap();
					let arguments = inner.next().unwrap();

					chunk
						.append(&self.render_expression_raw(target), 0)
						.write("(");

					let mut inner = arguments.into_inner();
					while let Some(argument) = inner.next() {
						chunk.append(&self.render_expression_raw(argument), 0);
						if inner.peek().is_some() {
							chunk.write(", ");
						}
					}

					chunk.write(")");
				}

				R::unary_expression => {
					let mut inner = pair.into_inner();
					let operator = inner.next().unwrap();
					let target = inner.next().unwrap();

					chunk
						.write(operator.as_str())
						.write(" ")
						.append(&self.render_expression_raw(target), 0);
				}

				R::expression => {
					let mut inner = pair.into_inner();
					let target = inner.next().unwrap();

					chunk
						.write("(")
						.append(&self.render_expression_raw(target), 0)
						.write(")");
				}

				R::member_expression => {
					let mut inner = pair.into_inner();

					while let Some(member) = inner.next() {
						let rule = member.as_rule();
						let target = member.into_inner().next().unwrap();

						match rule {
							R::member => {
								chunk
									.write(".")
									.append(&self.render_expression_raw(target), 0);
							}

							R::optional_memeber => {
								chunk
									.write("?.")
									.append(&self.render_expression_raw(target), 0);
							}

							R::computed_member => {
								chunk
									.write("[")
									.append(&self.render_expression_raw(target), 0)
									.write("]");
							}

							_ => unreachable!(),
						}
					}
				}

				R::ident => {
					chunk.write("this.").write(pair.as_str());
				}

				R::literal => {
					chunk.write(pair.as_str());
				}

				_ => unimplemented!(),
			}
		}

		chunk
	}

	fn render_expression(&mut self, expression: Pair<R>) -> Chunk {
		let expr_chunk = self.render_expression_raw(expression.clone());

		let mut inner = expression.into_inner();
		let left = inner.next().unwrap();
		let right = inner.next();

		fn is_ident(left: &Pair<R>, right: &Option<Pair<R>>) -> bool {
			left.as_rule() == R::ident && right.is_none()
		}

		fn is_member(_left: &Pair<R>, right: &Option<Pair<R>>) -> bool {
			right.is_some() && right.as_ref().unwrap().as_rule() == R::member_expression
		}

		let mut chunk = Chunk::new();
		if inner.peek().is_none() && (is_ident(&left, &right) || is_member(&left, &right)) {
			chunk.write("this.$reference(");
		} else {
			chunk.write("this.$computed(() => ");
		}
		chunk.append(&expr_chunk, 0);
		chunk.write(")");

		chunk
	}

	fn render_insert(&mut self, parent: &str, child: &str, anchor: &str) {
		let insert = self.lazy_import("insert", None, DEBRIX_INTERNAL);

		self.c
			.init_insert
			.write(&insert)
			.write("(")
			.write(parent)
			.write(", ")
			.write(child);

		if anchor != "undefined" {
			self.c.init_insert.write(", ").write(anchor);
		}

		self.c.init_insert.write(");\n");
	}

	fn render_node(&mut self, pair: Pair<R>) -> String {
		debug_pair(&pair);

		match pair.as_rule() {
			R::comment => self.render_comment(pair),
			R::element => self.render_element(pair),
			R::text => self.render_text(pair),
			R::text_binding => self.render_text_binding(pair),

			_ => unimplemented!(),
		}
	}

	fn render_comment(&mut self, pair: Pair<R>) -> String {
		let span = pair.as_span();
		let mut inner = pair.into_inner();
		let data = inner.next().unwrap();

		let create_comment = self.lazy_import("comment", None, DEBRIX_INTERNAL);
		let ident = self.get_unique("comment");

		self.c
			.init
			.write("const ")
			.write(&ident)
			.write(" = ")
			.map(span.start_pos().line_col())
			.write(&create_comment)
			.write("(\"")
			.write(data.as_str())
			.write("\");\n");

		ident
	}

	fn render_element(&mut self, pair: Pair<R>) -> String {
		let span = pair.as_span();
		let mut inner = pair.into_inner();
		let tag = inner.next().unwrap();
		let mut inner_tag = inner.next().unwrap().into_inner();
		let children = inner.next();

		let create_element = self.lazy_import("element", None, DEBRIX_INTERNAL);
		let ident = self.get_unique(tag.as_str());

		self.c
			.init
			.map(span.start_pos().line_col())
			.write("const ")
			.write(&ident)
			.write(" = ")
			.write(&create_element)
			.write("(\"")
			.map(tag.as_span().start_pos().line_col())
			.write(tag.as_str())
			.map(tag.as_span().end_pos().line_col())
			.write("\");\n");

		while let Some(pair) = inner_tag.next() {
			match pair.as_rule() {
				R::static_attribute => {
					let span = pair.as_span();
					let mut inner = pair.into_inner();
					let name = inner.next().unwrap();
					let value = inner.next().unwrap();

					let set_attr = self.lazy_import("attr", None, DEBRIX_INTERNAL);

					self.c
						.init
						.map(span.start_pos().line_col())
						.write(&set_attr)
						.write("(")
						.write(&ident)
						.write(", \"")
						.map(name.as_span().start_pos().line_col())
						.write(name.as_str())
						.map(name.as_span().end_pos().line_col())
						.write("\", \"")
						.map(value.as_span().start_pos().line_col())
						.write(value.as_str())
						.map(value.as_span().end_pos().line_col())
						.write("\");\n");
				}

				R::attribute_binding => {
					let span = pair.as_span();
					let mut inner = pair.into_inner();
					let name = inner.next().unwrap();

					let expr = inner.next().unwrap();
					let expr_span = expr.as_span();
					let expr = self.render_expression(expr);

					let bind_attr = self.lazy_import("bind_attr", None, DEBRIX_INTERNAL);

					self.c
						.init_bind
						.map(span.start_pos().line_col())
						.write(&bind_attr)
						.write("(")
						.write(&ident)
						.write(", \"")
						.map(name.as_span().start_pos().line_col())
						.write(name.as_str())
						.map(name.as_span().end_pos().line_col())
						.write("\", \"")
						.map(expr_span.start_pos().line_col())
						.append(&expr, 0)
						.map(expr_span.end_pos().line_col())
						.write("\");\n");
				}

				R::binding => {
					let span = pair.as_span();
					let mut inner = pair.into_inner();
					let name = inner.next().unwrap();

					let expr = inner.next().unwrap();
					let expr_span = expr.as_span();
					let expr = self.render_expression(expr);

					let bind = self.lazy_import("bind", None, DEBRIX_INTERNAL);

					self.c
						.init_bind
						.map(span.start_pos().line_col())
						.write(&bind)
						.write("(")
						.write(&ident)
						.write(", ")
						.map(name.as_span().start_pos().line_col())
						.write(name.as_str())
						.map(name.as_span().end_pos().line_col())
						.write(", ")
						.map(expr_span.start_pos().line_col())
						.append(&expr, 0)
						.map(expr_span.end_pos().line_col())
						.write(");\n");
				}

				_ => unimplemented!(),
			}
		}

		if let Some(children) = children {
			let mut children = children.into_inner();

			while let Some(pair) = children.next() {
				let child = self.render_node(pair);
				self.render_insert(&ident, &child, "undefined");
			}
		}

		ident
	}

	fn render_text(&mut self, pair: Pair<R>) -> String {
		let create_text = self.lazy_import("text", None, DEBRIX_INTERNAL);
		let ident = self.get_unique("text");

		self.c
			.init
			.write("const ")
			.write(&ident)
			.write(" = ")
			.map(pair.as_span().start_pos().line_col())
			.write(&create_text)
			.write("(\"")
			.write(pair.as_str())
			.write("\");\n");

		ident
	}

	fn render_text_binding(&mut self, pair: Pair<R>) -> String {
		let span = pair.as_span();
		let mut inner = pair.into_inner();
		let expr = inner.next().unwrap();
		let expr_span = expr.as_span();
		let expr = self.render_expression(expr);

		let create_text = self.lazy_import("text", None, DEBRIX_INTERNAL);
		let bind_text = self.lazy_import("bind_text", None, DEBRIX_INTERNAL);
		let ident = self.get_unique("text_binding");

		self.c
			.init
			.write("const ")
			.write(&ident)
			.write(" = ")
			.map(span.start_pos().line_col())
			.write(&create_text)
			.write("(\"")
			.write("\");\n");

		self.c
			.init_bind
			.map(span.start_pos().line_col())
			.write(&bind_text)
			.write("(")
			.write(&ident)
			.write(", ")
			.map(expr_span.start_pos().line_col())
			.append(&expr, 0)
			.map(expr_span.end_pos().line_col())
			.write(");\n");

		ident
	}

	pub fn render(&mut self, mut pairs: Pairs<R>) -> Result<Chunk, Error<'a, R>> {
		// TODO: Import the binders when used in a binding.
		DEBRIX_DEFAULT_BINDERS.map(|i| self.lazy_import(i, None, DEBRIX));

		let parent = self.get_unique("parent");
		let init = self.get_unique("init");
		let data = self.get_unique("data");

		while let Some(pair) = pairs.next() {
			match pair.as_rule() {
				R::EOI => break,

				R::import => {
					let mut inner = pair.into_inner();
					let clause = inner.next().unwrap();
					let source = inner.next().unwrap();

					let mut inner = clause.into_inner();
					let mut ident: Option<Pair<R>> = None;
					let mut local: Option<Pair<R>> = None;
					let mut usage: Option<Pair<R>> = None;

					for _ in 0..1 {
						if let Some(pair) = inner.next() {
							match pair.as_rule() {
								R::ident => {
									if ident.is_some() {
										unreachable!()
									}

									ident = Some(pair);
								}

								R::clause_rename => {
									if local.is_some() {
										unreachable!()
									}

									local = Some(pair.into_inner().next().unwrap());
								}

								R::usage_specifier => {
									if usage.is_some() {
										unreachable!()
									}

									usage = Some(pair.into_inner().next().unwrap());
								}

								_ => unreachable!(),
							}
						}
					}

					let accual = self.lazy_import(
						ident
							.and_then(|i| Some(i.as_str()))
							.or(Some("default"))
							.unwrap(),
						local.and_then(|l| Some(l.as_str())),
						&utils::str_val(source.as_str()),
					);

					if let Some(usage) = usage {
						match usage.as_str() {
							"model" => {
								self.c
									.data
									.write(&data)
									.write(" = new ")
									.write(&accual)
									.write("(options.props);\n");
							}

							"data" => {
								self.c
									.data
									.write(&data)
									.write(" = ")
									.write(&accual)
									.write(";\n");
							}

							x => unimplemented!("{}", x),
						};
					}
				}

				R::comment | R::element | R::text | R::text_binding => {
					let insert = self.lazy_import("insert", None, DEBRIX_INTERNAL);
					let ident = self.render_node(pair);

					self.c
						.insert
						.write(&insert)
						.write("(")
						.write(&parent)
						.write(", ")
						.write(&ident)
						.write(");\n");
				}

				x => unimplemented!("{:?}", x),
			}
		}

		for (source, entries) in &self.imports.clone() {
			self.render_import(entries, source)?;
		}

		let mut chunk = Chunk::new();

		chunk
			.append(&self.c.global, 0)
			.write("\n\nfunction ")
			.write(&init)
			.write("() {\n")
			.append(&self.c.init, 1)
			.write("\n")
			.append(&self.c.init_bind, 1)
			.write("\n")
			.append(&self.c.init_insert, 1)
			.write("\n\n\treturn function (")
			.write(&parent)
			.write(") {\n")
			.append(&self.c.insert, 2)
			.write("\n\t}\n\n}\n\n")
			.write("export default class {\n")
			.write("\tconstructor(options = {}) {\n")
			.write("\t\tlet ")
			.write(&data)
			.write(";\n")
			.append(&self.c.data, 2)
			.write("\n\t\tthis._insert = init.call(data);\n")
			.write("\t}\n\n")
			.write("\tinsert(target, anchor) {\n")
			.write("\t\tthis._insert(target, anchor);\n")
			.write("\t}\n\n")
			.write("\tdestroy() {\n")
			.write("\t\tthis.element.remove();\n")
			.write("\t}\n")
			.write("}\n");

		Ok(chunk)
	}
}
