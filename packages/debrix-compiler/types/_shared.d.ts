export enum Target {
	Client,
	Hydration,
	Server,
}

export type Mapping = [number, number, number, number];

export interface BuildResult {
	source: string;
	mappings: Mapping[];
}

export type Error = CompilerError | ParserError;

export class CompilerError extends globalThis.Error {
	start: number;
	end: number;
	_message: string;
}

export class ParserError extends globalThis.Error {
	start: number;
	end: never;
	positives: string[];
}
