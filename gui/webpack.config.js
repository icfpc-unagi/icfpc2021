const path = require('path');
const CopyPlugin = require('copy-webpack-plugin');
// const HtmlWebpackPlugin = require('html-webpack-plugin');

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
    extensions: [ '.tsx', '.ts', '.js' ],
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
