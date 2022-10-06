use super::*;
mod renderer;

pub fn render(document: ast::Document) -> Result<Chunk, Error> {
	match renderer::Renderer::new().render(document) {
		Ok(chunk) => Ok(chunk),
		Err(err) => Err(err),
	}
}
