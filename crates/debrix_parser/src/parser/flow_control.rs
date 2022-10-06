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

	#[test]
	fn test_flow_control_when() {
		let mut parser = Parser::new("#when foo { bar }".to_owned());
		parser.parse_flow_control().unwrap();
	}

	#[test]
	fn test_flow_control_else() {
		let mut parser = Parser::new("#else { bar }".to_owned());
		parser.parse_flow_control().unwrap();
	}

	#[test]
	fn test_flow_control_else_when() {
		let mut parser = Parser::new("#else when foo { bar }".to_owned());
		parser.parse_flow_control().unwrap();
	}

	#[test]
	fn test_flow_control_each() {
		let mut parser = Parser::new("#each foo of bar { baz }".to_owned());
		parser.parse_flow_control().unwrap();
	}
}
