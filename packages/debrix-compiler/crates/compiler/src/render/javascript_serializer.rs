use super::*;

const OVERRIDES_PROPERTY: [&str; 6] = ["null", "undefined", "NaN", "Infinity", "false", "true"];

pub struct JavascriptSerializer {
	pub local_vars: Vec<String>,
}

impl JavascriptSerializer {
	pub fn new() -> JavascriptSerializer {
		Self {
			local_vars: Vec::new(),
		}
	}

	pub fn serialize(&self, expr: &ast::javascript::Expression) -> Chunk {
		self._serialize(expr, true)
	}

	pub fn _serialize(&self, expr: &ast::javascript::Expression, thisify: bool) -> Chunk {
		let mut chunk = Chunk::new();

		match expr {
			ast::javascript::Expression::Identifier(expr) => {
				return self.serialize_identifier(expr, thisify);
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
					.append(&self._serialize(&expr.operand, thisify));
			}
			ast::javascript::Expression::Binary(expr) => {
				chunk
					.append(&self._serialize(&expr.left, thisify))
					.write(" ")
					.write(&expr.operator.to_string())
					.write(" ")
					.append(&self._serialize(&expr.right, thisify));
			}
			ast::javascript::Expression::Conditional(expr) => {
				chunk
					.append(&self._serialize(&expr.condition, thisify))
					.write(" ? ")
					.append(&self._serialize(&expr.consequent, thisify))
					.write(" : ")
					.append(&self._serialize(&expr.alternate, thisify));
			}
			ast::javascript::Expression::Call(expr) => {
				chunk
					.append(&self._serialize(&expr.callee, thisify))
					.write("(");

				let mut arguments = expr.arguments.iter().peekable();
				while let Some(arg) = arguments.next() {
					chunk.append(&self._serialize(arg, thisify));

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
					.append(&self._serialize(&expr.callee, thisify));

				if expr.arguments.len() > 0 {
					chunk.write("(");

					let mut arguments = expr.arguments.iter().peekable();
					while let Some(arg) = arguments.next() {
						chunk.append(&self._serialize(arg, thisify));

						if arguments.peek().is_some() {
							chunk.write(", ");
						}
					}

					chunk.write(")");
				}
			}
			ast::javascript::Expression::Member(expr) => {
				chunk.append(&self._serialize(&expr.object, thisify));

				if !expr.optional && !expr.computed {
					chunk.write(".");
				} else if expr.optional && !expr.computed {
					chunk.write("?.");
				} else if !expr.optional && expr.computed {
					chunk.write("[");
				} else if expr.optional && expr.computed {
					chunk.write("?.[");
				}

				chunk.append(&self._serialize(&expr.property, false));

				if expr.computed {
					chunk.write("]");
				}
			}
			ast::javascript::Expression::Function(expr) => {
				chunk.map(expr.start).write("(");

				let mut params = expr.parameters.iter().peekable();
				while let Some(param) = params.next() {
					chunk.append(&self._serialize(param, false));

					if params.peek().is_some() {
						chunk.write(", ");
					}
				}

				chunk
					.write(") => ")
					.append(&self._serialize(&expr.body, thisify));
			}
			ast::javascript::Expression::Assignment(expr) => {
				chunk
					.append(&self._serialize(&expr.left, thisify))
					.write(" ")
					.write(&expr.operator.to_string())
					.write(" ")
					.append(&self._serialize(&expr.right, thisify));
			}
			ast::javascript::Expression::Spread(expr) => {
				chunk
					.map(expr.start)
					.write("...")
					.append(&self._serialize(&expr.argument, thisify));
			}
			ast::javascript::Expression::Template(expr) => {
				return self.serialize_template(expr);
			}
			ast::javascript::Expression::TaggedTemplate(expr) => {
				chunk
					.append(&self._serialize(&expr.tag, thisify))
					.append(&self.serialize_template(&expr.quasi));
			}
			ast::javascript::Expression::Object(expr) => {
				chunk.map(expr.start).write("{");

				let mut props = expr.properties.iter().peekable();
				while let Some(prop) = props.next() {
					match prop {
						ast::javascript::ObjectProperty::Keyed(prop) => {
							chunk.append(&self.serialize_identifier(&prop.key, false));

							if let Some(value) = &prop.value {
								chunk.write(": ").append(&self._serialize(value, thisify));
							}
						}
						ast::javascript::ObjectProperty::Computed(prop) => {
							chunk
								.map(prop.start)
								.write("[")
								.append(&self._serialize(&prop.key, thisify))
								.write("]: ")
								.append(&self._serialize(&prop.value, thisify));
						}
						ast::javascript::ObjectProperty::Spread(prop) => {
							chunk
								.map(prop.start)
								.write("...")
								.append(&self._serialize(&prop.argument, thisify));
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
					chunk.append(&self._serialize(&expr, thisify));

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
					.append(&self._serialize(&expr.expression, thisify))
					.write(")")
					.map(expr.end);
			}
			ast::javascript::Expression::Empty(expr) => {
				chunk.map(expr.start).write(";").map(expr.end);
			}
		}

		chunk
	}

	fn serialize_identifier(
		&self,
		expr: &ast::javascript::IdentifierExpression,
		thisify: bool,
	) -> Chunk {
		let mut chunk = Chunk::new();

		let is_local = self.local_vars.contains(&expr.name);
		let force_local = is_local || OVERRIDES_PROPERTY.contains(&expr.name.as_ref());
		let is_reserved = RESERVED_JAVASCRIPT_KEYWORDS.contains(&expr.name.as_ref());

		if force_local && is_reserved {
			unreachable!(
				"Invalid local identifier {} was previously defined.",
				&expr.name
			);
		}

		if !force_local && thisify && is_reserved {
			// renders: this["identifier"]
			chunk
				.map(expr.start)
				.write("this[")
				.write(&in_string(&expr.name))
				.write("]")
				.map(expr.end);
		} else if !force_local && thisify {
			// renders: ("identifier" in this ? this["identifier"] : identifier)
			chunk
				.map(expr.start)
				.write("(")
				.write(&in_string(&expr.name))
				.write(" in this ? this[")
				.write(&in_string(&expr.name))
				.write("] : ")
				.write(&expr.name)
				.write(")")
				.map(expr.end);
		} else {
			// renders: identifier
			chunk.map(expr.start).write(&expr.name).map(expr.end);
		}

		chunk
	}

	fn serialize_template(&self, expr: &ast::javascript::TemplateLiteral) -> Chunk {
		// TODO: serialize all inline expressions in the template
		let mut chunk = Chunk::new();
		chunk.map(expr.start).write(&expr.raw).map(expr.end);
		chunk
	}
}
