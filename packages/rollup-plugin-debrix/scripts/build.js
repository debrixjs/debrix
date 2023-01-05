// @ts-check

import { build } from 'esbuild';
import { writeFile } from 'node:fs/promises';
import { exec, parallel } from '../../../utils/build';

parallel(
	() => exec(
		[
			'node',
			require.resolve('typescript/lib/tsc.js'),
			'--declaration',
			'--emitDeclarationOnly',
			'--outDir types',
		]
	),

	() => build({
		entryPoints: ['./src/plugin.ts'],
		outfile: './index.js',
		format: 'cjs',
		platform: 'node',
	}),

	() => build({
		entryPoints: ['./src/plugin.ts'],
		outfile: './index.mjs',
		format: 'esm',
		platform: 'node',
	}),

	() => writeFile(
		'./index.d.ts',
		"export * from './types/plugin';\nexport { default } from './types/plugin';\n"
	),
);
