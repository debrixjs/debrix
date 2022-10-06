// @ts-check

/** @type {import('eslint').Linter.Config} */
module.exports = {
	extends: '@debrixjs',
	env: {
		es2021: true,
		node: true,
	},
	parserOptions: {
		project: './packages/*/tsconfig.json'
	}
};
