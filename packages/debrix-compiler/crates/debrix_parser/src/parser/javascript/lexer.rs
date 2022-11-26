use super::*;

pub struct Token {
	pub kind: TokenKind,
	pub start: usize,
	pub end: usize,
}

impl Token {
	pub fn new(kind: TokenKind, start: usize, end: usize) -> Self {
		Token { kind, start, end }
	}

	pub fn from_length(kind: TokenKind, start: usize, length: usize) -> Self {
		Self::new(kind, start, start + length)
	}
}

#[derive(Clone, PartialEq, Debug)]
pub enum TokenKind {
	EOF,

	Identifier,
	Numeric,
	String,
	Template,

	True,       // true
	False,      // false
	Null,       // null
	Delete,     // delete
	In,         // in
	Instanceof, // instanceof
	New,        // new
	Return,     // return
	This,       // this
	Typeof,     // typeof
	Void,       // void

	Plus,                     // +
	Minus,                    // -
	Increment,                // ++
	Decrement,                // --
	Not,                      // !
	BitNot,                   // ~
	Multiply,                 // *
	Exponentiate,             // **
	Divide,                   // /
	Modulo,                   // %
	LessThan,                 // <
	GreaterThan,              // >
	LessThanEqual,            // <=
	GreaterThanEqual,         // >=
	LeftShift,                // <<
	RightShift,               // >>
	UnsignedRightShift,       // >>>
	Equal,                    // ==
	NotEqual,                 // !=
	StrictEqual,              // ===
	StrictNotEqual,           // !==
	BitAnd,                   // &
	BitXor,                   // ^
	BitOr,                    // |
	LogicalAnd,               // &&
	LogicalOr,                // ||
	QuestionMark,             // ?
	Colon,                    // :
	Assign,                   // =
	PlusAssign,               // +=
	MinusAssign,              // -=
	MultiplyAssign,           // *=
	ExponentiateAssign,       // **=
	DivideAssign,             // /=
	ModuloAssign,             // %=
	LeftShiftAssign,          // <<=
	RightShiftAssign,         // >>=
	UnsignedRightShiftAssign, // >>>=
	BitAndAssign,             // &=
	BitXorAssign,             // ^=
	BitOrAssign,              // |=
	Comma,                    // ,
	Arrow,                    // =>
	OpenParen,                // (
	CloseParen,               // )
	OpenBracket,              // [
	CloseBracket,             // ]
	OpenBrace,                // {
	CloseBrace,               // }
	Dot,                      // .
	Ellipsis,                 // ...
	Semicolon,                // ;
}

pub struct Lexer<'a> {
	scanner: &'a mut Scanner,
}

impl<'a> Lexer<'a> {
	pub fn new(scanner: &'a mut Scanner) -> Self {
		Self { scanner }
	}

	pub fn is_done(&self) -> bool {
		self.scanner.is_done()
	}

	pub fn scanner(&self) -> &Scanner {
		self.scanner
	}

	pub fn scanner_mut(&mut self) -> &mut Scanner {
		self.scanner
	}

	pub fn cursor(&self) -> usize {
		self.scanner.cursor()
	}

