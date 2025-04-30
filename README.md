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


```
stellar contract build  
```

```
ls target/wasm32-unknown-unknown/release/*.wasm
```

```
stellar contract optimize --wasm target/wasm32-unknown-unknown/release/increment.wasm
```

```
stellar contract deploy --wasm target/wasm32-unknown-unknown/release/increment.wasm --source alice --network testnet --alias increment
```

https://stellar.expert/explorer/testnet/contract/CDZRJLICDQA2H7NROUFX6PYOYW53IQ433SA7CSN6UO6B54DN6VPTHZQR
CDZRJLICDQA2H7NROUFX6PYOYW53IQ433SA7CSN6UO6B54DN6VPTHZQ

## Tests

Replace the placeholder code in contracts/increment/src/test.rs with the following increment test code.

```
#![cfg(test)]
use crate::{IncrementContract, IncrementContractClient};
use soroban_sdk::Env;

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register(IncrementContract, ());
    let client = IncrementContractClient::new(&env, &contract_id);

    assert_eq!(client.increment(), 1);
    assert_eq!(client.increment(), 2);
    assert_eq!(client.increment(), 3);
}
```

Run the code below:
```
cargo test
```

If you want to see the output of the testing, run the following code:

```
cargo test -- --nocapture
```

## Take it further

Can you figure out how to add get_current_value function to the contract? What about decrement or reset functions?


# 3 Simple String Contract

To build all the smart contracts, run:

```
cargo build --workspace --target wasm32-unknown-unknown --release
```

```
stellar contract optimize --wasm target/wasm32-unknown-unknown/release/simple_string_contract.wasm
```

# 4 Accounting Contract

Struct --> https://developers.stellar.org/docs/learn/encyclopedia/contract-development/types/custom-types


# Simple String Contract

## Deploy
stellar contract deploy --wasm target/wasm32-unknown-unknown/release/simple_string_contract.wasm --source alice --network testnet --alias simple-string-contract

🔗 https://stellar.expert/explorer/testnet/contract/CB2CMITJWD2D4R7Q4KASOLGS6225ZEIESFZURJBK6U2N25ARVHLFXVZP
✅ Deployed!
CAZNVFCZ2KFSAKH67H4FBIYKZNO572VFLJORKZOLXRGTLKTTO547SWVE

## Invoke set_user_name
stellar contract invoke --id CAZNVFCZ2KFSAKH67H4FBIYKZNO572VFLJORKZOLXRGTLKTTO547SWVE --source alice --network testnet -- get_user_name

## Invoke get_user_name with Mert function parameter
stellar contract invoke --id CAZNVFCZ2KFSAKH67H4FBIYKZNO572VFLJORKZOLXRGTLKTTO547SWVE --source alice --network testnet -- set_user_name --value Mert

# Stellar Asset Contract
https://developers.stellar.org/docs/build/guides/tokens/deploying-a-sac

## Deploy
stellar contract deploy --wasm target/wasm32-unknown-unknown/release/simple_string_contract.wasm --source alice --network testnet --alias simple-string-contract

🔗 https://stellar.expert/explorer/testnet/contract/CCJF5DJLXRQB44BRYLZJNUSHZMOM3GVNJQDP5M7DW2IEL6HRQQWE57GL
✅ Deployed!
CCJF5DJLXRQB44BRYLZJNUSHZMOM3GVNJQDP5M7DW2IEL6HRQQWE57GL

## Invoke deploy_sac

stellar contract invoke --id CCJF5DJLXRQB44BRYLZJNUSHZMOM3GVNJQDP5M7DW2IEL6HRQQWE57GL --source alice --network testnet -- deploy_sac --serialized_asset 0000000144414d31000000000000000000000000000000000000000000000000000000000000000000000000

serialized_asset is the Stellar Asset XDR serialized to bytes. Refer to [soroban_sdk::xdr::Asset] XDR

```
#![cfg(test)]

extern crate alloc;
use alloc::vec::Vec;

use crate::{StellarAssetContract, StellarAssetContractClient};
use soroban_sdk::{Bytes, Env, log};
use soroban_sdk::xdr::{AccountId, Asset, AlphaNum4, AssetCode4, PublicKey, Uint256, Limits, WriteXdr};

/* 
​To deploy a Stellar Asset Contract (SAC) using the Soroban SDK, you need to serialize both the asset code and 
the issuer's address into the appropriate XDR format. This serialized asset is then passed to the with_stellar_asset 
method for deployment.
*/

#[test]
fn test_stellar_asset_contract_1() {
    let env = Env::default();
    let contract_id = env.register(StellarAssetContract, ());
    let client = StellarAssetContractClient::new(&env, &contract_id);

    pub fn create_serialized_asset(env: &Env) -> Bytes {
        let asset_code = AssetCode4(*b"USDC"); // Must be 4 bytes
        let issuer = AccountId(PublicKey::PublicKeyTypeEd25519(Uint256([0u8; 32])));
    
        let alpha_num4 = AlphaNum4 {
            asset_code,
            issuer,
        };
    
        let asset = Asset::CreditAlphanum4(alpha_num4);
    
        let serialized: Vec<u8> = asset.to_xdr(Limits::none()).expect("serialization failed");
    
        let bytes = Bytes::from_slice(env, &serialized);

        log!(env, "Serialized Asset Bytes (hex):", bytes);

        bytes
    }

    create_serialized_asset(&env);

}
```

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

pub struct AssetCode4(pub [u8; 4]);
pub fn as_slice(&self) -> &[u8]

pub struct AlphaNum12 {
    pub asset_code: AssetCode12,
    pub issuer: AccountId,
}

```

# Crowdfunding Contract

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
