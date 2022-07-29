mod chunk;
mod dom;

pub use chunk::Chunk;

use pest::iterators::Pairs;
use crate::{parser::R, error::Error};

pub fn render_dom(pairs: Pairs<R>) -> Result<Chunk, Error<R>> {
	let mut renderer = dom::Renderer::new();
	match renderer.render(pairs) {
		Ok(chunk) => Ok(chunk),
		Err(err) => Err(err),
	}
}
