use ethereum_vanity_address::generator::Choice;
use ethers::core::rand;


#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

// Use secure randomness in production
#[cfg(not(feature = "dhat-heap"))]
fn create_rng() -> impl rand::RngCore {
    rand::thread_rng()
}

// Use deterministic randomness in development
#[cfg(feature = "dhat-heap")]
fn create_rng() -> impl rand::RngCore {
    use rand::{rngs::StdRng, SeedableRng};
    StdRng::from_seed([0u8; 32])
}

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    match Choice::collect_args() {
        Ok(choice) => {
            let mut rng = create_rng();
            choice.vanity_address_generator(&mut rng);
        }
        Err(e) => eprintln!("Error: {e}"),
    }
}