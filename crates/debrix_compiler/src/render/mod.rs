use crate::*;

mod chunk;
mod dom;

pub use chunk::Chunk;
use crate::error::Error;

pub use dom::render as render_dom;

#[rustfmt::skip]
pub(crate) const RESERVED_JAVASCRIPT_KEYWORDS: [&str; 64] = [
	"abstract", "arguments", "await", "boolean", "break", "byte", "case", "catch", "char",
	"class", "const", "continue", "debugger", "default", "delete", "do", "double", "else", "enum",
	"eval", "export", "extends", "false", "final", "finally", "float", "for", "function", "goto",
	"if", "implements", "import", "in", "instanceof", "int", "interface", "let", "long", "native",
	"new", "null", "package", "private", "protected", "public", "return", "short", "static",
	"super", "switch", "synchronized", "this", "throw", "throws", "transient", "true", "try",
	"typeof", "var", "void", "volatile", "while", "with", "yield"
];
