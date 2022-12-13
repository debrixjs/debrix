use neon::prelude::*;

pub fn int_to_target(int: usize) -> debrix_compiler::Target {
	match int {
		0 => debrix_compiler::Target::Client,
		1 => debrix_compiler::Target::Hydration,
		2 => debrix_compiler::Target::Server,
		_ => unreachable!(),
	}
}

fn build(mut cx: FunctionContext) -> JsResult<JsObject> {
	let input = cx.argument::<JsString>(0)?.value(&mut cx);
	let target = int_to_target(cx.argument::<JsNumber>(1)?.value(&mut cx) as usize);

	let result = match debrix_compiler::build(input, target) {
		Ok(result) => result,
		Err(err) => {
			return match err {
				debrix_compiler::Error::ParserError(err) => {
					let js_err = cx.empty_object();

					let js_type = cx.number(1);
					let js_message = cx.string(format!("{:?}", err));
					let js_start = cx.number(err.position as f64);

					let js_positives = cx.empty_array();
					for (i, s) in err.positives.iter().enumerate() {
						let js_str = cx.string(s);
						js_positives.set(&mut cx, i as u32, js_str)?;
					}

					js_err.set(&mut cx, "type", js_type)?;
					js_err.set(&mut cx, "message", js_message)?;
					js_err.set(&mut cx, "start", js_start)?;
					js_err.set(&mut cx, "positives", js_positives)?;
					cx.throw(js_err)
				}
				debrix_compiler::Error::CompilerError(err) => {
					let js_err = cx.error(format!("{:?}", err))?;

					let js_type = cx.number(0);
					let js_message = cx.string(format!("{:?}", err));
					let js_start = cx.number(err.start as f64);
					let js_end = cx.number(err.end as f64);
					let js_message2 = cx.string(err.message);

					js_err.set(&mut cx, "type", js_type)?;
					js_err.set(&mut cx, "message", js_message)?;
					js_err.set(&mut cx, "start", js_start)?;
					js_err.set(&mut cx, "end", js_end)?;
					js_err.set(&mut cx, "_message", js_message2)?;
					cx.throw(js_err)
				}
			};
		}
	};

	let result_js = cx.empty_object();
	let source_js = cx.string(result.source);
	result_js.set(&mut cx, "source", source_js).unwrap();
	let mappings_js = JsArray::new(&mut cx, result.mappings.len() as u32);
	for (i, pair) in result.mappings.iter().enumerate() {
		let pair_js = JsArray::new(&mut cx, 2);
		let pair_0_js = cx.number(pair.0 as f64);
		let pair_1_js = cx.number(pair.1 as f64);
		pair_js.set(&mut cx, 0, pair_0_js).unwrap();
		pair_js.set(&mut cx, 1, pair_1_js).unwrap();
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
