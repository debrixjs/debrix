export const Target = Object.freeze({
	0: 'Client',
	1: 'Hydration',
	2: 'Server',

	Client: 0,
	Hydration: 1,
	Server: 2,
});

export function _validate(args) {
	const [input, target] = args;

	if (typeof input !== 'string')
		throw new Error('invalid input');

	if (typeof target !== 'number' || target < 0 || target > 2 || target % 1 !== 0)
		throw new Error('invalid target');
}

export function _createError(obj) {
	if (obj.type === 0) /* compiler error */ {
		let err = new CompilerError(obj.message);
		err.start = obj.start;
		err.end = obj.end;
		err._message = obj._message;
		return err;
	}

	if (obj.type === 1) /* parser error */ {
		let err = new ParserError(obj.message);
		err.start = obj.start;
		err.end = obj.end;
		err.positives = obj.positives;
		return err;
	}

	throw new Error('invalid error type');
}

export class CompilerError extends Error {
	constructor(msg) { super(msg); }
}

export class ParserError extends Error {
	constructor(msg) { super(msg); }
}
