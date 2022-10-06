use crate::*;

impl Parser {
	pub fn parse_element(&mut self) -> Result<ast::Element, ParserError> {
		let start = self.scanner.cursor();
		let mut self_closing = false;
		let mut attributes = Vec::new();
		let mut bindings = None;
		let mut children = Vec::new();
		let mut end_tag = None;

		if !self.scanner.take("<") {
			return Err(self.expected(&["<"]));
		}

		let tag_name = self.parse_tag_name()?;
		self.skip_whitespace();

		loop {
			if let Some(char) = self.scanner.peek() {
				if char.is_whitespace() {
					self.scanner.next();
					continue;
				}

				match char {
					'(' => {
						bindings = self.parse_bindings()?;
					}
					'>' | '/' => {
						break;
					}
					'\0' | '"' | '\'' | '=' => {
						self.scanner.next();
						return Err(self.unexpected());
					}
					_ => attributes.push(self.parse_attribute()?),
				}
			} else {
				return Err(self.unexpected());
			}
		}

		if self.scanner.take("/") {
			self_closing = true;
		}

		if !self.scanner.take(">") {
			return Err(self.expected(&[">"]));
		}

		if !self_closing {
			self.skip_whitespace();
			children = self.parse_children()?;
			self.skip_whitespace();

			let start = self.scanner.cursor();

			if !self.scanner.take("</") {
				return Err(self.expected(&["</"]));
			}

			if !self.scanner.take(&tag_name.name) {
				return Err(self.expected(&[&tag_name.name]));
			}

			if !self.scanner.take(">") {
				return Err(self.expected(&[">"]));
			}

			let end = self.scanner.cursor();
			end_tag = Some(ast::Range { start, end });
		}

		Ok(ast::Element {
			start,
			end: self.scanner.cursor(),
			tag_name,
			end_tag,
			attributes,
			bindings,
			children,
		})
	}

	fn parse_tag_name(&mut self) -> Result<ast::Identifier, ParserError> {
		let start = self.scanner.cursor();
		let mut name = String::new();

		if let Some(char) = self.scanner.peek().cloned() {
			self.scanner.next();
			name.push(char);
		} else {
			return Err(self.unexpected());
		}

		while let Some(char) = self.scanner.peek().cloned() {
			if char.is_whitespace() || char == '>' {
				break;
			}

			self.scanner.next();
			name.push(char);
		}

		Ok(ast::Identifier {
			start,
			end: self.scanner.cursor(),
			name,
		})
	}

	fn parse_attribute(&mut self) -> Result<ast::Attribute, ParserError> {
		let start = self.scanner.cursor();

		let name = self.parse_attribute_name()?;
		self.skip_whitespace();

		if !self.scanner.take("=") {
			return Err(self.expected(&["="]));
		}
		self.skip_whitespace();

		if let Some(char) = self.scanner.peek() {
			if char == &'(' {
				self.scanner.next();
				self.skip_whitespace();

				let expr = self.parse_javascript()?;
				self.skip_whitespace();

				if !self.scanner.take(")") {
					return Err(self.expected(&[")"]));
				}

				return Ok(ast::Attribute::Binding(ast::Binding {
					start,
					end: self.scanner.cursor(),
					name,
					value: expr,
				}));
			}

			if char == &'"' || char == &'\'' {
				let value = self.parse_string()?;

				return Ok(ast::Attribute::Static(ast::StaticAttribute {
					start,
					end: self.scanner.cursor(),
					name,
					value: Some(value),
				}));
			}

			Err(self.expected(&["\"", "'", "("]))
		} else {
			return Err(self.unexpected());
		}
	}

	fn parse_attribute_name(&mut self) -> Result<ast::Identifier, ParserError> {
		let start = self.scanner.cursor();
		let mut name = String::new();

		while let Some(char) = self.scanner.peek() {
			if char == &'\0' || char == &'"' || char == &'\'' || char == &'/' || char == &'>' {
				return Err(self.unexpected());
			}

			if char.is_whitespace() || char == &'=' {
				break;
			}

			name.push(*char);
			self.scanner.next();
		}

		Ok(ast::Identifier {
			start,
			end: self.scanner.cursor(),
			name,
		})
	}

	fn parse_bindings(&mut self) -> Result<Option<ast::NodeCollection<ast::Binding>>, ParserError> {
		let start = self.scanner.cursor();

		if !self.scanner.take("(") {
			return Err(self.expected(&["("]));
		}
		self.skip_whitespace();

		let mut bindings = Vec::new();

		while let Some(char) = self.scanner.peek().cloned() {
			if char == ')' {
				self.scanner.next();
				break;
			}

			bindings.push(self.parse_binding()?);
			self.skip_whitespace();

			if char == ',' {
				self.scanner.next();
				self.skip_whitespace();
				continue;
			} else {
				if !self.scanner.take(")") {
					return Err(self.expected(&[")"]));
				}

				break;
			}
		}

		Ok(Some(ast::NodeCollection {
			start,
			end: self.scanner.cursor(),
			nodes: bindings,
		}))
	}

	fn parse_binding(&mut self) -> Result<ast::Binding, ParserError> {
		let start = self.scanner.cursor();

		let name = self.parse_identifier()?;
		self.skip_whitespace();

		if !self.scanner.take(":") {
			return Err(self.expected(&[":"]));
		}
		self.skip_whitespace();

		let expr = self.parse_javascript()?;
		self.skip_whitespace();

		Ok(ast::Binding {
			start,
			end: self.scanner.cursor(),
			name,
			value: expr,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_element() {
		let mut parser = Parser::new("<div></div>".to_owned());
		let element = parser.parse_element().unwrap();

		assert_eq!(element.tag_name.name, "div");
		assert_eq!(element.attributes.len(), 0);
		assert!(element.bindings.is_none());
		assert_eq!(element.children.len(), 0);
	}
}
