# Hello Terra 

~~# [LIVE APP](http://example.com) / [CRATE DOCS](http://example.com)~~

# GOALS (not there yet!)

## User Experience
* Login with wallet from anywhere
* Reactive to live changes on the network 
* Tool to upload new contracts
* Simple fake bank account (deposit, withdraw, view balance)

## Technical
* Shared native Rust types across the stack for catching errors at compile-time
* Frontend main application is pure Rust/WebAssembly, written in the Dominator framework
* Wallet is handled via a (usually) hidden iframe and a postMessage communication layer
* No backend required, the `user wallet` <-> `user contract id` mapping is stored on the chain via a pre-instantiated separate contract
* Documentation of the shared data structures is generated automatically and browseable online
* Continuous Integration/Deployment of frontend via Github Actions
* Good DX (live-reloads when sources change, cross-platform, simple commands, separate local vs. release settings, workspace, etc.)
* Uses Nightly as future-proofing for unstable features (GATs, const generics, existential types, etc.)
* Reduce finicky dependencies where possible

# Local Development

## Bootstrapping / one-time setup

1. Install required tooling (rust, npm, cargo make)
2. install wasm-opt via [binaryen releases](https://github.com/WebAssembly/binaryen/releases) and put it somewhere on your path
3. copy `.env.sample` to `.env` change whatever values 
4. `npm install`
5. in `frontend/iframe` also `npm install`


## Frontend dev

- `cargo make frontend`

This will start both the main app and the iframe in parallel, as well as open a browser tab for each individually (though the iframe is typically meant to be hidden)

## Contract building 

- `cargo make contract-registry-release`
- `cargo make contract-account-release`

Then grab the contract from `dist/contracts/*`, and upload it via the web tool

## Contract testing 

- `cargo make contract-registry-test`
- `cargo make contract-account-test`