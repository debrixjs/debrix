use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn build(input: &str) -> js_sys::Object {
	let result = debrixc::build(input).unwrap();
	let js_result = js_sys::Object::new();
	js_sys::Reflect::set(&js_result, &"source".into(), &result.source.into()).unwrap();
	let js_mappings = js_sys::Array::new();
	for mapping in result.mappings {
		let js_mapping = js_sys::Array::new();
		js_mapping.push(&mapping.0.into());
		js_mapping.push(&mapping.1.into());
		js_mapping.push(&mapping.2.into());
		js_mapping.push(&mapping.3.into());
		js_mappings.push(&js_mapping);
	}
	js_sys::Reflect::set(&js_result, &"mappings".into(), &js_mappings).unwrap();
	js_result
}
