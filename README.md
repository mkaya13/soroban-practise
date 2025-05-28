# Soraban

Soraban is the SC platform in the Stellar Network, where we can write our Smart Contracts in the
Rust programming language. Which will get compiled into web assembly for the deployment.

# Getting Started with Projects

Rust Workspace Project
.
├── .stellar
├── contracts
├── target
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── README.md

## README.md

Provides an overview of the project, usage instructions, and relevant documentation for developers.

## LICENSE

Specifies the legal terms under which the project’s code can be used, modified, and distributed.

## Cargo.toml

- The Cargo.toml file at the root of the project is set up as Rust Workspace, which allows us to include multiple smart contracts in one project.

- The Cargo.toml file sets the workspace’s members as all contents of the contracts directory and sets the workspace’s soroban-sdk dependency version including the testutils feature, which will allow test utilities to be generated for calling the contract in tests.

- Make sure that members are designed as below: 

```
members = [
  "contracts/*",
]
```

## Cargo.lock

Records exact versions of dependencies used in the project to ensure reproducible builds.

## .gitignore

Lists files and directories that Git should ignore, such as build artifacts and sensitive data.

## contracts

- Each contract should have its own Cargo.toml file, which relies on the top-level Cargo.toml that we just discussed.

## stellar

- Holds the smart contract credentials deployed to Stellar network.

## target

- Configuring the target/release profile to optimize the contract build is critical. Soroban contracts have a maximum size of 64KB. Rust programs, even small ones, without these configurations almost always exceed this size.

# 1- hello-world-contract

- `hello-world-contract/Cargo.toml` This is where we can specify contract-specific package information.

