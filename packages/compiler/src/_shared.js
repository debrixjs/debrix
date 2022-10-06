export const Target = Object.freeze({
	0: 'Client',
	1: 'Hydration',
	2: 'Server',

	Client: 0,
	Hydration: 1,
	Server: 2,
});

export function validate(args) {
	const [input, target] = args;

	if (typeof input !== 'string')
		throw new Error('invalid input');

	if (typeof target !== 'number' || target < 0 || target > 2 || target % 1 !== 0)
		throw new Error('invalid target');
}
