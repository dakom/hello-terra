# Hello Terra 

# [LIVE DEMO](https://dakom.github.io/hello-terra)
# [SHARED CRATE DOCS](https://dakom.github.io/hello-terra/docs/shared)

# User Experience

* Login via Terra Station chrome extension, mobile app or manual entry
* No other auth or database (state is persisted on-chain, contract addresses are stored locally in-browser)
* Simple bootstrapping for LocalTerra (no need for user to manually compile/select .wasm files) 
* Simple fake bank account (deposit, withdraw, view balance)

# Dev experience

* Playtest all contracts with a frontend (this example only has one - but would be very useful important for Cosmwasm's Actor model!)
* With a debug setting - live reload all 4 frontend parts when source changes (main app, iframe controller, contracts, media)
* Cargo-make commands for composing different build pipelines
* Boilerplate to make the Rust-everywhere pipeline work, just add new message types
* Separate deployment targets, easy-to-find configuration files
* Async Rust wrappers

# Implementation details 

* Wallet providers are abstracted away (e.g. manual mode is internal, not via the official Terra provider, switching is seamless)
* No need or use for JSON schema (real cargo docs - see above)
* Store real Rust types in the contract (frontend has cosmwasm_std too!), so no silly String<>Addr conversion or error-prone number types
* Contracts and frontend are guaranteed to typecheck with eachother _at compile-time_
* Frontend main application is pure Rust/WebAssembly, written in the Dominator framework (declarative, fast, and awesome)
* Detecting and deploying a new contract is done automatically via Blake3 hash, id, and localstorage lookup
* Continuous Integration/Deployment of full app via Github Actions (live demo - see above)

All the heavy lifting is under the hood, so these are possible:

### Example code for querying a contract

```
let summary = ContractQueryMsg::new(QueryMsg::AccountSummary)
    .query::<AccountSummary>()
    .await;
```

### Example code for executing a contract

```
let balance = ContractExecuteMsg::new(ExecuteMsg::Transfer{
        dest: SomeAddr
    })
    .execute::<TransferBalance>()
    .await;
```


### Iframe / Wallet abstraction details

This part requires a bit more heavy lifting

`wallet-provider` depends on React, and `terra.js` doesn't bundle nicely with rollup/wasm

So instead of wrangling this into the core application, it's separated out into its own iframe and message-passing mechanism. The only time the UI needs to be displayed is for QR code.

Having this abstraction does have a benefit, since it allows for new wallet providers or even dealing with different chains across a generic API

This does have one downside - the typescript and Rust types for the high-level communication wrappers need to be kept in sync and only a few API calls are supported at the moment. However, This is only needed for these wrappers, not each message use-case. For example, adding new contract message request/response payloads doesn't require any further work (it's all kept in Rust and the wallet is oblivious to the on-the-wire json format). Adding more Terra.JS functionality like Bank Queries and whatnot would only need to be done once.

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


# Build commands

There's a variety in the Makefile.toml which can be composed in various ways.

`cargo make dev` will get all the watchers and builders going for dev. All the output will be in one terminal.

Here's some of the pieces it uses:

## Frontend dev

- `cargo make frontend-dev`

This will start the main app, the iframe app, and the media server in parallel, as well as open a browser tab for each app individually (though the iframe is typically meant to be hidden - recommended to close that tab and just work off the main tab, logs and line numbers show in the console there too)

## Contract building 

- `cargo make contracts-build-release`

This will compile, optimize, and generate a hash file for the contracts.
The output will be in `./frontend/media/contracts/`

However this is .gitignored for CI

Running this with the frontend dev simultaneously will cause a "new system" flow live

Currently, this project isn't setup for contract migrations

`cargo make contracts-dev` sets up a watcher to run the build-release

## Contract testing 

- `cargo make contracts-test`

# CI Setup

1. replace `dakom` in `.github/workflows/build.yml` with your github username
2. create/add `GH_PAT` to your repo's secrets (it's a github deployment token [for deploying to gh_pages](https://github.com/maxheld83/ghpages/pull/18))