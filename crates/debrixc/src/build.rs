mod document;
mod expression;

use crate::{chunk::Chunk, scope::Scope};
use std::collections::{HashMap, hash_map::Entry};

pub use self::document::BuildResult;

pub const DEBRIX_INTERNAL: &str = "@debrix/internal";

struct Chunks {
	head: Chunk,
	init: Chunk,
	bind: Chunk,
	insert: Chunk,
}

impl Chunks {
	fn new() -> Self {
		Self {
			head: Default::default(),
			init: Default::default(),
			bind: Default::default(),
			insert: Default::default(),
		}
	}
}

impl Default for Chunks {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Build {
	chunks: Chunks,
	imports: HashMap<String, Vec<(String, String)>>,
	scope: Scope,
}

impl Build {
	pub fn new() -> Self {
		Self {
			chunks: Default::default(),
			imports: Default::default(),
			scope: Default::default(),
		}
	}

	pub fn import(&mut self, specifier: &str, source: &str) -> String {
		let entry = self.imports.entry(source.to_owned());

		if let Entry::Vacant(entry) = entry {
			let local = self.scope.unique(specifier);
			entry.insert(vec![(specifier.to_owned(), local.to_owned())]);
			local
		} else if let Entry::Occupied(mut entry) = entry {
			let imports = entry.get_mut();
			let existing = imports.iter().find(|import| import.0 == specifier);
			if let Some(existing) = existing {
				existing.1.clone()
			} else {
				let local = self.scope.unique(specifier);
				imports.push((specifier.to_owned(), local.to_owned()));
				local
			}
		} else {
			unreachable!();
		}
	}
}
