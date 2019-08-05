//based on https://github.com/marko-js-samples/marko-webpack/blob/master/webpack.config.js
const { NODE_ENV } = process.env;
const isProd = NODE_ENV === "production";
const isDev = !isProd;

module.exports = {
  entry: "./index.js",
  output: {
    path: __dirname,
    filename: "static/bundle.js"
  },
  devServer: isDev ? {
    overlay: true,
    stats: "minimal",
    contentBase: __dirname,
    port: 8008,
    historyApiFallback: true
  } : undefined,
  resolve: {
    extensions: [".js", ".marko"]
  },
  module: {
    rules: [
      {
        test: /\.marko$/,
        loader: "marko-loader"
      },
      {
        test: /\.scss$/,
        use: [
          "style-loader",
          "css-loader",
          "sass-loader"
        ]
      }
    ]
  }
};
