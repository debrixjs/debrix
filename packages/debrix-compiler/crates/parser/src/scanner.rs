#[derive(Clone)]
pub struct Scanner {
	cursor: usize,
	chars: Vec<char>,
	string: String,
}

impl Scanner {
	pub fn new(string: &str) -> Self {
		Self {
			cursor: 0,
			chars: string.chars().collect(),
			string: string.to_owned(),
		}
	}

	pub fn cursor(&self) -> usize {
		self.cursor
	}

	#[allow(dead_code)]
	pub fn set_cursor(&mut self, cursor: usize) {
		self.cursor = cursor;
	}

	pub fn peek(&self) -> Option<&char> {
		self.chars.get(self.cursor)
	}

	pub fn is_done(&self) -> bool {
		self.cursor == self.chars.len()
	}

	pub fn next(&mut self) -> Option<&char> {
		if self.cursor == self.chars.len() {
			return None;
		}

		self.cursor += 1;
		self.chars.get(self.cursor)
	}

	pub fn back(&mut self) -> bool {
		if self.cursor == 0 {
			false
		} else {
			self.cursor -= 1;
			true
		}
	}

	pub fn take(&mut self, target: &str) -> bool {
		if self.test(target) {
			self.cursor += target.len();
			true
		} else {
			false
		}
	}

	pub fn test(&mut self, target: &str) -> bool {
		for (i, charx) in target.chars().enumerate() {
			if let Some(chary) = self.chars.get(self.cursor + i) {
				if &charx != chary {
					return false;
				}
			} else {
				return false;
			}
		}

		true
	}

	pub fn slice(&self, start: usize, end: usize) -> &str {
		self.string.get(start..end).unwrap()
	}
}
