/* eslint-disable @typescript-eslint/no-unsafe-argument */
/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/no-unsafe-call */
/* eslint-disable @typescript-eslint/no-unsafe-member-access */

import { initSync, build as _build } from 'debrix.wasm.lib';
import bytes from 'debrix.wasm';

initSync(bytes);

module.exports.listener = function ([handle, input, target]: any) {
	const result = _build(input, target);
	module.exports.postMessage([handle, result.result, result.error]);
};
