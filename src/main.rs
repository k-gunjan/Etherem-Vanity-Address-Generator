use ethereum_vanity_address::eth_wallet::generate_random_wallet;

fn main() {
    let start_string = "a";
    let ends_string = "b";
    let anywhere_string = "c";
    let mut choice = Choice::new();
    choice.set_start_string(start_string);
    choice.set_emds_string(ends_string);
    choice.set_middle_string(anywhere_string);

    choice.vanity_address_generator();
}

struct Choice {
    start_string: Option<String>,
    ends_string: Option<String>,
    anywhere_string: Option<String>,
}

impl Choice {
    fn new() -> Self {
        Self {
            start_string: None,
            ends_string: None,
            anywhere_string: None,
        }
    }
    fn set_start_string(&mut self, s: &str) {
        self.start_string = if s.len() > 0 {
            Some(s.to_string())
        } else {
            None
        }
    }
    fn set_emds_string(&mut self, s: &str) {
        self.ends_string = if s.len() > 0 {
            Some(s.to_string())
        } else {
            None
        }
    }
    fn set_middle_string(&mut self, s: &str) {
        self.anywhere_string = if s.len() > 0 {
            Some(s.to_string())
        } else {
            None
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

    fn vanity_address_generator(&self) {
        let mut count = 0;
        loop {
            let wallet = generate_random_wallet();
            let address = wallet.address;
            // println!("checking address: {}", &address);
            if self.check_pattern(&address) {
                println!("found address: {:?}", &address);
                println!("private key: {:?}", wallet.secret_key);
                break;
            }
            count += 1;
            println!("tried {} addresses", count);
        }
    }
}
