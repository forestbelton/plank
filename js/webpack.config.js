var path = require('path');
var UglifyJSPlugin = require('uglifyjs-webpack-plugin');

module.exports = {
    entry: ['whatwg-fetch', './index.js'],
    output: {
        path: process.env.OUTPUT_DIRECTORY || path.resolve(__dirname, '..', 'static'),
        filename: 'bundle.js'
    },
    module: {
        rules: [
            {
                test: /\.js$/,
                exclude: /node_modules/,
                use: 'babel-loader'
            },
            {
                test: /\.css$/,
                use: ['style-loader', 'css-loader']
            }
        ]
    },
    plugins: [
        new UglifyJSPlugin()
    ]
};
