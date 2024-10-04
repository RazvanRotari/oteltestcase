 const path = require('path');
 const HtmlWebpackPlugin = require('html-webpack-plugin');

 module.exports = {

  mode: 'development',
   entry: {
     print: './src/print.js',
   },
	 devtool: 'inline-source-map',
	 devServer: {
    static: './public',
  },
   plugins: [
     new HtmlWebpackPlugin({
      title: 'Development',
     }),
   ],
   output: {
     filename: '[name].bundle.js',
     path: path.resolve(__dirname, 'dist'),
     clean: true,
		  publicPath: '/'
   },
  optimization: {

    runtimeChunk: 'single',

  },
 };
