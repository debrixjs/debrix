// @ts-check

const ts = rules => Object.fromEntries(
	Object.entries(rules).map(([key, value]) => ['@typescript-eslint/' + key, value])
);

/** @type {import('eslint').Linter.Config} */
module.exports = {
	extends: ['eslint:recommended'],
	env: {
		es2021: true,
		node: true,
	},
	parserOptions: {
		ecmaVersion: 'latest',
		sourceType: 'module',
		project: './packages/*/tsconfig.json'
	},
	rules: {
		// Code style rules
		indent: ['error', 'tab', { SwitchCase: 1 }],
		quotes: ['error', 'single'],
		semi: ['error', 'always'],
		'eol-last': ['error', 'always'],
		'linebreak-style': ['error', 'unix'],
		'quote-props': ['error', 'as-needed'],

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
				// Code style rules
				semi: 'off',
				...ts({ semi: ['error', 'always'] }),

				...ts({
					'ban-ts-comment': ['error', { 'ts-expect-error': 'allow-with-description' }],
					'no-empty-function': 'off',
					'no-empty-interface': 'off',
					'no-explicit-any': 'off',
					'no-non-null-assertion': 'off',
					'no-unused-vars': ['warn', { argsIgnorePattern: '^_' }],
					'strict-boolean-expressions': 'error',
				})
			},
		}
	]
};
