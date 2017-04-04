const path = require('path')

module.exports = {
  context: __dirname,  // project path
  entry: './js/ClientApp.js',  // entry point to app
  devtool: 'source-map',
  output: {
    path: path.join(__dirname, '/public'),
    filename: 'bundle.js'
  },
  devServer: {
    publicPath: '/public/',  // setting up static file serving
    historyApiFallback: true  // reroute 404s to the homepage
  },
  resolve: {
    extensions: ['.js', '.json']  // the type of file extensions it'll try to find when importing without an extension
  },
  stats: {
    colors: true,
    reasons: true,
    chunks: true
  },
  module: {
    rules: [
      {
        enforce: 'pre',
        test: /\.js$/,
        loader: 'eslint-loader',
        exclude: /node_modules/
      },
      {
        test: /\.json$/,  // load json from files
        loader: 'json-loader'
      },
      {
        include: path.resolve(__dirname, 'js'),  // only files that are in the /js/ directory
        test: /\.js$/,  // if it ends in js
        loader: 'babel-loader'  // run it through babel-loader
      },
      {
        test: /\.css/,
        use: [
          // pass multiple loaders
          // less/sass loader if you want

          'style-loader',
          {
            loader: 'css-loader',
            options: {
              url: false
            }
          }
        ]
      }
    ]
  }
}
