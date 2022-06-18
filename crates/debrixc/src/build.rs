mod expression;

use crate::chunk::Chunk;
use crate::literal::Literal;
use crate::parser::{self, Rule};
use crate::scope::Scope;
use crate::utils;
use pest::{
	iterators::{Pair, Pairs},
	Parser,
};
use std::collections::HashMap;

enum Using {
	Literal(Literal),
	Import(String),
}

pub struct BuildResult {
	pub source: String,
	pub mappings: Vec<(usize, usize, usize, usize)>,
}

pub struct Build {
	head: Chunk,
	init: Chunk,
	bind: Chunk,
	insert: Chunk,

	fn_comment: String,
	fn_element: String,
	fn_text: String,
	fn_bind: String,
	fn_bind_text: String,
	fn_bind_attr: String,

	imports: HashMap<String, Vec<(String, String)>>,
	scope: Scope
}

impl Build {
	pub fn new() -> Self {
		let mut scope = Scope::new();

		Self {
			head: Chunk::new(),
			init: Chunk::new(),
			bind: Chunk::new(),
			insert: Chunk::new(),

			fn_comment: scope.unique("comment"),
			fn_element: scope.unique("element"),
			fn_text: scope.unique("text"),
			fn_bind: scope.unique("bind"),
			fn_bind_text: scope.unique("bind_text"),
			fn_bind_attr: scope.unique("bind_attr"),

			imports: [(
				"@debrix/internal".to_owned(),
				["comment", "element", "text", "bind", "bind_text", "bind_attr"]
					.map(|x| (x.to_owned(), x.to_owned()))
					.into_iter()
					.collect::<Vec<(String, String)>>(),
			)]
			.into_iter()
			.collect(),

			scope
		}
	}

	pub fn build(&mut self, input: &str) -> BuildResult {
		let pair = parser::Parser::parse(Rule::document, input)
			.unwrap()
			.next()
			.unwrap();

		let mut pairs = pair.into_inner();
		let _head = self.head(pairs.next().unwrap());
		let body_as_str = pairs.next().unwrap().as_str();

		let body = parser::Parser::parse(Rule::body, body_as_str)
			.unwrap()
			.next()
			.unwrap();

		let root = self.initialize_node(body.into_inner().next().unwrap());

		for (source, imports) in self.imports.iter() {
			let mut has_block = false;
			self.head.append("import ");
			for (index, (specifier, local)) in imports.iter().enumerate() {
				match specifier.as_ref() {
					"default" => {
						self.head.append(local);
					}

					"*" => {
						self.head.append("* as ");
						self.head.append(local);
						self.head.append(" ");
					}

					_ => {
						if !has_block {
							self.head.append("{ ");
							has_block = true;
						}

						self.head.append(specifier);
						if specifier != local {
							self.head.append(" as ");
							self.head.append(local);
						}
					}
				}

				if index == imports.len() - 1 {
					if has_block {
						self.head.append("} ");
					} else {
						self.head.append(" ");
					}
				} else {
					self.head.append(", ")
				}
			}
			self.head.append("from \"");
			self.head.append(source);
			self.head.append("\";\n");
		}

		let source = format!(
			r#"{head}

function init() {{
{init}
{bind}
{insert}
	return {root};
}}

export default class {{
	constructor(options) {{
		this.element = init();
	}}

	insert(target, anchor) {{
		if (anchor)
			target.insertBefore(this.element, anchor);
		else
			target.appendChild(this.element);
	}}

	destroy() {{
		this.element.remove();
	}}
}}"#,
			head = utils::format_lines(&self.head.source, 0),
			init = utils::format_lines(&self.init.source, 1),
			bind = utils::format_lines(&self.bind.source, 1),
			insert = utils::format_lines(&self.insert.source, 1)
		);

		BuildResult {
			source: source.to_owned(),
			mappings: vec![],
		}
	}

	pub fn import(&mut self, specifier: &str, source: &str) -> String {
		let local = self.scope.unique(specifier);
		self.imports
			.entry(source.to_owned())
			.and_modify(|e| e.push((specifier.to_owned(), local.to_owned())))
			.or_insert(vec![(specifier.to_owned(), local.to_owned())]);
		local
	}

