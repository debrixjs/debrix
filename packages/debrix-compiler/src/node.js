/* #ESM */
import { createRequire } from 'node:module';
const require = createRequire(import.meta.url);
/* /ESM */
const compiler = require('../lib/debrix.node');

import { Target, validate } from './_shared.js';
export { Target };

export class CompilerError extends Error {
	constructor(msg) { super(msg); }
}

export class ParserError extends Error {
	constructor(msg) { super(msg); }
}

export function build(input, target = Target.Client) {
	validate([input, target]);

	try {
		return compiler.build(input, target);
	} catch (obj) {
		if (obj.type === 0) /* compiler error */ {
			let err = new CompilerError(obj.message);
			err.start = obj.start;
			err.end = obj.end;
			err._message = obj._message;
			throw err;
		}

		if (obj.type === 1) /* parser error */ {
			let err = new ParserError(obj.message);
			err.start = obj.start;
			err.end = obj.end;
			err.positives = obj.positives;
			throw err;
		}
	}
}
