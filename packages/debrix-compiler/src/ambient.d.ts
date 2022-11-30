declare module 'debrix.wasm.lib' {
	export interface WasmBuildResult {
		result: any,
		error: any,
	}

	export function initSync(bytes: unknown): void;
	export function build(input: string, target: number): WasmBuildResult;
}

declare module 'debrix.wasm' {
	let bytes: any;
	export default bytes;
}

declare module 'debrix.node' {
	export function build(input: string, target: number): any;
}
