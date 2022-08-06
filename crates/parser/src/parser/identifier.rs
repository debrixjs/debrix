use crate::*;

impl Parser {
	pub fn is_identifier(&self) -> bool {
		match self.iter.peek() {
			Some(ch) => ch.is_alphabetic() || ch == '_',
			None => false,
		}
	}
	
	pub fn parse_identifier(&mut self) -> Result<ast::Identifier, ParserError> {
		let start = self.iter.position();
		let mut name = String::new();

		while let Some(ch) = self.iter.next() {
			if ch.is_alphanumeric() || ch == '_' {
				name.push(ch);
			} else {
				break;
			}
		}

		self.iter.back();
	
		let end = self.iter.position();
	
		Ok(ast::Identifier {
			name,
			location: Location::new(start, end),
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_is_identifier() {
		let mut parser = Parser::new("abc");
		let ident = parser.parse_identifier().unwrap();

		assert_eq!(ident.name, "abc");
	}
}
