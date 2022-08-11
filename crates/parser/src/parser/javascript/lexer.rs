use crate::*;

#[derive(Debug, PartialEq)]
pub enum Token {
	EOF, // end of file

	Identifier(String),
	StringLiteral(String),
	NumberLiteral(f64),
	TemplateLiteral(String),

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

pub fn scan(iter: &mut ChIter) -> Result<Token, ParserError> {
	if let Some(ch) = iter.peek() {
		let ch = ch.clone();

		if ch.is_ascii_whitespace() {
			iter.next();
			return scan(iter);
		}

		if ch == '/' {
			if let Some(ch) = iter.peek_next() {
				if ch == '/' {
					iter.skip_n(2);

					while let Some(ch) = iter.next() {
						if ch == '\n' {
							return scan(iter);
						}
					}

					// Reached EOF, which is also the end of the comment
					return Ok(Token::EOF);
				} else if ch == '*' {
					iter.skip_n(2);

					while let Some(ch) = iter.next() {
						if ch == '*' {
							if let Some(ch) = iter.next() {
								if ch == '/' {
									return scan(iter);
								}
							}
						}
					}

					return Err(ParserError::eof(iter.position()));
				}
			}
		}

		if ch.is_ascii_digit() {
			return scan_number(iter);
		}

		// Also includes boolean, keywords, and null
		if ch.is_ascii_alphabetic() || ch == '_' || ch == '$' {
			return scan_identifier(iter);
		}

		if ch == '"' || ch == '\'' {
			return scan_string(iter);
		}

		if ch == '`' {
			return scan_template(iter);
		}

		// The next character will always be consumed.
		iter.next();

		match ch {
			'!' => match iter.peek() {
				Some('=') => {
					iter.next();
					match iter.peek() {
						Some('=') => {
							iter.next();
							Ok(Token::StrictNotEqual)
						}
						_ => Ok(Token::NotEqual),
					}
				}
				_ => Ok(Token::Not),
			},
			'%' => match iter.peek() {
				Some('=') => {
					iter.next();
					Ok(Token::ModuloAssign)
				}
				_ => Ok(Token::Modulo),
			},
			'&' => match iter.peek() {
				Some('&') => {
					iter.next();
					Ok(Token::LogicalAnd)
				}
				Some('=') => {
					iter.next();
					Ok(Token::BitAndAssign)
				}
				_ => Ok(Token::BitAnd),
			},
			'*' => match iter.peek() {
				Some('*') => {
					iter.next();
					match iter.peek() {
						Some('=') => {
							iter.next();
							Ok(Token::ExponentiateAssign)
						}
						_ => Ok(Token::Exponentiate),
					}
				}
				Some('=') => {
					iter.next();
					Ok(Token::MultiplyAssign)
				}
				_ => Ok(Token::Multiply),
			},
			'+' => match iter.peek() {
				Some('+') => {
					iter.next();
					Ok(Token::Increment)
				}
				Some('=') => {
					iter.next();
					Ok(Token::PlusAssign)
				}
				_ => Ok(Token::Plus),
			},
			'-' => match iter.peek() {
				Some('-') => {
					iter.next();
					Ok(Token::Decrement)
				}
				Some('=') => {
					iter.next();
					Ok(Token::MinusAssign)
				}
				_ => Ok(Token::Minus),
			},
			'.' => match iter.peek() {
				Some('.') => match iter.next() {
					Some('.') => {
						iter.next();
						Ok(Token::Ellipsis)
					}
					_ => Err(ParserError::unexpected(
						Location::from_length(
							iter.offset(),
							1,
							iter.borrow_content(),
						),
						&["."],
					)),
				},
				_ => Ok(Token::Dot),
			},
			'/' => match iter.peek() {
				Some('=') => {
					iter.next();
					Ok(Token::DivideAssign)
				}
				_ => Ok(Token::Divide),
			},
			':' => Ok(Token::Colon),
			';' => Ok(Token::Semicolon),
			',' => Ok(Token::Comma),
			'<' => match iter.peek() {
				Some('<') => {
					iter.next();
					match iter.peek() {
						Some('=') => {
							iter.next();
							Ok(Token::LeftShiftAssign)
						}
						_ => Ok(Token::LeftShift),
					}
				}
				Some('=') => {
					iter.next();
					Ok(Token::LessThanEqual)
				}
				_ => Ok(Token::LessThan),
			},
			'=' => match iter.peek() {
				Some('=') => {
					iter.next();
					match iter.peek() {
						Some('=') => {
							iter.next();
							Ok(Token::StrictEqual)
						}
						_ => Ok(Token::Equal),
					}
				}
				Some('>') => {
					iter.next();
					Ok(Token::Arrow)
				}
				_ => Ok(Token::Assign),
			},
			'>' => match iter.peek() {
				Some('>') => {
					iter.next();
					match iter.peek() {
						Some('>') => {
							iter.next();
							match iter.peek() {
								Some('=') => {
									iter.next();
									Ok(Token::UnsignedRightShiftAssign)
								}
								_ => Ok(Token::UnsignedRightShift),
							}
						}
						Some('=') => {
							iter.next();
							Ok(Token::RightShiftAssign)
						}
						_ => Ok(Token::RightShift),
					}
				}
				Some('=') => {
					iter.next();
					Ok(Token::GreaterThanEqual)
				}
				_ => Ok(Token::GreaterThan),
			},
			'?' => Ok(Token::QuestionMark),
			'[' => Ok(Token::OpenBracket),
			']' => Ok(Token::CloseBracket),
			'^' => match iter.peek() {
				Some('=') => {
					iter.next();
					Ok(Token::BitXorAssign)
				}
				_ => Ok(Token::BitXor),
			},
			'{' => Ok(Token::OpenBrace),
			'|' => match iter.peek() {
				Some('|') => {
					iter.next();
					Ok(Token::LogicalOr)
				}
				Some('=') => {
					iter.next();
					Ok(Token::BitOrAssign)
				}
				_ => Ok(Token::BitOr),
			},
			'}' => Ok(Token::CloseBrace),
			'(' => Ok(Token::OpenParen),
			')' => Ok(Token::CloseParen),
			'~' => Ok(Token::BitNot),
			_ => Err(ParserError::unexpected(
				Location::from_length(iter.offset(), 1, iter.borrow_content()),
				&[&ch.to_string()],
			)),
		}
	} else {
		Ok(Token::EOF)
	}
}

fn scan_number(iter: &mut ChIter) -> Result<Token, ParserError> {
	let mut number = String::new();
	let mut is_float = false;

	while let Some(ch) = iter.next() {
		if ch.is_ascii_digit() {
			number.push(ch);
		} else if !is_float && ch == '.' {
			number.push(ch);
			is_float = true;
		} else {
			break;
		}
	}

	iter.back();

	Ok(Token::NumberLiteral(number.parse().unwrap()))
}

fn scan_identifier(iter: &mut ChIter) -> Result<Token, ParserError> {
	let mut identifier = String::new();

	while let Some(ch) = iter.next() {
		if ch.is_ascii_alphabetic() || ch.is_ascii_digit() || ch == '_' || ch == '$' {
			identifier.push(ch);
		} else {
			break;
		}
	}
	
	iter.back();

	Ok(match identifier.as_str() {
		"true" => Token::True,
		"false" => Token::False,
		"null" => Token::Null,
		"delete" => Token::Delete,
		"in" => Token::In,
		"instanceof" => Token::Instanceof,
		"new" => Token::New,
		"return" => Token::Return,
		"this" => Token::This,
		"typeof" => Token::Typeof,
		"void" => Token::Void,
		_ => Token::Identifier(identifier),
	})
}

fn scan_string(iter: &mut ChIter) -> Result<Token, ParserError> {
	if let Some(ch) = iter.next() {
		let quote = ch;

		if quote != '"' && quote != '\'' {
			return Err(ParserError::expected(
				Location::from_length(iter.offset(), 1, iter.borrow_content()),
				&["\"", "'"],
			));
		}

		let mut string = ch.to_string();

		while let Some(ch) = iter.next() {
			string.push(ch);

			if ch == quote {
				break;
			}

			if ch == '\\' {
				if let Some(ch) = iter.next() {
					string.push(ch);
				} else {
					return Err(ParserError::eof(iter.position()));
				}
			}
		}

		Ok(Token::StringLiteral(string))
	} else {
		return Err(ParserError::eof(iter.position()));
	}
}

fn scan_template(iter: &mut ChIter) -> Result<Token, ParserError> {
	let mut template = String::new();

	if let Some(ch) = iter.next() {
		if ch != '`' {
			return Err(ParserError::expected(
				Location::from_length(iter.offset(), 1, iter.borrow_content()),
				&["`"],
			));
		}
	} else {
		return Err(ParserError::eof(iter.position()));
	}

	while let Some(ch) = iter.next() {
		if ch == '`' {
			break;
		}

		template.push(ch);
	}

	Ok(Token::TemplateLiteral(template))
}
