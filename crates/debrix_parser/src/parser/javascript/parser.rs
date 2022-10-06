use super::*;

pub struct JavascriptParser<'a> {
	tokens: TokenBuffer<'a>,
}

impl<'a> JavascriptParser<'a> {
	pub fn new(scanner: &'a mut Scanner) -> Self {
		Self {
			tokens: TokenBuffer::new(Lexer::new(scanner)),
		}
	}

	fn parse_expression_lazy(&mut self) -> Result<ast::javascript::Expression, usize> {
		let token = self.tokens.scan()?;
		if is_eof(&token.kind) {
			return Err(self.tokens.cursor());
		}

		let token_kind = token.kind.clone();
		self.tokens.unscan();

		Ok(match token_kind {
			TokenKind::Identifier => self.parse_identifier()?.into(),
			TokenKind::String => self.parse_string()?.into(),
			TokenKind::Numeric => self.parse_number()?.into(),
			TokenKind::Template => self.parse_template()?.into(),
			TokenKind::True | TokenKind::False => self.parse_boolean()?.into(),
			TokenKind::Null => self.parse_null()?.into(),
			TokenKind::Minus
			| TokenKind::Plus
			| TokenKind::Increment
			| TokenKind::Decrement
			| TokenKind::Not
			| TokenKind::BitNot
			| TokenKind::Typeof
			| TokenKind::Void
			| TokenKind::Delete => self.parse_unary()?.into(),
			TokenKind::OpenParen => self.parse_arrow_function_or_parenthesized()?,
			TokenKind::OpenBracket => self.parse_array()?.into(),
			TokenKind::OpenBrace => self.parse_object()?.into(),
			TokenKind::New => self.parse_new()?.into(),

			_ => return Err(self.tokens.cursor()),
		})
	}

	pub fn parse_expression(&mut self) -> Result<ast::javascript::Expression, usize> {
		let left = self.parse_expression_lazy()?;

		if self.tokens.is_done() {
			return Ok(left);
		}

		let token = self.tokens.scan()?;
		if is_eof(&token.kind) {
			return Err(self.tokens.cursor());
		}

		let token_kind = token.kind.clone();
		self.tokens.unscan();

		Ok(match token_kind {
			TokenKind::Plus
			| TokenKind::Minus
			| TokenKind::Multiply
			| TokenKind::Divide
			| TokenKind::Modulo
			| TokenKind::Exponentiate
			| TokenKind::LeftShift
			| TokenKind::RightShift
			| TokenKind::UnsignedRightShift
			| TokenKind::LessThan
			| TokenKind::GreaterThan
			| TokenKind::LessThanEqual
			| TokenKind::GreaterThanEqual
			| TokenKind::Equal
			| TokenKind::NotEqual
			| TokenKind::StrictEqual
			| TokenKind::StrictNotEqual
			| TokenKind::BitAnd
			| TokenKind::BitOr
			| TokenKind::BitXor
			| TokenKind::LogicalAnd
			| TokenKind::LogicalOr
			| TokenKind::Instanceof
			| TokenKind::In => self.parse_binary(left)?.into(),
			TokenKind::Assign
			| TokenKind::PlusAssign
			| TokenKind::MinusAssign
			| TokenKind::MultiplyAssign
			| TokenKind::ExponentiateAssign
			| TokenKind::DivideAssign
			| TokenKind::ModuloAssign
			| TokenKind::LeftShiftAssign
			| TokenKind::RightShiftAssign
			| TokenKind::UnsignedRightShiftAssign
			| TokenKind::BitAndAssign
			| TokenKind::BitXorAssign
			| TokenKind::BitOrAssign => self.parse_assignment(left)?.into(),
			TokenKind::QuestionMark => self.parse_member_or_conditional(left)?.into(),
			TokenKind::OpenParen => self.parse_call(left)?.into(),
			TokenKind::Dot | TokenKind::OpenBracket => self.parse_member(left)?.into(),
			TokenKind::Template => self.parse_tagged_template(left)?.into(),

			_ => left,
		})
	}

