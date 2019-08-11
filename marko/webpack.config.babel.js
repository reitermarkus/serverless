// based on https://github.com/marko-js-samples/marko-webpack/blob/master/webpack.config.js

import webpack from 'webpack'
import { hostname } from 'os'

const { NODE_ENV } = process.env
const PRODUCTION = NODE_ENV === 'production'

const VARIABLES = {
  GATEWAY_URL: `http://${hostname()}:8080`,
}

export default {
  entry: './index.js',
  output: {
    path: __dirname,
    filename: 'static/bundle.js',
  },
  devServer: PRODUCTION ? undefined : {
    overlay: true,
    stats: 'minimal',
    contentBase: __dirname,
    port: 8008,
    proxy: {
      '/function': 'http://localhost:8080'
    },
    historyApiFallback: true,
  },
  resolve: {
    extensions: ['.js', '.marko']
  },
  plugins: [
    new webpack.DefinePlugin(Object.keys(VARIABLES).reduce((o, k) => {
      o[`process.env.${k}`] = JSON.stringify(VARIABLES[k])
      return o
    }, {}))
  ],
  module: {
    rules: [
      {
        test: /\.m?js$/,
        exclude: /node_modules/,
        use: [
          {
            loader: 'babel-loader',
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
