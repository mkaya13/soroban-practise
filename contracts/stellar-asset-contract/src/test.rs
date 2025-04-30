#![cfg(test)]

extern crate alloc;
use alloc::vec::Vec;

// use crate::{StellarAssetContract, StellarAssetContractClient};
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
    // let contract_id = env.register(StellarAssetContract, ());
    // let client = StellarAssetContractClient::new(&env, &contract_id);

    pub fn create_serialized_asset(env: &Env) -> Bytes {
        let asset_code = AssetCode4(*b"DAM1"); // Must be 4 bytes
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


