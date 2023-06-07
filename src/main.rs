use ethereum_vanity_address::generator::Choice;

fn main() {
    match Choice::collect_args() {
        Ok(choice) => choice.vanity_address_generator(),
        Err(e) => println!("Error: {e}"),
    }
}
