import rust from "@wasm-tool/rollup-plugin-rust";
import serve from "rollup-plugin-serve";
import livereload from "rollup-plugin-livereload";
import copy from "rollup-plugin-copy-watch";
import injectProcessEnv from 'rollup-plugin-inject-process-env';
import { terser } from "rollup-plugin-terser";
import path from "path";

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
        sourcemap: DEV,
    },
    context: "window",
    plugins: getPlugins(),
};

function getPlugins() {
    const watchPatterns = [
        `./contracts/**/*`,
        `./shared/**/*`,
        `./frontend/media/**/*`,
        `./frontend/*.html`,
        `./frontend/iframe/src/**/*`,
        `./frontend/src/**/*`,
        `./frontend/Cargo.toml`,
    ].map(x => path.resolve(x));

    const cargoArgs = ["--features", process.env.REMOTE_TARGET];

    const copyArgs = {
        targets: [{
            src: './frontend/index.html',
            dest: 'dist/frontend',
            transform: (contents, filename) => {
                const appJs = DEV ? "/app.js" : "/hello-terra/app.js"
                return contents.toString().replace('__APP_JS__', appJs);
            }
        }]
    };

    if(DEV) {
        Object.assign(copyArgs, { 
            watch: "./frontend/*.html" 
        });
    }

    const plugins = [
        copy(copyArgs),
        rust({
            serverPath: DEV ? `/`: '/hello-terra/',
            watch: DEV,
            debug: DEV,
            watchPatterns,
            cargoArgs
        }),
        injectProcessEnv({
            REMOTE_TARGET: process.env.REMOTE_TARGET,
            MEDIA_DEV_PORT: process.env.MEDIA_DEV_PORT,
            WALLET_MNEMONIC_KEY: process.env.WALLET_MNEMONIC_KEY,
            WALLET_MNEMONIC_HOST: process.env.WALLET_MNEMONIC_HOST,
            WALLET_MNEMONIC_CHAIN: process.env.WALLET_MNEMONIC_CHAIN,
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
