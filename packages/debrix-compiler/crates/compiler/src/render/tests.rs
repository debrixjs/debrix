use super::*;

fn compile(s: &str) -> Result<Chunk, Error> {
	build(s.to_owned(), Target::Client)
}

#[test]
fn test_impl_nested_components() {
	compile(
		r#"
		<div>
			<div>
				<slot />
			</div>
		</div>

		<p as="bar">
			<slot />
		</p>
		"#,
	)
	.unwrap();
}

#[test]
fn test_use_nested_components() {
	compile(
		r#"
		using component foo, { component bar } from "nowhere"

		<foo>
			<bar>baz</bar>
		</foo>
		"#,
	)
	.unwrap();
}

#[test]
fn test_illegal_as_attribute() {
	assert!(compile(r#"<div as></div>"#).is_err());
}
