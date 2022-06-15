use crate::build::Build;

pub trait Plugin {
	// Name suggestion: suppliment (e.g. node)
	fn get_name<'a>(&self) -> &'a str;

	fn setup(&self, _build: Build) {}
}

pub struct PluginsManager<'a> {
	_plugins: &'a [&'a dyn Plugin],
}

impl<'a> PluginsManager<'a> {
	pub fn new(plugins: &'a [&'a dyn Plugin]) -> Self {
		Self { _plugins: plugins }
	}

	pub fn dispatch(_hook: &str) {}
}
