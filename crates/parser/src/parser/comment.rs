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

	#[test]
	fn test_comment() {
		let mut parser = Parser::new("<!-- hello world -->".to_owned());
		let comment = parser.parse_comment().unwrap();

		assert_eq!(comment.comment, " hello world ");
	}
}
