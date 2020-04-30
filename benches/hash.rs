#![feature(test)]

extern crate rand;
extern crate test;
extern crate pairing;
extern crate poseidon_hash;

use rand::{Rand, thread_rng};
use pairing::bn256::{Bn256, Fr};
use poseidon_hash::group_hash::{BlakeHasher};
use poseidon_hash::poseidon_hash;
use poseidon_hash::bn256::Bn256PoseidonParams;
use poseidon_hash::specialization::specialization_macro::PosendonR2C1;

#[bench]
fn bench_rate_2(b: &mut test::Bencher) {
    let params = Bn256PoseidonParams::new_checked_2_into_1();
    let rng = &mut thread_rng();
    let input = (0..2).map(|_| Fr::rand(rng)).collect::<Vec<_>>();

    b.iter(|| {
        poseidon_hash::<Bn256>(&params, &input)
    });
}

#[bench]
fn bench_rate_specialized_2(b: &mut test::Bencher) {
    let params = Bn256PoseidonParams::new_checked_2_into_1();
    let rng = &mut thread_rng();
    let input = (0..2).map(|_| Fr::rand(rng)).collect::<Vec<_>>();

    b.iter(|| {
        let mut p = PosendonR2C1::<Bn256>::new(&params);
        for &i in input.iter() {
            p.absorb_single_value(i);
        }
        p.squeeze_out_single()
    });
}

#[bench]
fn bench_rate_2_80_bits(b: &mut test::Bencher) {
    let rate = 2u32;
    let capacity = 1u32;
    let num_full = 8u32;
    let num_partial = 52u32;
    let security_level = 80u32;
    let params = Bn256PoseidonParams::new_for_params::<BlakeHasher>(capacity, rate, num_partial, num_full, security_level);
    let rng = &mut thread_rng();
    let input = (0..rate).map(|_| Fr::rand(rng)).collect::<Vec<_>>();

    b.iter(|| {
        poseidon_hash::<Bn256>(&params, &input)
    });
}

#[bench]
fn bench_rate_10_120_bits(b: &mut test::Bencher) {
    let rate = 10u32;
    let capacity = 1u32;
    let num_full = 8u32;
    let num_partial = 88u32;
    let security_level = 120u32;
    let params = Bn256PoseidonParams::new_for_params::<BlakeHasher>(capacity, rate, num_partial, num_full, security_level);
    let rng = &mut thread_rng();
    let input = (0..rate).map(|_| Fr::rand(rng)).collect::<Vec<_>>();

    b.iter(|| {
        poseidon_hash::<Bn256>(&params, &input)
    });
}