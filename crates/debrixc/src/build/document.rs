mod body;
mod head;

use super::Build;
use crate::parser::{self, Rule};
use crate::pest::Parser;
use crate::utils;

pub struct BuildResult {
	pub source: String,
	pub mappings: Vec<(usize, usize, usize, usize)>,
}

impl Build {
	pub fn build(&mut self, input: &str) -> BuildResult {
		let pair = parser::Parser::parse(Rule::document, input)
			.unwrap()
			.next()
			.unwrap();

		let mut pairs = pair.into_inner();

		self.build_head(pairs.next().unwrap());

		let body_as_str = pairs.next().unwrap().as_str();

		let body = parser::Parser::parse(Rule::body, body_as_str)
			.unwrap()
			.next()
			.unwrap();

		let root = self.build_body(body.into_inner().next().unwrap());

		for (source, imports) in self.imports.iter() {
			let mut has_block = false;
			self.chunks.head.append("import ");
			for (index, (specifier, local)) in imports.iter().enumerate() {
				match specifier.as_ref() {
					"default" => {
						self.chunks.head.append(local);
					}

					"*" => {
						self.chunks.head.append("* as ");
						self.chunks.head.append(local);
						self.chunks.head.append(" ");
					}

					_ => {
						if !has_block {
							self.chunks.head.append("{ ");
							has_block = true;
						}

						self.chunks.head.append(specifier);
						if specifier != local {
							self.chunks.head.append(" as ");
							self.chunks.head.append(local);
						}
					}
				}

				if index == imports.len() - 1 {
					if has_block {
						self.chunks.head.append(" } ");
					} else {
						self.chunks.head.append(" ");
					}
				} else {
					self.chunks.head.append(", ")
				}
			}
			self.chunks.head.append("from \"");
			self.chunks.head.append(source);
			self.chunks.head.append("\";\n");
		}

		let source = format!(
			r#"{head}

function init() {{
{init}
{bind}
{insert}
	return {root};
}}

export default class {{
	constructor(options) {{
		this.element = init();
	}}

	insert(target, anchor) {{
		if (anchor)
			target.insertBefore(this.element, anchor);
		else
			target.appendChild(this.element);
	}}

	destroy() {{
		this.element.remove();
	}}
}}"#,
			head = utils::format_lines(&self.chunks.head.source, 0),
			init = utils::format_lines(&self.chunks.init.source, 1),
			bind = utils::format_lines(&self.chunks.bind.source, 1),
			insert = utils::format_lines(&self.chunks.insert.source, 1)
		);

		BuildResult {
			source: source.to_owned(),
			mappings: vec![],
		}
	}
}
