const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const dist = path.resolve(__dirname, "dist");

module.exports = {
  mode: "production", // development?
  entry: {
    index: "./js/index.js",
  },
  output: {
    path: dist,
    filename: "[name].js", // bundle.js?
  },
  module: {
    rules: [
      {
        test: /\.m?js$/,
        exclude: /node_modules/,
        use: {
          loader: "babel-loader",
          options: {
            presets: [["@babel/preset-env", { targets: "defaults" }]],
          },
        },
      },
      {
        test: /\.css$/,
        use: ["style-loader", "css-loader"],
      },
    ],
  },
  devServer: {
    contentBase: dist,
  },
  plugins: [
    new CopyPlugin([path.resolve(__dirname, "static")]),
    new WasmPackPlugin({
      crateDirectory: __dirname,
    }),
    // new HtmlWebPackPlugin({
    //   template: path.resolve(__dirname, "static/index.html"),
    //   filename: "index.html",
    // }),
  ],
  experiments: {
    asyncWebAssembly: true,
  },
};
