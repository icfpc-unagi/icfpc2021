const path = require('path');
const CopyPlugin = require('copy-webpack-plugin');
// const HtmlWebpackPlugin = require('html-webpack-plugin');
// const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin"); // "@wasm-tool/wasm-pack-plugin": "^1.4.0",

module.exports = {
  mode: 'development',
  entry: './src/index.ts',

  devtool: 'inline-source-map',
  plugins: [
    // new HtmlWebpackPlugin({
    //   // title: 'pixi-ts',
    // }),
    new CopyPlugin({
      patterns: [
        {
          from: 'index.html',
        },
        // {
        //   from: 'assets/**',
        // },
      ],
    }),
    // new WasmPackPlugin({
    //   crateDirectory: path.join(__dirname, "crate")
    // }),
  ],
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      },
    ],
  },
  resolve: {
    extensions: [ '.tsx', '.ts', '.js', '.wasm' ],
  },
  experiments: {
    asyncWebAssembly: true
    // syncWebAssembly: true
  },
  output: {
    filename: 'bundle.js',
    path: path.resolve(__dirname, 'dist'),
  },
  devServer: {
    contentBase: path.join(__dirname, 'dist'),
    compress: true,
    host: '0.0.0.0',
    port: 8888,
    hot: true
  }
};
