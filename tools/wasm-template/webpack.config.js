const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const webpack = require('webpack');

module.exports = {
    entry: './index.js',
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'index.js',
    },
    plugins: [
        new HtmlWebpackPlugin({
            template: "index.html"
        }),
        // Have this example work in Edge which doesn't ship `TextEncoder` or
        // `TextDecoder` at this time.
        new webpack.ProvidePlugin({
            TextDecoder: ['text-encoding', 'TextDecoder'],
            TextEncoder: ['text-encoding', 'TextEncoder']
        })
    ],
    devServer: {
        contentBase: [__dirname, path.join(__dirname, '../../resources')],
        port: 3000
    },
    module: {
        rules: [
            {
            test: /\.js$/,
            loader: require.resolve('@open-wc/webpack-import-meta-loader'),
            },
        ],
    },
    node: {
        fs: "empty"
    },
    externals: [ 'utf-8-validate', 'bufferutil'],
    mode: 'development'
};