```
[package]
name = "hello-world-contract"
version = "0.0.0"
edition = "2021"
publish = false
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

- Stellar smart contracts are written in Rust (or supported languages) and must be compiled into Wasm bytecode, the only format that the Soroban runtime on the Stellar blockchain can execute.

To build a smart contract to deploy or run, use the command below in the current smart contract folder:

`cargo build --target wasm32-unknown-unknown --release`

or 

`stellar contract build`

To build all the smart contracts:

```
cargo build --workspace --target wasm32-unknown-unknown --release
```

## Optimizing Builds

Use stellar contract optimize to further minimize the size of the .wasm. First, re-install stellar-cli with the opt feature:

`cargo install --locked stellar-cli --features opt`  // Need to be done once

Then build an optimized .wasm file to make sure that wasm file is optimized:

`stellar contract optimize --wasm target/wasm32-unknown-unknown/release/hello_world_contract.wasm` (From project root)

or 

`stellar contract optimize --wasm ../../target/wasm32-unknown-unknown/release/hello_world_contract.wasm`

This will optimize and output a new hello_world_contract.optimized.wasm file in the same location as the input .wasm.

## Deploy to Testnet

- Before deploying to testnet, make sure that you have an admin wallet account.

## Generate a Keypair for 'alice' (if not already done):

1. If you haven't created the alice identity yet, generate it using the Stellar CLI:
`stellar keys generate alice`

2. Fund the 'alice' Account on the Testnet:
Stellar Testnet accounts need to be funded to become active. You can use the Friendbot service to fund the account. First, retrieve the public key for alice:
`stellar keys address alice`

3. If you need more fund, the you can use the following link:
`curl "https://friendbot.stellar.org/?addr=GCMPPXWUJGFPFXOMH3LUCBGFBTQMIOI7VDXZH6JTV64JLUYVHQV5M7VC"`

## Deploy
To deploy your HelloWorld contract, run the following command:

- From root folder:
```
stellar contract deploy --wasm target/wasm32-unknown-unknown/release/hello_world_contract.optimized.wasm --source alice --network testnet --alias hello_world_contract
```

🌎 Submitting deploy transaction…
🔗 https://stellar.expert/explorer/testnet/contract/CCF3UWFR5FG7RKVKRX4EDSFEFCTJSYNZNZLYBI5SKCQQOBEXC2LYI3IR
✅ Deployed!
⚠️  Overwriting existing contract id: CDKROEKYO62P3VV5FYMGBSIZ43EXRFXSVXYXXVU52FZDFCBVWD6UVRXT
CCF3UWFR5FG7RKVKRX4EDSFEFCTJSYNZNZLYBI5SKCQQOBEXC2LYI3IR

### Blockchain Explorer:
https://stellar.expert/explorer/testnet/contract/CCF3UWFR5FG7RKVKRX4EDSFEFCTJSYNZNZLYBI5SKCQQOBEXC2LYI3IR

## Interact
- To interact with the deployed smart contracts, use invoke keyword.

```
stellar contract invoke --id CCF3UWFR5FG7RKVKRX4EDSFEFCTJSYNZNZLYBI5SKCQQOBEXC2LYI3IR --source alice --network testnet -- hello --to WORLD
```

```
stellar contract invoke --id CCF3UWFR5FG7RKVKRX4EDSFEFCTJSYNZNZLYBI5SKCQQOBEXC2LYI3IR --source alice --network testnet -- hello --to Mert
```

# 2. calculator-contract

- Create a new contract called "calculator-contract"

```
stellar contract build  
```

```
stellar contract optimize --wasm target/wasm32-unknown-unknown/release/calculator_contract.wasm
```

```
stellar contract deploy --wasm target/wasm32-unknown-unknown/release/calculator_contract.optimized.wasm --source alice --network testnet --alias calculator
```

```
ℹ️  Signing transaction: 62cae26553a188cacfe21c45a07ecb2b6331f92d9962d06919ad13b547451ceb
🌎 Submitting deploy transaction…
🔗 https://stellar.expert/explorer/testnet/contract/CC3MIF4X76RSEVJVM6YAIOMKTSMOD7FBZ6MPD5FMIZL74QQAPM32CD4H
✅ Deployed!
CC3MIF4X76RSEVJVM6YAIOMKTSMOD7FBZ6MPD5FMIZL74QQAPM32CD4H
```

```
https://stellar.expert/explorer/testnet/contract/CC3MIF4X76RSEVJVM6YAIOMKTSMOD7FBZ6MPD5FMIZL74QQAPM32CD4H
```

## Invoke the increment-contract

```
stellar contract invoke --id CC3MIF4X76RSEVJVM6YAIOMKTSMOD7FBZ6MPD5FMIZL74QQAPM32CD4H --source alice --network testnet -- increment
```

```
stellar contract invoke --id CC3MIF4X76RSEVJVM6YAIOMKTSMOD7FBZ6MPD5FMIZL74QQAPM32CD4H --source alice --network testnet -- double
```

```
stellar contract invoke --id CC3MIF4X76RSEVJVM6YAIOMKTSMOD7FBZ6MPD5FMIZL74QQAPM32CD4H --source alice --network testnet -- get_current_value
```

```
stellar contract invoke --id CC3MIF4X76RSEVJVM6YAIOMKTSMOD7FBZ6MPD5FMIZL74QQAPM32CD4H --source alice --network testnet -- decrement
```

## Tests

- Run the code below from respective contract's folder:

- (Go to contract folder)

```
cargo test
```

- If you want to see the output of the testing, run the following code:
```
cargo test -- --nocapture
```

# 3 simple-string-contract

´- Same build and deployment steps above.

```stellar contract build```
```stellar contract optimize --wasm ../../target/wasm32-unknown-unknown/release/simple_string_contract.wasm```
```stellar contract deploy --wasm target/wasm32-unknown-unknown/release/simple_string_contract.optimized.wasm --source alice --network testnet```

```
🔗 https://stellar.expert/explorer/testnet/tx/93c493d6b07cde658223cdae0b128ce889fdf0e41a8af667fd605922b5602c98
ℹ️  Signing transaction: 93c493d6b07cde658223cdae0b128ce889fdf0e41a8af667fd605922b5602c98
🌎 Submitting deploy transaction…
🔗 https://stellar.expert/explorer/testnet/contract/CD6U76EFD773RJ7WDHWBQQC5IXSBYOWGEB5GLNKZOCWT3VLRRW5I4ZLC
✅ Deployed!
CD6U76EFD773RJ7WDHWBQQC5IXSBYOWGEB5GLNKZOCWT3VLRRW5I4ZLC
```

## Invoke the Contract

```
stellar contract invoke --id CD6U76EFD773RJ7WDHWBQQC5IXSBYOWGEB5GLNKZOCWT3VLRRW5I4ZLC --source alice --network testnet -- set_user_name --value PatikaDev
```

```stellar contract invoke --id CD6U76EFD773RJ7WDHWBQQC5IXSBYOWGEB5GLNKZOCWT3VLRRW5I4ZLC --source alice --network testnet -- get_user_name```

# 4 accounting-contract

´- Same build and deployment steps above.

For Struct Documentation --> https://developers.stellar.org/docs/learn/encyclopedia/contract-development/types/custom-types

# 5- Stellar Asset Contract

We will be deploying the stellar asset contract, and in order to tokenize assets, we will be invoking the deploy_sac function with it's corresponding parameter. For more info, please take a look at the documentation below:
https://developers.stellar.org/docs/build/guides/tokens/deploying-a-sac

## Build, Optimize and Deploy

```stellar contract build```
```stellar contract optimize --wasm ../../target/wasm32-unknown-unknown/release/stellar_asset_contract.wasm```
```stellar contract deploy --wasm ../../target/wasm32-unknown-unknown/release/stellar_asset_contract.optimized.wasm --source alice --network testnet```

- Output:
```
ℹ️  Signing transaction: 63206ab1dfef37d82d0788103b8ddc2261f2a2dfc5d5a86bd5c5914ee6e90e21
🌎 Submitting deploy transaction…
🔗 https://stellar.expert/explorer/testnet/contract/CCNNWBPS5QI25Q26EZ7AVE2NYKAGAMDUB2YJKO7Z66ALHZVZ3KEW2GAN
✅ Deployed!
CCNNWBPS5QI25Q26EZ7AVE2NYKAGAMDUB2YJKO7Z66ALHZVZ3KEW2GAN
```

## Run Test Script to Obtain Tokenized Asset

```
cargo test -- --nocapture
```

- Make sure to add `stellar-strkey = "0.0.7"` on Cargo.toml dependencies.

- serialized_asset parameter must be either AlphaNum4 or AlphaNum12.

```
pub enum Asset {
    Native,
    CreditAlphanum4(AlphaNum4),
    CreditAlphanum12(AlphaNum12),
}

