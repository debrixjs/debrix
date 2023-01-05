// @ts-check

const ts = (rules) =>
	Object.fromEntries(
		Object.entries(rules).map(([key, value]) => [
			'@typescript-eslint/' + key,
			value,
		])
	);

// Note! Sort eslint rules alphabetically!

/** @type {import('eslint').Linter.Config} */
module.exports = {
	env: {
		es2021: true,
		node: true,
		browser: true,
	},
	parserOptions: {
		ecmaVersion: 'latest',
		sourceType: 'module',
		project: [
			'./packages/*/tsconfig.json',
			'./utils/tsconfig.json'
		],
	},
	extends: ['eslint:recommended', 'prettier'],
	rules: {
		eqeqeq: 'error',
		'comma-dangle': ['error', 'only-multiline'],
		'no-cond-assign': 'off',
		'no-constant-condition': 'warn',
		'no-empty': 'off',
		'no-empty-function': 'off',
		'no-restricted-imports': ['error', { patterns: ['**/index'] }],
		'no-unused-vars': ['warn', { argsIgnorePattern: '^_' }],
		'prefer-as-const': 'off',
	},
	overrides: [
		{
			files: ['*.ts', '*.tsx', '*.mts', '*.cts'],
			extends: [
				'plugin:@typescript-eslint/recommended',
				'plugin:@typescript-eslint/recommended-requiring-type-checking',
				'plugin:@typescript-eslint/strict',
			],
			rules: {
				...ts({
					'ban-ts-comment': [
						'error',
						{ 'ts-expect-error': 'allow-with-description' },
					],
					'no-empty-function': 'off',
					'no-empty-interface': 'off',
					'no-explicit-any': 'off',
					'no-invalid-void-type': ['warn', { allowAsThisParameter: true }],
					'no-non-null-assertion': 'off',
					'no-unused-vars': ['warn', { argsIgnorePattern: '^_' }],
					'strict-boolean-expressions': 'error',
				}),
			},
		},
	],
};
