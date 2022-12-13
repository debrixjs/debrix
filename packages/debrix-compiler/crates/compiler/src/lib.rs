mod error;
mod render;
mod utils;

pub(crate) use {
	debrix_parser::{ast, ParserError},
	utils::*,
};

use render::render_dom;

pub use error::Error;
pub use render::Chunk;

pub enum Target {
	Client,
	Hydration,
	Server,
}

pub fn build(input: String, target: Target) -> Result<Chunk, Error> {
	match debrix_parser::parse_document(input) {
		Ok(document) => match target {
			Target::Client => render_dom(document),
			Target::Hydration => unimplemented!(),
			Target::Server => unimplemented!(),
		},
		Err(err) => Err(Error::ParserError(err)),
	}
}
