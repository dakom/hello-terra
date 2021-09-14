# Hello Terra 

# [LIVE DEMO](https://dakom.github.io/hello-terra)
# [SHARED CRATE DOCS](https://dakom.github.io/hello-terra/docs/shared)

# GOALS

## User Experience
* Login via Terra Station chrome extension or mobile app
* No other auth or database (state is persisted on-chain, contract addresses are stored locally in-browser)
* Reactive to live changes on the network (correct balance shown in multiple tabs)
* Simple bootstrapping for new system (no need for user to manually compile/select .wasm files) 
* Simple fake bank account (deposit, withdraw, view balance)

# Technical

* No need or use for JSON schema (real cargo docs - see above)
* Native Rust types for message payloads enfoced _at compile-time_ (contracts and frontend are guaranteed to typecheck with eachother) 
* Frontend main application is pure Rust/WebAssembly, written in the Dominator framework (declarative, fast, and awesome)
* Wallet is handled via a (usually) hidden iframe and a postMessage communication layer (can be abstracted/expanded)
* Detecting and deploying a new contract is done automatically via Blake3 hash, id, and localstorage lookup
* Continuous Integration/Deployment of full app via Github Actions (live demo - see above)
* Good DX (live-reloads when sources change, cross-platform, simple commands, separate local vs. release settings, workspace, etc.)

## Iframe / Wallet abstraction

`wallet-provider` depends on React, and `terra.js` doesn't bundle nicely with rollup/wasm

So instead of wrangling this into the core application, it's separated out into its own iframe. The only time the UI needs to be displayed is for QR code.

Having this abstraction is also theoretically nice since it can allow adding new wallet providers or dealing with different chains across a generic API

This does have one downside - the typescript and Rust types for the high-level communication wrappers need to be kept in sync. However, This is only needed for these wrappers, not each message use-case. For example, adding new contract message request/response payloads doesn't require any further work (it's all kept in Rust and the wallet is oblivious to the on-the-wire json format). Adding more Terra.JS functionality like Bank Queries and whatnot would only need to be done once.

# Local Development

## Bootstrapping / one-time setup

1. Install required tooling (rust, npm, cargo make)
    - rust
    - npm
    - cargo make: cargo install cargo-make
    - b3sum: cargo install b3sum
    - wasm-opt: download at [binaryen releases](https://github.com/WebAssembly/binaryen/releases) and put it somewhere on your path
2. copy `.env.sample` to `.env` change whatever values 
3. `npm install`
4. in `frontend/iframe` also `npm install`


## Frontend dev

- `cargo make frontend`

This will start the main app, the iframe app, and the media server in parallel, as well as open a browser tab for each app individually (though the iframe is typically meant to be hidden - recommended to close that tab and just work off the main tab, logs and line numbers show in the console there too)

## Contract building 

- `cargo make contracts-build-release`

This will compile, optimize, and generate a hash file for the contracts.
The output will be in `./frontend/media/contracts/`

However this is .gitignored for CI

Running this with the frontend dev simultaneously will cause a "new system" flow live

Currently, this project isn't setup for contract migrations

## Contract testing 

- `cargo make contracts-test`

# CI Setup

1. replace `dakom` in `.github/workflows/build.yml` with your github username
2. create/add `GH_PAT` to your repo's secrets (it's a github deployment token [for deploying to gh_pages](https://github.com/maxheld83/ghpages/pull/18))