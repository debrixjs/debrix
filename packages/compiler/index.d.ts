export type Mapping = [number, number, number, number];

export interface BuildResult {
	source: string;
	mappings: Mapping[];
}

export function build(input: string): BuildResult;