	fn head(&mut self, head: Pair<Rule>) {
		let mut using: Vec<(String, Using)> = Vec::new();
		let mut imports: HashMap<String, Vec<(String, String, Option<String>)>> = HashMap::new();

		for pair in head.into_inner() {
			match pair.as_rule() {
				Rule::using => {
					let span = pair.as_span();
					let mut inner = pair.into_inner();

					let using_pair = inner.next().unwrap();
					let kind = using_pair.as_rule();

					match kind {
						Rule::using_import => {
							let mut inner = using_pair.into_inner();
							let clause = inner.next().unwrap();
							let mut clause_inner = clause.into_inner();
							let original = clause_inner.next().unwrap();
							let alias = clause_inner.next();
							let source = inner.next().unwrap();
							if let Some(alias) = alias {
								let unique = self.scope.unique(alias.as_str());
								let is_all = original.as_str() == "*";
								self.head.map(span.start_pos().line_col());
								self.head.append("import ");
								if is_all {
									self.head.append("{ ");
								}
								self.head.map(original.as_span().start_pos().line_col());
								self.head.append(original.as_str());
								self.head.append(" ");
								self.head.map(original.as_span().end_pos().line_col());
								self.head.append("as ");
								self.head.map(alias.as_span().start_pos().line_col());
								self.head.append(&unique);
								self.head.append(" ");
								if is_all {
									self.head.append("} ");
								}
								self.head.map(alias.as_span().end_pos().line_col());
								self.head.append("from ");
								using.push((
									alias.as_str().to_owned(),
									Using::Import(unique.to_owned()),
								));
							} else {
								let unique = self.scope.unique(&original.as_str());
								self.head.map(span.start_pos().line_col());
								self.head.append("import ");
								self.head.map(original.as_span().start_pos().line_col());
								self.head.append(&unique);
								self.head.append(" ");
								self.head.map(original.as_span().end_pos().line_col());
								self.head.append("from ");
								using.push((
									original.as_str().to_owned(),
									Using::Import(unique.to_owned()),
								));
							}
							self.head.map(source.as_span().start_pos().line_col());
							self.head.append(source.as_str());
							self.head.map(source.as_span().end_pos().line_col());
							self.head.append(";\n");
						}

						Rule::using_literal => {
							let mut inner = using_pair.into_inner();
							let id = inner.next().unwrap();
							let literal = inner.next().unwrap();
							using.push((
								id.as_str().to_owned(),
								Using::Literal(Literal::parse(&literal)),
							));
						}

						_ => unreachable!(),
					}
				}

				Rule::import => {
					let span = pair.as_span();
					let mut inner = pair.into_inner();

					let clauses = inner.next().unwrap();
					let source = inner.next().unwrap();
					let source_literal_value = Literal::parse(&source);
					let source_str = if let Literal::String(source_str) = source_literal_value {
						source_str
					} else {
						unreachable!();
					};

					let imports = imports.entry(source_str).or_insert(Vec::new());

					self.head.map(span.start_pos().line_col());
					self.head.append("import ");

					let mut outer = clauses.into_inner();
					while let Some(clause) = outer.next() {
						let rule = clause.as_rule();
						let span = clause.as_span();

						match rule {
							Rule::import_clause_default => {
								let mut inner = clause.into_inner();
								let kind = inner.next().unwrap();
								let name = inner.next().unwrap();

								let unique_name = self.scope.unique(name.as_str());
								self.head.map(name.as_span().start_pos().line_col());
								self.head.append(&unique_name);

								if outer.peek().is_none() {
									self.head.append(" ");
								} else {
									self.head.append(", ");
								}

								imports.push((
									kind.as_str().to_owned(),
									"default".to_owned(),
									Some(unique_name.to_owned()),
								));
							}

							Rule::import_clause_all => {
								let mut inner = clause.into_inner();
								let kind = inner.next().unwrap();
								let name = inner.next().unwrap();

								let unique_name = self.scope.unique(name.as_str());
								self.head.map(span.start_pos().line_col());
								self.head.append("* as ");
								self.head.map(name.as_span().start_pos().line_col());
								self.head.append(&unique_name);

								if outer.peek().is_none() {
									self.head.append(" ");
								} else {
									self.head.append(", ");
								}

								imports.push((
									kind.as_str().to_owned(),
									"*".to_owned(),
									Some(name.as_str().to_owned()),
								));
							}

							Rule::import_clauses_named => {
								self.head.append("{ ");

								let mut outer = clause.into_inner();
								while let Some(clause) = outer.next() {
									let mut inner = clause.into_inner();
									let kind = inner.next().unwrap();
									let name = inner.next().unwrap();
									let alias = inner.next();

									if let Some(alias) = alias {
										let unique = self.scope.unique(alias.as_str());
										self.head.map(name.as_span().start_pos().line_col());
										self.head.append(name.as_str());
										self.head.map(name.as_span().end_pos().line_col());
										self.head.append(" as ");
										self.head.map(alias.as_span().start_pos().line_col());
										self.head.append(&unique);

										imports.push((
											kind.as_str().to_owned(),
											name.as_str().to_owned(),
											if unique == alias.as_str() {
												None
											} else {
												Some(unique.to_owned())
											},
										));
									} else {
										let name_str = name.as_str();
										let unique = self.scope.unique(name_str);

										if unique == name_str {
											self.head.map(name.as_span().start_pos().line_col());
											self.head.append(&unique);
										} else {
											self.head.map(name.as_span().start_pos().line_col());
											self.head.append(name.as_str());
											self.head.append(" as ");
											self.head.append(&unique);
										}

										imports.push((
											kind.as_str().to_owned(),
											name_str.to_owned(),
											if unique == name_str {
												None
											} else {
												Some(unique.to_owned())
											},
										));
									}

									if outer.peek().is_none() {
										self.head.append(" ");
									} else {
										self.head.append(", ");
									}
								}

								self.head.append("} ");
							}

							_ => unreachable!(),
						}
					}

					self.head.map(span.end_pos().line_col());
					self.head.append("from ");
					self.head.map(source.as_span().start_pos().line_col());
					self.head.append(source.as_str());
					self.head.map(source.as_span().end_pos().line_col());
					self.head.append(";\n");
				}

				Rule::EOI => break,
				_ => unimplemented!(),
			}
		}
	}

