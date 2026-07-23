mod util;
pub mod eth_wallet {

    use ethers::core::rand::RngCore;
    use tiny_keccak::keccak256;
    use web3::types::Address;

    use secp256k1::{PublicKey, Secp256k1, SecretKey};

    #[derive(Debug)]
    pub struct Wallet {
        pub secret_key: SecretKey,
        pub public_key: PublicKey,
        pub address: Address,
    }

    impl Wallet {
        pub fn new(secret_key: SecretKey, public_key: PublicKey) -> Self {
            let addr: Address = public_key_address(&public_key);
            Wallet {
                secret_key: secret_key,
                public_key: public_key,
                address: addr,
            }
        }
    }

    pub fn generate_random_wallet<R: RngCore>(rng: &mut R) -> Wallet {
        let secp = Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(rng);
        Wallet::new(secret_key, public_key)
    }

    pub fn public_key_address(public_key: &PublicKey) -> Address {
        let public_key = public_key.serialize_uncompressed();
        debug_assert_eq!(public_key[0], 0x04);
        let hash = keccak256(&public_key[1..]);
        Address::from_slice(&hash[12..])
    }
}

pub mod generator {

    use super::eth_wallet::generate_random_wallet;
    use super::util::{cli_display, is_hex_string};
    use ethers::core::rand::RngCore;

    use std::env;
    pub struct Choice {
        start_string: Option<String>,
        ends_string: Option<String>,
        anywhere_string: Option<String>,
    }

    impl Choice {
        pub fn new() -> Self {
            Self {
                start_string: None,
                ends_string: None,
                anywhere_string: None,
            }
        }
        pub fn set_start_string(&mut self, s: &str) {
            self.start_string = if s.len() > 0 {
                Some(s.to_string())
            } else {
                None
            }
        }
        pub fn set_emds_string(&mut self, s: &str) {
            self.ends_string = if s.len() > 0 {
                Some(s.to_string())
            } else {
                None
            }
        }
        pub fn set_middle_string(&mut self, s: &str) {
            self.anywhere_string = if s.len() > 0 {
                Some(s.to_string())
            } else {
                None
            }
        }
        pub fn collect_args() -> Result<Self, String> {
            let mut choice = Choice::new();

            // Retrieve the command line arguments
            let args: Vec<String> = env::args().collect();

            // Iterate over the arguments
            let mut iter = args.iter().skip(1); // Skip the first argument (program name)

            while let Some(arg) = iter.next() {
                match arg.as_str() {
                    "--start-string" => {
                        if let Some(value) = iter.next() {
                            if is_hex_string(value) {
                                choice.set_start_string(value);
                            }
                        }
                    }
                    "--end-string" => {
                        if let Some(value) = iter.next() {
                            if is_hex_string(value) {
                                choice.set_emds_string(value);
                            }
                        }
                    }
                    "--middle-string" => {
                        if let Some(value) = iter.next() {
                            if is_hex_string(value) {
                                choice.set_middle_string(value);
                            }
                        }
                    }
                    _ => {
                        // Handle unrecognized arguments
                        // println!("Unrecognized argument: {}", arg);
                    }
                }
            }
            // check at least one choice made
            if choice.start_string.is_some()
                || choice.ends_string.is_some()
                || choice.anywhere_string.is_some()
            {
                Ok(choice)
            } else {
                return Err("Invalid choices".to_string());
            }
        }

        fn check_pattern(&self, address: &str) -> bool {
            let mut result = true;
            if let Some(start_string) = &self.start_string {
                if result && address.starts_with(start_string) {
                    result = true
                } else {
                    result = false
                }
            }
            if let Some(ends_string) = &self.ends_string {
                if result && address.ends_with(ends_string) {
                    result = true
                } else {
                    result = false
                }
            }
            if let Some(anywhere_string) = &self.anywhere_string {
                if result && address.contains(anywhere_string) {
                    result = true
                } else {
                    result = false
                }
            }
            result
        }

        pub fn vanity_address_generator<R: RngCore>(&self, rng: &mut R) {
            let mut count = 0;
            // hex::encode_to_slice needs double the size
            let mut address = [0u8; 40];
            loop {
                let wallet = generate_random_wallet(rng);
                hex::encode_to_slice(wallet.address, &mut address).unwrap();
                // println!("checking address: {}", &address);
                if self.check_pattern(std::str::from_utf8(&address).unwrap()) {
                    println!("");
                    println!(
                        "found address: {:?}",
                        std::str::from_utf8(&address).unwrap()
                    );
                    println!("private key: {:?}", hex::encode(wallet.secret_key.as_ref()));
                    break;
                }
                count += 1;
                // println!("tried {} addresses", count);
                if !cli_display(&format!("tried {:8} addresses", count)).is_ok() {
                    println!("error in std out")
                };
            }
        }
    }
}
