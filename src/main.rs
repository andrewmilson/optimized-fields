#![feature(test)]

extern crate test;

use ark_ff::FftField;
use ark_ff::Field;
use ark_ff::UniformRand;
use std::mem::size_of;

mod fp_u64;

/// Goldilocks Field
// #[derive(ark_ff::MontConfig)]
// #[modulus = "13"]
// // #[generator = "2"]
// // pub struct FpSConfig;
// pub type FpS = Fp64<MontBackend<FpSConfig, 1>>;

#[derive(ark_ff::MontConfig)]
#[modulus = "4611624995532046337"]
#[generator = "3"]
pub struct FpParams;
pub type Fp = ark_ff::Fp64<ark_ff::MontBackend<FpParams, 1>>;

fn main() {
    let omega = fp_u64::Fp::get_root_of_unity(32).unwrap();
    let neg_one = -fp_u64::Fp::ONE;
    println!("{}", omega.pow([32 / 2]));
    println!("{}", neg_one);
    println!("{}", size_of::<Fp>());
    println!("{}", size_of::<fp_u64::Fp>());
    println!("{}", neg_one);
    println!("YOOOOO!!!");
}

#[cfg(test)]
mod custom_impl_tests {
    use crate::fp_u64::Fp;
    use ark_ff::Field;
    use ark_ff::PrimeField;
    use ark_ff::UniformRand;
    use rand::SeedableRng;
    use rand_pcg::Pcg64;
    use test::Bencher;

    #[bench]
    fn inverse_1000(b: &mut Bencher) {
        let mut rng = Pcg64::seed_from_u64(42);
        let items = (0..1000).map(|_| Fp::rand(&mut rng)).collect::<Vec<Fp>>();

        b.iter(|| items.iter().map(|item| item.inverse()).collect::<Vec<_>>())
    }

    #[bench]
    fn sum_20000(b: &mut Bencher) {
        let mut rng = Pcg64::seed_from_u64(42);
        let items = (0..20000).map(|_| Fp::rand(&mut rng)).collect::<Vec<Fp>>();

        b.iter(|| items.iter().sum::<Fp>())
    }

    #[bench]
    fn double_20000(b: &mut Bencher) {
        let mut rng = Pcg64::seed_from_u64(42);
        let items = (0..20000).map(|_| Fp::rand(&mut rng)).collect::<Vec<Fp>>();

        b.iter(|| items.iter().map(|item| item.double()).collect::<Vec<Fp>>())
    }

    #[bench]
    fn product_1000(b: &mut Bencher) {
        let mut rng = Pcg64::seed_from_u64(42);
        let items = (0..1000).map(|_| Fp::rand(&mut rng)).collect::<Vec<Fp>>();

        b.iter(|| items.iter().product::<Fp>());

        println!("{}", items.iter().product::<Fp>().into_bigint());
    }
}

#[cfg(test)]
mod derived_impl_tests {
    use crate::Fp;
    use ark_ff::Field;
    use ark_ff::PrimeField;
    use ark_ff::UniformRand;
    use rand::SeedableRng;
    use rand_pcg::Pcg64;
    use test::Bencher;

    #[bench]
    fn inverse_1000(b: &mut Bencher) {
        let mut rng = Pcg64::seed_from_u64(42);
        let items = (0..1000).map(|_| Fp::rand(&mut rng)).collect::<Vec<Fp>>();

        b.iter(|| items.iter().map(|item| item.inverse()).collect::<Vec<_>>())
    }

    #[bench]
    fn sum_20000(b: &mut Bencher) {
        let mut rng = Pcg64::seed_from_u64(42);
        let items = (0..20000).map(|_| Fp::rand(&mut rng)).collect::<Vec<Fp>>();

        b.iter(|| items.iter().sum::<Fp>())
    }

    #[bench]
    fn product_1000(b: &mut Bencher) {
        let mut rng = Pcg64::seed_from_u64(42);
        let items = (0..1000).map(|_| Fp::rand(&mut rng)).collect::<Vec<Fp>>();

        b.iter(|| items.iter().product::<Fp>());

        println!("{}", items.iter().product::<Fp>().into_bigint());
    }
}
