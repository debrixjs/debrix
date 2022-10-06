use super::*;

pub struct TokenBuffer<'a> {
	buffer: Vec<Token>,
	length: usize,
	lexer: Lexer<'a>,
}

impl<'a> TokenBuffer<'a> {
	pub fn new(lexer: Lexer<'a>) -> Self {
		Self {
			buffer: Vec::new(),
			length: 0,
			lexer,
		}
	}

	pub fn cursor(&self) -> usize {
		self.lexer.cursor()
	}

	pub fn scanner(&self) -> &Scanner {
		self.lexer.scanner()
	}

	pub fn peek(&self) -> &Token {
		if self.buffer.is_empty() || self.length == 0 {
			panic!("cannot peek");
		}

		self.buffer.get(self.length - 1).unwrap()
	}

	pub fn scan(&mut self) -> Result<&Token, usize> {
		if self.length < self.buffer.len() {
			self.length += 1;
			let token = self.buffer.get(self.length - 1).unwrap();
			self.lexer.scanner_mut().set_cursor(token.end);
			return Ok(token);
		}

		if self.is_done() {
			return Ok(self.buffer.last().unwrap());
		}

		let token = self.lexer.scan()?;
		self.buffer.push(token);
		self.length += 1;

		Ok(self.buffer.last().unwrap())
	}

	pub fn unscan(&mut self) {
		if self.length == 0 {
			panic!("cannot unscan");
		}

		self.length -= 1;

		if self.length == 0 {
			let token = self.buffer.first().unwrap();
			self.lexer.scanner_mut().set_cursor(token.start);
		} else {
			let token = self.buffer.get(self.length - 1).unwrap();
			self.lexer.scanner_mut().set_cursor(token.end);
		}
	}

	pub fn is_done(&mut self) -> bool {
		self.lexer.is_done()
	}
}
