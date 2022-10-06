pub struct Chunk {
	pub source: String,
	/// From-to mappings of the source.
	pub mappings: Vec<(usize, usize)>,
	pos: usize,
}

impl Chunk {
	pub fn new() -> Self {
		Self {
			source: String::new(),
			mappings: Vec::new(),
			pos: 0,
		}
	}

	pub fn map(&mut self, pos: usize) -> &mut Self {
		self.mappings.push((pos, self.pos));
		self
	}

	pub fn write(&mut self, s: &str) -> &mut Self {
		self.source.push_str(s);
		self.pos += s.len();
		self
	}

	pub fn append(&mut self, chunk: &Self) -> &mut Self {
		self.source += &chunk.source;

		for mapping in &chunk.mappings {
			self.mappings.push((
				mapping.0,
				self.pos + mapping.1,
			));
		}

		self
	}
}
