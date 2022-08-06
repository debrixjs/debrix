use crate::*;

pub struct ParserError {
	location: Location,
	positives: Vec<String>,
	negatives: Vec<String>,
}

impl ParserError {
	pub fn expected(location: Location, positives: &[&str]) -> Self {
		let positives = positives.iter().map(|s| s.to_string()).collect();
		Self {
			location,
			positives,
			negatives: Vec::new(),
		}
	}

	pub fn unexpected(location: Location, negatives: &[&str]) -> Self {
		let negatives = negatives.iter().map(|s| s.to_string()).collect();
		Self {
			location,
			positives: Vec::new(),
			negatives,
		}
	}

	pub fn eof(position: Position) -> Self {
		Self {
			location: Location::new(position.clone(), position),
			positives: Vec::new(),
			negatives: ["EOF".to_owned()].to_vec(),
		}
	}
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

		fn fmt_list(list: &[String]) -> String {
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

		write!(f, "At {:?}, ", self.location.start)?;

		if !self.positives.is_empty() {
			write!(f, "expected {}", fmt_list(&self.positives))?;

			if self.negatives.is_empty() {
				write!(f, ".")?;
			} else {
				write!(f, ", but ")?;
			}
		}

		if !self.negatives.is_empty() {
			write!(f, "unexpected {}", fmt_list(&self.negatives))?;
		}

		Ok(())
	}
}
