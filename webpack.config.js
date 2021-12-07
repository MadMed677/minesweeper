const path = require("path");
const webpack = require('webpack');
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const dist = path.resolve(__dirname, "dist");

module.exports = {
  mode: "production",
  entry: {
    index: "./js/bootstrap"
  },
  resolve: {
    extensions: ['.ts', '.js']
  },
  output: {
    path: dist,
    filename: "[name].js"
  },
  devServer: {
    contentBase: dist,
  },
  module: {
    rules: [
      {
        test: /\.ts?/,
        loader: 'ts-loader'
      }
    ]
  },
  plugins: [
    new CopyPlugin([
      path.resolve(__dirname, "static")
    ]),

    new WasmPackPlugin({
      crateDirectory: __dirname,
    }),

    new webpack.ProvidePlugin({
      PIXI: 'pixi.js',
    }),
  ]
};
