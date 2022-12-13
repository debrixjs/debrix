use crate::ast::*;

#[derive(Debug)]
pub struct FlowControlWhen {
	pub start: usize,
	pub end: usize,
	pub condition: Box<javascript::Expression>,
	pub children: Vec<Node>,
	pub chain: Vec<FlowControlElse>,
}

#[derive(Debug)]
pub struct FlowControlElse {
	pub start: usize,
	pub end: usize,
	pub condition: Option<javascript::Expression>,
	pub children: Vec<Node>
}

#[derive(Debug)]
pub struct FlowControlEach {
	pub start: usize,
	pub end: usize,
	pub iterator: Box<javascript::IdentifierExpression>,
	pub iterable: Box<javascript::Expression>,
	pub children: Vec<Node>,
}

#[derive(Debug)]
pub enum FlowControl {
	When(FlowControlWhen),
	Each(FlowControlEach),
}

impl FlowControl {
	pub fn start(&self) -> usize {
		match self {
			FlowControl::When(node) => node.start,
			FlowControl::Each(node) => node.start,
		}
	}

	pub fn end(&self) -> usize {
		match self {
			FlowControl::When(node) => node.end,
			FlowControl::Each(node) => node.end,
		}
	}
}

impl From<FlowControl> for Node {
	fn from(node: FlowControl) -> Node {
		Node::FlowControl(node)
	}
}

impl From<FlowControlWhen> for Node {
	fn from(node: FlowControlWhen) -> Node {
		(FlowControl::from(node)).into()
	}
}

impl From<FlowControlEach> for Node {
	fn from(node: FlowControlEach) -> Node {
		(FlowControl::from(node)).into()
	}
}

impl From<FlowControlWhen> for FlowControl {
	fn from(node: FlowControlWhen) -> FlowControl {
		FlowControl::When(node)
	}
}

impl From<FlowControlEach> for FlowControl {
	fn from(node: FlowControlEach) -> FlowControl {
		FlowControl::Each(node)
	}
}
