/* #ESM */
import { createRequire } from 'node:module';
const require = createRequire(import.meta.url);
/* /ESM */
const compiler = require('../lib/debrix.node');

import { Target, _createError, _validate } from './_shared.js';
export { CompilerError, ParserError, Target } from './_shared.js';

export function build(input, target = Target.Client) {
	_validate([input, target]);

	try {
		return compiler.build(input, target);
	} catch (err) {
		throw _createError(err);
	}
}
