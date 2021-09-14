# Hello Terra 

~~# [LIVE APP](http://example.com) / [CRATE DOCS](http://example.com)~~

# GOALS

## User Experience
* Login via Terra Station chrome extension or mobile app
* No other auth or database (state is persisted on-chain, contract addresses are stored locally in-browser)
* Reactive to live changes on the network 
* Simple bootstrapping for new system (no need for user to manually compile/select .wasm files) 
* Simple fake bank account (deposit, withdraw, view balance)

# Technical

* No need or use for JSON schema (real cargo docs!)
* Native Rust types for message payloads - enforces contract/frontend match eachother _at compile-time_
* Frontend main application is pure Rust/WebAssembly, written in the Dominator framework (declarative, fast, and awesome)
* Wallet is handled via a (usually) hidden iframe and a postMessage communication layer
* Detecting and deploying a new contract is done automatically via hash id and localstorage lookup
* Documentation of the shared data structures is generated automatically and browseable online
* Continuous Integration/Deployment of frontend via Github Actions
* Good DX (live-reloads when sources change, cross-platform, simple commands, separate local vs. release settings, workspace, etc.)
* Reduce finicky dependencies where possible

## Iframe / Wallet abstraction

`wallet-provider` depends on React, and `terra.js` doesn't bundle nicely with rollup/wasm

So instead of wrangling this into the core application, it's separated out into its own iframe. The only time the UI needs to be displayed is for QR code.

Having this abstraction is also theoretically nice since it can allow adding new wallet providers or dealing with different chains across a generic API

This does have one downside - the typescript and Rust types for the high-level communication wrappers need to be kept in sync. However, This is only needed for these wrappers, not each request/response type. For example - adding new contract message payloads doesn't require any further work (it's all kept in Rust and the wallet is oblivious to the on-the-wire json format). Adding more Terra.JS functionality like Bank Queries and whatnot would only need to be done once.

# Local Development

## Bootstrapping / one-time setup

1. Install required tooling (rust, npm, cargo make)
2. install wasm-opt via [binaryen releases](https://github.com/WebAssembly/binaryen/releases) and put it somewhere on your path
3. copy `.env.sample` to `.env` change whatever values 
4. `npm install`
5. in `frontend/iframe` also `npm install`


## Frontend dev

- `cargo make frontend`

This will start the main app, the iframe app, and the media server in parallel, as well as open a browser tab for each app individually (though the iframe is typically meant to be hidden - recommended to close that tab and just work off the main tab, logs and line numbers show in the console there too)

## Contract building 

- `cargo make contract-registry-release`

Running this with the frontend dev simultaneously will cause a "new system" flow live

Currently, this project isn't setup for contract migrations

## Contract testing 

- `cargo make contract-registry-test`