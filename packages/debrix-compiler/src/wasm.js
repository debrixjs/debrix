import { initSync, build as _build } from '__debrix.wasm.js';
import bytes from '../lib/debrix.wasm.js';
import { Target, _createError, _validate } from './_shared.js';
export { CompilerError, ParserError, Target } from './_shared.js';

initSync(bytes);

export async function build(input, target = Target.Client) {
	_validate([input, target]);

	const result = _build(input, target);

	if ('error' in result)
		throw _createError(result.error);

	return result.result;
}
