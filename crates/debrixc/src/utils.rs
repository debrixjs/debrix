use crate::render::Chunk;

/// Sets indentation and removes trailing whitespaces for each line in the string.
pub fn set_indent(string: &str, indent: usize) -> String {
	let indent = "\t".repeat(indent).to_owned();

	string
		.split("\n")
		.map(|line| indent.clone() + &line.trim())
		.collect::<Vec<_>>()
		.join("\n")
}

/// Sets indentation and removes trailing whitespaces for each line and remaps all mappings in the chunk.
pub fn set_chunk_indent(mut chunk: Chunk, indent: usize) -> Chunk {
	chunk.source = set_indent(&chunk.source, indent);

	for mapping in &mut chunk.mappings {
		mapping.1 += 1;
	}

	chunk
}

/// Sets indentation using [set_chunk_indent] and removes leaing and trailing whitespaces.
pub fn format_chunk(mut chunk: Chunk, indent: usize) -> Chunk {
	chunk.source = chunk.source.trim().to_owned();
	set_chunk_indent(chunk, indent)
}
