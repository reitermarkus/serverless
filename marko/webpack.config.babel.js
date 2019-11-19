// based on https://github.com/marko-js-samples/marko-webpack/blob/master/webpack.config.js

import path from 'path'
import webpack from 'webpack'
import { hostname } from 'os'
import HtmlWebpackPlugin from 'html-webpack-plugin'
import { CleanWebpackPlugin } from 'clean-webpack-plugin'
import MiniCssExtractPlugin from 'mini-css-extract-plugin'

const { NODE_ENV } = process.env
const PRODUCTION = NODE_ENV === 'production'

const VARIABLES = {
  GATEWAY_URL: `http://${hostname()}:8080`,
}

export default {
  mode: PRODUCTION ? 'production' : 'development',
  entry: './src/index.js',
  output: {
    filename: 'bundle.js',
    path: path.resolve(__dirname, 'dist'),
  },
  devtool: PRODUCTION ? undefined : 'source-map',
  devServer: PRODUCTION ? undefined : {
    overlay: true,
    stats: 'minimal',
    contentBase: __dirname,
    disableHostCheck: true,
    host: '0.0.0.0',
    port: 8008,
    proxy: {
      '/function': 'http://localhost:8080',
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
    }, {})),
    new CleanWebpackPlugin(),
    new HtmlWebpackPlugin({
      title: 'Serverless UI',
      meta: {
        'viewport': 'width=device-width, initial-scale=1',
      },
    }),
    new MiniCssExtractPlugin(),
  ],
  module: {
    rules: [
      {
        test: /\.m?js$/,
        exclude: /node_modules/,
        use: [
          'babel-loader',
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
          {
            loader: MiniCssExtractPlugin.loader,
            options: {
              hmr: !PRODUCTION,
            },
          },
          'css-loader',
          'sass-loader',
        ],
      },
    ],
  },
}
