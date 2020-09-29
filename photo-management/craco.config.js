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
          files: ['**/*.tsx'],
          rules: {
            '@typescript-eslint/no-unused-vars': [
              'warn',
              {
                argsIgnorePattern: '^_',
                varsIgnorePattern: '^_',
              },
            ],
          },
        },
      ],
    },
  },
};
