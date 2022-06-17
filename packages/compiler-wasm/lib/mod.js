import * as debrixc_wasm from './wasm/debrixc_wasm';
import wasm from './wasm/debrixc_wasm_bg.wasm';

function decode(wasm) {
	wasm = atob(wasm);
	const bytes = new Uint8Array(wasm.length);
	for (let i = 0; i < wasm.length; ++i)
		bytes[i] = wasm.charCodeAt(i);
	return bytes;
}

export default async function instantiate() {
	await debrixc_wasm.default(decode(wasm));
	return {
		build(input) {
			return debrixc_wasm.build(input);
		}
	};
}
