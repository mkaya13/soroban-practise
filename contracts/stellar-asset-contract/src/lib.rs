#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Bytes, Address};

#[contract]
pub struct StellarAssetContract;

#[contractimpl]
impl StellarAssetContract {
    
    pub fn deploy_sac(env: Env, serialized_asset: Bytes) -> Address {
        // Create the Deployer with Asset
        let deployer = env.deployer().with_stellar_asset(serialized_asset);
        let _ = deployer.deployed_address();
        // Deploy the Stellar Asset Contract
        let sac_address = deployer.deploy();

        sac_address
    }
}

mod test;
