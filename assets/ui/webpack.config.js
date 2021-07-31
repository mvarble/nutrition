const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');

module.exports = (_, { mode }) => ({
  mode,
  target: 'web',
  entry: path.join(__dirname, "src", "index.js"),
  output: {
    publicPath: '',
    path: path.join(__dirname, "dist"),
    filename: path.join("js", "bundle.js"),
  },
  module: {
    rules: [
      {
        test: /\.js$/, 
        exclude: /node_modules/,
        use: {
          loader: 'babel-loader',
          options: {
            presets: ['@babel/preset-react', '@babel/preset-env'],
          },
        },
      },
      {
        test: /\.s(a|c)ss$/,
        use: [
          { 
            loader: MiniCssExtractPlugin.loader,
            options: {
              publicPath: '/',
            },
          },
          { 
            loader: "css-loader",
            options: { 
              esModule: true,
              modules: { 
                namedExport: true,
                auto: /\.module\.\w+$/,
              },
            },
          },
          { loader: "sass-loader" },
        ],
      },
      {
        test: /\.(woff(2)?|ttf|eot)$/,
        type: 'asset/resource',
        generator: {
          filename: path.join('fonts', '[name][ext]'),
        },
      },
      {
        test: /\.(svg|png|gif|jpeg|jpg)$/i,
        type: 'asset/resource',
        generator: {
          filename: path.join('images', '[name][ext]'),
        },
      },
    ],
  },
  plugins: [
    new HtmlWebpackPlugin({
      filename: 'index.html',
      template: path.join('src', 'template.html'),
      title: 'Nutrition API',
    }),
    new MiniCssExtractPlugin({ filename: path.join('css', '[name].css') }),
  ],
  watch: mode === 'development',
  watchOptions: {
    ignored: /node_modules/,
  },
});
