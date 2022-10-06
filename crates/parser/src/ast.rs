mod comment;
mod dependency;
mod element;
mod flow_control;
mod identifier;
pub mod javascript;
mod literal;
mod text;

pub use comment::*;
pub use dependency::*;
pub use element::*;
pub use flow_control::*;
pub use identifier::*;
pub use literal::*;
pub use text::*;

#[derive(Debug)]
pub enum Node {
	DependencyStatement(DependencyStatement),
	Comment(Comment),
	Element(Element),
	Text(Text),
	TextBinding(TextBinding),
	FlowControl(FlowControl),
}

impl Node {
	pub fn start(&self) -> usize {
		match self {
			Node::DependencyStatement(node) => node.start,
			Node::Comment(node) => node.start,
			Node::Element(node) => node.start,
			Node::Text(node) => node.start,
			Node::TextBinding(node) => node.start,
			Node::FlowControl(node) => node.start(),
		}
	}
	
	pub fn end(&self) -> usize {
		match self {
			Node::DependencyStatement(node) => node.end,
			Node::Comment(node) => node.end,
			Node::Element(node) => node.end,
			Node::Text(node) => node.end,
			Node::TextBinding(node) => node.end,
			Node::FlowControl(node) => node.end(),
		}
	}
}

#[derive(Debug)]
pub struct Range {
	pub start: usize,
	pub end: usize,
}

#[derive(Debug)]
pub struct Document {
	pub children: Vec<Node>,
}

#[derive(Debug)]
pub struct NodeCollection<N> {
	pub start: usize,
	pub end: usize,
	pub nodes: Vec<N>,
}

impl<N> Default for NodeCollection<N> {
	fn default() -> Self {
		Self {
			start: 0,
			end: 0,
			nodes: Vec::new(),
		}
	}
}
