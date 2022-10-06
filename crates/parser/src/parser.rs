mod comment;
mod dependency;
mod element;
mod flow_control;
mod identifier;
mod javascript;
mod literal;
mod text;

use std::ops::Deref;

pub(crate) use identifier::*;

use crate::*;

pub struct Parser {
	scanner: Scanner,
	debug: bool,
}

impl Parser {
	pub fn new(input: String) -> Self {
		Self {
			scanner: Scanner::new(&input),
			debug: false
		}
	}

	#[allow(dead_code)]
	pub fn set_debug(&mut self, value: bool) {
		self.debug = value;
	}

	fn skip_whitespace(&mut self) {
		loop {
			if let Some(char) = self.scanner.peek() {
				if char.is_whitespace() {
					self.scanner.next();
					continue;
				}

				if char == &'/' {
					if let Some(char) = self.scanner.next() {
						match char {
							&'/' => {
								while let Some(char) = self.scanner.next() {
									if char == &'\n' {
										break;
									}
								}

								continue;
							}

							&'*' => {
								while let Some(char) = self.scanner.next() {
									if char == &'*' {
										self.scanner.next();

										if self.scanner.take("/") {
											break;
										}
									}
								}

								continue;
							}

							_ => {
								self.scanner.back();
								self.scanner.back();
								break;
							}
						}
					} else {
						self.scanner.back();
						break;
					}
				}
			}

			break;
		}
	}

	fn unexpected(&self) -> ParserError {
		let error = ParserError {
			position: self.scanner.cursor(),
			positives: Vec::new(),
		};

		if self.debug {
			panic!("{:?}", error);
		}

		error
	}

	fn expected(&self, positives: &[&str]) -> ParserError {
		let error = ParserError {
			position: self.scanner.cursor(),
			positives: positives
				.into_iter()
				.map(|x| x.deref().to_owned())
				.collect(),
		};

		if self.debug {
			panic!("{:?}", error);
		}

		error
	}

	pub fn next(&mut self) -> Result<Option<ast::Node>, ParserError> {
		self.skip_whitespace();

		if self.scanner.is_done() {
			return Ok(None);
		}

		if self.scanner.test("using") {
			return Ok(Some(self.parse_dependency_statement()?.into()));
		}

		if self.scanner.test("<") {
			return Ok(Some(self.parse_element()?.into()));
		}

		Err(self.unexpected())
	}

	pub fn parse_children(&mut self) -> Result<Vec<ast::Node>, ParserError> {
		let mut children = Vec::new();

		loop {
			if let Some(char) = self.scanner.peek().cloned() {
				if self.scanner.test("</") || char == '}' {
					break;
				}

				if char == '<' {
					self.scanner.next();
					if let Some(char) = self.scanner.peek() {
						children.push(if char == &'!' {
							self.scanner.back();
							self.parse_comment()?.into()
						} else {
							self.scanner.back();
							self.parse_element()?.into()
						});
					}
					continue;
				}

				if char == '{' {
					children.push(self.parse_text_binding()?.into());
					continue;
				}

				if char == '#' {
					children.push(self.parse_flow_control()?.into());
					continue;
				}

				let text = self.parse_text()?;

				if &text.content != "" {
					children.push(ast::Node::Text(text));
				}
			} else {
				return Err(self.unexpected());
			}
		}

		Ok(children)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	// NOTE! This test module should only contain tests specific to the methods defined in this file.
	// Methods as well as their tests which are not common utillity should be specified in a seperate file.

	#[test]
	fn test_skip_whitespace() {
		let mut parser = Parser::new("foo bar".to_owned());
		parser.scanner.set_cursor(3);
		parser.skip_whitespace();
		assert_eq!(parser.scanner.peek(), Some(&'b'));
	}

	#[test]
	fn test_skip_comment() {
		let mut parser = Parser::new("foo//bar\nbaz".to_owned());
		parser.scanner.set_cursor(3);
		parser.skip_whitespace();
		assert_eq!(parser.scanner.peek(), Some(&'b'));
	}

	#[test]
	fn test_skip_multiline_comment() {
		let mut parser = Parser::new("foo/*\nbar\n*/baz".to_owned());
		parser.scanner.set_cursor(3);
		parser.skip_whitespace();
		assert_eq!(parser.scanner.peek(), Some(&'b'));
	}
}
