use crate::*;

impl Parser {
	pub fn parse_comment(&mut self) -> Result<ast::Comment, ParserError> {
		let start = self.scanner.cursor();

		if !self.scanner.take("<!--") {
			return Err(self.expected(&["<!--"]));
		}

		let mut comment = String::new();

		while let Some(char) = self.scanner.peek().cloned() {
			if char == '-' {
				if let Some(char) = self.scanner.next() {
					if char == &'-' {
						if let Some(char) = self.scanner.next() {
							if char == &'>' {
								self.scanner.next();
								break;
							} else {
								comment.push('-');
								comment.push('-');
								comment.push(char.to_owned());
							}
						}
					} else {
						comment.push('-');
						comment.push(char.to_owned());
					}
				}
			}

			comment.push(char.to_owned());
			self.scanner.next();
		}

		Ok(ast::Comment {
			start,
			end: self.scanner.cursor(),
			comment,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn parse(input: &str) -> ast::Comment {
		let mut parser = Parser::new(input.to_owned());
		parser.set_debug(true);
		let node = parser.parse_comment().unwrap();
		assert!(parser.is_done());
		node
	}

	#[test]
	fn test_comment() {
		let comment = parse("<!-- hello world -->");
		assert_eq!(comment.comment, " hello world ");
	}
}
