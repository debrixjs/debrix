use crate::ast::*;

#[derive(Debug)]
pub struct FlowControlWhen {
	pub start: usize,
	pub end: usize,
	pub condition: Box<javascript::Expression>,
	pub children: Vec<Node>,
}

#[derive(Debug)]
pub struct FlowControlElse {
	pub start: usize,
	pub end: usize,
	pub condition: Option<javascript::Expression>,
	pub children: Vec<Node>,
}

#[derive(Debug)]
pub struct FlowControlEach {
	pub start: usize,
	pub end: usize,
	pub iterator: Box<javascript::Expression>,
	pub iterable: Box<javascript::Expression>,
	pub children: Vec<Node>,
}

#[derive(Debug)]
pub enum FlowControl {
	When(FlowControlWhen),
	Else(FlowControlElse),
	Each(FlowControlEach),
}

impl FlowControl {
	pub fn start(&self) -> usize {
		match self {
			FlowControl::When(node) => node.start,
			FlowControl::Else(node) => node.start,
			FlowControl::Each(node) => node.start,
		}
	}

	pub fn end(&self) -> usize {
		match self {
			FlowControl::When(node) => node.end,
			FlowControl::Else(node) => node.end,
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

impl From<FlowControlElse> for Node {
	fn from(node: FlowControlElse) -> Node {
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

impl From<FlowControlElse> for FlowControl {
	fn from(node: FlowControlElse) -> FlowControl {
		FlowControl::Else(node)
	}
}

impl From<FlowControlEach> for FlowControl {
	fn from(node: FlowControlEach) -> FlowControl {
		FlowControl::Each(node)
	}
}
