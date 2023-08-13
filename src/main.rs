use ethereum_vanity_address::generator::Choice;

fn main() {
    match Choice::collect_args() {
        Ok(choice) => choice.vanity_address_generator_multi_threaded(),
        Err(e) => println!("Error: {e}"),
    }
}
