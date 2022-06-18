use super::Build;
use crate::{literal::Literal, parser::Rule};
use pest::iterators::Pair;
use std::collections::HashMap;

enum Using {
	Literal(Literal),
	Import(String),
}

impl Build {
	pub fn build_head(&mut self, pair: Pair<Rule>) {
		let mut using: Vec<(String, Using)> = Vec::new();
		let mut imports: HashMap<String, Vec<(String, String, Option<String>)>> = HashMap::new();

		for pair in pair.into_inner() {
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
								self.chunks.head.map(span.start_pos().line_col());
								self.chunks.head.append("import ");
								if is_all {
									self.chunks.head.append("{ ");
								}
								self.chunks.head.map(original.as_span().start_pos().line_col());
								self.chunks.head.append(original.as_str());
								self.chunks.head.append(" ");
								self.chunks.head.map(original.as_span().end_pos().line_col());
								self.chunks.head.append("as ");
								self.chunks.head.map(alias.as_span().start_pos().line_col());
								self.chunks.head.append(&unique);
								self.chunks.head.append(" ");
								if is_all {
									self.chunks.head.append("} ");
								}
								self.chunks.head.map(alias.as_span().end_pos().line_col());
								self.chunks.head.append("from ");
								using.push((
									alias.as_str().to_owned(),
									Using::Import(unique.to_owned()),
								));
							} else {
								let unique = self.scope.unique(&original.as_str());
								self.chunks.head.map(span.start_pos().line_col());
								self.chunks.head.append("import ");
								self.chunks.head.map(original.as_span().start_pos().line_col());
								self.chunks.head.append(&unique);
								self.chunks.head.append(" ");
								self.chunks.head.map(original.as_span().end_pos().line_col());
								self.chunks.head.append("from ");
								using.push((
									original.as_str().to_owned(),
									Using::Import(unique.to_owned()),
								));
							}
							self.chunks.head.map(source.as_span().start_pos().line_col());
							self.chunks.head.append(source.as_str());
							self.chunks.head.map(source.as_span().end_pos().line_col());
							self.chunks.head.append(";\n");
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

					self.chunks.head.map(span.start_pos().line_col());
					self.chunks.head.append("import ");

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
								self.chunks.head.map(name.as_span().start_pos().line_col());
								self.chunks.head.append(&unique_name);

								if outer.peek().is_none() {
									self.chunks.head.append(" ");
								} else {
									self.chunks.head.append(", ");
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
								self.chunks.head.map(span.start_pos().line_col());
								self.chunks.head.append("* as ");
								self.chunks.head.map(name.as_span().start_pos().line_col());
								self.chunks.head.append(&unique_name);

								if outer.peek().is_none() {
									self.chunks.head.append(" ");
								} else {
									self.chunks.head.append(", ");
								}

								imports.push((
									kind.as_str().to_owned(),
									"*".to_owned(),
									Some(name.as_str().to_owned()),
								));
							}

							Rule::import_clauses_named => {
								self.chunks.head.append("{ ");

								let mut outer = clause.into_inner();
								while let Some(clause) = outer.next() {
									let mut inner = clause.into_inner();
									let kind = inner.next().unwrap();
									let name = inner.next().unwrap();
									let alias = inner.next();

									if let Some(alias) = alias {
										let unique = self.scope.unique(alias.as_str());
										self.chunks.head.map(name.as_span().start_pos().line_col());
										self.chunks.head.append(name.as_str());
										self.chunks.head.map(name.as_span().end_pos().line_col());
										self.chunks.head.append(" as ");
										self.chunks.head.map(alias.as_span().start_pos().line_col());
										self.chunks.head.append(&unique);

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
											self.chunks.head.map(name.as_span().start_pos().line_col());
											self.chunks.head.append(&unique);
										} else {
											self.chunks.head.map(name.as_span().start_pos().line_col());
											self.chunks.head.append(name.as_str());
											self.chunks.head.append(" as ");
											self.chunks.head.append(&unique);
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
										self.chunks.head.append(" ");
									} else {
										self.chunks.head.append(", ");
									}
								}

								self.chunks.head.append("} ");
							}

							_ => unreachable!(),
						}
					}

					self.chunks.head.map(span.end_pos().line_col());
					self.chunks.head.append("from ");
					self.chunks.head.map(source.as_span().start_pos().line_col());
					self.chunks.head.append(source.as_str());
					self.chunks.head.map(source.as_span().end_pos().line_col());
					self.chunks.head.append(";\n");
				}

				Rule::EOI => break,
				_ => unimplemented!(),
			}
		}
	}
}
