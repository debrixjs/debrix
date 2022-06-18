pub mod build;
pub mod chunk;
pub mod debug;
pub mod literal;
pub mod parser;
pub mod plugin;
pub mod scope;
pub mod utils;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use build::Build;

pub fn build(input: &str) -> build::BuildResult {
	Build::new().build(input)
}
