use ark_ff::fields::Fp64;
use ark_ff::BigInt;
use ark_ff::Field;
use ark_ff::Zero;
use std::marker::PhantomData;

/// Field modulus
const MODULUS: u64 = 18446744069414584321;

/// Square of auxiliary modulus R for Montgomery reduction `R2 ≡ (2^64)^2 mod P`
const R2: u64 = 18446744065119617025;

pub struct FpParams;
impl ark_ff::FpConfig<1> for FpParams {
    const MODULUS: ark_ff::BigInt<1> = BigInt([MODULUS]);

    const GENERATOR: Fp64<Self> = into_mont(7);

    const ZERO: Fp64<Self> = into_mont(0);

    const ONE: Fp64<Self> = into_mont(1);

    const TWO_ADICITY: u32 = 32;

    const TWO_ADIC_ROOT_OF_UNITY: Fp64<Self> = into_mont(1753635133440165772);

    const SQRT_PRECOMP: Option<ark_ff::SqrtPrecomputation<Fp64<Self>>> = None;

    fn add_assign(a: &mut Fp64<Self>, b: &Fp64<Self>) {
        // We compute a + b = a - (p - b).
        let (x1, c1) = (a.0).0[0].overflowing_sub(MODULUS - (b.0).0[0]);
        let adj = 0u32.wrapping_sub(c1 as u32);
        (a.0).0[0] = x1.wrapping_sub(adj as u64);
    }

    fn sub_assign(a: &mut Fp64<Self>, b: &Fp64<Self>) {
        let (x1, c1) = (a.0).0[0].overflowing_sub((b.0).0[0]);
        let adj = 0u32.wrapping_sub(c1 as u32);
        (a.0).0[0] = x1.wrapping_sub(adj as u64);
    }

    fn double_in_place(a: &mut Fp64<Self>) {
        let (result, over) = (a.0).0[0].overflowing_shl(1);
        (a.0).0[0] = result.wrapping_sub(MODULUS * (over as u64));
    }

    fn mul_assign(a: &mut Fp64<Self>, b: &Fp64<Self>) {
        (a.0).0[0] = mont_red((a.0).0[0] as u128 * (b.0).0[0] as u128);
    }

    fn sum_of_products(a: &[Fp64<Self>], b: &[Fp64<Self>]) -> Fp64<Self> {
        a.iter().zip(b).map(|(a, b)| *a * b).sum()
    }

    fn square_in_place(a: &mut Fp64<Self>) {
        let temp = *a;
        Self::mul_assign(a, &temp);
    }

    fn inverse(a: &Fp64<Self>) -> Option<Fp64<Self>> {
        if a.is_zero() {
            None
        } else {
            // compute base^(M - 2) using 72 multiplications
            let t2 = a.square() * a;
            let t3 = t2.square() * a;
            let t6 = exp_acc::<3>((t3.0).0[0], (t3.0).0[0]);
            let t12 = exp_acc::<6>(t6, t6);
            let t24 = exp_acc::<12>(t12, t12);
            let t30 = ark_ff::Fp(BigInt([exp_acc::<6>(t24, t6)]), PhantomData);
            let t31 = t30.square() * a;
            let t63 = ark_ff::Fp(
                BigInt([exp_acc::<32>((t31.0).0[0], (t31.0).0[0])]),
                PhantomData,
            );
            Some(t63.square() * a)
        }
    }

    fn from_bigint(other: ark_ff::BigInt<1>) -> Option<Fp64<Self>> {
        let inner = other.0[0];
        if inner.is_zero() {
            Some(Self::ZERO)
        } else if inner < MODULUS {
            Some(into_mont(other.0[0]))
        } else {
            None
        }
    }

    fn into_bigint(other: Fp64<Self>) -> ark_ff::BigInt<1> {
        BigInt([mont_red((other.0).0[0] as u128)])
    }
}

pub type Fp = Fp64<FpParams>;

/// Converts a canonical representation into Montgomery representation
#[inline(always)]
const fn into_mont(value: u64) -> Fp {
    ark_ff::Fp(BigInt([mont_red(value as u128 * R2 as u128)]), PhantomData)
}

/// Performs Montgomery reduction
#[inline(always)]
const fn mont_red(x: u128) -> u64 {
    // See reference above for a description of the following implementation.
    let xl = x as u64;
    let xh = (x >> 64) as u64;
    let (a, e) = xl.overflowing_add(xl << 32);
    let b = a.wrapping_sub(a >> 32).wrapping_sub(e as u64);
    let (r, c) = xh.overflowing_sub(b);
    r.wrapping_sub(0u32.wrapping_sub(c as u32) as u64)
}

/// Squares `base` N times and multiplies the result by the tail value.
#[inline(always)]
const fn exp_acc<const N: usize>(base: u64, tail: u64) -> u64 {
    let mut result = base;
    let mut i = 0;
    while i < N {
        result = mont_red(result as u128 * result as u128);
        i += 1;
    }
    mont_red(result as u128 * tail as u128)
}
