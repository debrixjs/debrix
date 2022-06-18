use std::collections::HashMap;

pub struct Scope {
	unqiues: HashMap<String, usize>,
}

impl Scope {
	pub fn new() -> Self {
		Self {
			unqiues: Default::default(),
		}
	}

	pub fn unique(&mut self, specifier: &str) -> String {
		let count = self.unqiues.entry(specifier.to_owned()).or_insert(0);
		let mut unique = specifier.to_owned();
		if *count > 0 {
			unique += "_";
			unique += &((*count + 1).to_string());
			if self.unqiues.contains_key(&unique) {
				self.unique(&unique)
			} else {
				let count = self.unqiues.get_mut(specifier).unwrap();
				*count += 1;
				unique
			}
		} else {
			*count += 1;
			unique
		}
	}
}
