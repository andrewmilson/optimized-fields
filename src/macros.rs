#[macro_export]
macro_rules! field_compare {
    (prime; $test_name:expr; $mod_name:ident; $( $field:ident ),+) => {
        mod $mod_name {
            use super::*;
            use ark_ff::Field;
            use ark_ff::PrimeField;
            use ark_ff::UniformRand;

            fn bench_compare(c: &mut Criterion) {
                const SAMPLES: usize = 1000;
                let mut group = c.benchmark_group($test_name);
                $(
                    let field_name = stringify!($field);
                    let mut rng = ark_std::test_rng();
                    let field_elements_left = (0..SAMPLES)
                        .map(|_| <$field>::rand(&mut rng))
                        .collect::<Vec<_>>();
                    let field_elements_right = (0..SAMPLES)
                        .map(|_| <$field>::rand(&mut rng))
                        .collect::<Vec<_>>();
                    let test_type = "Addition";
                    group.bench_with_input(BenchmarkId::new(field_name, test_type), test_type, |b, _| {
                        let mut i = 0;
                        b.iter(|| {
                            i = (i + 1) % SAMPLES;
                            field_elements_left[i] + field_elements_right[i]
                        })
                    });
                    let test_type = "Subtraction";
                    group.bench_with_input(BenchmarkId::new(field_name, test_type), test_type, |b, _| {
                        let mut i = 0;
                        b.iter(|| {
                            i = (i + 1) % SAMPLES;
                            field_elements_left[i] - field_elements_right[i]
                        })
                    });
                    let test_type = "Negation";
                    group.bench_with_input(BenchmarkId::new(field_name, test_type), test_type, |b, _| {
                        let mut i = 0;
                        b.iter(|| {
                            i = (i + 1) % SAMPLES;
                            -field_elements_left[i]
                        })
                    });
                    let test_type = "Double";
                    group.bench_with_input(BenchmarkId::new(field_name, test_type), test_type, |b, _| {
                        let mut i = 0;
                        b.iter(|| {
                            i = (i + 1) % SAMPLES;
                            field_elements_left[i].double()
                        })
                    });
                    let test_type = "Multiplication";
                    group.bench_with_input(BenchmarkId::new(field_name, test_type), test_type, |b, _| {
                        let mut i = 0;
                        b.iter(|| {
                            i = (i + 1) % SAMPLES;
                            field_elements_left[i] * field_elements_right[i]
                        })
                    });
                    let test_type = "Square";
                    group.bench_with_input(BenchmarkId::new(field_name, test_type), test_type, |b, _| {
                        let mut i = 0;
                        b.iter(|| {
                            i = (i + 1) % SAMPLES;
                            field_elements_left[i].square()
                        })
                    });
                    let test_type = "Inverse";
                    group.bench_with_input(BenchmarkId::new(field_name, test_type), test_type, |b, _| {
                        let mut i = 0;
                        b.iter(|| {
                            i = (i + 1) % SAMPLES;
                            field_elements_left[i].inverse().unwrap()
                        })
                    });
                    let test_type = "Sum of products of size 2";
                    group.bench_with_input(BenchmarkId::new(field_name, test_type), test_type, |b, _| {
                        let mut i = 0;
                        b.iter(|| {
                            i = (i + 1) % SAMPLES;
                            let j = (i + 1) % SAMPLES;
                            <$field>::sum_of_products(
                                &[field_elements_left[i], field_elements_right[j]],
                                &[field_elements_left[j], field_elements_right[i]],
                            )
                        })
                    });
                    let test_type = "Naive sum of products of size 2";
                    group.bench_with_input(BenchmarkId::new(field_name, test_type), field_name, |b, _| {
                        let mut i = 0;
                        b.iter(|| {
                            i = (i + 1) % SAMPLES;
                            let j = (i + 1) % SAMPLES;
                            field_elements_left[i] * field_elements_left[j]
                                + field_elements_right[j] * field_elements_right[i]
                        })
                    });
                )*
                group.finish();
            }

            criterion::criterion_group!(benches, bench_compare);
        }
    };
}

// #[macro_export]
// macro_rules! f_bench {

// compare_field_common!($curve_name, $F);
// compare_sqrt!($curve_name, $F);
// compare_prime_field!($curve_name, $F);

// fn bench_fields(c: &mut Criterion) {
//     const SAMPLES: usize = 1000;

//     let mut rng = ark_std::test_rng();
//     let generic_field_elements_left = (0..SAMPLES)
//         .map(|_| Generic::rand(&mut rng))
//         .collect::<Vec<_>>();
//     let generic_field_elements_right = (0..SAMPLES)
//         .map(|_| Generic::rand(&mut rng))
//         .collect::<Vec<_>>();

//     let mut rng = ark_std::test_rng();
//     let specialized_field_elements_left = (0..SAMPLES)
//         .map(|_| Specialized::rand(&mut rng))
//         .collect::<Vec<_>>();
//     let specialized_field_elements_right = (0..SAMPLES)
//         .map(|_| Specialized::rand(&mut rng))
//         .collect::<Vec<_>>();

//     let mut arithmetic = c.benchmark_group("Fp=18446744069414584321");
//     for test in [Test::Addition, Test::Multiplication].iter() {
//         arithmetic.bench_with_input(BenchmarkId::new("Generic", test), test,
// |b, test| {             let mut i = 0;

//             match test {
//                 Test::Multiplication => b.iter(|| {
//                     i = (i + 1) % SAMPLES;
//                     generic_field_elements_left[i] *
// generic_field_elements_right[i]                 }),
//                 Test::Addition => b.iter(|| {
//                     i = (i + 1) % SAMPLES;
//                     generic_field_elements_left[i] +
// generic_field_elements_right[i]                 }),
//             }
//         });
//         arithmetic.bench_with_input(BenchmarkId::new("Specialized", test),
// test, |b, test| {             let mut i = 0;

//             match test {
//                 Test::Multiplication => b.iter(|| {
//                     i = (i + 1) % SAMPLES;
//                     specialized_field_elements_left[i] *
// specialized_field_elements_right[i]                 }),
//                 Test::Addition => b.iter(|| {
//                     i = (i + 1) % SAMPLES;
//                     specialized_field_elements_left[i] +
// specialized_field_elements_right[i]                 }),
//             }
//         });
//     }
//     // arithmetic.bench_function("Specialized", |b| {});
// }
