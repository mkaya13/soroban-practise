#![cfg(test)]

extern crate alloc;
use alloc::vec::Vec;

// use crate::{StellarAssetContract, StellarAssetContractClient};
use soroban_sdk::{ Env, Bytes, log };
use soroban_sdk::xdr::{AccountId, Asset, AlphaNum4, AssetCode4, PublicKey, Uint256, Limits, WriteXdr};

/* 
​To deploy a Stellar Asset Contract (SAC) using the Soroban SDK, we need to serialize both the asset code and 
the issuer's address into the appropriate XDR format.

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

Asset_Code:Public_Key
T001:GCMPPXWUJGFPFXOMH3LUCBGFBTQMIOI7VDXZH6JTV64JLUYVHQV5M7VC

*/

#[test]
fn serialize_4_bytes_and_12_bytes_asset_codes() {

    let env = Env::default();
    // let contract_id = env.register(SACManagerContract, ());
    // let client = SACManagerContractClient::new(&env, &contract_id);

    pub fn serialize_4_bytes_asset_code(env: &Env) -> Bytes {
        let asset_code = AssetCode4(*b"T001"); // Must be 4 bytes
        // let issuer = AccountId(PublicKey::PublicKeyTypeEd25519(Uint256([0u8; 32]))); --> Generates a public key with 32 zeros.

        let public_key_bytes: [u8; 32] = stellar_strkey::ed25519::PublicKey::from_string(
            "GCMPPXWUJGFPFXOMH3LUCBGFBTQMIOI7VDXZH6JTV64JLUYVHQV5M7VC"  // Your Wallet Account for Admin Tokenization
        )
        .unwrap()
        .0;

        /* The code above:
        It takes a G... Stellar public key (which is base32-encoded, with checksums and type tags),
        Decodes it properly,
        Validates that it's a real Ed25519 public key,
        Extracts the 32 raw bytes inside.
         */

        let issuer = AccountId(PublicKey::PublicKeyTypeEd25519(Uint256(public_key_bytes)));

        let alpha_num4 = AlphaNum4 {
            asset_code,
            issuer,
        };
    
        let asset = Asset::CreditAlphanum4(alpha_num4);
    
        let serialized: Vec<u8> = asset.to_xdr(Limits::none()).expect("serialization failed");
    
        let bytes = Bytes::from_slice(env, &serialized);

        log!(env, "Serialized Asset with 4 Bytes is (hex):", bytes);

        bytes
    }

    serialize_4_bytes_asset_code(&env);  // T001, GCMPPXWUJGFPFXOMH3LUCBGFBTQMIOI7VDXZH6JTV64JLUYVHQV5M7VC


}


