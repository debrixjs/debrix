mod error;
mod render;
mod utils;

pub(crate) use {parser::ast, utils::*};

use render::{render_dom};

pub use render::Chunk;
pub use error::Error;

pub enum Target {
	Client,
	Hydration,
	Server,
}

pub fn build(input: String, target: Target) -> Result<Chunk, Error> {
	match parser::parse_document(input) {
		Ok(document) => match target {
			Target::Client => render_dom(document),
			Target::Hydration => unimplemented!(),
			Target::Server => unimplemented!(),
		},
		Err(err) => Err(Error::ParserError(err)),
	}
}