	fn parse_identifier(&mut self) -> Result<ast::javascript::IdentifierExpression, usize> {
		let token = self.tokens.scan()?;
		let start = token.start;
		let end = token.end;
		Ok(ast::javascript::IdentifierExpression {
			start,
			end,
			name: self.tokens.scanner().slice(start, end).to_owned(),
		})
	}

	fn parse_string(&mut self) -> Result<ast::javascript::StringLiteral, usize> {
		let token = self.tokens.scan()?;
		let start = token.start;
		let end = token.end;
		Ok(ast::javascript::StringLiteral {
			start,
			end,
			value: self.tokens.scanner().slice(start + 1, end - 1).to_owned(),
			raw: self.tokens.scanner().slice(start, end).to_owned(),
			quote: self
				.tokens
				.scanner()
				.slice(start, start + 1)
				.chars()
				.next()
				.unwrap(),
		})
	}

	fn parse_number(&mut self) -> Result<ast::javascript::NumberLiteral, usize> {
		let token = self.tokens.scan()?;
		let start = token.start;
		let end = token.end;
		let raw = self.tokens.scanner().slice(start, end);
		Ok(ast::javascript::NumberLiteral {
			start,
			end,
			value: raw.parse().unwrap(),
			raw: raw.to_owned(),
		})
	}

	fn parse_boolean(&mut self) -> Result<ast::javascript::BooleanLiteral, usize> {
		let token = self.tokens.scan()?;
		let start = token.start;
		let end = token.end;
		let raw = self.tokens.scanner().slice(start, end);
		Ok(ast::javascript::BooleanLiteral {
			start,
			end,
			value: raw.parse().unwrap(),
			raw: raw.to_owned(),
		})
	}

	fn parse_null(&mut self) -> Result<ast::javascript::NullLiteral, usize> {
		let token = self.tokens.scan()?;
		let start = token.start;
		let end = token.end;
		Ok(ast::javascript::NullLiteral {
			start,
			end,
			raw: "null".to_owned(),
		})
	}

	fn parse_template(&mut self) -> Result<ast::javascript::TemplateLiteral, usize> {
		let token = self.tokens.scan()?;
		let start = token.start;
		let end = token.end;
		Ok(ast::javascript::TemplateLiteral {
			start,
			end,
			raw: self.tokens.scanner().slice(start + 1, end - 1).to_owned(),
		})
	}

	fn parse_unary(&mut self) -> Result<ast::javascript::UnaryExpression, usize> {
		let operator = self.tokens.scan()?;
		let start = operator.start;
		let operator = match operator.kind {
			TokenKind::Minus => ast::javascript::UnaryOperator::Minus,
			TokenKind::Plus => ast::javascript::UnaryOperator::Plus,
			TokenKind::Increment => ast::javascript::UnaryOperator::Increment,
			TokenKind::Decrement => ast::javascript::UnaryOperator::Decrement,
			TokenKind::Not => ast::javascript::UnaryOperator::Not,
			TokenKind::BitNot => ast::javascript::UnaryOperator::BitwiseNot,
			TokenKind::Typeof => ast::javascript::UnaryOperator::Typeof,
			TokenKind::Void => ast::javascript::UnaryOperator::Void,
			TokenKind::Delete => ast::javascript::UnaryOperator::Delete,

			_ => unreachable!("{}:{}", operator.start, operator.end),
		};

		let token = self.tokens.scan()?;
		if is_eof(&token.kind) {
			return Err(self.tokens.cursor());
		}

		let operand = self.tokens.peek();
		let expression = ast::javascript::UnaryExpression {
			start,
			end: operand.end,
			operator,
			operand: Box::new(self.parse_expression()?),
		};

		Ok(expression)
	}

