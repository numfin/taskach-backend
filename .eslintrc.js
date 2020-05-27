module.exports = {
  root: true,
  extends: [
    "plugin:@typescript-eslint/eslint-recommended",
    "plugin:@typescript-eslint/recommended",
  ],
  parser: "@typescript-eslint/parser",
  plugins: ["@typescript-eslint"],
  rules: {
    "no-console": 1,
    "no-debugger": 1,
    "import/prefer-default-export": "off",
    "import/no-cycle": "off",
    "import/no-extraneous-dependencies": "off",
    "import/extensions": "off",
    "import/no-unresolved": "off",
    "no-empty-function": "off",
    "@typescript-eslint/explicit-function-return-type": "off",
    "@typescript-eslint/no-unused-vars": [
      1,
      {
        vars: "all",
        args: "after-used",
        ignoreRestSiblings: false,
      },
    ],
  },
};
