module.exports = {
  style: {
    postcss: {
      plugins: [require('tailwindcss')('./tailwind.config.js'), require('postcss-nested')],
    },
  },
  typescript: {
    enableTypeChecking: false,
  },
  eslint: {
    configure: {
      extends: 'react-app',
      overrides: [
        {
          files: ['**/*.tsx', '**/*.ts'],
          rules: {
            '@typescript-eslint/no-unused-vars': [
              'warn',
              {
                argsIgnorePattern: '^_',
                varsIgnorePattern: '^_',
              },
            ],
            'no-use-before-define': 'off',
            '@typescript-eslint/no-use-before-define': 'off',
          },
        },
      ],
    },
  },
};
