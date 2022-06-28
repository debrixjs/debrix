use super::Build;
use crate::{chunk::Chunk, parser::Rule};
use pest::iterators::Pair;

fn very_expression(pair: &Pair<Rule>) {
	if pair.as_rule() != Rule::expression {
		panic!("Node is not an expression.");
	}
}

impl Build {
	pub fn build_expression_as_reference(&mut self, expression: Pair<Rule>) -> Chunk {
		very_expression(&expression);

		let expr_chunk = self.build_expression(expression.clone());

		let mut inner = expression.into_inner();
		let left = inner.next().unwrap();
		let right = inner.next();

		fn is_ident(left: &Pair<Rule>, right: &Option<Pair<Rule>>) -> bool {
			left.as_rule() == Rule::ident && right.is_none()
		}

		fn is_member(_left: &Pair<Rule>, right: &Option<Pair<Rule>>) -> bool {
			right.is_some() && right.as_ref().unwrap().as_rule() == Rule::member_expression
		}

		let mut chunk = Chunk::new();
		if inner.peek().is_none() && (is_ident(&left, &right) || is_member(&left, &right)) {
			chunk.append("this.$accessor(");
		} else {
			chunk.append("this.$computed(() => ");
		}
		chunk.append_chunk(expr_chunk);
		chunk.append(")");

		chunk
	}

	pub fn build_expression(&mut self, expression: Pair<Rule>) -> Chunk {
		very_expression(&expression);

		let mut chunk = Chunk::new();
		let mut inner = expression.into_inner();

		while let Some(pair) = inner.next() {
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

				Rule::ident => {
					chunk.append("this.");
					chunk.append(pair.as_str());
				}

				Rule::literal => {
					chunk.append(pair.as_str());
				}

				_ => unimplemented!(),
			}
		}

		chunk
	}
}
