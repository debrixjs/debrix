pub struct Chunk {
	pub source: String,
	pub mappings: Vec<(usize, usize, usize, usize)>,
}

impl Chunk {
	pub fn new() -> Self {
		Chunk {
			source: Default::default(),
			mappings: Default::default(),
		}
	}

	fn pos(&self) -> (usize, usize) {
		let lines = self.source.lines();
		(lines.clone().count(), lines.last().unwrap_or("").len())
	}

	pub fn map(&mut self, from: (usize, usize)) {
		let to = self.pos();
		self.mappings.push((from.0, from.1, to.0, to.1));
	}

	pub fn append(&mut self, source: &str) {
		self.source += source;
	}

	pub fn append_chunk(&mut self, chunk: Chunk) {
		let to = self.pos();
		self.mappings.extend(
			chunk
				.mappings
				.iter()
				.map(|mapping| (mapping.0, mapping.1, mapping.2 + to.0, mapping.3 + to.1)),
		);
		self.source += &chunk.source;
	}
}
