var webpack = require('webpack')
var os = require('os')

//based on https://github.com/marko-js-samples/marko-webpack/blob/master/webpack.config.js
const { NODE_ENV } = process.env
const isProd = NODE_ENV === 'production'
const isDev = !isProd

var hostname = os.hostname()
process.env.GATEWAY_URL = `http://${hostname}:8080`

module.exports = {
  entry: './index.js',
  output: {
    path: __dirname,
    filename: 'static/bundle.js',
  },
  devServer: isDev ? {
    overlay: true,
    stats: 'minimal',
    contentBase: __dirname,
    port: 8008,
    historyApiFallback: true,
  } : undefined,
  resolve: {
    extensions: [".js", ".marko"]
  },
  plugins: [
    // Pass variables to `.marko` files.
    new webpack.DefinePlugin({
      'process.env.GATEWAY_URL': JSON.stringify(process.env.GATEWAY_URL),
    })
  ],
  module: {
    rules: [
      {
        test: /\.m?js$/,
        exclude: /node_modules/,
        use: [
          {
            loader: 'babel-loader',
            options: {
              presets: [['@babel/env', { "modules": "commonjs" }]],
              plugins: [
                'add-module-exports',
                // Pass variables to `.js` files.
                [
                  'transform-define',
                  {
                    'process.env.GATEWAY_URL': process.env.GATEWAY_URL,
                  },
                ],
              ],
            },
          },
          'eslint-loader',
        ]
      },
      {
        test: /\.marko$/,
        loader: '@marko/webpack/loader'
      },
      {
        test: /\.(scss|sass)$/,
        use: [
          'style-loader',
          'css-loader',
          'sass-loader',
        ],
      },
    ],
  },
}
