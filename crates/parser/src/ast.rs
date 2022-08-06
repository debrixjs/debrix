use crate::*;

mod comment;
mod dependency;
mod element;
pub mod javascript;
mod identifier;
mod literal;
mod text;

pub use comment::*;
pub use dependency::*;
pub use element::*;
pub use identifier::*;
pub use literal::*;
pub use text::*;

pub enum Node {
	DependencyStatement(DependencyStatement),
	Comment(Comment),
	Element(Element),
	Text(Text),
	TextBinding(TextBinding)
}

pub trait IntoNode {
	fn into_node(self) -> Node;
}

pub struct NodeCollection<N> {
	pub location: Location,
	pub nodes: Vec<N>,
}

impl<N> Default for NodeCollection<N> {
	fn default() -> Self {
		Self {
			location: Location::default(),
			nodes: Vec::new(),
		}
	}
}
