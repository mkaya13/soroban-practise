# What is Soraban

Soraban is the SC platform in the Stellar Network, where we can write our Smart Contracts in the
Rust programming language. Which will get compiled into web assembly for the deployment.

# For Soroban Documentation, Enter the Page Below

https://developers.stellar.org/docs/build/smart-contracts/overview 


# Getting Started

# 1- Hello World

- Create a new project using the init command to create a soroban-hello-world project.
`stellar contract init soroban-hello-world`

- The init command will create a Rust workspace project, using the recommended structure for including Soroban contracts. Let’s take a look at the project structure:
.
├── Cargo.lock
├── Cargo.toml
├── README.md
└── contracts
    ├── hello_world
    │   ├── Cargo.toml
    │   ├── Makefile
    │   ├── src
    │   │   ├── lib.rs
    │   │   └── test.rs

## Cargo.toml

- The Cargo.toml file at the root of the project is set up as Rust Workspace, which allows us to include multiple smart contracts in one project.

## Rust Workspace

- The Cargo.toml file sets the workspace’s members as all contents of the contracts directory and sets the workspace’s soroban-sdk dependency version including the testutils feature, which will allow test utilities to be generated for calling the contract in tests.

## release Profile

- Configuring the release profile to optimize the contract build is critical. Soroban contracts have a maximum size of 64KB. Rust programs, even small ones, without these configurations almost always exceed this size.

## release-with-logs Profile

- Configuring a release-with-logs profile can be useful if you need to build a .wasm file that has logs enabled for printing debug logs when using the stellar-cli. Note that this is not necessary to access debug logs in tests or to use a step-through-debugger.

## Contracts Directory

- The contracts directory is where Soroban contracts will live, each in their own directory. There is already a hello_world contract in there to get you started.

### Contract-specific Cargo.toml file

- Each contract should have its own Cargo.toml file, which relies on the top-level Cargo.toml that we just discussed.

- This is where we can specify contract-specific package information.

```
[package]
name = "hello-world"
version = "0.0.0"
edition = "2021"
publish = false
```

The crate-type is configured to cdylib which is required for building contracts.

```
[lib]
crate-type = ["cdylib"]
doctest = false
```

We also have included the soroban-sdk dependency, configured to use the version from the workspace Cargo.toml.

```
[dependencies]
soroban-sdk = { workspace = true }

[dev-dependencies]
soroban-sdk = { workspace = true, features = ["testutils"] }
```

## Contract Source Code

Creating a Soroban contract involves writing Rust code in the project’s lib.rs file.

All contracts should begin with #![no_std] to ensure that the Rust standard library is not included in the build. The Rust standard library is large and not well suited to being deployed into small programs like those deployed to blockchains.

`#![no_std]`

The contract imports the types and macros that it needs from the soroban-sdk crate.

`use soroban_sdk::{contract, contractimpl, vec, Env, String, Vec};`

Many of the types available in typical Rust programs, such as std::vec::Vec, are not available, as there is no allocator and no heap memory in Soroban contracts. The soroban-sdk provides a variety of types like Vec, Map, Bytes, BytesN, Symbol, that all utilize the Soroban environment's memory and native capabilities. Primitive values like u128, i128, u64, i64, u32, i32, and bool can also be used. Floats and floating point math are not supported.

Contract inputs must not be references.

The #[contract] attribute designates the Contract struct as the type to which contract functions are associated. This implies that the struct will have contract functions implemented for it.

```
#[contract]
pub struct Contract;
```

Contract functions are defined within an impl block for the struct, which is annotated with #[contractimpl]. It is important to note that contract functions should have names with a maximum length of 32 characters. Additionally, if a function is intended to be invoked from outside the contract, it should be marked with the pub visibility modifier. It is common for the first argument of a contract function to be of type Env, allowing access to a copy of the Soroban environment, which is typically necessary for various operations within the contract.

```
#[contractimpl]
impl Contract {
    pub fn hello(env: Env, to: String) -> Vec<String> {
        vec![&env, String::from_str(&env, "Hello"), to]
    }
}
```

Putting those pieces together a simple contract looks like this.

```
#![no_std]
use soroban_sdk::{contract, contractimpl, vec, Env, String, Vec};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn hello(env: Env, to: String) -> Vec<String> {
        vec![&env, String::from_str(&env, "Hello"), to]
    }
}

mod test;
```

## Build the contract

To build a smart contract to deploy or run, use the `stellar contract build` command.
`stellar contract build`

This is a small wrapper around cargo build that sets the target to wasm32-unknown-unknown and the profile to release. You can think of it as a shortcut for the following command:

`cargo build --target wasm32-unknown-unknown --release`

## Optimizing Builds

Use stellar contract optimize to further minimize the size of the .wasm. First, re-install stellar-cli with the opt feature:

`cargo install --locked stellar-cli --features opt`

Then build an optimized .wasm file:

`stellar contract optimize --wasm target/wasm32-unknown-unknown/release/hello_world.wasm`

This will optimize and output a new hello_world.optimized.wasm file in the same location as the input .wasm.

# 2. Deploy to Testnet

## Generate a Keypair for 'alice' (if not already done):

1. If you haven't created the alice identity yet, generate it using the Stellar CLI:
`stellar keys generate alice`

2. Fund the 'alice' Account on the Testnet:
Stellar Testnet accounts need to be funded to become active. You can use the Friendbot service to fund the account. First, retrieve the public key for alice:
`stellar keys address alice`

3. Fund the "alice" testnet Account:
`curl "https://friendbot.stellar.org/?addr=GCMPPXWUJGFPFXOMH3LUCBGFBTQMIOI7VDXZH6JTV64JLUYVHQV5M7VC"`


## Deploy
To deploy your HelloWorld contract, run the following command:

```
stellar contract deploy `
  --wasm target/wasm32-unknown-unknown/release/hello_world.optimized.wasm `
  --source alice `
  --network testnet `
  --alias hello_world
```

### Example Deployed Contract Address: 
CBGCX4UFHIM7VYTAUNXT7F6BCEOLD5UVSSFHQBRVA324YGHO7D4QOC2A

### Blockchain Explorer:
https://stellar.expert/explorer/testnet/contract/CBGCX4UFHIM7VYTAUNXT7F6BCEOLD5UVSSFHQBRVA324YGHO7D4QOC2A

## Interact
Using the code we wrote in Write a Contract and the resulting .wasm file we built in Build, run the following command to invoke the hello function.

```
stellar contract invoke `
  --id CBGCX4UFHIM7VYTAUNXT7F6BCEOLD5UVSSFHQBRVA324YGHO7D4QOC2A `
  --source alice `
  --network testnet `
  -- `
  hello `
  --to RPC
```

```
stellar contract invoke --id CBGCX4UFHIM7VYTAUNXT7F6BCEOLD5UVSSFHQBRVA324YGHO7D4QOC2A --source alice --network testnet -- hello --to RPC
```

# 3. Storing Data

Create a new contract called "increment"

```
stellar contract init . --name increment
```