	fn parse_arrow_function_or_parenthesized(
		&mut self,
	) -> Result<ast::javascript::Expression, usize> {
		let mut token_count = 0;

		loop {
			let token = self.tokens.scan()?;
			token_count += 1;

			match token.kind {
				TokenKind::CloseParen => break,
				TokenKind::EOF => return Err(self.tokens.cursor()),
				_ => continue,
			}
		}

		let has_next = !self.tokens.is_done();
		let next = self.tokens.scan()?.kind.clone();

		if has_next {
			self.tokens.unscan();
		}

		for _ in 0..token_count {
			self.tokens.unscan();
		}

		if next == TokenKind::Arrow {
			Ok(self.parse_arrow_function()?.into())
		} else {
			Ok(self.parse_parenthesized()?.into())
		}
	}

	fn parse_arrow_function(&mut self) -> Result<ast::javascript::FunctionExpression, usize> {
		let start = self.tokens.cursor();
		let mut parameters: Vec<ast::javascript::Expression> = Vec::new();

		let token = self.tokens.scan()?;
		if token.kind != TokenKind::OpenParen {
			return Err(token.start);
		}

		loop {
			let token = self.tokens.scan()?;
			match token.kind {
				TokenKind::CloseParen => break,

				TokenKind::Identifier => {
					self.tokens.unscan();
					parameters.push(self.parse_identifier()?.into());
				}

				TokenKind::Ellipsis => {
					let start = self.tokens.cursor();
					let argument = self.parse_identifier()?.into();

					parameters.push(ast::javascript::Expression::Spread(
						ast::javascript::SpreadExpression {
							start,
							end: self.tokens.cursor(),
							argument: Box::new(argument),
						},
					));
				}

				_ => return Err(token.start),
			}

			let token = self.tokens.scan()?.kind.clone();
			if is_eof(&token) {
				return Err(self.tokens.cursor());
			}

			if token == TokenKind::Comma {
				continue;
			} else {
				break;
			}
		}

		let token = self.tokens.scan()?;
		if token.kind != TokenKind::Arrow {
			return Err(self.tokens.cursor());
		}

		let body = self.parse_expression()?;

		Ok(ast::javascript::FunctionExpression {
			start,
			end: self.tokens.cursor(),
			parameters,
			body: Box::new(body),
		})
	}

	fn parse_parenthesized(&mut self) -> Result<ast::javascript::ParenthesizedExpression, usize> {
		let start = self.tokens.cursor();

		let token = self.tokens.scan()?;
		if token.kind != TokenKind::OpenParen {
			return Err(token.start);
		}

		let expression = self.parse_expression()?;

		let token = self.tokens.scan()?;
		if token.kind != TokenKind::CloseParen {
			return Err(token.start);
		}

		Ok(ast::javascript::ParenthesizedExpression {
			start,
			end: self.tokens.cursor(),
			expression: Box::new(expression),
		})
	}

	fn parse_array(&mut self) -> Result<ast::javascript::ArrayExpression, usize> {
		let start = self.tokens.cursor();
		let mut elements = Vec::new();

		let token = self.tokens.scan()?;
		if token.kind != TokenKind::OpenBracket {
			return Err(token.start);
		}

		loop {
			let start = self.tokens.cursor();

			match self.tokens.scan()?.kind {
				TokenKind::CloseBracket => break,

				TokenKind::Comma => {
					elements.push(ast::javascript::Expression::Empty(
						ast::javascript::EmptyExpression { start, end: start },
					));
				}

				_ => {
					self.tokens.unscan();
					let expression = self.parse_expression()?;
					elements.push(expression);
				}
			}

			match self.tokens.scan()?.kind {
				TokenKind::CloseBracket => break,
				TokenKind::Comma => continue,
				_ => return Err(self.tokens.cursor()),
			}
		}

		Ok(ast::javascript::ArrayExpression {
			start,
			end: self.tokens.cursor(),
			elements,
		})
	}

