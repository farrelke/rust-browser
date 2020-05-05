const path = require("path");
const webpack = require("webpack");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const netlifyBuild =  process.argv.indexOf('--netlify') !== -1;
const dist = path.resolve(__dirname, netlifyBuild ? "netlify" : "dist");

module.exports = {
  mode: "production",
  entry: {
    index: "./js/index.js"
  },
  output: {
    path: dist,
    filename: "[name].js"
  },
  devServer: {
    contentBase: dist
  },
  plugins: [
    new CopyPlugin([path.resolve(__dirname, "static")]),

    new WasmPackPlugin({
      crateDirectory: __dirname
    }),

    new webpack.DefinePlugin({
      NETLIFY: JSON.stringify(netlifyBuild)
    })
  ]
};
