use crate::*;

impl Parser {
	pub fn parse_flow_control(&mut self) -> Result<ast::FlowControl, ParserError> {
		let start = self.scanner.cursor();

		if !self.scanner.take("#") {
			return Err(self.expected(&["#"]));
		}

		if self.scanner.take("when") {
			self.skip_whitespace();

			let condition = self.parse_javascript()?;
			self.skip_whitespace();
			let children = self.parse_flow_control_children()?;

			return Ok(ast::FlowControl::When(ast::FlowControlWhen {
				start,
				end: self.scanner.cursor(),
				condition: Box::new(condition),
				children,
			}));
		}

		if self.scanner.take("else") {
			self.skip_whitespace();

			let mut condition = None;
			if self.scanner.take("when") {
				self.skip_whitespace();
				condition = Some(self.parse_javascript()?);
			}

			self.skip_whitespace();
			let children = self.parse_flow_control_children()?;

			return Ok(ast::FlowControl::Else(ast::FlowControlElse {
				start,
				end: self.scanner.cursor(),
				condition,
				children,
			}));
		}

		if self.scanner.take("each") {
			self.skip_whitespace();

			let iterator = self.parse_javascript()?;
			self.skip_whitespace();

			if !self.scanner.take("of") {
				return Err(self.expected(&["of"]));
			}

			self.skip_whitespace();

			let iterable = self.parse_javascript()?;
			self.skip_whitespace();
			let children = self.parse_flow_control_children()?;

			return Ok(ast::FlowControl::Each(ast::FlowControlEach {
				start,
				end: self.scanner.cursor(),
				iterable: Box::new(iterable),
				iterator: Box::new(iterator),
				children
			}));
		}

		Err(self.unexpected())
	}

	fn parse_flow_control_children(&mut self) -> Result<Vec<ast::Node>, ParserError> {
		if !self.scanner.take("{") {
			return Err(self.expected(&["{"]));
		}
		
		self.skip_whitespace();
		let children = self.parse_children()?;
		self.skip_whitespace();

		if !self.scanner.take("}") {
			return Err(self.expected(&["}"]));
		}

		Ok(children)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn parse(input: &str) -> ast::FlowControl {
		let mut parser = Parser::new(input.to_owned());
		parser.set_debug(true);
		parser.parse_flow_control().unwrap()
	}

	#[test]
	fn test_flow_control_when() {
		parse("#when foo { bar }");
	}

	#[test]
	fn test_flow_control_else() {
		parse("#else { bar }");
	}

	#[test]
	fn test_flow_control_else_when() {
		parse("#else when foo { bar }");
	}

	#[test]
	fn test_flow_control_each() {
		parse("#each foo of bar { baz }");
	}
}
