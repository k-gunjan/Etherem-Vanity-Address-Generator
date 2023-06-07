pub mod eth_wallet {

    use ethers::core::rand::thread_rng;
    use tiny_keccak::keccak256;
    use web3::types::Address;

    use secp256k1::{PublicKey, Secp256k1, SecretKey};
    #[derive(Debug)]
    pub struct Wallet {
        pub secret_key: String,
        pub public_key: String,
        pub address: String,
    }

    impl Wallet {
        pub fn new(secret_key: &SecretKey, public_key: &PublicKey) -> Self {
            let addr: Address = public_key_address(&public_key);
            Wallet {
                secret_key: hex::encode(&secret_key.secret_bytes()),
                public_key: public_key.to_string(),
                address: hex::encode(addr),
            }
        }
    }

    // fn generate_wallet() -> LocalWallet {
    //     let signer = LocalWallet::new(&mut thread_rng());
    //     // println!("address is:{:?}", signer.address())
    //     // address is:0xaab4e228cb5a8a2a4d88fa6603ef05f923edee93
    //     signer
    // }

    pub fn generate_random_wallet() -> Wallet {
        let secp = Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(&mut thread_rng());
        Wallet::new(&secret_key, &public_key)
    }

    pub fn public_key_address(public_key: &PublicKey) -> Address {
        let public_key = public_key.serialize_uncompressed();
        debug_assert_eq!(public_key[0], 0x04);
        let hash = keccak256(&public_key[1..]);
        Address::from_slice(&hash[12..])
    }
}
