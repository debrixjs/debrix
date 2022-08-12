use crate::*;

impl Parser {
	pub fn parse_element(&mut self) -> Result<ast::Element, ParserError> {
		let start = self.iter.position();

		self.expect('<')?;

		let tag = self.parse_tag_name()?;
		self.skip_whitespace()?;

		let mut self_closing = false;
		let mut attributes = Vec::new();
		let mut bindings = None;
		let mut children = Vec::new();

		loop {
			if let Some(ch) = self.iter.peek_next() {
				if ch.is_ascii_whitespace() {
					continue;
				}

				match ch {
					'(' => {
						bindings = self.parse_bindings()?;

						if let Some(ch) = self.iter.next() {
							if ch != '>' {
								return Err(ParserError::expected(
									self.iter.at_length(1),
									&[">"],
								));
							}
						} else {
							return Err(ParserError::eof(self.iter.position()));
						}
					}
					'>' => {
						self.iter.next();
						break;
					}
					'\0' | '"' | '\'' | '/' | '=' => {
						return Err(ParserError::unexpected(
							self.iter.at_length(1),
							&["\0", "\"", "'", "/", "="],
						));
					}
					_ => attributes.push(self.parse_attribute()?),
				}
			} else {
				return Err(ParserError::eof(self.iter.position()));
			}
		}

		if let Some(ch) = self.iter.peek_next() {
			if ch == '/' {
				self.iter.next();
				self_closing = true;
			}
		} else {
			return Err(ParserError::eof(self.iter.position()));
		}

		self.expect('>')?;

		if !self_closing {
			children = self.parse_children()?;

			self.expect_str("</")?;
			self.expect_str(&tag.name)?;
			self.expect('>')?;
		}

		let end = self.iter.position();

		Ok(ast::Element {
			location: Location::new(start, end),
			tag,
			self_closing,
			attributes,
			bindings,
			children,
		})
	}

	fn parse_tag_name(&mut self) -> Result<ast::Identifier, ParserError> {
		let start = self.iter.position();
		let mut tag_name = String::new();

		if let Some(ch) = self.iter.peek() {
			tag_name.push(ch);
		} else {
			return Err(ParserError::eof(self.iter.position()));
		}

		while let Some(ch) = self.iter.peek_next() {
			if ch.is_whitespace() || ch == '>' {
				break;
			}

			self.iter.next();
			tag_name.push(ch);
		}

		let end = self.iter.position();

		Ok(ast::Identifier {
			location: Location::new(start, end),
			name: tag_name,
		})
	}

	fn parse_attribute(&mut self) -> Result<ast::Attribute, ParserError> {
		let start = self.iter.position();

		let name = self.parse_attribute_name()?;
		self.skip_whitespace()?;

		self.expect('=')?;
		self.skip_whitespace()?;

		if let Some(ch) = self.iter.peek() {
			if ch == '(' {
				self.iter.next();
				self.skip_whitespace()?;

				let expr = self.parse_javascript_expression(&[')'])?;
				self.skip_whitespace()?;

				self.expect(')')?;

				let end = self.iter.position();

				return Ok(ast::Attribute::Binding(ast::Binding {
					location: Location::new(start, end),
					name,
					value: expr,
				}));
			}

			if ch == '"' || ch == '\'' {
				let value = self.parse_string()?;
				let end = self.iter.position();

				return Ok(ast::Attribute::Static(ast::StaticAttribute {
					location: Location::new(start, end),
					name,
					value: Some(value),
				}));
			}

			Err(ParserError::expected(
				self.iter.at_length(1),
				&["\"", "'", "("],
			))
		} else {
			return Err(ParserError::eof(self.iter.position()));
		}
	}

	fn parse_attribute_name(&mut self) -> Result<ast::Identifier, ParserError> {
		let start = self.iter.position();
		let mut name = String::new();

		while let Some(ch) = self.iter.next() {
			if ch == '\0' || ch == '"' || ch == '\'' || ch == '/' || ch == '>' {
				return Err(ParserError::unexpected(
					self.iter.at_length(1),
					&["\0", "\"", "'", "/", ">"],
				));
			}

			if ch.is_whitespace() || ch == '=' {
				break;
			}

			name.push(ch);
		}

		let end = self.iter.position();

		Ok(ast::Identifier {
			location: Location::new(start, end),
			name,
		})
	}

	fn parse_bindings(&mut self) -> Result<Option<ast::NodeCollection<ast::Binding>>, ParserError> {
		let start = self.iter.position();

		self.expect('(')?;
		self.skip_whitespace()?;

		let mut bindings = Vec::new();

		while let Some(ch) = self.iter.peek() {
			if ch == ')' {
				self.iter.next();
				break;
			}

			bindings.push(self.parse_binding()?);
			self.skip_whitespace()?;

			if ch == ',' {
				self.iter.next();
				self.skip_whitespace()?;
				continue;
			} else {
				self.expect(')')?;
			}
		}

		let end = self.iter.position();

		Ok(Some(ast::NodeCollection {
			location: Location::new(start, end),
			nodes: bindings,
		}))
	}

	fn parse_binding(&mut self) -> Result<ast::Binding, ParserError> {
		let start = self.iter.position();

		let name = self.parse_identifier()?;
		self.skip_whitespace()?;

		self.expect(':')?;
		self.skip_whitespace()?;

		let expr = self.parse_javascript_expression(&[',', ')'])?;
		self.skip_whitespace()?;

		let end = self.iter.position();

		Ok(ast::Binding {
			location: Location::new(start, end),
			name,
			value: expr,
		})
	}

	fn parse_children(&mut self) -> Result<Vec<ast::Node>, ParserError> {
		let mut children = Vec::new();

		while let Some(ch) = self.iter.peek() {
			if self.test_str("</") {
				break;
			}

			if ch == '<' {
				if let Some(ch) = self.iter.peek_n(2) {
					children.push(
						if ch == '!' {
							self.parse_comment()?.into_node()
						} else {
							self.parse_element()?.into_node()
						}
					);
				}
			}

			if ch == '{' {
				children.push(self.parse_text_binding()?.into_node());
			}

			children.push(ast::Node::Text(self.parse_text()?));
		}

		Ok(children)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_element() {
		let mut parser = Parser::new("<div></div>");
		let element = parser.parse_element().unwrap();

		assert_eq!(element.tag.name, "div");
		assert_eq!(element.attributes.len(), 0);
		assert!(element.bindings.is_none());
		assert_eq!(element.children.len(), 0);
	}
}
