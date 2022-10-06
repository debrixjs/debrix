use parser::ParserError;

#[derive(Debug)]
pub enum Error {
	ParserError(ParserError),
	CompilerError(CompilerError),
}

impl Error {
	pub fn compiler(start: usize, end: usize, message: &str) -> Error {
		Error::CompilerError(CompilerError {
			start,
			end,
			message: message.to_owned(),
		})
	}
}

#[derive(Debug)]
pub struct CompilerError {
	pub start: usize,
	pub end: usize,
	pub message: String,
}

impl CompilerError {
	pub fn new(start: usize, end: usize, message: &str) -> Self {
		Self {
			start,
			end,
			message: message.to_owned(),
		}
	}
}
