import { build, Mapping as DebrixMapping, Target } from '@debrix/compiler';
import { createFilter, FilterPattern } from '@rollup/pluginutils';
import { Plugin } from 'rollup';
import { encode, SourceMapMappings, SourceMapLine } from 'sourcemap-codec';

export interface DebrixOptions {
	include?: FilterPattern
	exclude?: FilterPattern
	target?: Target

	/**
	 * Wether to resolve all paths starting with "self.". Only works in debrix components.
	 * 
	 * @default true
	 * 
	 * @example
	 * ```debrix
	 * using model from 'self.model.ts'
	 * // resolves to
	 * using model from './my-component.model.ts'
	 * ```
	 */
	resolveSelf?: boolean
}

function toMappings(source: string, mappings: DebrixMapping[]): SourceMapMappings {
	const chars = Array.from(source);
	const newlines: number[] = [];

	for (let index = 0; index < chars.length; index++) {
		if (chars[index] === '\n')
			newlines.push(index);
	}

	const mappingLines = new Map<number, SourceMapLine>();
	let length = 0;

	const getMappingLine = (line: number) => {
		if (mappingLines.has(line))
			return mappingLines.get(line)!;

		length = Math.max(length, line);

		const array: SourceMapLine = [];
		mappingLines.set(line, array);
		return array;
	};

	function offsetToLineAndColumn(offset: number): [number, number] {
		if (offset > chars.length)
			throw new Error('invalid mapping');

		let line = 0, column = 0;

		for (let index = 0; index < offset; index++) {
			if (chars[index] === '\n')
				++line;
			else
				++column;
		}

		return [line, column];
	}

	for (const [fromOffset, toOffset] of mappings) {
		const from = offsetToLineAndColumn(fromOffset);
		const to = offsetToLineAndColumn(toOffset);
		getMappingLine(to[0]).push([to[1], 0, ...from]);
	}

	return Array.from(
		{ length },
		(_, i) => getMappingLine(i)
	);
}

export default function debrix(options: DebrixOptions = {}): Plugin {
	const filter = createFilter(options.include ?? /\.(debr)?ix$/, options.exclude);

	return {
		name: 'debrix',

		...options.resolveSelf !== false && {
			resolveId(source, importer, options) {
				if (options.isEntry || importer === undefined)
					return null;

				if (!filter(importer))
					return null;
	
				if (!source.startsWith('self.'))
					return null;
	
				return importer.replace(/\.[^./]+$/, '') + source.slice(4);
			},
		},

		transform(code, id) {
			if (!filter(id)) return;

			const { source, mappings } = build(code);
			const mappingsEncoded = encode(toMappings(source, mappings));

			return {
				code: source,
				map: {
					sourcesContent: [code],
					version: 3,
					sources: [id],
					names: [],
					mappings: mappingsEncoded
				}
			};
		}
	};
}
