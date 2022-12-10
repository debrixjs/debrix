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

			if is_identifier(&char) {
				let usage = self.parse_identifier()?;
				let mut local = None;

				self.skip_whitespace();

				if let Some(char) = self.scanner.peek().cloned() {
					let cursor = self.scanner.cursor();

					if self.scanner.take("from") {
						self.skip_whitespace();

						if self.scanner.test("from") {
							local = Some(ast::Identifier {
								start: cursor,
								end: cursor + 4,
								name: "from".to_owned(),
							});
						} else {
							self.scanner.set_cursor(cursor);
						}
					} else if is_identifier(&char) {
						local = Some(self.parse_identifier()?);
					}
				} else {
					self.unexpected();
				}

				default = Some(ast::DependencyDefaultSpecifier {
					start,
					end: self.scanner.cursor(),
					local,
					usage,
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
					} else if is_identifier(&char) {
						let usage = self.parse_identifier()?;
						self.skip_whitespace();
						let imported = self.parse_identifier()?;
						self.skip_whitespace();
						let mut local = None;

						if self.scanner.take("as") {
							self.skip_whitespace();
							local = Some(self.parse_identifier()?);
						}

						named.push(ast::DependencyNamedSpecifier {
							start,
							end: self.scanner.cursor(),
							imported,
							local,
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

	fn parse(input: &str) -> ast::DependencyStatement {
		let mut parser = Parser::new(input.to_owned());
		parser.set_debug(true);
		let node = parser.parse_dependency_statement().unwrap();
		assert!(parser.is_done());
		node
	}

	#[test]
	fn test_dependency_statement_with_unnamed_default() {
		let statement = parse("using foo from \"bar\"");

		assert_eq!(statement.default.as_ref().unwrap().usage.name, "foo");
		assert_eq!(statement.source.value, "bar");
	}

	#[test]
	fn test_dependency_statement_with_named_default() {
		let statement = parse("using foo bar from \"baz\"");

		assert_eq!(statement.default.as_ref().unwrap().usage.name, "foo");
		assert_eq!(
			statement
				.default
				.as_ref()
				.unwrap()
				.local
				.as_ref()
				.unwrap()
				.name,
			"bar"
		);
		assert_eq!(statement.source.value, "baz");
	}

	#[test]
	fn test_dependency_statement_with_default_named_from() {
		let statement = parse("using foo from from \"bar\"");

		assert_eq!(statement.default.as_ref().unwrap().usage.name, "foo");
		assert_eq!(
			statement
				.default
				.as_ref()
				.unwrap()
				.local
				.as_ref()
				.unwrap()
				.name,
			"from"
		);
		assert_eq!(statement.source.value, "bar");
	}

	#[test]
	fn test_dependency_statement_with_named() {
		let statement = parse("using { foo bar } from \"baz\"");
		let specifier = statement.named.as_ref().unwrap().nodes.get(0);

		assert_eq!(specifier.as_ref().unwrap().usage.name, "foo");
		assert_eq!(specifier.as_ref().unwrap().imported.name, "bar");
		assert_eq!(statement.source.value, "baz");
	}

	#[test]
	fn test_dependency_statement_with_named_alias() {
		let statement = parse("using { foo bar as baz } from \"qux\"");
		let specifier = statement.named.as_ref().unwrap().nodes.get(0);

		assert_eq!(specifier.as_ref().unwrap().usage.name, "foo");
		assert_eq!(specifier.as_ref().unwrap().imported.name, "bar");
		assert_eq!(
			specifier.as_ref().unwrap().local.as_ref().unwrap().name,
			"baz"
		);
		assert_eq!(statement.source.value, "qux");
	}

	#[test]
	fn test_dependency_statement_with_named_default_and_named() {
		let statement = parse("using foo bar, { baz qux } from \"quux\"");
		let named_specifier = statement.named.as_ref().unwrap().nodes.get(0);

		assert_eq!(statement.default.as_ref().unwrap().usage.name, "foo");
		assert_eq!(
			statement
				.default
				.as_ref()
				.unwrap()
				.local
				.as_ref()
				.unwrap()
				.name,
			"bar"
		);
		assert_eq!(named_specifier.as_ref().unwrap().usage.name, "baz");
		assert_eq!(named_specifier.as_ref().unwrap().imported.name, "qux");
		assert_eq!(statement.source.value, "quux");
	}
}
