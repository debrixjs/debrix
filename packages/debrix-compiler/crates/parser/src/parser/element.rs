use crate::*;

impl Parser {
	pub fn parse_element(&mut self) -> Result<ast::Element, ParserError> {
		let start = self.scanner.cursor();
		let mut self_closing = false;
		let mut attributes = Vec::new();
		let mut children = Vec::new();
		let mut start_tag = ast::Range::new(start, 0);
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

		start_tag.end = self.scanner.cursor();

		if !self_closing {
			children = self.parse_children()?;

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
			start_tag,
			end_tag,
			attributes,
			children,
		})
	}

	fn parse_tag_name(&mut self) -> Result<ast::Identifier, ParserError> {
		let start = self.scanner.cursor();
		let mut name = String::new();

		if let Some(char) = self.scanner.peek().cloned() {
			self.scanner.next();
			assert!(char != '#');
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

		if self.scanner.take("{") {
			if self.scanner.take("...") {
				let value = self.parse_javascript()?;

				if !self.scanner.take("}") {
					return Err(self.expected(&["}"]));
				}
				
				Ok(ast::Attribute::Spread(ast::SpreadAttribute {
					start,
					end: self.scanner.cursor(),
					value
				}))
			} else {
				let name = self.parse_javascript_identifier()?;

				if !self.scanner.take("}") {
					return Err(self.expected(&["}"]));
				}
				
				Ok(ast::Attribute::ShortBinding(ast::ShortBindingAttribute {
					start,
					end: self.scanner.cursor(),
					name
				}))
			}
		} else {
			let name = self.parse_attribute_name()?;
			self.skip_whitespace();
	
			if !self.scanner.take("=") {
				return Err(self.expected(&["="]));
			}
			self.skip_whitespace();
	
			if let Some(char) = self.scanner.peek() {
				match char {
					&'"' | &'\'' => {
						let value = self.parse_string()?;
	
						Ok(ast::Attribute::Static(ast::StaticAttribute {
							start,
							end: self.scanner.cursor(),
							name,
							value: Some(value),
						}))
					}
	
					&'{' => {
						self.scanner.next();
						self.skip_whitespace();
	
						let expr = self.parse_javascript()?;
						self.skip_whitespace();
	
						if !self.scanner.take("}") {
							return Err(self.expected(&["}"]));
						}
	
						Ok(ast::Attribute::Binding(ast::BindingAttribute {
							start,
							end: self.scanner.cursor(),
							name,
							value: expr,
						}))
					},
	
					_ => {
						return Err(self.expected(&["\"", "'", "{"]));
					}
				}
			} else {
				Err(self.unexpected())
			}
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
}

#[cfg(test)]
mod tests {
	use super::*;

	fn parse(input: &str) -> ast::Element {
		let mut parser = Parser::new(input.to_owned());
		parser.set_debug(true);
		parser.parse_element().unwrap()
	}

	#[test]
	fn test_element() {
		let element = parse("<div></div>");

		assert_eq!(element.tag_name.name, "div");
		assert_eq!(element.attributes.len(), 0);
		assert_eq!(element.children.len(), 0);
	}

	#[test]
	fn test_parse_inner_surrounding_spaces() {
		let element = parse("<p> foo </p>");

		assert_eq!(element.children.len(), 1);

		match element.children.get(0).unwrap() {
			ast::Node::Text(text) => {
				assert_eq!(text.content, " foo ");
			}

			_ => panic!("expected text"),
		}
	}

	#[test]
	fn test_accurate_tag_ranges() {
		let element = parse("<p></p>");

		assert_eq!(element.start_tag.start, 0);
		assert_eq!(element.start_tag.end, 3);

		if let Some(range) = element.end_tag {
			assert_eq!(range.start, 3);
			assert_eq!(range.end, 7);
		} else {
			panic!("expected end_tag")
		}
	}
}
