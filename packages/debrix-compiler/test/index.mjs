import test from 'node:test';
import assert from 'node:assert/strict';
import * as node from '../node/index.mjs';
import * as wasm from '../wasm/index.mjs';

const document = `
	using model from 'self.model.ts'

	<p>Hello {name}!</p>
`;

let nodeResult, wasmResult;

test('Node dist compiles document', async () => {
	const { source, mappings } = nodeResult = await node.build(document);
	assert(source.length > 0, 'source is expected to have length');
	assert(mappings.length > 0, 'mappings is expected to have length');
});

test('WASM dist compiles document', async () => {
	const { source, mappings } = wasmResult = await wasm.build(document);
	assert(source.length > 0, 'source is expected to have length');
	assert(mappings.length > 0, 'mappings is expected to have length');
});

test('WASM and Node dist have the same result', () => {
	assert.deepEqual(nodeResult, wasmResult);
});
