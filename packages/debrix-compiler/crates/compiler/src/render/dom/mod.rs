use super::*;

mod component;
mod document;
mod fragment;

pub(crate) use component::*;
pub(crate) use document::*;
pub(crate) use fragment::*;

pub fn render(document: ast::Document) -> Result<Chunk, Error> {
	match Document::new().render(document) {
		Ok(chunk) => Ok(chunk),
		Err(err) => Err(err),
	}
}
