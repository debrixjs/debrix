use crate::*;

impl Parser {
	pub fn parse_comment(&mut self) -> Result<ast::Comment, ParserError> {
		let start = self.iter.position();

		self.expect_str("<!--")?;

		let mut comment = String::new();

		while let Some(ch) = self.iter.next() {
			if ch == '-' {
				if let Some(ch) = self.iter.next() {
					if ch == '-' {
						if let Some(ch) = self.iter.next() {
							if ch == '>' {
								break;
							} else {
								comment.push('-');
								comment.push('-');
								comment.push(ch);
							}
						}
					} else {
						comment.push('-');
						comment.push(ch);
					}
				}
			}

			comment.push(ch);
		}

		let end = self.iter.position();

		Ok(ast::Comment {
			location: Location::new(start, end),
			comment
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_comment() {
		let mut parser = Parser::new("<!-- hello world -->");
		let comment = parser.parse_comment().unwrap();
		
		assert_eq!(comment.comment, " hello world ");
	}
}
