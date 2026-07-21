use criterion::{criterion_group, criterion_main, Criterion};
use ethereum_vanity_address::eth_wallet::{generate_random_wallet, Wallet};
use ethereum_vanity_address::generator::Choice;
use ethers::core::rand::{rngs::StdRng, SeedableRng};
use std::hint::black_box;

fn bench_generate_random_wallet(c: &mut Criterion) {
    c.bench_function("bench eth address generation", |b| {
        let seed = [0u8; 32];
        let mut rng = StdRng::from_seed(seed);
        b.iter(|| {
            let wallet = generate_random_wallet(&mut rng);
        });
    });
}

criterion_group!(benches, bench_generate_random_wallet);
criterion_main!(benches);
