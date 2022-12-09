use crate::*;

mod lexer;
mod parser;
mod token_buffer;

pub(crate) use self::lexer::*;
pub(crate) use self::parser::*;
pub(crate) use self::token_buffer::*;

#[cfg(test)]
mod tests;

pub(crate) fn is_eof(kind: &TokenKind) -> bool {
	matches!(kind, TokenKind::EOF)
}

impl Parser {
	pub fn parse_javascript(&mut self) -> Result<ast::javascript::Expression, ParserError> {
		Ok(JavascriptParser::new(&mut self.scanner).parse_expression()?)
	}

	pub fn parse_javascript_identifier(&mut self) -> Result<ast::javascript::IdentifierExpression, ParserError> {
		Ok(JavascriptParser::new(&mut self.scanner).parse_identifier()?)
	}
}
