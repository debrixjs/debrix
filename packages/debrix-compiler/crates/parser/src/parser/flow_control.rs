use crate::*;

impl Parser {
	pub fn parse_flow_control(&mut self) -> Result<ast::FlowControl, ParserError> {
		let start = self.scanner.cursor();

		if !self.scanner.take("<#") {
			return Err(self.expected(&["<#"]));
		}

		if self.scanner.take("when") {
			self.skip_whitespace();

			if !self.scanner.take("{") {
				return Err(self.expected(&["{"]));
			}

			self.skip_whitespace();
			let condition = self.parse_javascript()?;
			self.skip_whitespace();

			if !self.scanner.take("}") {
				return Err(self.expected(&["}"]));
			}

			if !self.scanner.take(">") {
				return Err(self.expected(&[">"]));
			}

			self.skip_whitespace();
			let children = self.parse_children()?;

			let mut chain = Vec::new();
			loop {
				let cursor = self.scanner.cursor();
				self.skip_whitespace();

				if self.scanner.test("<#") {
					chain.push(self.parse_flow_control_else()?);
				} else {
					self.scanner.set_cursor(cursor);
					break;
				}
			}

			if !self.scanner.take("</") {
				return Err(self.expected(&["</"]));
			}

			if !self.scanner.take("when") {
				return Err(self.expected(&["when"]));
			}

			if !self.scanner.take(">") {
				return Err(self.expected(&[">"]));
			}

			return Ok(ast::FlowControl::When(ast::FlowControlWhen {
				start,
				end: self.scanner.cursor(),
				condition: Box::new(condition),
				children,
				chain,
			}));
		}

		if self.scanner.take("each") {
			self.skip_whitespace();

			if !self.scanner.take("{") {
				return Err(self.expected(&["{"]));
			}

			self.skip_whitespace();
			let iterator = self.parse_javascript_identifier()?;
			self.skip_whitespace();

			if !self.scanner.take("in") {
				return Err(self.expected(&["in"]));
			}

			self.skip_whitespace();

			let iterable = self.parse_javascript()?;
			self.skip_whitespace();

			if !self.scanner.take("}") {
				return Err(self.expected(&["}"]));
			}

			if !self.scanner.take(">") {
				return Err(self.expected(&[">"]));
			}

			self.skip_whitespace();
			let children = self.parse_children()?;

			if !self.scanner.take("</") {
				return Err(self.expected(&["</"]));
			}

			if !self.scanner.take("each") {
				return Err(self.expected(&["each"]));
			}

			if !self.scanner.take(">") {
				return Err(self.expected(&[">"]));
			}

			return Ok(ast::FlowControl::Each(ast::FlowControlEach {
				start,
				end: self.scanner.cursor(),
				iterable: Box::new(iterable),
				iterator: Box::new(iterator),
				children,
			}));
		}

		Err(self.unexpected())
	}

	fn parse_flow_control_else(&mut self) -> Result<ast::FlowControlElse, ParserError> {
		let start = self.scanner.cursor();
		let mut condition = None;

		if !self.scanner.take("<#") {
			return Err(self.expected(&["<#"]));
		}

		if !self.scanner.take("else") {
			return Err(self.expected(&["else"]));
		}

		if self.skip_whitespace() {
			if self.scanner.take("when") {
				self.skip_whitespace();

				if !self.scanner.take("{") {
					return Err(self.expected(&["{"]));
				}
	
				self.skip_whitespace();
				condition = Some(self.parse_javascript()?);
				self.skip_whitespace();
	
				if !self.scanner.take("}") {
					return Err(self.expected(&["}"]));
				}

				self.skip_whitespace();
			}
		}

		if !self.scanner.take(">") {
			return Err(self.expected(&[">"]));
		}

		let children = self.parse_children()?;

		return Ok(ast::FlowControlElse {
			start,
			end: self.scanner.cursor(),
			condition,
			children,
		});
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
		let node = parse("<#when {foo}>bar</when>");

		match node {
			ast::FlowControl::When(node) => {
				assert!(node.children.len() == 1);
				assert!(node.chain.len() == 0);

				match *node.condition {
					ast::javascript::Expression::Identifier(expr) => {
						assert!(&expr.name == "foo");
					}
					_ => panic!("expected identifier"),
				}

				match node.children.get(0).unwrap() {
					ast::Node::Text(node) => {
						assert_eq!(&node.content, "bar");
					}
					_ => panic!("expected text"),
				}
			}
			_ => panic!("expected when"),
		}
	}

	#[test]
	fn test_flow_control_else() {
		let node = parse("<#when {foo}>bar<#else>baz</when>");

		match node {
			ast::FlowControl::When(node) => {
				assert!(node.children.len() == 1);
				assert!(node.chain.len() == 1);

				let chained = node.chain.get(0).unwrap();
				assert!(chained.children.len() == 1);
				assert!(chained.condition.is_none());

				match chained.children.get(0).unwrap() {
					ast::Node::Text(node) => {
						assert_eq!(&node.content, "baz");
					}
					_ => panic!("expected text"),
				}
			}
			_ => panic!("expected when"),
		}
	}

	#[test]
	fn test_flow_control_else_when() {
		let node = parse("<#when {foo}>bar<#else when {baz}>qux</when>");
		match node {
			ast::FlowControl::When(node) => {
				assert!(node.children.len() == 1);
				assert!(node.chain.len() == 1);

				let chained = node.chain.get(0).unwrap();
				assert!(chained.children.len() == 1);

				if let Some(expr) = &chained.condition {
					match expr {
						ast::javascript::Expression::Identifier(expr) => {
							assert!(&expr.name == "baz");
						}
						_ => panic!("expected identifier"),
					}
				} else {
					panic!("expected condition");
				}
			}
			_ => panic!("expected when"),
		}
	}

	#[test]
	fn test_flow_control_each() {
		let node = parse("<#each {foo in bar}>baz</each>");

		match node {
			ast::FlowControl::Each(node) => {
				assert!(node.children.len() == 1);
				assert!(&*node.iterator.name == "foo");

				match *node.iterable {
					ast::javascript::Expression::Identifier(expr) => {
						assert!(&expr.name == "bar");
					}
					_ => panic!("expected identifier"),
				}

				match node.children.get(0).unwrap() {
					ast::Node::Text(node) => {
						assert_eq!(&node.content, "baz");
					}
					_ => panic!("expected text"),
				}
			}
			_ => panic!("expected each"),
		}
	}
}
