const ts = rule => `@typescript-eslint/${rule}`

/** @type {import('eslint').Linter.Config} */
module.exports = {
	env: {
		browser: true,
		es2021: true,
		node: true
	},
	parserOptions: {
		ecmaVersion: "latest",
		sourceType: "module"
	},
	overrides: [
		{
			files: "**/*{.js,.mjs,.cjs}",
			extends: [
				"eslint:recommended"
			],
			rules: {
				// Sort alphabetically to avoid merge conflicts
				indent: ["error", "tab", { SwitchCase: 1 }],
				quotes: ["error", "single"],
				semi: ["error", "always"],
				"eol-last": ["error", "never"],
				"eqeqeq": "error",
				"linebreak-style": ["error", "unix"],
				"no-cond-assign": "off",
				"no-constant-condition": "warn",
				"no-empty": "off",
				"no-empty-function": "off",
				"no-restricted-imports": ["error", { patterns: ["**/index"] }],
				"no-unused-vars": ["warn", { argsIgnorePattern: "^_" }],
				"prefer-as-const": "off",
				"quote-props": ["error", "as-needed"],
			},
		},
		{
			files: "**/*.ts",
			extends: [
				"eslint:recommended",
				"plugin:@typescript-eslint/recommended"
			],
			parser: "@typescript-eslint/parser",
			plugins: [
				"@typescript-eslint"
			],
			parserOptions: {
				project: [
					"./tsconfig.eslint.json",
					"./packages/*/tsconfig.json"
				]
			},
			rules: {
				indent: ["error", "tab", { SwitchCase: 1 }],
				quotes: ["error", "single"],
				"eol-last": ["error", "never"],
				"eqeqeq": "error",
				"linebreak-style": ["error", "unix"],
				"no-cond-assign": "off",
				"no-constant-condition": "warn",
				"no-empty": "off",
				"no-restricted-imports": ["error", { patterns: ["**/index"] }],
				"quote-props": ["error", "as-needed"],
				[ts("no-unused-vars")]: ["warn", { argsIgnorePattern: "^_" }],
				[ts("ban-ts-comment")]: "error",
				[ts("no-empty-function")]: "off",
				[ts("no-empty-interface")]: "off",
				[ts("no-explicit-any")]: "off",
				[ts("no-non-null-assertion")]: "off",
				[ts("prefer-as-const")]: "off",
				[ts("restrict-plus-operands")]: "error",
				[ts("restrict-template-expressions")]: "error",
				[ts("semi")]: ["error", "always"],
				[ts("strict-boolean-expressions")]: "error",
			},
		}
	]
}