	pub fn scan(&mut self) -> Result<Token, usize> {
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

				break;
			}
		}

		if self.scanner.is_done() {
			return Ok(Token::from_length(TokenKind::EOF, self.scanner.cursor(), 0));
		}

		let start = self.scanner.cursor();
		let char = self.scanner.peek().unwrap().clone();

		if char.is_alphabetic() || char == '_' || char == '$' {
			let mut ident = String::new();

			while let Some(char) = self.scanner.peek() {
				if char.is_alphanumeric() || char == &'_' || char == &'$' {
					ident.push(char.to_owned());
					self.scanner.next();
					continue;
				} else {
					break;
				}
			}

			assert_eq!(start + ident.len(), self.scanner.cursor());

			return Ok(Token::new(
				match ident.as_ref() {
					"true" => TokenKind::True,
					"false" => TokenKind::False,
					"null" => TokenKind::Null,
					"delete" => TokenKind::Delete,
					"in" => TokenKind::In,
					"instanceof" => TokenKind::Instanceof,
					"new" => TokenKind::New,
					"return" => TokenKind::Return,
					"this" => TokenKind::This,
					"typeof" => TokenKind::Typeof,
					"void" => TokenKind::Void,
					_ => TokenKind::Identifier,
				},
				start,
				self.scanner.cursor(),
			));
		}

		if char.is_numeric() {
			let mut has_dot = false;

			while let Some(char) = self.scanner.next() {
				if char.is_numeric() {
					continue;
				}

				if !has_dot && char == &'.' {
					has_dot = true;
				} else {
					break;
				}
			}

			return Ok(Token::new(TokenKind::Numeric, start, self.scanner.cursor()));
		}

		if char == '"' || char == '\'' {
			let quote = char.clone();

			while let Some(char) = self.scanner.next() {
				if char == &quote {
					self.scanner.next();
					break;
				} else if char == &'\\' {
					if self.scanner.is_done() {
						return Err(self.scanner.cursor());
					}
				}
			}

			return Ok(Token::new(TokenKind::String, start, self.scanner.cursor()));
		}

		if char == '`' {
			while let Some(char) = self.scanner.next().cloned() {
				if char == '`' {
					self.scanner.next();
					break;
				} else if char == '\\' {
					if self.scanner.is_done() {
						return Err(self.scanner.cursor());
					}
				} else if char == '$' {
					if self.scanner.is_done() {
						return Err(self.scanner.cursor());
					}

					if char == '{' {
						while let Some(char) = self.scanner.next() {
							if char == &'}' {
								break;
							}
						}
					} else {
						self.scanner.back();
					}
				}
			}

			return Ok(Token::new(
				TokenKind::Template,
				start,
				self.scanner.cursor(),
			));
		}

		// Consume the first character of the operator.
		self.scanner.next();

		match char {
			'!' => {
				if self.scanner.take("=") {
					if self.scanner.take("=") {
						return Ok(Token::new(
							TokenKind::StrictNotEqual,
							start,
							self.scanner.cursor(),
						));
					}

					return Ok(Token::new(
						TokenKind::NotEqual,
						start,
						self.scanner.cursor(),
					));
				}

				Ok(Token::new(TokenKind::Not, start, self.scanner.cursor()))
			}

			'%' => {
				if self.scanner.take("=") {
					return Ok(Token::new(
						TokenKind::ModuloAssign,
						start,
						self.scanner.cursor(),
					));
				}

				Ok(Token::new(TokenKind::Modulo, start, self.scanner.cursor()))
			}

			'&' => {
				if self.scanner.take("&") {
					return Ok(Token::new(
						TokenKind::LogicalAnd,
						start,
						self.scanner.cursor(),
					));
				}

				if self.scanner.take("=") {
					return Ok(Token::new(
						TokenKind::BitAndAssign,
						start,
						self.scanner.cursor(),
					));
				}

				Ok(Token::new(TokenKind::BitAnd, start, self.scanner.cursor()))
			}

			'*' => {
				if self.scanner.take("*") {
					if self.scanner.take("=") {
						return Ok(Token::new(
							TokenKind::ExponentiateAssign,
							start,
							self.scanner.cursor(),
						));
					}

					return Ok(Token::new(
						TokenKind::Exponentiate,
						start,
						self.scanner.cursor(),
					));
				}

				if self.scanner.take("=") {
					return Ok(Token::new(
						TokenKind::MultiplyAssign,
						start,
						self.scanner.cursor(),
					));
				}

				Ok(Token::new(
					TokenKind::Multiply,
					start,
					self.scanner.cursor(),
				))
			}

			'+' => {
				if self.scanner.take("+") {
					return Ok(Token::new(
						TokenKind::Increment,
						start,
						self.scanner.cursor(),
					));
				}

				if self.scanner.take("=") {
					return Ok(Token::new(
						TokenKind::PlusAssign,
						start,
						self.scanner.cursor(),
					));
				}

				Ok(Token::new(TokenKind::Plus, start, self.scanner.cursor()))
			}

			'-' => {
				if self.scanner.take("-") {
					return Ok(Token::new(
						TokenKind::Decrement,
						start,
						self.scanner.cursor(),
					));
				}

				if self.scanner.take("=") {
					return Ok(Token::new(
						TokenKind::MinusAssign,
						start,
						self.scanner.cursor(),
					));
				}

				Ok(Token::new(TokenKind::Minus, start, self.scanner.cursor()))
			}

			'/' => {
				// Comments have already been skipped.

				if self.scanner.take("=") {
					return Ok(Token::new(
						TokenKind::DivideAssign,
						start,
						self.scanner.cursor(),
					));
				}

				Ok(Token::new(TokenKind::Divide, start, self.scanner.cursor()))
			}

			'<' => {
				if self.scanner.take("<") {
					if self.scanner.take("=") {
						return Ok(Token::new(
							TokenKind::LeftShiftAssign,
							start,
							self.scanner.cursor(),
						));
					}

					return Ok(Token::new(
						TokenKind::LeftShift,
						start,
						self.scanner.cursor(),
					));
				}

				if self.scanner.take("=") {
					return Ok(Token::new(
						TokenKind::LessThanEqual,
						start,
						self.scanner.cursor(),
					));
				}

				Ok(Token::new(
					TokenKind::LessThan,
					start,
					self.scanner.cursor(),
				))
			}

			'>' => {
				if self.scanner.take(">") {
					if self.scanner.take("=") {
						return Ok(Token::new(
							TokenKind::RightShiftAssign,
							start,
							self.scanner.cursor(),
						));
					}

					if self.scanner.take(">") {
						if self.scanner.take("=") {
							return Ok(Token::new(
								TokenKind::UnsignedRightShiftAssign,
								start,
								self.scanner.cursor(),
							));
						}

						return Ok(Token::new(
							TokenKind::UnsignedRightShift,
							start,
							self.scanner.cursor(),
						));
					}

					return Ok(Token::new(
						TokenKind::RightShift,
						start,
						self.scanner.cursor(),
					));
				}

				if self.scanner.take("=") {
					return Ok(Token::new(
						TokenKind::GreaterThanEqual,
						start,
						self.scanner.cursor(),
					));
				}

				Ok(Token::new(
					TokenKind::GreaterThan,
					start,
					self.scanner.cursor(),
				))
			}

			'=' => {
				if self.scanner.take("=") {
					if self.scanner.take("=") {
						return Ok(Token::new(
							TokenKind::StrictEqual,
							start,
							self.scanner.cursor(),
						));
					}

					return Ok(Token::new(TokenKind::Equal, start, self.scanner.cursor()));
				}

				if self.scanner.take(">") {
					return Ok(Token::new(TokenKind::Arrow, start, self.scanner.cursor()));
				}

				Ok(Token::new(TokenKind::Assign, start, self.scanner.cursor()))
			}

			'^' => {
				if self.scanner.take("=") {
					return Ok(Token::new(
						TokenKind::BitXorAssign,
						start,
						self.scanner.cursor(),
					));
				}

				Ok(Token::new(TokenKind::BitXor, start, self.scanner.cursor()))
			}

			'|' => {
				if self.scanner.take("|") {
					return Ok(Token::new(
						TokenKind::LogicalOr,
						start,
						self.scanner.cursor(),
					));
				}

				if self.scanner.take("=") {
					return Ok(Token::new(
						TokenKind::BitOrAssign,
						start,
						self.scanner.cursor(),
					));
				}

				Ok(Token::new(TokenKind::BitOr, start, self.scanner.cursor()))
			}

			'.' => {
				if self.scanner.take(".") {
					if self.scanner.take(".") {
						return Ok(Token::new(
							TokenKind::Ellipsis,
							start,
							self.scanner.cursor(),
						));
					} else {
						return Err(self.scanner.cursor() + 1);
					}
				}

				Ok(Token::new(TokenKind::Dot, start, self.scanner.cursor()))
			}

			'(' => Ok(Token::new(
				TokenKind::OpenParen,
				start,
				self.scanner.cursor(),
			)),
			')' => Ok(Token::new(
				TokenKind::CloseParen,
				start,
				self.scanner.cursor(),
			)),
			'[' => Ok(Token::new(
				TokenKind::OpenBracket,
				start,
				self.scanner.cursor(),
			)),
			']' => Ok(Token::new(
				TokenKind::CloseBracket,
				start,
				self.scanner.cursor(),
			)),
			'{' => Ok(Token::new(
				TokenKind::OpenBrace,
				start,
				self.scanner.cursor(),
			)),
			'}' => Ok(Token::new(
				TokenKind::CloseBrace,
				start,
				self.scanner.cursor(),
			)),

			',' => Ok(Token::new(TokenKind::Comma, start, self.scanner.cursor())),
			';' => Ok(Token::new(
				TokenKind::Semicolon,
				start,
				self.scanner.cursor(),
			)),
			':' => Ok(Token::new(TokenKind::Colon, start, self.scanner.cursor())),
			'?' => Ok(Token::new(
				TokenKind::QuestionMark,
				start,
				self.scanner.cursor(),
			)),
			'~' => Ok(Token::new(TokenKind::BitNot, start, self.scanner.cursor())),

			_ => Err(self.scanner.cursor()),
		}
	}
}
