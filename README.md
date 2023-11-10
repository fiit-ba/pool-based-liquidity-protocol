If you want to use this code, please cite our article describing this solution:

**IEEE style** K. Košťál, M. N. Bahar, R. Gazdík and M. Ries, "Advancing Decentralized Finance: A Comprehensive Pool-based Liquidity Protocol," 2023 Fifth International Conference on Blockchain Computing and Applications (BCCA), Kuwait, Kuwait, 2023, pp. 1-8.
# Advancing Decentralized Finance: A Comprehensive Pool-based Liquidity Protocol

## Structure of repository

The embedded implementation and testing solution has the following tree structure:

- /artifacts — folder containing generated artifacts for Redspot compilation
    and testing,
    - btoken_contract.contract — .contract version of BToken,
    - btoken_contract.wasm — .wasm version of BToken,
    - liquidity_pool_manager_contract.contract — .contract version of
          LiquidityPoolManager,
    - liquidity_pool_manager_contract.wasm — .wasm version of LiquidityPoolManager.
- /contracts — OpenBrush library features,
- /project — folder containing the project,
    - /contracts — folder containing the smart contracts,
        - /btoken — folder containing the btoken smart contracts,
        - /liquidity_pool_manager — folder containing the LiquidityPoolManager contract,
        - /loan — folder containing the Loan smart contract,
        - /stablecoin — folder containing the StableCoin smart contract,
        - /mod.rs — file specifying what is the content of current folder.

    - /traits — folder containing traits for smart contracts,
        - /btoken.rs — trait for BToken,
        - /liquidity_pool_manager.rs — trait for LiquidityPoolManager,
        - /loan.rs — trait for Loan,
        - /mod.rs — file specifying what is the content of current folder,
        - /stablecoin.rs — trait for stablecoin.
    - /Cargo.toml — Cargo setup for project,
    - /lib.rs — file specifying project as a whole.
- /tests — test folder,
    - /setup — setup folder for tests
        - /chai.ts — setup file,
        - /hooks.ts — setup file.
    - /helpers.ts — setup file,
    - /test.ts – test scenario file.
- /Cargo.toml – setup for Cargo (to know how to find utils and contracts),
- /lib.rs – defining used folders,
- /package.json – json dependencies,
- /redspot.config.ts – Redspot setup file,
- /tsconfig.json – tsconfig config file.


## Programming manual

We will use the Linux operating system to implement our project, specifically it's Ubuntu 20.04 LTS distribution. To use the ink! programming language, we will need the following prerequisites:

1. Rust
2. WebAssembly binary
3. cargo-contract package

### Rust

Since Substrate (primary Polkadot SDK used) is built with the Rust programming language, the first thing we will need to do is prepare the computer for Rust
development.

1. **Build dependencies** Use a terminal shell to execute the following com-
mands:
    ```SQF
    sudo apt update
    # May prompt for location information
    sudo apt install -y git clang curl libssl-dev llvm libudev-dev pkg-config
    ```
2. **Rust developer environment** We will use rustup tool to manage Rust
toolchain. We need to install and configure rustup:
    ```SQF
    # Install
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    
    # Configure
    source ~/.cargo/env
    ```

Configure the Rust toolchain to default to the latest stable version, add nightly
and the nightly wasm target:

```SQF
rustup default stable
rustup update
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly   
```
To see what Rust toolchain we are presently using, run:
```SQF
rustup show
```

### WebAssembly binary

To install the binaryen package run this command:
```SQF
sudo apt install binaryen
```

### cargo-contract package

The cargo-contract package provides a command-line interface for working with
smart contracts using the ink! language. To install the cargo-contract package run
this command:
```SQF
cargo install cargo-contract --force --locked
```

We can verify installation by running:
```SQF
cargo contract --help
```

After installing 3 prerequisites mentioned mentioned at the beginning of the chap-
ter, we are finally ready to start developing a new smart contract projects with
ink!

### smart contract compilation

The compilation of smart contracts takes place either manually with each single
smart contract folder using (in our case 4 compilations, BToken, StableCoin, Loan
and LendingPoolManager):
```SQF
cargo +nightly contract build --force --locked
```
This action creates a Cargo.lock file and a folder named target (the folder is usually larger than 1GB, so it takes up a lot of disk space). We navigate in the target/ink folder and their compilation created 3 new files, which are the main output of the whole event, namely:

- contract_name.wasm containing compiled smart contract,
- metadata.json containing metadata of smart contract,
- contract_name.contract containing both of them.

An easier way to compile the project is with the Redspot tool, which was also used
for testing. As a prerequisite for the Redspot tool are the Node.js, npm, and Yarn
tools, which we install using:
```SQF
sudo apt install nodejs
npm install -g npm
node -v
npm -v
npm install --global yarn
```
Subsequently, to initialize the entire Redspot tool, you must run the command
from the first folder layer of the project (where redspot.config.ts is located):

```
yarn
```

The Redspot tool compilation can be started using:

```
npx redspot compile
```
This command starts the compiler for all the files found in the path specified in the configuration, and the results of the .wasm, .json, and .contract files are located in the artifacts folder. Using the Redspot tool, it is also possible to run tests using the command:

```
npx redspot test --network development
```

### cargo-contract-node

As already mentioned in the work, the implementation and testing took place
on a local solution called substrate-contract-node, which we will need to run the
program. As a rule, updates of this tool are published quite often, so it is always
best to use the latest version, in our case, it was the version substrate-contracts-node 0.15.0-c27e43e. The installation of this tool is available with the command (without newlines):

```SQF
cargo install contracts-node --git https://github.com/paritytech/substrate-contracts-node.git --force --locked
```
Once the tool is started it runs and simulates the running of the blockchain until
we stop it, and is started by command:

```
substrate-contracts-node --dev
```

## Interaction with Polkadot

As a tool for interacting with smart contracts, we mostly used the polkadot.js.org/apps tool, which is an online web-based tool for interacting with Polkadot blockchains.
We set up a connection to Development / Local Node and connected to it. Smart contracts are uploadable to the blockchain in the Developer / Contracts / Upload & Deploy code option. We choose which accounts
are initiating the constructor and we insert compiled .contract file or .wasm + json
files. In the case of uploading a lending_pool_manager contract, the hashes of the
Loan contract and the BToken contract must also be added, which are available
for copying in the code hashes section after upload. From now on, we can interact
with all recorded smart contracts; in the contracts section, we can open individual
initializations, thus accessing the functions of these contracts.

