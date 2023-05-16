use ark_algebra_bench_templates::*;
use ark_ff_optimized::field_compare;
use ark_ff_optimized::fp31::Fp as Specialized;
use criterion::criterion_main;

#[derive(ark_ff::MontConfig)]
#[modulus = "2147483647"]
#[generator = "3"]
pub struct FpParams;
pub type Generic = ark_ff::Fp64<ark_ff::MontBackend<FpParams, 1>>;

field_compare!(prime; "Fp=2147483647"; fp18446744069414584321; Generic, Specialized);
criterion_main!(fp18446744069414584321::benches);
