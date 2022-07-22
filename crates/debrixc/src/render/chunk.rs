pub struct Chunk {
	pub source: String,
	pub mappings: Vec<(usize, usize, usize, usize)>,
	pos: (usize, usize),
}

impl Chunk {
	pub fn new() -> Self {
		Self {
			source: String::new(),
			mappings: Vec::new(),
			pos: (0, 0),
		}
	}

	pub fn map(&mut self, pos: (usize, usize)) -> &mut Self {
		self.mappings.push((pos.0, pos.1, self.pos.0, self.pos.1));
		self
	}

	pub fn write(&mut self, s: &str) -> &mut Self {
		self.source.push_str(s);

		for c in s.chars() {
			if c == '\n' {
				self.pos.0 += 1;
				self.pos.1 = 0;
			} else {
				self.pos.1 += 1;
			}
		}

		self
	}

	pub fn append(&mut self, chunk: &Self, ident: usize) -> &mut Self {
		let mut lines = (&chunk.source).lines().into_iter().peekable();

		while let Some(line) = lines.next() {
			self.write(&"\t".repeat(ident));
			self.write(line);

			if lines.peek().is_some() {
				self.write("\n");
			}
		}

		for mapping in &chunk.mappings {
			self.mappings.push((
				mapping.0,
				mapping.1,
				self.pos.0 + mapping.2,
				self.pos.1 + mapping.3,
			));
		}

		self
	}
}
