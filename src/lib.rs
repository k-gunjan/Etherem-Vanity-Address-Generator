mod util;
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

pub mod generator {
    use num_cpus;

    use super::eth_wallet::generate_random_wallet;
    use super::util::{cli_display, is_hex_string};
    use std::sync::{Arc, Mutex};
    use std::thread;

    use std::env;
    #[derive(Clone)]
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

        fn check_pattern(&self, address: &String) -> bool {
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

        pub fn vanity_address_generator(&self) {
            let mut count = 0;
            loop {
                let wallet = generate_random_wallet();
                let address = wallet.address;
                // println!("checking address: {}", &address);
                if self.check_pattern(&address) {
                    println!("");
                    println!("found address: {:?}", &address);
                    println!("private key: {:?}", wallet.secret_key);
                    break;
                }
                count += 1;
                // println!("tried {} addresses", count);
                if !cli_display(&format!("tried {:8} addresses", count)).is_ok() {
                    println!("error in std out")
                };
            }
        }
        pub fn vanity_address_generator_multi_threaded(&self) {
            let num_threads: usize = num_cpus::get();
            let found = Arc::new(Mutex::new(false));
            // let found_clone = Arc::new(&found);

            let mut thread_handles = vec![];
            for i in 0..num_threads {
                let found_clone = Arc::clone(&found);
                let self_clone = self.clone();
                let handle = thread::spawn(move || {
                    println!(
                        "started thread no: {} , thread_id: {:?}",
                        i,
                        thread::current().id()
                    );
                    let data = &*found_clone;
                    while !*data.lock().unwrap() {
                        let wallet = generate_random_wallet();
                        let address = wallet.address;
                        if self_clone.check_pattern(&address) {
                            println!("found in thread:{:?}", thread::current().id());
                            println!("found address: {:?}", &address);
                            println!("private key: {:?}", wallet.secret_key);
                            let mut state = data.lock().unwrap();
                            *state = true;
                        }
                    }
                });
                thread_handles.push(handle);
            }

            for t in thread_handles {
                t.join().unwrap();
            }
        }
    }
}
