mod debug;
mod error;
mod parser;
mod render;
mod utils;

use error::{Error, ParsingError};
use render::{render_dom, Chunk};
use parser::R;

extern crate pest;
#[macro_use]
extern crate pest_derive;

pub fn build(input: &str) -> Result<Chunk, Error<R>> {
	match parser::parse(input) {
		Ok(pairs) => render_dom(pairs),
		Err(e) => Err(ParsingError::new_from_err(input, e).into_err()),
	}
}
