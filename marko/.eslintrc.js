module.exports = {
  'parser': 'babel-eslint',
  'env': {
    'browser': true,
    'es6': true
  },
  'extends': 'eslint:recommended',
  'globals': {
    '__dirname': 'readonly',
    'Atomics': 'readonly',
    'SharedArrayBuffer': 'readonly',
    'process': 'readonly',
  },
  'parserOptions': {
    'ecmaVersion': 2018,
    'sourceType': 'module'
  },
  'rules': {
    'indent': [
      'error',
      2
    ],
    'quotes': [
      'error',
      'single'
    ],
    'semi': [
      'error',
      'never'
    ],
    'no-var': [
      'error'
    ],
    'eqeqeq': [
      'error',
      'smart'
    ]
  }
}
