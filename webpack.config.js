const path = require("path");
const dist = path.resolve(__dirname, "dist");

const HtmlWebpackPlugin = require("html-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const CopyPlugin = require('copy-webpack-plugin');

const appConfig = {
  entry: "./src/main.js",
  devServer: {
    contentBase: dist,
    clientLogLevel: 'info',
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: "./src/index.html"
    })
  ],
  resolve: {
    extensions: [".js"]
  },
  output: {
    path: dist,
    filename: "main.js"
  }
};

const workerConfig = {
  entry: "./src/worker.js",
  target: "webworker",
  plugins: [
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "./pkg")
    }),
    new CopyPlugin([{ from: path.resolve(__dirname, "./assets/"), to: path.resolve(dist, "./assets/"), logLevel: "debug" }]),
  ],
  resolve: {
    extensions: [".js", ".wasm"]
  },
  output: {
    path: dist,
    filename: "worker.js"
  }
};

module.exports = [appConfig, workerConfig];