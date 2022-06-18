use super::Build;
use crate::{build::DEBRIX_INTERNAL, literal::Literal, parser::Rule};
use pest::iterators::{Pair, Pairs};

impl Build {
	fn initialize_node(&mut self, pair: Pair<Rule>) -> String {
		match pair.as_rule() {
			Rule::comment => {
				let mut inner = pair.into_inner();
				let comment_data = inner.next().unwrap();

				let comment = self.import("comment", DEBRIX_INTERNAL);
				let id = self.scope.unique("comment");
				self.chunks.init.append("const ");
				self.chunks.init.append(&id);
				self.chunks.init.append(" = ");
				self.chunks.init.append(&comment);
				self.chunks.init.append("(\"");
				// TODO: escape comment to string
				self.chunks.init.append(comment_data.as_str());
				self.chunks.init.append("\");\n");

				id
			}

			Rule::element => {
				let mut inner = pair.into_inner();
				let tag_name = inner.next().unwrap();
				let tag_name_str = tag_name.as_str();
				let tag_inner = inner.next().unwrap();
				let children = inner.next();

				let element = self.import("element", DEBRIX_INTERNAL);
				let id = self.scope.unique(tag_name_str);
				self.chunks.init.append("const ");
				self.chunks.init.append(&id);
				self.chunks.init.append(" = ");
				self.chunks.init.append(&element);
				self.chunks.init.append("(\"");
				self.chunks.init.append(tag_name_str);
				self.chunks.init.append("\");\n");

				for pair in tag_inner.into_inner() {
					match pair.as_rule() {
						Rule::attribute_static => {
							let mut inner = pair.into_inner();
							let name = inner.next().unwrap();
							let value = inner.next().unwrap();
							let literal_value = Literal::parse(&value);

							self.chunks.init.append(&format!(
								"{id}.setAttribute({name}, {value});\n",
								name = name.as_str(),
								value = literal_value.as_str()
							));
						}

						Rule::attribute_binding => {
							let mut inner = pair.into_inner();
							let name = inner.next().unwrap();
							let value = inner.next().unwrap();

							let bind_attr = self.import("bind_attr", DEBRIX_INTERNAL);
							self.chunks.bind.append(&bind_attr);
							self.chunks.bind.append("(");
							self.chunks.bind.append(&name.as_str());
							self.chunks.bind.append(", ");
							let expression = self.build_expression(value).source;
							self.chunks.bind.append(&expression);
							self.chunks.bind.append(");");
						}

						Rule::binding => {
							let mut inner = pair.into_inner();
							let ident = inner.next().unwrap();
							let value = inner.next().unwrap();

							let bind = self.import("bind", DEBRIX_INTERNAL);
							self.chunks.bind.append(&bind);
							self.chunks.bind.append("(");
							self.chunks.bind.append(&ident.as_str());
							self.chunks.bind.append(", ");
							let expression = self.build_expression(value).source;
							self.chunks.bind.append(&expression);
							self.chunks.bind.append(");\n");
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
				let text = self.import("text", DEBRIX_INTERNAL);
				let id = self.scope.unique("text");
				self.chunks.init.append(&format!(
					"const {id} = {text}(\"{data}\");\n",
					data = pair.as_str(),
				));

				id
			}

			Rule::text_binding => {
				let mut inner = pair.into_inner();
				let expression = inner.next().unwrap();

				let text = self.import("text", DEBRIX_INTERNAL);
				let id = self.scope.unique("text");
				self.chunks
					.init
					.append(&format!("const {id} = {text}(\"\");\n"));
				let bind_text = self.import("bind_text", DEBRIX_INTERNAL);
				self.chunks.bind.append(&bind_text);
				self.chunks.bind.append("(");
				let expression = self.build_expression(expression).source;
				self.chunks.bind.append(&expression);
				self.chunks.bind.append(");\n");

				id
			}

			_ => unimplemented!(),
		}
	}

	fn insert_nodes(&mut self, parent: &str, inner: Pairs<Rule>) {
		for pair in inner {
			let insert = self.import("insert", DEBRIX_INTERNAL);
			let id = self.initialize_node(pair);
			self.chunks.insert.append(&insert);
			self.chunks.insert.append("(");
			self.chunks.insert.append(parent);
			self.chunks.insert.append(", ");
			self.chunks.insert.append(&id);
			self.chunks.insert.append(");\n");
		}
	}

	pub fn build_body(&mut self, pair: Pair<Rule>) -> String {
		self.initialize_node(pair)
	}
}
