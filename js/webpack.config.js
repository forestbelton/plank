var path = require('path');

module.exports = {
    entry: ['whatwg-fetch', './index.js'],
    output: {
        path: path.resolve(__dirname, '..', 'static'),
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
    }
};