import eslintConfig from "@antfu/eslint-config";
import pluginVue from "eslint-plugin-vue";
import nuxtConfig from "./.nuxt/eslint.config.mjs";

export default eslintConfig(
	...pluginVue.configs["flat/strongly-recommended"],
	// General
	{
		typescript: true,
		vue: true,
		unocss: true,
		stylistic: {
			indent: "tab",
			quotes: "double"
		},
		rules: {
			curly: "off",
			"no-console": "off",
			"no-new-func": "off",
			"style/semi": ["error", "always"],
			"style/indent": ["error", "tab"],
			"style/quote-props": ["warn", "as-needed"],
			"style/comma-dangle": ["warn", "never"],
			"style/brace-style": ["warn", "1tbs"],
			"style/arrow-parens": ["error", "always"],
			"vue/block-order": [
				"error",
				{
					order: ["template", "script", "style"]
				}
			],
			"vue/script-indent": [
				"error",
				"tab",
				{
					baseIndent: 1
				}
			],
			"antfu/top-level-function": "off",
			"node/prefer-global/process": ["off"],
			"semi": ["error", "always"],
			"quotes": ["error", "double"],
		}
	},

	// Vue
	{
		files: ["**/*.vue"],
		rules: {
			"style/indent": "off",
			"vue/max-attributes-per-line": [
				"error",
				{
					singleline: {
						max: 1
					},
					multiline: {
						max: 1
					}
				}
			]
		}
	},

	nuxtConfig()
);
