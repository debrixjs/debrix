import { exec as _exec, ExecOptions } from 'node:child_process';
import { createRequire } from 'node:module';
import { build, BuildOptions, Loader, Plugin } from 'esbuild';
import { NodeResolvePlugin } from '@esbuild-plugins/node-resolve';
import { readFile } from 'node:fs/promises';
import { extname } from 'node:path';
import { mergeWith as _mergeWith } from 'lodash';

export { build } from 'esbuild';

const require = createRequire(import.meta.url);

function isString(value: unknown): value is string {
	return typeof value === 'string';
}

function isArray(value: unknown): value is unknown[] {
	return Array.isArray(value);
}

export async function parallel(
	...tasks: (() => Promise<unknown>)[]
): Promise<void> {
	await Promise.all(tasks.map((fn) => fn()));
}

export function exec(command: string | string[], options?: ExecOptions) {
	return new Promise((resolve, reject) => {
		const commandStr = Array.isArray(command)
			? command.filter(isString).join(' ')
			: command;

		const proc = _exec(commandStr, options, (err, stdout, stderr) => {
			if (err) reject(err);
			else resolve({ stdout, stderr });
		});

		proc.stdout?.pipe(process.stdout);
		proc.stderr?.pipe(process.stderr);
	});
}

export interface DeclarationsOptions {
	/**
	 * @default '.'
	 */
	project?: string;

	/**
	 * @default 'types'
	 */
	out?: string;
}

export async function declarations(options?: DeclarationsOptions) {
	await exec([
		'node',
		require.resolve('typescript/lib/tsc.js'),
		`--project ${options?.project ?? '.'}`,
		'--declaration',
		'--emitDeclarationOnly',
		`--outDir ${options?.out ?? 'types'}`,
	]);
}

export function extend<T1 extends object, T2 extends object>(
	...objects: readonly [T1, T2]
): T1 & T2;
export function extend<T1 extends object, T2 extends object, T3 extends object>(
	...objects: readonly [T1, T2, T3]
): T1 & T2 & T3;
export function extend<
	T1 extends object,
	T2 extends object,
	T3 extends object,
	T4 extends object
>(...objects: readonly [T1, T2, T3, T4]): T1 & T2 & T3 & T4;
export function extend<
	T1 extends object,
	T2 extends object,
	T3 extends object,
	T4 extends object,
	T5 extends object
>(...objects: readonly [T1, T2, T3, T4, T5]): T1 & T2 & T3 & T4 & T5;
export function extend(...objects: readonly object[]): unknown;
export function extend(...objects: object[]) {
	const customizer = (a: unknown, b: unknown) => {
		if (isArray(a) && isArray(b)) return a.concat(b);
	};

	return _mergeWith({}, ...objects, customizer) as unknown;
}

export function buildOptions(...objects: BuildOptions[]) {
	return extend(...objects) as BuildOptions;
}

export async function buildToBuf(options: BuildOptions): Promise<Uint8Array> {
	return (await build({ ...options, write: false })).outputFiles[0]!.contents;
}

// The options interface is not exported by '@esbuild-plugins/node-resolve'
export type NodeResolvePluginOptions = Parameters<typeof NodeResolvePlugin>[0];

export function externalPlugin(): Plugin {
	return {
		name: 'external',
		setup(build) {
			build.onResolve({ filter: /\?external$/ }, (args) => {
				return {
					path: args.path.slice(0, -9),
					external: true,
				};
			});
		},
	};
}

export function nodeResolvePlugin(options?: NodeResolvePluginOptions): Plugin {
	return NodeResolvePlugin({
		extensions: ['.js', '.ts'],
		onResolved: (id) => {
			if (id.startsWith('node:')) return { external: true };
		},
		...options,
	});
}

export function virtualPlugin(
	files: Record<string, string | Uint8Array>
): Plugin {
	return {
		name: 'virtual',
		setup(build) {
			const aliases = Object.keys(files);
			const filter = new RegExp(
				`^(${aliases.map((x) => escapeRegExp(x)).join('|')})$`
			);

			build.onResolve({ filter }, (args) => {
				return {
					namespace: 'virtual',
					path: args.path,
				};
			});

			build.onLoad({ filter, namespace: 'virtual' }, (args) => {
				return {
					contents: files[args.path],
				};
			});
		},
	};
}

function extToLoader(ext: string): Loader {
	switch (ext) {
		case '.js':
			return 'js';

		case '.ts':
			return 'ts';

		default:
			return 'default';
	}
}

export interface ReplacePluginOptions {
	replace: [RegExp | string, string][];
	filter: RegExp;
}

export function replacePlugin({
	replace,
	filter,
}: ReplacePluginOptions): Plugin {
	return {
		name: 'replace',
		setup(build) {
			build.onLoad({ filter }, async (args) => {
				let contents = await readFile(args.path, 'utf8');

				for (const [searchValue, replaceValue] of replace)
					contents = contents.replace(searchValue, replaceValue);

				return {
					contents,
					loader: extToLoader(extname(args.path)),
				};
			});
		},
	};
}

function escapeRegExp(string: string): string {
	return string.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}
