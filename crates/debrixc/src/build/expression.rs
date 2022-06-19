use super::Build;
use crate::{chunk::Chunk, parser::Rule};
use pest::iterators::Pair;

impl Build {
	pub fn build_expression(&mut self, expression: Pair<Rule>) -> Chunk {
		if expression.as_rule() != Rule::expression {
			panic!("Node is not an expression.");
		}

		let mut chunk = Chunk::new();
		let pair = expression.into_inner().next().unwrap();

		match pair.as_rule() {
			Rule::binary_expression => {
				let mut inner = pair.into_inner();
				let operator = inner.next().unwrap();
				let right = inner.next().unwrap();

				chunk.append(" ");
				chunk.append(operator.as_str());
				chunk.append(" ");
				chunk.append_chunk(self.build_expression(right));
			}

			Rule::new_expression => {
				let mut inner = pair.into_inner();
				let target = inner.next().unwrap();
				let arguments = inner.next();

				chunk.append("new ");
				chunk.append_chunk(self.build_expression(target));

				if let Some(arguments) = arguments {
					chunk.append(" (");
					let mut inner = arguments.into_inner();
					while let Some(argument) = inner.next() {
						chunk.append_chunk(self.build_expression(argument));
						if inner.peek().is_some() {
							chunk.append(", ");
						}
					}
					chunk.append(")");
				}
			}

			Rule::call_expression => {
				let mut inner = pair.into_inner();
				let target = inner.next().unwrap();
				let arguments = inner.next().unwrap();

				chunk.append_chunk(self.build_expression(target));
				chunk.append("(");
				let mut inner = arguments.into_inner();
				while let Some(argument) = inner.next() {
					chunk.append_chunk(self.build_expression(argument));
					if inner.peek().is_some() {
						chunk.append(", ");
					}
				}
				chunk.append(")");
			}

			Rule::unary_expression => {
				let mut inner = pair.into_inner();
				let operator = inner.next().unwrap();
				let target = inner.next().unwrap();

				chunk.append(operator.as_str());
				chunk.append(" ");
				chunk.append_chunk(self.build_expression(target));
			}

			Rule::expression => {
				let mut inner = pair.into_inner();
				let target = inner.next().unwrap();

				chunk.append("(");
				chunk.append_chunk(self.build_expression(target));
				chunk.append(")");
			}

			Rule::member_expression => {
				let mut inner = pair.into_inner();

				while let Some(member) = inner.next() {
					let rule = member.as_rule();
					let target = member.into_inner().next().unwrap();

					match rule {
						Rule::member => {
							chunk.append(".");
							chunk.append_chunk(self.build_expression(target));
						}

						Rule::optional_memeber => {
							chunk.append("?.");
							chunk.append_chunk(self.build_expression(target));
						}

						Rule::computed_member => {
							chunk.append("[");
							chunk.append_chunk(self.build_expression(target));
							chunk.append("]");
						}

						_ => unreachable!(),
					}
				}
			}

			Rule::ident | Rule::literal => {
				chunk.append(pair.as_str());
			}

			_ => unimplemented!(),
		}

		chunk
	}
}
