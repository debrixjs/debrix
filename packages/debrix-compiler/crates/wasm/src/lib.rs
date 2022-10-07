use wasm_bindgen::prelude::*;

fn serialize_chunk(chunk: &debrix_compiler::Chunk) -> js_sys::Object {
	let serialized = js_sys::Object::new();

	// chunk.source
	js_sys::Reflect::set(&serialized, &"source".into(), &(&chunk.source).into()).unwrap();

	// chunk.mappings
	let mappings = js_sys::Array::new();
	for mapping in &chunk.mappings {
		let js_mapping = js_sys::Array::new();
		js_mapping.push(&mapping.0.into());
		js_mapping.push(&mapping.1.into());
		mappings.push(&js_mapping);
	}
	js_sys::Reflect::set(&serialized, &"mappings".into(), &mappings).unwrap();

	serialized
}

#[allow(unused_must_use)]
fn serialize_error(error: &debrix_compiler::Error) -> js_sys::Object {
	let serialized = js_sys::Object::new();

	match error {
		debrix_compiler::Error::CompilerError(err) => {
			js_sys::Reflect::set(&serialized, &"type".into(), &0_f64.into());
			js_sys::Reflect::set(&serialized, &"message".into(), &format!("{:?}", err).into());
			js_sys::Reflect::set(&serialized, &"start".into(), &(err.start as f64).into());
			js_sys::Reflect::set(&serialized, &"end".into(), &(err.end as f64).into());
			js_sys::Reflect::set(&serialized, &"_message".into(), &err.message.clone().into());
		}
		debrix_compiler::Error::ParserError(err) => {
			let positives = js_sys::Array::new();
			for str in &err.positives {
				positives.push(&str.into());
			}

			js_sys::Reflect::set(&serialized, &"type".into(), &1_f64.into());
			js_sys::Reflect::set(&serialized, &"message".into(), &format!("{:?}", err).into());
			js_sys::Reflect::set(&serialized, &"start".into(), &(err.position as f64).into());
			js_sys::Reflect::set(&serialized, &"positives".into(), &positives);
		}
	}

	serialized
}

fn serialize_result<T, E>(result: Result<T, E>) -> js_sys::Object
where
	T: wasm_bindgen::JsCast,
	E: wasm_bindgen::JsCast,
{
	let serialized = js_sys::Object::new();

	match result {
		Ok(result) => js_sys::Reflect::set(&serialized, &"result".into(), &result.into()).unwrap(),
		Err(err) => js_sys::Reflect::set(&serialized, &"error".into(), &err.into()).unwrap(),
	};

	serialized
}

#[wasm_bindgen]
pub fn build(input: &str, target: usize) -> js_sys::Object {
	let target = dist_shared::int_to_target(target);

	serialize_result(match debrix_compiler::build(input.to_owned(), target) {
		Ok(result) => Ok(serialize_chunk(&result)),
		Err(err) => Err(serialize_error(&err)),
	})
}
