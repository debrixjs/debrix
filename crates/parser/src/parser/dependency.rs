use crate::*;

impl Parser {
	pub fn parse_dependency_statement(&mut self) -> Result<ast::DependencyStatement, ParserError> {
		let start = self.scanner.cursor();
		let mut default: Option<ast::DependencyDefaultSpecifier> = None;
		let mut named: Vec<ast::DependencyNamedSpecifier> = Vec::new();
		let mut named_start = 0_usize;
		let mut named_end = 0_usize;

		if !self.scanner.take("using") {
			return Err(self.expected(&["using"]));
		}

		self.skip_whitespace();

		if let Some(char) = self.scanner.peek().cloned() {
			let start = self.scanner.cursor();

			if char == '#' {
				self.scanner.next();
				let usage = self.parse_identifier()?;
				let end = self.scanner.cursor();

				default = Some(ast::DependencyDefaultSpecifier {
					start,
					end,
					local: None,
					usage: Some(usage),
				});
			}

			if is_identifier(&char) {
				let local = self.parse_identifier()?;

				default = Some(ast::DependencyDefaultSpecifier {
					start,
					end: self.scanner.cursor(),
					local: Some(local),
					usage: None,
				});
			}
		} else {
			self.unexpected();
		}

		let mut has_named = false;

		if default.is_some() {
			self.skip_whitespace();

			if self.scanner.take(",") {
				self.skip_whitespace();

				let cursor = self.scanner.cursor();
				if self.scanner.take("{") {
					named_start = cursor;
					has_named = true;
				}
			}
		} else {
			let cursor = self.scanner.cursor();
			if self.scanner.take("{") {
				named_start = cursor;
				has_named = true;
			} else {
				return Err(self.expected(&["specifier"]));
			}
		}

		if has_named {
			loop {
				self.skip_whitespace();

				if let Some(char) = self.scanner.peek().cloned() {
					if char == '}' {
						self.scanner.next();
						self.skip_whitespace();

						named_end = self.scanner.cursor();
						break;
					} else if char == '#' {
						let start = self.scanner.cursor();

						self.scanner.next();
						self.skip_whitespace();

						let usage = self.parse_identifier()?;

						named.push(ast::DependencyNamedSpecifier {
							start,
							end: self.scanner.cursor(),
							imported: None,
							local: None,
							usage: Some(usage),
						});
					} else if is_identifier(&char) {
						let imported = self.parse_identifier()?;
						let mut usage = None;

						self.skip_whitespace();
						if self.scanner.take("as") {
							if let Some(char) = self.scanner.peek() {
								if char == &'#' {
									if self.scanner.next().is_none() {
										return Err(self.unexpected());
									}

									usage = Some(self.parse_identifier()?)
								} else if is_identifier(char) {
									usage = Some(self.parse_identifier()?)
								} else {
									return Err(self.expected(&["identifier"]));
								}
							} else {
								return Err(self.unexpected());
							}
						}

						named.push(ast::DependencyNamedSpecifier {
							start,
							end: self.scanner.cursor(),
							imported: Some(imported),
							local: None,
							usage,
						});
					} else {
						return Err(self.unexpected());
					}
				} else {
					return Err(self.unexpected());
				}
			}
		}

		let named = if has_named {
			Some(ast::NodeCollection {
				start: named_start,
				end: named_end,
				nodes: named,
			})
		} else {
			None
		};

		if !self.scanner.take("from") {
			return Err(self.expected(&["from"]));
		}

		self.skip_whitespace();

		let source = self.parse_string()?;

		Ok(ast::DependencyStatement {
			start,
			end: self.scanner.cursor(),
			default,
			named,
			source,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_dependency_statement() {
		let mut parser = Parser::new("using foo from \"bar\"".to_owned());
		let statement = parser.parse_dependency_statement().unwrap();

		assert_eq!(statement.default.unwrap().local.unwrap().name, "foo");
		assert_eq!(statement.source.value, "bar");
	}
}