	fn parse_object(&mut self) -> Result<ast::javascript::ObjectExpression, usize> {
		let start = self.tokens.cursor();
		let mut properties = Vec::new();

		let token = self.tokens.scan()?;
		if token.kind != TokenKind::OpenBrace {
			return Err(token.start);
		}

		loop {
			let start = self.tokens.cursor();

			match self.tokens.scan()?.kind {
				TokenKind::CloseBrace => break,

				TokenKind::Identifier => {
					self.tokens.unscan();
					let key = self.parse_identifier()?;
					let mut value = None;

					let token = self.tokens.scan()?;
					if token.kind == TokenKind::Colon {
						value = Some(self.parse_expression()?);
					} else {
						self.tokens.unscan();
					}

					properties.push(ast::javascript::ObjectProperty::Keyed(
						ast::javascript::ObjectKeyedProperty {
							start,
							end: self.tokens.cursor(),
							key: Box::new(key),
							value,
						},
					))
				}

				TokenKind::OpenBracket => {
					let key = self.parse_expression()?;

					let token = self.tokens.scan()?;
					if token.kind != TokenKind::CloseBracket {
						return Err(self.tokens.cursor());
					}

					let token = self.tokens.scan()?;
					if token.kind != TokenKind::Colon {
						return Err(self.tokens.cursor());
					}

					let value = self.parse_expression()?;

					properties.push(ast::javascript::ObjectProperty::Computed(
						ast::javascript::ObjectComputedProperty {
							start,
							end: self.tokens.cursor(),
							key: Box::new(key),
							value: Box::new(value),
						},
					))
				}

				TokenKind::Ellipsis => {
					// Scan away the ellipsis
					self.tokens.scan()?;

					let argument = self.parse_expression()?;

					properties.push(ast::javascript::ObjectProperty::Spread(
						ast::javascript::SpreadExpression {
							start,
							end: self.tokens.cursor(),
							argument: Box::new(argument),
						},
					))
				}

				_ => return Err(self.tokens.cursor()),
			}

			match self.tokens.scan()?.kind {
				TokenKind::Comma => continue,
				TokenKind::CloseBrace => break,
				_ => return Err(self.tokens.cursor()),
			}
		}

		Ok(ast::javascript::ObjectExpression {
			start,
			end: self.tokens.cursor(),
			properties,
		})
	}

	fn parse_new(&mut self) -> Result<ast::javascript::NewExpression, usize> {
		let start = self.tokens.cursor();

		let token = self.tokens.scan()?;
		if token.kind != TokenKind::New {
			return Err(self.tokens.cursor());
		}

		let callee = self.parse_expression_lazy()?;
		let call = self.parse_call(callee)?;

		Ok(ast::javascript::NewExpression {
			start,
			end: call.end,
			callee: call.callee,
			arguments: call.arguments,
		})
	}

