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
		match token {
			Token::EOF => Err(ParserError::eof(self.iter.position())),

			Token::Identifier(name) => Ok(ast::Expression::Identifier(ast::IdentifierExpression {
				location: Location::new(start.clone(), self.iter.position()),
				name: name.to_owned(),
			})),

			Token::NumberLiteral(number) => Ok(ast::Expression::Literal(ast::Literal::Number(
				ast::NumberLiteral {
					location: Location::new(start, self.iter.position()),
					value: number.to_owned(),
				},
			))),

			Token::StringLiteral(string) => Ok(ast::Expression::Literal(ast::Literal::String(
				ast::StringLiteral {
					location: Location::new(start, self.iter.position()),
					value: string
						.clone()
						.trim_matches(|c| c == '\'' || c == '"')
						.to_owned(),
					quote: string.chars().next().unwrap(),
				},
			))),

			Token::True => Ok(ast::Expression::Literal(ast::Literal::Boolean(
				ast::BooleanLiteral {
					location: Location::new(start, self.iter.position()),
					value: true,
				},
			))),

			Token::False => Ok(ast::Expression::Literal(ast::Literal::Boolean(
				ast::BooleanLiteral {
					location: Location::new(start, self.iter.position()),
					value: false,
				},
			))),

			Token::Null => Ok(ast::Expression::Literal(ast::Literal::Null(
				ast::NullLiteral {
					location: Location::new(start, self.iter.position()),
				},
			))),

			Token::TemplateLiteral(template) => {
				Ok(ast::Expression::Template(ast::TemplateLiteral {
					location: Location::new(start, self.iter.position()),
					raw: template.to_owned(),
				}))
			}

			Token::OpenParen => {
				let expr = self.parse_javascript_expression(end)?;

				if lexer::scan(&mut self.iter)? != Token::CloseParen {
					return Err(ParserError::expected(
						Location::from_length(self.iter.offset(), 1, self.iter.borrow_content()),
						&[")"],
					));
				}

				Ok(ast::Expression::Parenthesized(
					ast::ParenthesizedExpression {
						location: Location::new(start, self.iter.position()),
						expression: Box::new(expr),
					},
				))
			}

			Token::OpenBrace => {
				let mut properties = vec![];
				let mut token_pos = self.iter.position();
				let mut token = lexer::scan(&mut self.iter)?;

				loop {
					let start = self.iter.position();
					let expr =
						self.parse_javascript_expression_from(&token, token_pos.clone(), end)?;

					match lexer::scan(&mut self.iter)? {
						Token::Colon => {
							let value = self.parse_javascript_expression(end)?;

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
					location: Location::new(start, self.iter.position()),
					properties,
				}))
			}

			Token::OpenBracket => {
				let mut elements = vec![];
				let mut token_pos = self.iter.position();
				let mut token = lexer::scan(&mut self.iter)?;

				loop {
					let expr =
						self.parse_javascript_expression_from(&token, token_pos.clone(), end)?;

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
					location: Location::new(start, self.iter.position()),
					elements,
				}))
			}

			_ => todo!(),
		}
	}
}
