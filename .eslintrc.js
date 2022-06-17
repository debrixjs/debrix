const ts = rule => `@typescript-eslint/${rule}`;

const shared = {
	indent: ["error", "tab", { SwitchCase: 1 }],
	quotes: ["error", "single"],
	semi: ["error", "always"],
	"eol-last": ["error", "always"],
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
};

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
			rules: shared,
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
				...shared,

				"no-unused-vars": "off",
				[ts("no-unused-vars")]: ["warn", { argsIgnorePattern: "^_" }],

				"prefer-as-const": "off",
				[ts("prefer-as-const")]: "off",

				"semi": "off",
				[ts("semi")]: ["error", "always"],

				[ts("ban-ts-comment")]: "error",
				[ts("no-empty-function")]: "off",
				[ts("no-empty-interface")]: "off",
				[ts("no-explicit-any")]: "off",
				[ts("no-non-null-assertion")]: "off",
				[ts("restrict-plus-operands")]: "error",
				[ts("restrict-template-expressions")]: "error",
				[ts("strict-boolean-expressions")]: "error",
			},
		}
	]
}
