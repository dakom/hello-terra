# Hello Terra 

# [LIVE DEMO](https://dakom.github.io/hello-terra)
# [SHARED CRATE DOCS](https://dakom.github.io/hello-terra/docs/shared)

# Video walkthrough
[![Video walkthrough](https://img.youtube.com/vi/UlW1-DnXXes/0.jpg)](https://www.youtube.com/watch?v=UlW1-DnXXes)

# Status

* Working totally fine in LocalTerra
* Bombay only works with Manual wallet mode (don't worry - I'm not stealing your wallet, you can compile the source here and compare the hash to be sure - but I'd still suggest creating a separate wallet for playing around with anyway!)

# Dev experience
* Contracts and frontend are guaranteed to typecheck with eachother _at compile-time_ due to sharing the Rust types in a common crate. (right now the request/response are separately defined, but it would be straightforward to unify them too)
* Use third-party native Rust types like [Decimal](https://docs.rs/cosmwasm-std/latest/cosmwasm_std/struct.Decimal.html) everywhere - prevent (de)serialization errors, floating point errors, and straight up human errors by never worrying about them in the first place! String<>Addr and whatnot conversions be ye gone!
* Simply call `ContractExecuteMsg(msg).execute().await` or `ContractQueryMsg(msg).query().await` on the _actual_ Rust structs defined in the [shared crate](shared). Not only does (de)serialization work as expected (without having to look at a schema!), these return proper Rust futures which can be used in the full Rust async/await ecosystem (note: cancelling won't abort the low-level RPC/XHR/Promises since we're at the mercy of the Terra.JS API for that, and that dependency layer doesn't support cancellation)
* Since we don't need a schema, we don't generate one. Use Cargo Docs instead (see above). Though a schema could be generated too for outside projects to interface with our contracts.
* Playtest all contracts with a frontend and bootstrapping mechanism when contracts are re-compiled. It's not as fast as unit tests - so don't use it for that. Instead, compare against manually compiling, uploading, and instantiating contracts on change. Compared to that, this automated approach is very fast when it comes time to properly integrate - especially with multiple interdependent contracts
* Cargo-make commands for composing different build pipelines, live reloading, etc. etc.
* Simple interface to the wallet-provider bridge. New types don't need to be added often since they are just the wrappers, but when they do it's easy. 
* Separate deployment targets, easy-to-find configuration files
* Ci all setup properly

# User Experience

* Login via Terra Station chrome extension, mobile app or manual entry (note: mobile is untested and may require more configuration - switching networks live currently [doesn't react](https://github.com/terra-money/wallet-provider/issues/25))
* No other auth or database (state is persisted on-chain, hub address is stored locally in-browser or hardcoded, account addresses are instantiated from the hub)
* Running against LocalTerra? Just hit the "bootstrap" button. Want to start fresh again? Just wipe your LocalStorage.
* Simple fake bank account (deposit, withdraw, view balance) and fake bank (info of total deposits across accounts)


# Implementation details 

* Wallet providers are abstracted away (e.g. manual mode is internal, not via the official Terra provider, user can't tell the difference)
* Frontend main application is pure Rust/WebAssembly, written in the [Dominator](https://github.com/Pauan/rust-dominator) framework (declarative, fast, and awesome)
* Detecting and deploying a new contract is done automatically via Blake3 hash, id, and localstorage lookup - this all happens internally via application/build logic, prior to connecting to the chain (i.e. it's not a replacement for solutions like etherscan etc.)


### Example code for querying a contract

```
if let Ok(summary) = ContractQueryMsg::new(QueryMsg::AccountSummary)
    .query::<AccountSummary>()
    .await { ... }

```

### Example code for executing a contract

```
if let Ok(balance) = ContractExecuteMsg::new(
    ExecuteMsg::Transfer{
        dest: SomeAddr
    })
    .execute::<TransferBalance>()
    .await { ... }
```


### Iframe / Wallet abstraction details

`wallet-provider` depends on React, and `terra.js` doesn't bundle nicely with rollup/wasm

So instead of wrangling this into the core application, it's separated out into its own iframe and message-passing mechanism. The only time the UI needs to be displayed is for QR code.

Having this abstraction does have a benefit, since it allows for new wallet providers or even dealing with different chains across a generic API

This does have one downside though - the typescript and Rust types for the high-level communication wrappers need to be kept in sync and only a few API calls are supported at the moment. However, This is only needed for these wrappers, not each message use-case. For example, adding new contract message request/response payloads doesn't require any further work (it's all kept in Rust and the wallet is oblivious to the on-the-wire json format). Adding more Terra.JS functionality like Bank Queries and whatnot would only need to be done once.

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

# Testing

## Contract testing 

- `cargo make contracts-test` (one-off)
- `cargo make contracts-test-watch` (with a watcher for file changes)

# CI Setup

1. replace `dakom` in `.github/workflows/build.yml` with your github username
2. create/add `GH_PAT` to your repo's secrets (it's a github deployment token [for deploying to gh_pages](https://github.com/maxheld83/ghpages/pull/18))