	fn parse_binary(
		&mut self,
		left: ast::javascript::Expression,
	) -> Result<ast::javascript::BinaryExpression, usize> {
		let token = self.tokens.scan()?;

		let operator: ast::javascript::BinaryOperator = match token.kind {
			TokenKind::Plus => ast::javascript::BinaryOperator::Plus,
			TokenKind::Minus => ast::javascript::BinaryOperator::Minus,
			TokenKind::Multiply => ast::javascript::BinaryOperator::Multiply,
			TokenKind::Divide => ast::javascript::BinaryOperator::Divide,
			TokenKind::Modulo => ast::javascript::BinaryOperator::Modulo,
			TokenKind::Exponentiate => ast::javascript::BinaryOperator::Exponent,
			TokenKind::LeftShift => ast::javascript::BinaryOperator::LeftShift,
			TokenKind::RightShift => ast::javascript::BinaryOperator::RightShift,
			TokenKind::UnsignedRightShift => ast::javascript::BinaryOperator::UnsignedRightShift,
			TokenKind::LessThan => ast::javascript::BinaryOperator::LessThan,
			TokenKind::GreaterThan => ast::javascript::BinaryOperator::GreaterThan,
			TokenKind::LessThanEqual => ast::javascript::BinaryOperator::LessThanOrEqual,
			TokenKind::GreaterThanEqual => ast::javascript::BinaryOperator::GreaterThanOrEqual,
			TokenKind::Equal => ast::javascript::BinaryOperator::Equal,
			TokenKind::NotEqual => ast::javascript::BinaryOperator::NotEqual,
			TokenKind::StrictEqual => ast::javascript::BinaryOperator::StrictEqual,
			TokenKind::StrictNotEqual => ast::javascript::BinaryOperator::StrictNotEqual,
			TokenKind::BitAnd => ast::javascript::BinaryOperator::BitwiseAnd,
			TokenKind::BitOr => ast::javascript::BinaryOperator::BitwiseOr,
			TokenKind::BitXor => ast::javascript::BinaryOperator::BitwiseXor,
			TokenKind::LogicalAnd => ast::javascript::BinaryOperator::LogicalAnd,
			TokenKind::LogicalOr => ast::javascript::BinaryOperator::LogicalOr,
			TokenKind::Instanceof => ast::javascript::BinaryOperator::InstanceOf,
			TokenKind::In => ast::javascript::BinaryOperator::In,

			_ => unreachable!(),
		};

		let right = self.parse_expression()?;

		Ok(ast::javascript::BinaryExpression {
			start: left.start(),
			end: self.tokens.cursor(),
			left: Box::new(left),
			operator,
			right: Box::new(right),
		})
	}

	fn parse_assignment(
		&mut self,
		left: ast::javascript::Expression,
	) -> Result<ast::javascript::AssignmentExpression, usize> {
		let token = self.tokens.scan()?;

		let operator: ast::javascript::AssignmentOperator = match token.kind {
			TokenKind::Assign => ast::javascript::AssignmentOperator::Equal,
			TokenKind::PlusAssign => ast::javascript::AssignmentOperator::PlusEqual,
			TokenKind::MinusAssign => ast::javascript::AssignmentOperator::MinusEqual,
			TokenKind::MultiplyAssign => ast::javascript::AssignmentOperator::MultiplyEqual,
			TokenKind::ExponentiateAssign => ast::javascript::AssignmentOperator::ExponentEqual,
			TokenKind::DivideAssign => ast::javascript::AssignmentOperator::DivideEqual,
			TokenKind::ModuloAssign => ast::javascript::AssignmentOperator::ModuloEqual,
			TokenKind::LeftShiftAssign => ast::javascript::AssignmentOperator::LeftShiftEqual,
			TokenKind::RightShiftAssign => ast::javascript::AssignmentOperator::RightShiftEqual,
			TokenKind::BitAndAssign => ast::javascript::AssignmentOperator::BitwiseAndEqual,
			TokenKind::BitXorAssign => ast::javascript::AssignmentOperator::BitwiseXorEqual,
			TokenKind::BitOrAssign => ast::javascript::AssignmentOperator::BitwiseOrEqual,
			TokenKind::UnsignedRightShiftAssign => {
				ast::javascript::AssignmentOperator::UnsignedRightShiftEqual
			}

			_ => unreachable!(),
		};

		let right = self.parse_expression()?;

		Ok(ast::javascript::AssignmentExpression {
			start: left.start(),
			end: self.tokens.cursor(),
			left: Box::new(left),
			operator,
			right: Box::new(right),
		})
	}

	fn parse_member_or_conditional(
		&mut self,
		left: ast::javascript::Expression,
	) -> Result<ast::javascript::Expression, usize> {
		if self.tokens.scan()?.kind != TokenKind::QuestionMark {
			return Err(self.tokens.cursor());
		}

		let token_kind = self.tokens.scan()?.kind.clone();
		self.tokens.unscan();
		self.tokens.unscan();

		Ok(match token_kind {
			TokenKind::Dot => self.parse_member(left)?.into(),
			_ => self.parse_conditional(left)?.into(),
		})
	}

