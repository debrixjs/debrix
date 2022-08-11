use crate::*;

mod lexer;
use ast::javascript as ast;
use lexer::Token;

#[cfg(test)]
mod tests;

impl Parser {
	pub fn parse_javascript_expression(
		&mut self,
		end: &[char],
	) -> Result<ast::Expression, ParserError> {
		let start = self.iter.position();
		let token = lexer::scan(&mut self.iter)?;
		self.parse_javascript_expression_from(&token, start, end)
	}

	fn parse_javascript_expression_from(
		&mut self,
		token: &Token,
		start: Position,
		end: &[char],
	) -> Result<ast::Expression, ParserError> {
		let lstart = start.clone();

		let expr = match token {
			Token::EOF => Err(ParserError::eof(self.iter.position())),

			Token::Identifier(name) => Ok(ast::Expression::Identifier(ast::IdentifierExpression {
				location: Location::new(lstart, self.iter.position()),
				name: name.to_owned(),
			})),

			Token::NumberLiteral(number) => Ok(ast::Expression::Literal(ast::Literal::Number(
				ast::NumberLiteral {
					location: Location::new(lstart, self.iter.position()),
					value: number.to_owned(),
				},
			))),

			Token::StringLiteral(string) => Ok(ast::Expression::Literal(ast::Literal::String(
				ast::StringLiteral {
					location: Location::new(lstart, self.iter.position()),
					value: string
						.clone()
						.trim_matches(|c| c == '\'' || c == '"')
						.to_owned(),
					quote: string.chars().next().unwrap(),
				},
			))),

			Token::True => Ok(ast::Expression::Literal(ast::Literal::Boolean(
				ast::BooleanLiteral {
					location: Location::new(lstart, self.iter.position()),
					value: true,
				},
			))),

			Token::False => Ok(ast::Expression::Literal(ast::Literal::Boolean(
				ast::BooleanLiteral {
					location: Location::new(lstart, self.iter.position()),
					value: false,
				},
			))),

			Token::Null => Ok(ast::Expression::Literal(ast::Literal::Null(
				ast::NullLiteral {
					location: Location::new(lstart, self.iter.position()),
				},
			))),

			Token::TemplateLiteral(template) => {
				Ok(ast::Expression::Template(ast::TemplateLiteral {
					location: Location::new(lstart, self.iter.position()),
					raw: template.to_owned(),
				}))
			}

			Token::OpenParen => {
				let mut items = vec![];
				// position is used to throw error is comma isn't allowed
				let mut first_comma_position = None;

				let mut token_pos = self.iter.position();
				let mut token = lexer::scan(&mut self.iter)?;

				loop {
					let expr = self.parse_javascript_expression_from(
						&token,
						token_pos.clone(),
						&[',', ')'],
					)?;

					match lexer::scan(&mut self.iter)? {
						Token::Comma => {
							if first_comma_position.is_none() {
								first_comma_position = Some(self.iter.offset());
							}

							items.push(expr);

							token_pos = self.iter.position();
							token = lexer::scan(&mut self.iter)?;

							if token == Token::CloseParen {
								break;
							}
						}

						Token::CloseParen => {
							items.push(expr);
							break;
						}

						_ => {
							return Err(ParserError::expected(
								Location::from_length(
									self.iter.offset(),
									1,
									self.iter.borrow_content(),
								),
								&[",", "]"],
							))
						}
					}
				}

				self.skip_whitespace()?;

				if self.try_str("=>") {
					let mut parameters = vec![];

					for item in items {
						match item {
							ast::Expression::Identifier(id) => parameters.push(id),
							_ => {
								return Err(ParserError::expected(
									Location::from_length(
										self.iter.offset(),
										1,
										self.iter.borrow_content(),
									),
									&["identifier"],
								))
							}
						}
					}

					let body = self.parse_javascript_expression(&[])?;

					Ok(ast::Expression::Function(ast::FunctionExpression {
						location: Location::new(lstart, self.iter.position()),
						parameters,
						body: Box::new(body),
					}))
				} else {
					if let Some(start) = first_comma_position {
						return Err(ParserError::unexpected(Location::from_length(start, 1, self.iter.borrow_content()), &[","]));
					}

					let expression = items.swap_remove(0);

					Ok(ast::Expression::Parenthesized(ast::ParenthesizedExpression {
						location: Location::new(lstart, self.iter.position()),
						expression: Box::new(expression),
					}))
				}
			}

			Token::OpenBrace => {
				let mut properties = vec![];
				let mut token_pos = self.iter.position();
				let mut token = lexer::scan(&mut self.iter)?;

				loop {
					let start = self.iter.position();
					let expr = self.parse_javascript_expression_from(
						&token,
						token_pos.clone(),
						&[':', ',', '}'],
					)?;

					match lexer::scan(&mut self.iter)? {
						Token::Colon => {
							let value = self.parse_javascript_expression(&[',', '}'])?;

							properties.push(ast::ObjectProperty {
								location: Location::new(start, self.iter.position()),
								key: Some(expr),
								value,
							});

							match lexer::scan(&mut self.iter)? {
								Token::Comma => continue,
								Token::CloseBrace => break,
								_ => {
									return Err(ParserError::expected(
										Location::from_length(
											self.iter.offset(),
											1,
											self.iter.borrow_content(),
										),
										&[",", "}"],
									))
								}
							}
						}

						Token::Comma => {
							properties.push(ast::ObjectProperty {
								location: Location::new(start, self.iter.position()),
								key: None,
								value: expr,
							});

							token_pos = self.iter.position();
							token = lexer::scan(&mut self.iter)?;

							if token == Token::CloseBrace {
								break;
							}
						}

						Token::CloseBrace => {
							break;
						}

						_ => {
							return Err(ParserError::expected(
								Location::from_length(
									self.iter.offset(),
									1,
									self.iter.borrow_content(),
								),
								&[",", ":"],
							))
						}
					}
				}

				Ok(ast::Expression::Object(ast::ObjectExpression {
					location: Location::new(lstart, self.iter.position()),
					properties,
				}))
			}

			Token::OpenBracket => {
				let mut elements = vec![];
				let mut token_pos = self.iter.position();
				let mut token = lexer::scan(&mut self.iter)?;

				loop {
					let expr = self.parse_javascript_expression_from(
						&token,
						token_pos.clone(),
						&[',', ']'],
					)?;

					match lexer::scan(&mut self.iter)? {
						Token::Comma => {
							elements.push(expr);

							token_pos = self.iter.position();
							token = lexer::scan(&mut self.iter)?;

							if token == Token::CloseBracket {
								break;
							}
						}

						Token::CloseBracket => {
							elements.push(expr);
							break;
						}

						_ => {
							return Err(ParserError::expected(
								Location::from_length(
									self.iter.offset(),
									1,
									self.iter.borrow_content(),
								),
								&[",", "]"],
							))
						}
					}
				}

				Ok(ast::Expression::Array(ast::ArrayExpression {
					location: Location::new(lstart, self.iter.position()),
					elements,
				}))
			}

			Token::Delete
			| Token::Typeof
			| Token::Void
			| Token::Plus
			| Token::Minus
			| Token::Increment
			| Token::Decrement
			| Token::Not
			| Token::BitNot => {
				let operator = match token {
					Token::Delete => ast::UnaryOperator::Delete,
					Token::Typeof => ast::UnaryOperator::Typeof,
					Token::Void => ast::UnaryOperator::Void,
					Token::Plus => ast::UnaryOperator::Plus,
					Token::Minus => ast::UnaryOperator::Minus,
					Token::Increment => ast::UnaryOperator::Increment,
					Token::Decrement => ast::UnaryOperator::Decrement,
					Token::Not => ast::UnaryOperator::Not,
					Token::BitNot => ast::UnaryOperator::BitwiseNot,

					_ => unreachable!(),
				};

				let next = self.parse_javascript_expression(end)?;

				Ok(ast::Expression::Unary(ast::UnaryExpression {
					location: Location::new(lstart, self.iter.position()),
					operator,
					operand: Box::new(next),
				}))
			}

			Token::New => {
				let callee = self.parse_javascript_expression(&['('])?;

				let mut arguments = vec![];

				if lexer::scan(&mut self.iter)? == Token::OpenParen {
					let mut token_pos = self.iter.position();
					let mut token = lexer::scan(&mut self.iter)?;

					loop {
						let expr = self.parse_javascript_expression_from(
							&token,
							token_pos.clone(),
							&[',', ')'],
						)?;

						match lexer::scan(&mut self.iter)? {
							Token::Comma => {
								arguments.push(expr);

								token_pos = self.iter.position();
								token = lexer::scan(&mut self.iter)?;

								if token == Token::CloseParen {
									break;
								}
							}

							Token::CloseParen => {
								arguments.push(expr);
								break;
							}

							_ => {
								return Err(ParserError::expected(
									Location::from_length(
										self.iter.offset(),
										1,
										self.iter.borrow_content(),
									),
									&[",", ")"],
								))
							}
						}
					}
				}

				Ok(ast::Expression::New(ast::NewExpression {
					location: Location::new(lstart, self.iter.position()),
					callee: Box::new(callee),
					arguments,
				}))
			}

			_ => todo!(),
		}?;

		self.skip_whitespace()?;

		if self.iter.peek().map_or(true, |c| end.contains(&c)) {
			return Ok(expr);
		}

		let rstart = start.clone();
		let token = lexer::scan(&mut self.iter)?;

		match token {
			Token::EOF => Err(ParserError::eof(self.iter.position())),

			Token::Plus
			| Token::Minus
			| Token::Multiply
			| Token::Divide
			| Token::Modulo
			| Token::Exponentiate
			| Token::LeftShift
			| Token::RightShift
			| Token::UnsignedRightShift
			| Token::LessThan
			| Token::GreaterThan
			| Token::LessThanEqual
			| Token::GreaterThanEqual
			| Token::Equal
			| Token::NotEqual
			| Token::StrictEqual
			| Token::StrictNotEqual
			| Token::BitAnd
			| Token::BitOr
			| Token::BitXor
			| Token::LogicalAnd
			| Token::LogicalOr
			| Token::Instanceof
			| Token::In => {
				let operator = match token {
					Token::Plus => ast::BinaryOperator::Plus,
					Token::Minus => ast::BinaryOperator::Minus,
					Token::Multiply => ast::BinaryOperator::Multiply,
					Token::Divide => ast::BinaryOperator::Divide,
					Token::Modulo => ast::BinaryOperator::Modulo,
					Token::Exponentiate => ast::BinaryOperator::Exponent,
					Token::LeftShift => ast::BinaryOperator::LeftShift,
					Token::RightShift => ast::BinaryOperator::RightShift,
					Token::UnsignedRightShift => ast::BinaryOperator::UnsignedRightShift,
					Token::LessThan => ast::BinaryOperator::LessThan,
					Token::GreaterThan => ast::BinaryOperator::GreaterThan,
					Token::LessThanEqual => ast::BinaryOperator::LessThanOrEqual,
					Token::GreaterThanEqual => ast::BinaryOperator::GreaterThanOrEqual,
					Token::Equal => ast::BinaryOperator::Equal,
					Token::NotEqual => ast::BinaryOperator::NotEqual,
					Token::StrictEqual => ast::BinaryOperator::StrictEqual,
					Token::StrictNotEqual => ast::BinaryOperator::StrictNotEqual,
					Token::BitAnd => ast::BinaryOperator::BitwiseAnd,
					Token::BitOr => ast::BinaryOperator::BitwiseOr,
					Token::BitXor => ast::BinaryOperator::BitwiseXor,
					Token::LogicalAnd => ast::BinaryOperator::LogicalAnd,
					Token::LogicalOr => ast::BinaryOperator::LogicalOr,
					Token::Instanceof => ast::BinaryOperator::InstanceOf,
					Token::In => ast::BinaryOperator::In,

					_ => unreachable!(),
				};

				let right = self.parse_javascript_expression(end)?;

				Ok(ast::Expression::Binary(ast::BinaryExpression {
					location: Location::new(rstart, self.iter.position()),
					operator,
					left: Box::new(expr),
					right: Box::new(right),
				}))
			}

			Token::QuestionMark => {
				let token_start = self.iter.position();
				let token = lexer::scan(&mut self.iter)?;

				match token {
					Token::Dot => {
						let token_start = self.iter.position();
						let token = lexer::scan(&mut self.iter)?;
						let token_location = Location::new(token_start, self.iter.position());

						match token {
							Token::Identifier(name) => {
								Ok(ast::Expression::Member(ast::MemberExpression {
									location: Location::new(rstart, self.iter.position()),
									object: Box::new(expr),
									property: Box::new(ast::Expression::Identifier(
										ast::IdentifierExpression {
											location: token_location,
											name,
										},
									)),
									computed: false,
									optional: true,
								}))
							}
							Token::OpenBracket => {
								let property = self.parse_javascript_expression(&[']'])?;
								self.expect(']')?;

								Ok(ast::Expression::Member(ast::MemberExpression {
									location: Location::new(rstart, self.iter.position()),
									object: Box::new(expr),
									property: Box::new(property),
									computed: true,
									optional: true,
								}))
							}
							_ => Err(ParserError::expected(token_location, &["identifier"])),
						}
					}

					_ => {
						let consequent =
							self.parse_javascript_expression_from(&token, token_start, &[':'])?;
						self.skip_whitespace()?;
						self.expect(':')?;
						self.skip_whitespace()?;
						let alternate = self.parse_javascript_expression(end)?;

						Ok(ast::Expression::Conditional(ast::ConditionalExpression {
							location: Location::new(rstart, self.iter.position()),
							condition: Box::new(expr),
							consequent: Box::new(consequent),
							alternate: Box::new(alternate),
						}))
					}
				}
			}

			Token::OpenParen => {
				let mut arguments = Vec::new();

				self.skip_whitespace()?;

				if self.iter.peek().map_or(true, |c| c != ')') {
					loop {
						let argument = self.parse_javascript_expression(&[',', ')'])?;
						arguments.push(argument);

						self.skip_whitespace()?;

						if self.iter.peek().map_or(true, |c| c != ',') {
							break;
						}

						self.iter.next();
						self.skip_whitespace()?;
					}
				}

				self.expect(')')?;

				Ok(ast::Expression::Call(ast::CallExpression {
					location: Location::new(rstart, self.iter.position()),
					callee: Box::new(expr),
					arguments,
				}))
			}

			Token::Dot => {
				let token_start = self.iter.position();
				let token = lexer::scan(&mut self.iter)?;
				let token_location = Location::new(token_start, self.iter.position());

				match token {
					Token::Identifier(name) => Ok(ast::Expression::Member(ast::MemberExpression {
						location: Location::new(rstart, self.iter.position()),
						object: Box::new(expr),
						property: Box::new(ast::Expression::Identifier(
							ast::IdentifierExpression {
								location: token_location,
								name,
							},
						)),
						computed: false,
						optional: false,
					})),
					_ => Err(ParserError::expected(token_location, &["identifier"])),
				}
			}

			Token::OpenBracket => {
				let property = self.parse_javascript_expression(&[']'])?;
				self.expect(']')?;

				Ok(ast::Expression::Member(ast::MemberExpression {
					location: Location::new(rstart, self.iter.position()),
					object: Box::new(expr),
					property: Box::new(property),
					computed: true,
					optional: false,
				}))
			}

			x => todo!("{:?}", x),
		}
	}
}
