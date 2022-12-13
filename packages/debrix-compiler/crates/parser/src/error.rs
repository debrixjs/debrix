use crate::*;

pub struct ParserError {
	pub position: usize,
	pub positives: Vec<String>,
}

impl fmt::Debug for ParserError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fn fmt_item(item: &str) -> String {
			if item.len() == 1 {
				let char = item.chars().next().unwrap();

				match char {
					'\0' => "NULL".to_string(),
					_ => format!("'{}'", char),
				}
			} else {
				item.to_owned()
			}
		}

		fn fmt_list(list: &Vec<String>) -> String {
			if list.len() == 1 {
				fmt_item(&list[0])
			} else {
				let mut result = String::new();
				for (i, item) in list.iter().enumerate() {
					if i == list.len() - 1 {
						result.push_str(" or ");
					} else if i != 0 {
						result.push_str(", ");
					}
					result.push_str(&fmt_item(item));
				}
				result
			}
		}

		write!(f, "Unexpected at {:?}", self.position)?;

		if !self.positives.is_empty() {
			write!(f, ", expected {}.", fmt_list(&self.positives))?;
		} else {
			write!(f, ".")?;
		}

		Ok(())
	}
}

impl From<usize> for ParserError {
	fn from(position: usize) -> ParserError {
		ParserError {
			position,
			positives: Vec::new(),
		}
	}
}