	fn parse_conditional(
		&mut self,
		left: ast::javascript::Expression,
	) -> Result<ast::javascript::ConditionalExpression, usize> {
		if self.tokens.scan()?.kind != TokenKind::QuestionMark {
			return Err(self.tokens.cursor());
		}

		let consequent = self.parse_expression()?;

		if self.tokens.scan()?.kind != TokenKind::Colon {
			return Err(self.tokens.cursor());
		}

		let alternate = self.parse_expression()?;

		Ok(ast::javascript::ConditionalExpression {
			start: left.start(),
			end: self.tokens.cursor(),
			condition: Box::new(left),
			consequent: Box::new(consequent),
			alternate: Box::new(alternate),
		})
	}

	fn parse_call(
		&mut self,
		left: ast::javascript::Expression,
	) -> Result<ast::javascript::CallExpression, usize> {
		if self.tokens.scan()?.kind != TokenKind::OpenParen {
			// Expected open paren
			return Err(self.tokens.cursor());
		}

		let mut arguments = Vec::new();

		loop {
			let start = self.tokens.cursor();
			let argument;

			match self.tokens.scan()?.kind {
				TokenKind::Ellipsis => {
					let expression = self.parse_expression()?;

					argument =
						ast::javascript::Expression::Spread(ast::javascript::SpreadExpression {
							start,
							end: self.tokens.cursor(),
							argument: Box::new(expression),
						})
				}

				_ => {
					self.tokens.unscan();
					argument = self.parse_expression()?
				}
			}

			arguments.push(argument);

			match self.tokens.scan()?.kind {
				TokenKind::Comma => continue,
				TokenKind::CloseParen => break,

				// Expected close paren
				TokenKind::EOF => return Err(self.tokens.cursor()),

				// Expected comma
				_ => return Err(self.tokens.cursor()),
			}
		}

		Ok(ast::javascript::CallExpression {
			start: left.start(),
			end: self.tokens.cursor(),
			arguments,
			callee: Box::new(left),
		})
	}

	fn parse_member(
		&mut self,
		left: ast::javascript::Expression,
	) -> Result<ast::javascript::MemberExpression, usize> {
		let mut computed = false;
		let mut optional = false;
		let property;

		match self.tokens.scan()?.kind {
			TokenKind::Dot => {
				property = self.parse_identifier()?.into();
			}

			TokenKind::QuestionMark => {
				optional = true;

				if self.tokens.scan()?.kind != TokenKind::Dot {
					// Expected dot
					return Err(self.tokens.cursor());
				}

				if self.tokens.scan()?.kind == TokenKind::OpenBracket {
					computed = true;
					property = self.parse_expression()?;

					if self.tokens.scan()?.kind != TokenKind::CloseBracket {
						// Expected CloseBracket
						return Err(self.tokens.cursor());
					}
				} else {
					self.tokens.unscan();
					property = self.parse_identifier()?.into();
				}
			}

			TokenKind::OpenBracket => {
				computed = true;
				property = self.parse_expression()?;

				if self.tokens.scan()?.kind != TokenKind::CloseBracket {
					// Expected CloseBracket
					return Err(self.tokens.cursor());
				}
			}

			// Expected dot
			_ => return Err(self.tokens.cursor()),
		}

		Ok(ast::javascript::MemberExpression {
			start: left.start(),
			end: self.tokens.cursor(),
			object: Box::new(left),
			computed,
			optional,
			property: Box::new(property),
		})
	}

	fn parse_tagged_template(
		&mut self,
		left: ast::javascript::Expression,
	) -> Result<ast::javascript::TaggedTemplateExpression, usize> {
		let quasi = self.parse_template()?;

		Ok(ast::javascript::TaggedTemplateExpression {
			start: left.start(),
			end: self.tokens.cursor(),
			tag: Box::new(left),
			quasi: Box::new(quasi),
		})
	}
}
