import { initSync, build as _build } from '__debrix.wasm.js';
import bytes from '../lib/debrix.wasm.js';
import { Target, validate } from './_shared.js';
export { Target };

initSync(bytes);

export async function build(input, target = Target.Client) {
	validate([input, target]);

	const result = _build(input, target);

	if ('error' in result)
		throw result.error;

	return result.result;
}
