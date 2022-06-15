use neon::prelude::*;

fn build(mut cx: FunctionContext) -> JsResult<JsObject> {
	let input = cx.argument::<JsString>(0)?;
	let result = debrixc::build(&input.value(&mut cx));

	let result_js = cx.empty_object();
	let source_js = cx.string(result.source);
	result_js.set(&mut cx, "source", source_js).unwrap();
	let mappings_js = JsArray::new(&mut cx, result.mappings.len() as u32);
	for (i, pair) in result.mappings.iter().enumerate() {
		let pair_js = JsArray::new(&mut cx, 4);
		let pair_0_js = cx.number(pair.0 as f64);
		let pair_1_js = cx.number(pair.1 as f64);
		let pair_2_js = cx.number(pair.2 as f64);
		let pair_3_js = cx.number(pair.3 as f64);
		pair_js.set(&mut cx, 0, pair_0_js).unwrap();
		pair_js.set(&mut cx, 1, pair_1_js).unwrap();
		pair_js.set(&mut cx, 2, pair_2_js).unwrap();
		pair_js.set(&mut cx, 3, pair_3_js).unwrap();
		mappings_js.set(&mut cx, i as u32, pair_js)?;
	}
	result_js.set(&mut cx, "mappings", mappings_js).unwrap();

	Ok(result_js)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
	cx.export_function("build", build)?;
	Ok(())
}