	fn generate_expression(&self, _expression: &Pair<Rule>) -> String {
		"(function(){/*expression*/}).bind(/*data*/)".to_owned()
	}

	fn insert_nodes(&mut self, parent: &str, inner: Pairs<Rule>) {
		for pair in inner {
			let id = self.initialize_node(pair);
			self.insert
				.append(&format!("{parent}.appendChild({id});\n"));
		}
	}

	fn initialize_node(&mut self, pair: Pair<Rule>) -> String {
		match pair.as_rule() {
			Rule::comment => {
				let mut inner = pair.into_inner();
				let comment_data = inner.next().unwrap();

				let id = self.scope.unique("comment");
				self.init.append(&format!(
					"const {id} = {fn_comment}(\"{data}\");\n",
					fn_comment = self.fn_comment,
					data = comment_data.as_str(),
				));

				id
			}

			Rule::element => {
				let mut inner = pair.into_inner();
				let tag_name = inner.next().unwrap();
				let tag_name_str = tag_name.as_str();
				let tag_inner = inner.next().unwrap();
				let children = inner.next();

				let id = self.scope.unique(tag_name_str);
				self.init.append(&format!(
					"const {id} = {fn_element}(\"{tag_name_str}\");\n",
					fn_element = self.fn_element
				));

				for pair in tag_inner.into_inner() {
					match pair.as_rule() {
						Rule::attribute_static => {
							let mut inner = pair.into_inner();
							let name = inner.next().unwrap();
							let value = inner.next().unwrap();
							let literal_value = Literal::parse(&value);

							self.init.append(&format!(
								"{id}.setAttribute({name}, {value});\n",
								name = name.as_str(),
								value = literal_value.as_str()
							));
						}

						Rule::attribute_binding => {
							let mut inner = pair.into_inner();
							let name = inner.next().unwrap();
							let value = inner.next().unwrap();

							self.init.append(&format!(
								"{fn_bind_attr}({name}, {value});\n",
								fn_bind_attr = self.fn_bind_attr,
								name = name.as_str(),
								value = self.generate_expression(&value)
							));
						}

						Rule::binding => {
							let mut inner = pair.into_inner();
							let ident = inner.next().unwrap();
							let value = inner.next().unwrap();

							self.bind.append(&format!(
								"{fn_bind}({ident}, {expression});\n",
								fn_bind = self.fn_bind,
								ident = ident.as_str(),
								expression = self.generate_expression(&value)
							));
						}

						_ => unimplemented!(),
					}
				}

				if let Some(children) = children {
					self.insert_nodes(&id, children.into_inner());
				}

				id
			}

			Rule::text => {
				let id = self.scope.unique("text");
				self.init.append(&format!(
					"const {id} = {fn_text}(\"{data}\");\n",
					fn_text = self.fn_text,
					data = pair.as_str(),
				));

				id
			}

			Rule::text_binding => {
				let mut inner = pair.into_inner();
				let expression = inner.next().unwrap();

				let id = self.scope.unique("text");
				self.init.append(&format!(
					"const {id} = {fn_text}(\"\");\n",
					fn_text = self.fn_text
				));
				self.bind.append(&format!(
					"{fn_bind_text}({expression});\n",
					fn_bind_text = self.fn_bind_text,
					expression = self.generate_expression(&expression)
				));

				id
			}

			_ => unimplemented!(),
		}
	}
}
