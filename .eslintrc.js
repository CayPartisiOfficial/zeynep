// eslint-disable-next-line no-undef
module.exports = {
	env: {
		node: true,
		es2021: true,
	},
	extends: [
		"eslint:recommended",
		"plugin:@typescript-eslint/recommended",
		"prettier",
	],
	overrides: [],
	parser: "@typescript-eslint/parser",
	parserOptions: {
		ecmaVersion: "latest",
		sourceType: "module",
	},
	plugins: ["@typescript-eslint"],
	rules: {
		"@typescript-eslint/no-unused-vars": [
			"warn",
			{
				varsIgnorePattern: "_",
			},
		],
	},
};
