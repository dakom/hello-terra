import rust from "@wasm-tool/rollup-plugin-rust";
import serve from "rollup-plugin-serve";
import livereload from "rollup-plugin-livereload";
import nodeResolve from "@rollup/plugin-node-resolve";
import copy from "rollup-plugin-copy-watch";
import injectProcessEnv from 'rollup-plugin-inject-process-env';
import { terser } from "rollup-plugin-terser";
import commonjs from '@rollup/plugin-commonjs';
import json from '@rollup/plugin-json';
import path from "path";
import nodePolyfills from 'rollup-plugin-polyfill-node';

require('dotenv').config({path:path.resolve('./.env')})

const { REMOTE_TARGET, FRONTEND_DEV_PORT} = process.env;
const DEV = REMOTE_TARGET === "local";

export default {
    input: {
        app: `./frontend/Cargo.toml`,
    },
    output: {
        dir: `./dist/frontend`,
        format: "iife",
        sourcemap: DEV
    },
    context: "window",
    plugins: getPlugins(),
    external: [
        "react-is",
        "react",
        "styled-components",
        "react-dom",
        "react/jsx-runtime",
        "@terra-money/terra.js",
        "qrcode.react",
    ]
};

function getPlugins() {
    const watchPatterns = [
        `./contracts/**/*`,
        `./shared/**/*`,
        `./frontend/**/*`,
    ].map(x => path.resolve(x));

    const plugins = [
        //json(),
        //nodePolyfills( /* options */ ),
        nodeResolve(),
        commonjs(),
        copy({
            watch: "public",
            targets: [{ src: "./frontend/public/**/*", dest: `./dist/frontend/` }],
        }),
        rust({
            serverPath: `/`,
            watch: DEV,
            debug: DEV,
            watchPatterns,
            cargoArgs: ["--features", process.env.REMOTE_TARGET],
        }),
        /*
        json(),
        nodeResolve({ 
            //module: false, // <-- this library is not an ES6 module
			//browser: true, 
            //preferBuiltins: true,
        }),
		commonjs({
            //transformMixedEsModules: true
        }),
        */
        injectProcessEnv({
            REMOTE_TARGET: process.env.REMOTE_TARGET,
            MEDIA_DEV_PORT: process.env.MEDIA_DEV_PORT,
			TERRA_DEV_PORT: process.env.TERRA_DEV_PORT,
			DEBUG_WALLET_MNEMONIC: process.env.DEBUG_WALLET_MNEMONIC,
			DEBUG_WALLET_ID: process.env.DEBUG_WALLET_ID,
        }),
    ];

    if (DEV) {
        let PORT = FRONTEND_DEV_PORT;
        if(!PORT || PORT == "") {
            PORT = "4000";
            console.warn("FRONTEND_DEV_PORT NOT SET IN .env, USING 4000");
        }
        plugins.push(
            ...[
                serve({
                    contentBase: `./dist/frontend`,
                    open: true,
                    historyApiFallback: true,
                    port: PORT,
                }),
				
                livereload("dist/frontend"),
            ]
        );
    } else {
        plugins.push(
            ...[
                terser()
            ]
        );
    }

    return plugins;
}