pub struct AlphaNum4 {
    pub asset_code: AssetCode4,
    pub issuer: AccountId,
}

pub struct AlphaNum12 {
    pub asset_code: AssetCode12,
    pub issuer: AccountId,
}
```

```
pub struct AlphaNum4 {
    pub asset_code: AssetCode4,
    pub issuer: AccountId,
}

Asset_Code:Public_Key
T001:GCMPPXWUJGFPFXOMH3LUCBGFBTQMIOI7VDXZH6JTV64JLUYVHQV5M7VC
```

- serialized_asset is the Stellar Asset XDR serialized to bytes. Refer to [soroban_sdk::xdr::Asset] XDR

## Invoke deploy_sac

stellar contract invoke --id CCNNWBPS5QI25Q26EZ7AVE2NYKAGAMDUB2YJKO7Z66ALHZVZ3KEW2GAN --source alice --network testnet -- deploy_sac --serialized_asset 00000001543030310000000098f7ded4498af2ddcc3ed74104c50ce0c4391fa8ef93f933afb895d3153c2bd6

```
ℹ️  Signing transaction: bddca040a53a03a46d16a22bac3a9d1158c52c233b8053bf8a9e7991fba4565e
"CB5MSFMO5TZLX3SL743KARE7VW5LTNLBXMCFFALAKPXEMKH5LNHZ7KV3"
```

```
🔗 https://stellar.expert/explorer/testnet/contract/CB5MSFMO5TZLX3SL743KARE7VW5LTNLBXMCFFALAKPXEMKH5LNHZ7KV3
```

# 6- Crowdfunding Contract

```
stellar contract deploy --wasm .\target\wasm32-unknown-unknown\release\crowdfunding_contract.optimized.wasm --source alice --network testnet --alias crowdfunding-contract
```

- 🔗 https://stellar.expert/explorer/testnet/contract/CDBJLX5EKCYSEYUKLQ32K37LFSWBLYGDHPFOLEHZAARFC55L5ON24UWN
- ✅ Deployed!
- ⚠️  Overwriting existing contract id: CBXLTLTL37IJNKZUNC2UQY3ZEVRTLV56NWVIRJO3VELBYU3X6UD2S5BS
- CDBJLX5EKCYSEYUKLQ32K37LFSWBLYGDHPFOLEHZAARFC55L5ON24UWN

## Invoke the Initialize Function - 

```
stellar contract invoke --id CDBJLX5EKCYSEYUKLQ32K37LFSWBLYGDHPFOLEHZAARFC55L5ON24UWN --source alice --network testnet -- initialize --recipient GCMPPXWUJGFPFXOMH3LUCBGFBTQMIOI7VDXZH6JTV64JLUYVHQV5M7VC --token CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC
```

## Invoke the Recipient Function - Read

```
stellar contract invoke --id CDBJLX5EKCYSEYUKLQ32K37LFSWBLYGDHPFOLEHZAARFC55L5ON24UWN --source alice --network testnet -- recipient
```

## Invoke the Token Function - Read

```
stellar contract invoke --id CDBJLX5EKCYSEYUKLQ32K37LFSWBLYGDHPFOLEHZAARFC55L5ON24UWN --source alice --network testnet -- token
``` 

## Invoke the Donate Function

```
stellar contract invoke --id CDBJLX5EKCYSEYUKLQ32K37LFSWBLYGDHPFOLEHZAARFC55L5ON24UWN --source alice --network testnet -- donate --donor GCMPPXWUJGFPFXOMH3LUCBGFBTQMIOI7VDXZH6JTV64JLUYVHQV5M7VC --amount 130000000
```
