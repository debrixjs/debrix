export type InternalErrorObject = any;

export interface InternalBuildObject {
	source: string;
	mappings: Mapping[];
}

export type Mapping = [number, number, number, number];

export interface Build {
	source: string;
	mappings: Mapping[];
}

export type Error = CompilerError | ParserError;

export class CompilerError extends Error {
	start!: number;
	end!: number;
	_message!: string;
}

export class ParserError extends Error {
	start!: number;
	end!: never;
	positives!: string[];
}

export enum Target {
	Client,
	Hydration,
	Server,
}

export function _validate(input: string, target: Target) {
	if (typeof input !== 'string') throw new Error('invalid input');

	if (
		typeof target !== 'number' ||
		target < 0 ||
		target > 2 ||
		target % 1 !== 0
	)
		throw new Error('invalid target');
}

export function _createError(obj: InternalErrorObject) {
	/* eslint-disable */
	if (obj.type === 0) {
		/* compiler error */ let err = new CompilerError(obj.message);
		err.start = obj.start;
		err.end = obj.end;
		err._message = obj._message;
		return err;
	}

	if (obj.type === 1) {
		/* parser error */ let err = new ParserError(obj.message);
		err.start = obj.start;
		(err.end as number) = obj.end;
		err.positives = obj.positives;
		return err;
	}

	throw new Error('invalid error type');
	/* eslint-enable */
}
