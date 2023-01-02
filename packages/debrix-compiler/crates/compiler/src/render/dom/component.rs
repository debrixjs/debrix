use super::*;

pub struct Component {}

impl Component {
	pub fn new() -> Self {
		Self {}
	}

	pub fn render(
		&mut self,
		doc: &mut Document,
		name: String,
		element: ast::Element,
		symbol: Option<String>,
		model_constructor: Option<String>,
	) -> Result<(), Error> {
		let h_component = doc.import("Component", None, INTERNAL_MODULE);

		let name = to_valid_identifier(&name);
		let fragment_name = doc
			.unique
			.ensure(&("render_".to_owned() + &snake_case(&name)));

		let fragment = Fragment::new();
		fragment.render_single(doc, fragment_name.clone(), element.into())?;

		doc.c_components
			.write("class ")
			.write(&name)
			.write(" extends ")
			.write(&h_component)
			.write(" {\n")
			.write("\tstatic render = ")
			.write(&fragment_name)
			.write(";\n");

			if let Some(c) = model_constructor {
				doc.c_components
					.write("\tstatic model = ")
					.write(&c)
					.write(";\n");
			}

			if let Some(sym) = symbol {
				doc.c_components
					.write("\tstatic __family = ")
					.write(&sym)
					.write(";\n");
			}

		doc.c_components.write("}\n\n");

		Ok(())
	}
}
