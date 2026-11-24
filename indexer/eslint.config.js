import js from "@eslint/js";
import tsPlugin from "@typescript-eslint/eslint-plugin";
import prettier from "eslint-config-prettier";

export default [
  { ignores: ["node_modules/", "generated/", "build/", ".envio/", "**/*.js", "**/*.mjs"] },
  js.configs.recommended,
  ...tsPlugin.configs["flat/recommended-type-checked"],
  prettier,
  {
    languageOptions: {
      parserOptions: { project: "./tsconfig.json", tsconfigRootDir: import.meta.dirname },
    },
    rules: {
      "@typescript-eslint/no-unused-vars": [
        "warn",
        { argsIgnorePattern: "^_", varsIgnorePattern: "^_" },
      ],
      "@typescript-eslint/explicit-function-return-type": "off",
      "@typescript-eslint/no-explicit-any": "warn",
      "@typescript-eslint/no-floating-promises": "error",
      "@typescript-eslint/no-misused-promises": "error",
      "@typescript-eslint/await-thenable": "error",
      "@typescript-eslint/require-await": "warn",
      "@typescript-eslint/no-unnecessary-type-assertion": "warn",
      "@typescript-eslint/no-unsafe-assignment": "warn",
      "@typescript-eslint/no-unsafe-argument": "warn",
      "@typescript-eslint/ban-ts-comment": "warn",
      "no-console": "off",
      eqeqeq: ["error", "always"],
      curly: ["warn", "all"],
      "prefer-const": "error",
    },
  },
  {
    files: ["src/handlers/*.ts"],
    rules: { "@typescript-eslint/require-await": "off" },
  },
];
