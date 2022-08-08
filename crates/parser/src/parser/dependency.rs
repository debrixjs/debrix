use crate::*;

impl Parser {
	fn parse_named_specifiers(
		&mut self,
	) -> Result<ast::NodeCollection<ast::DependencyNamedSpecifier>, ParserError> {
		let mut specifiers = ast::NodeCollection::default();

		if let Some(ch) = self.iter.peek_next() {
			if ch == '{' {
				specifiers.location.start = self.iter.position();
				self.iter.next();

				while let Some(ch) = self.iter.peek_next() {
					if ch.is_whitespace() {
						self.iter.next();
						continue;
					}

					if ch == '}' {
						self.iter.next();
						specifiers.location.end = self.iter.position();
						return Ok(specifiers);
					}

					if ch == '#' {
						self.iter.next();
						let ident = self.parse_identifier()?;

						specifiers.nodes.push(ast::DependencyNamedSpecifier {
							location: ident.location.clone(),
							imported: None,
							local: None,
							usage: Some(ident),
						});
					} else if self.is_identifier() {
						let ident = self.parse_identifier()?;

						self.skip_whitespace()?;

						if self.try_str("as")? {
							self.skip_whitespace()?;

							let local = self.parse_identifier()?;

							specifiers.nodes.push(ast::DependencyNamedSpecifier {
								location: Location::new(
									ident.location.start.clone(),
									local.location.end.clone(),
								),
								imported: Some(ident),
								local: Some(local),
								usage: None,
							});
						} else {
							specifiers.nodes.push(ast::DependencyNamedSpecifier {
								location: ident.location.clone(),
								imported: Some(ident),
								local: None,
								usage: None,
							});
						}

						continue;
					}
				}

				// unexpected eof
			}
		}

		// unexpected eof
		Err(ParserError::eof(self.iter.position()))
	}

	pub fn parse_dependency_statement(&mut self) -> Result<ast::DependencyStatement, ParserError> {
		let start = self.iter.position();
		self.iter.skip_n(5);

		self.skip_whitespace()?;

		let mut named_specifiers: Option<NodeCollection<ast::DependencyNamedSpecifier>> = None;
		let mut default_specifier: Option<ast::DependencyDefaultSpecifier> = None;
		let mut allow_named = true;

		if let Some(ch) = self.iter.peek_next() {
			if ch == '#' || self.is_identifier() {
				// Named specifiers are not allowed after the default specifier,
				// unless the default specifier has a proceeding comma.
				allow_named = false;

				if ch == '#' {
					let start = self.iter.position();
					self.iter.next();

					let ident = self.parse_identifier()?;
					let end = self.iter.position();

					default_specifier = Some(ast::DependencyDefaultSpecifier {
						location: Location::new(start, end),
						local: None,
						usage: Some(ident),
					});
				} else if self.is_identifier() {
					let start = self.iter.position();
					let ident = self.parse_identifier()?;
					let end = self.iter.position();

					default_specifier = Some(ast::DependencyDefaultSpecifier {
						location: Location::new(start, end),
						local: Some(ident),
						usage: None,
					});
				} else {
					unreachable!();
				}

				if let Some(ch) = self.iter.peek_next() {
					if ch == ',' {
						allow_named = true;
						self.iter.next();
					}

					self.skip_whitespace()?;
				} else {
					// unexpected eof
				}
			}

			if allow_named {
				if let Some(ch) = self.iter.peek_next() {
					if ch == '{' {
						named_specifiers = Some(self.parse_named_specifiers().unwrap());
					}
				} else {
					// unexpected eof
				}
			}

			self.skip_whitespace()?;

			self.expect_str("from")?;

			self.skip_whitespace()?;

			let source = self.parse_string()?;

			let end = self.iter.position();

			Ok(ast::DependencyStatement {
				location: Location::new(start, end),
				default: default_specifier,
				named: named_specifiers,
				source,
			})
		} else {
			Err(ParserError::eof(self.iter.position()))
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_dependency_statement() {
		let mut parser = Parser::new("using foo from \"bar\"");
		let statement = parser.parse_dependency_statement().unwrap();

		assert_eq!(statement.default.unwrap().local.unwrap().name, "foo");
		assert_eq!(statement.source.value, "bar");
	}
}
