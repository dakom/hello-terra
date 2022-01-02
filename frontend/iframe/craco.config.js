const { when, whenDev, whenProd, whenTest, ESLINT_MODES, POSTCSS_MODES } = require("@craco/craco");

// see https://github.com/facebook/create-react-app/issues/11756#issuecomment-1001162736
const webpack = require('webpack');

module.exports = {
    webpack: {
        configure: (config, { env, paths }) => { 
            config.resolve.fallback = {
                buffer: require.resolve('buffer'),
                stream: require.resolve('stream-browserify'),
            };
            config.plugins.push(
                new webpack.ProvidePlugin({
                    process: 'process/browser',
                    Buffer: ['buffer', 'Buffer'],
                }),
            );
            config.module.rules.push({
                test: [/\.js?$/, /\.ts?$/, /\.jsx?$/, /\.tsx?$/],
                enforce: 'pre',
                exclude: /node_modules/,
                use: ['source-map-loader'],
            });

            return config;
        }
    },
};
