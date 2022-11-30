/* eslint-disable @typescript-eslint/no-unsafe-argument */
/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/no-unsafe-call */
/* eslint-disable @typescript-eslint/no-unsafe-member-access */

import { initSync, build as _build } from 'debrix.wasm.lib';
import bytes from 'debrix.wasm';

initSync(bytes);

let postMessage: (data: unknown) => void;

// eslint-disable-next-line @typescript-eslint/no-unused-vars
function LISTENER([handle, input, target]: any) {
	const result = _build(input, target);
	postMessage([handle, result.result, result.error]);
}

(() => LISTENER)();
