use ethereum_vanity_address::generator::Choice;

fn main() {
    let start_string = "aaa";
    let ends_string = "bb";
    let anywhere_string = "c";
    let mut choice = Choice::new();
    choice.set_start_string(start_string);
    choice.set_emds_string(ends_string);
    choice.set_middle_string(anywhere_string);

    choice.vanity_address_generator();
}
