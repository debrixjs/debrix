use pest::{iterators::Pair, RuleType};

fn debug_pair_inner<T>(pair: &Pair<T>, indent_level: usize, is_newline: bool) -> String
where
	T: RuleType,
{
	let indent = if is_newline {
		"   ".repeat(indent_level)
	} else {
		"".to_string()
	};

	let children: Vec<_> = pair.clone().into_inner().collect();
	let len = children.len();
	let children: Vec<_> = children
		.into_iter()
		.map(|pair| {
			debug_pair_inner(
				&pair,
				if len > 1 {
					indent_level + 1
				} else {
					indent_level
				},
				len > 1,
			)
		})
		.collect();

	let dash = if is_newline { "- " } else { "" };

	match len {
		0 => format!(
			"{}{}{:?}: {:?}",
			indent,
			dash,
			pair.as_rule(),
			pair.as_span().as_str()
		),
		1 => format!("{}{}{:?} > {}", indent, dash, pair.as_rule(), children[0]),
		_ => format!(
			"{}{}{:?}\n{}",
			indent,
			dash,
			pair.as_rule(),
			children.join("\n")
		),
	}
}

pub fn debug_pair<T>(pair: &Pair<T>)
where
	T: RuleType,
{
	println!("/*\n{}\n*/\n\n", debug_pair_inner(pair, 0, true));
}
