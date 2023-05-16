//! An implementation of a 31-bit Mersenne prime (no 2^k roots of unity for k>1)
//! field with modulus `2^31 - 1`. Mersenne primes have a fast reductions to to
//! their binary representation.
//!
//! This field and its implementation has a couple of attractive properties:
//! * Addition never overflows a 32-bit int.
//! * Efficient for GPUs which optimize throughput for 32-bit and 16-bit arithmetic.
//! * Field arithmetic in this field can be implemented using a few 32-bit
//!   addition, subtractions, and shifts.

use ark_ff::{BigInt, FftField, Field, LegendreSymbol, One, PrimeField, SqrtPrecomputation, Zero};
use ark_serialize::{
    buffer_byte_size, CanonicalDeserialize, CanonicalDeserializeWithFlags, CanonicalSerialize,
    CanonicalSerializeWithFlags, Compress, EmptyFlags, Flags, SerializationError, Valid, Validate,
};
use ark_std::string::ToString;
use core::{
    fmt::{Debug, Display, Formatter},
    hash::Hash,
    iter::{Product, Sum},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    str::FromStr,
};

/// Field modulus `p = 2^31 - 1`
const MODULUS: u32 = 2_147_483_647;

const MODULUS_BIT_SIZE: u32 = 31;

#[derive(Clone, Copy, Default)]
pub struct Fp(pub u32);

impl Fp {
    #[doc(hidden)]
    #[inline]
    const fn is_gt_modulus(self) -> bool {
        self.0 > MODULUS
    }
    const fn is_geq_modulus(self) -> bool {
        self.0 >= MODULUS
    }

    const fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.0 = (self.0 & MODULUS) + (self.0 >> 31);
    }

    const fn sub_assign(&mut self, rhs: Self) {
        self.0 += MODULUS - rhs.0;
        self.0 = (self.0 & MODULUS) + (self.0 >> 31);
    }

    #[inline]
    const fn mul(self, rhs: Self) -> Self {
        let t = self.0 as u64 * (rhs.0 << 1) as u64;
        // we want to truncate here
        #[allow(clippy::cast_possible_truncation)]
        let t0 = t as u32 >> 1;
        let t1 = (t >> 32) as u32;
        let x = t0 + t1;
        Self((x & MODULUS) + (x >> 31))
    }

    #[inline]
    const fn sq(self) -> Self {
        self.mul(self)
    }

    const fn sqn(mut self, n: u32) -> Self {
        let mut i = 0;
        while i < n {
            self = self.sq();
            i += 1;
        }
        self
    }

    const fn is_zero(self) -> bool {
        self.0 == 0 || self.0 == MODULUS
    }

    const fn into_integer(self) -> u32 {
        if self.is_zero() {
            0
        } else {
            self.0
        }
    }
}

impl Field for Fp {
    type BasePrimeField = Self;
    type BasePrimeFieldIter = core::iter::Once<Self::BasePrimeField>;

    const SQRT_PRECOMP: Option<SqrtPrecomputation<Self>> = Some(SqrtPrecomputation::Case3Mod4 {
        modulus_plus_one_div_four: &[(MODULUS as u64 + 1) / 4],
    });

    const ZERO: Self = Self(0);

    const ONE: Self = Self(1);

    fn extension_degree() -> u64 {
        1
    }

    fn from_base_prime_field(elem: Self::BasePrimeField) -> Self {
        elem
    }

    fn to_base_prime_field_elements(&self) -> Self::BasePrimeFieldIter {
        core::iter::once(*self)
    }

    fn from_base_prime_field_elems(elems: &[Self::BasePrimeField]) -> Option<Self> {
        if elems.len() != usize::try_from(Self::extension_degree()).unwrap() {
            return None;
        }
        Some(elems[0])
    }

    #[inline]
    fn double(&self) -> Self {
        let mut temp = *self;
        temp.double_in_place();
        temp
    }

    #[inline]
    fn double_in_place(&mut self) -> &mut Self {
        let x = self.0 << 1;
        self.0 = (x & MODULUS) + (self.0 >> 30);
        self
    }

    #[inline]
    fn neg_in_place(&mut self) -> &mut Self {
        self.0 = MODULUS - self.0;
        self
    }

    #[inline]
    fn characteristic() -> &'static [u64] {
        const _MODULUS: &[u64] = &[MODULUS as u64];
        _MODULUS
    }

    #[inline]
    fn sum_of_products<const T: usize>(a: &[Self; T], b: &[Self; T]) -> Self {
        a.iter().zip(b).map(|(&a, b)| a * b).sum()
    }

    #[inline]
    fn from_random_bytes_with_flags<F: Flags>(_bytes: &[u8]) -> Option<(Self, F)> {
        todo!()
    }

    #[inline]
    fn square(&self) -> Self {
        let mut temp = *self;
        temp.square_in_place();
        temp
    }

    fn square_in_place(&mut self) -> &mut Self {
        *self = self.sq();
        self
    }

    #[inline]
    #[allow(clippy::just_underscores_and_digits, clippy::similar_names)]
    fn inverse(&self) -> Option<Self> {
        if self.is_zero() {
            None
        } else {
            // Used addchains
            let t100 = Self::sqn(*self, 2);
            let t101 = Self::mul(*self, t100);
            let t1010 = Self::sq(t101);
            let t1111 = Self::mul(t101, t1010);
            let t1111000 = Self::sqn(t1111, 3);
            let t1111101 = Self::mul(t101, t1111000);
            let t11111010 = Self::sq(t1111101);
            let t11111111 = Self::mul(t101, t11111010);
            let x16 = Self::mul(Self::sqn(t11111111, 8), t11111111);
            let x24 = Self::mul(Self::sqn(x16, 8), t11111111);
            Some(Self::mul(Self::sqn(x24, 7), t1111101))
        }
    }

    fn inverse_in_place(&mut self) -> Option<&mut Self> {
        self.inverse().map(|inverse| {
            *self = inverse;
            self
        })
    }

    /// The Frobenius map has no effect in a prime field.
    #[inline]
    fn frobenius_map_in_place(&mut self, _: usize) {}

    #[inline]
    fn legendre(&self) -> LegendreSymbol {
        let s = self.pow([(u64::from(MODULUS) - 1) / 2]);
        if s.is_zero() {
            LegendreSymbol::Zero
        } else if s.is_one() {
            LegendreSymbol::QuadraticResidue
        } else {
            LegendreSymbol::QuadraticNonResidue
        }
    }
}

impl PrimeField for Fp {
    type BigInt = BigInt<1>;
    const MODULUS: Self::BigInt = BigInt([MODULUS as u64]);
    const MODULUS_MINUS_ONE_DIV_TWO: Self::BigInt = Self::MODULUS.divide_by_2_round_down();
    const MODULUS_BIT_SIZE: u32 = Self::MODULUS.const_num_bits();
    const TRACE: Self::BigInt = Self::MODULUS.two_adic_coefficient();
    const TRACE_MINUS_ONE_DIV_TWO: Self::BigInt = Self::TRACE.divide_by_2_round_down();

    #[inline]
    fn from_bigint(r: BigInt<1>) -> Option<Self> {
        Some(Self::from(r.0[0]))
    }

    fn into_bigint(self) -> BigInt<1> {
        BigInt([self.into_integer().into()])
    }
}

impl FftField for Fp {
    const GENERATOR: Self = Self(3);
    const TWO_ADICITY: u32 = 1;
    const TWO_ADIC_ROOT_OF_UNITY: Self = Self(MODULUS - 1);
    const SMALL_SUBGROUP_BASE: Option<u32> = None;
    const SMALL_SUBGROUP_BASE_ADICITY: Option<u32> = None;
    const LARGE_SUBGROUP_ROOT_OF_UNITY: Option<Self> = None;
}

impl zeroize::Zeroize for Fp {
    fn zeroize(&mut self) {
        self.0.zeroize();
    }
}

impl Debug for Fp {
    fn fmt(&self, f: &mut Formatter<'_>) -> ark_std::fmt::Result {
        ark_std::fmt::Debug::fmt(&self.into_integer(), f)
    }
}

impl Zero for Fp {
    #[inline]
    fn zero() -> Self {
        Self::ZERO
    }

    #[inline]
    fn is_zero(&self) -> bool {
        (*self).is_zero()
    }
}

impl One for Fp {
    #[inline]
    fn one() -> Self {
        Self::ONE
    }

    #[inline]
    fn is_one(&self) -> bool {
        *self == Self::ONE
    }
}

impl PartialEq for Fp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 || (self.is_zero() && other.is_zero())
    }
}

impl Hash for Fp {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.into_integer().hash(state);
    }
}

impl Eq for Fp {}

/// Note that this implementation of `Ord` compares field elements viewing
/// them as integers in the range 0, 1, ..., `P::MODULUS` - 1. However, other
/// implementations of `PrimeField` might choose a different ordering, and
/// as such, users should use this `Ord` for applications where
/// any ordering suffices (like in a `BTreeMap`), and not in applications
/// where a particular ordering is required.
impl Ord for Fp {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.into_integer().cmp(&other.into_integer())
    }
}

/// Note that this implementation of `PartialOrd` compares field elements
/// viewing them as integers in the range 0, 1, ..., `P::MODULUS` - 1. However,
/// other implementations of `PrimeField` might choose a different ordering, and
/// as such, users should use this `PartialOrd` for applications where
/// any ordering suffices (like in a `BTreeMap`), and not in applications
/// where a particular ordering is required.
impl PartialOrd for Fp {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<num_bigint::BigUint> for Fp {
    fn from(other: num_bigint::BigUint) -> Self {
        (other % MODULUS)
            .to_u32_digits()
            .iter()
            .copied()
            .map(Self::from)
            .sum()
    }
}

impl From<Fp> for num_bigint::BigUint {
    fn from(fp: Fp) -> Self {
        fp.into_integer().into()
    }
}

impl From<BigInt<1>> for Fp {
    fn from(other: BigInt<1>) -> Self {
        other.0[0].into()
    }
}

impl From<Fp> for BigInt<1> {
    fn from(fp: Fp) -> Self {
        Self([fp.into_integer().into()])
    }
}

// TODO: change all the From implementations
impl From<u128> for Fp {
    fn from(other: u128) -> Self {
        #[allow(clippy::cast_possible_truncation)]
        Self((other % u128::from(MODULUS)) as u32)
    }
}

impl From<i128> for Fp {
    fn from(other: i128) -> Self {
        let abs = Self::from(other.unsigned_abs());
        if other.is_positive() {
            abs
        } else {
            -abs
        }
    }
}

impl From<bool> for Fp {
    fn from(other: bool) -> Self {
        Self(u32::from(other))
    }
}

impl From<u64> for Fp {
    fn from(other: u64) -> Self {
        #[allow(clippy::cast_possible_truncation)]
        Self((other % u64::from(MODULUS)) as u32)
    }
}

impl From<i64> for Fp {
    fn from(other: i64) -> Self {
        let abs = Self::from(other.unsigned_abs());
        if other.is_positive() {
            abs
        } else {
            -abs
        }
    }
}

impl From<u32> for Fp {
    fn from(other: u32) -> Self {
        Self(other % MODULUS)
    }
}

impl From<i32> for Fp {
    fn from(other: i32) -> Self {
        let abs = Self::from(other.unsigned_abs());
        if other.is_positive() {
            abs
        } else {
            -abs
        }
    }
}

impl From<u16> for Fp {
    fn from(other: u16) -> Self {
        Self(other.into())
    }
}

impl From<i16> for Fp {
    fn from(other: i16) -> Self {
        let abs = Self::from(other.unsigned_abs());
        if other.is_positive() {
            abs
        } else {
            -abs
        }
    }
}

impl From<u8> for Fp {
    fn from(other: u8) -> Self {
        Self(other.into())
    }
}

impl From<i8> for Fp {
    fn from(other: i8) -> Self {
        let abs = Self::from(other.unsigned_abs());
        if other.is_positive() {
            abs
        } else {
            -abs
        }
    }
}

impl ark_std::rand::distributions::Distribution<Fp> for ark_std::rand::distributions::Standard {
    #[inline]
    fn sample<R: ark_std::rand::Rng + ?Sized>(&self, rng: &mut R) -> Fp {
        loop {
            let mut tmp = Fp(rng.sample(Self));

            // Mask away the unused bits at the beginning.
            let mask = u32::MAX >> (32 - MODULUS_BIT_SIZE);
            tmp.0 &= mask;

            // TODO: check make sure the distrobution for 0 is right since 2^31 - 1 and 0 represent 0
            if !tmp.is_geq_modulus() {
                return tmp;
            }
        }
    }
}

// TODO:
impl CanonicalSerializeWithFlags for Fp {
    fn serialize_with_flags<W: ark_std::io::Write, F: Flags>(
        &self,
        mut writer: W,
        flags: F,
    ) -> Result<(), SerializationError> {
        // All reasonable `Flags` should be less than 8 bits in size
        // (256 values are enough for anyone!)
        if F::BIT_SIZE > 8 {
            return Err(SerializationError::NotEnoughSpace);
        }

        let b = self.into_integer().to_le_bytes();
        // TODO: Double check this
        // Mask out the bits of the last byte that correspond to the flag.
        writer.write_all(&[b[0], b[1], b[2], b[3], flags.u8_bitmask()])?;
        Ok(())
    }

    // Let `m = 8 * n` for some `n` be the smallest multiple of 8 greater
    // than `P::MODULUS_BIT_SIZE`.
    // If `(m - P::MODULUS_BIT_SIZE) >= F::BIT_SIZE` , then this method returns `n`;
    // otherwise, it returns `n + 1`.
    fn serialized_size_with_flags<F: Flags>(&self) -> usize {
        buffer_byte_size(MODULUS_BIT_SIZE as usize + F::BIT_SIZE)
    }
}

impl CanonicalSerialize for Fp {
    #[inline]
    fn serialize_with_mode<W: ark_std::io::Write>(
        &self,
        writer: W,
        _compress: Compress,
    ) -> Result<(), SerializationError> {
        self.serialize_with_flags(writer, EmptyFlags)
    }

    #[inline]
    fn serialized_size(&self, _compress: Compress) -> usize {
        self.serialized_size_with_flags::<EmptyFlags>()
    }
}

impl CanonicalDeserializeWithFlags for Fp {
    fn deserialize_with_flags<R: ark_std::io::Read, F: Flags>(
        mut reader: R,
    ) -> Result<(Self, F), SerializationError> {
        // All reasonable `Flags` should be less than 8 bits in size
        // (256 values are enough for anyone!)
        if F::BIT_SIZE > 8 {
            return Err(SerializationError::NotEnoughSpace);
        }
        // Calculate the number of bytes required to represent a field element
        // serialized with `flags`.
        let mut b = [0u8; 5];
        reader.read_exact(&mut b)?;
        let flags = F::from_u8_remove_flags(&mut b[b.len() - 1])
            .ok_or(SerializationError::UnexpectedFlags)?;
        let self_integer = u32::from_le_bytes(b[0..4].try_into().unwrap());
        Ok((Self(self_integer), flags))
    }
}

impl Valid for Fp {
    fn check(&self) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl CanonicalDeserialize for Fp {
    fn deserialize_with_mode<R: ark_std::io::Read>(
        reader: R,
        _compress: Compress,
        _validate: Validate,
    ) -> Result<Self, SerializationError> {
        Self::deserialize_with_flags::<R, EmptyFlags>(reader).map(|(r, _)| r)
    }
}

impl FromStr for Fp {
    type Err = ();

    /// Interpret a string of numbers as a (congruent) prime field element.
    /// Does not accept unnecessary leading zeroes or a blank string.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(());
        }

        if s == "0" {
            return Ok(Self::zero());
        }

        let mut res = Self::zero();

        let ten = Self(10);

        let mut first_digit = true;

        for c in s.chars() {
            match c.to_digit(10) {
                Some(c) => {
                    if first_digit {
                        if c == 0 {
                            return Err(());
                        }

                        first_digit = false;
                    }

                    res.mul_assign(&ten);
                    let digit = Self::from(u64::from(c));
                    res.add_assign(digit);
                }
                None => {
                    return Err(());
                }
            }
        }
        if res.is_gt_modulus() {
            Err(())
        } else {
            Ok(res)
        }
    }
}

/// Outputs a string containing the value of `self`,
/// represented as a decimal without leading zeroes.
impl Display for Fp {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let string = self.into_integer().to_string();
        write!(f, "{}", string.trim_start_matches('0'))
    }
}

impl Neg for Fp {
    type Output = Self;
    #[inline]
    #[must_use]
    fn neg(mut self) -> Self {
        Self::neg_in_place(&mut self);
        self
    }
}

impl<'a> Add<&'a Self> for Fp {
    type Output = Self;

    #[inline]
    fn add(mut self, other: &Self) -> Self {
        self.add_assign(*other);
        self
    }
}

impl<'a> Sub<&'a Self> for Fp {
    type Output = Self;

    #[inline]
    fn sub(mut self, other: &Self) -> Self {
        self.sub_assign(*other);
        self
    }
}

impl<'a> Mul<&'a Self> for Fp {
    type Output = Self;

    #[inline]
    fn mul(mut self, other: &Self) -> Self {
        self.mul_assign(other);
        self
    }
}

impl<'a> Div<&'a Self> for Fp {
    type Output = Self;

    /// Returns `self * other.inverse()` if `other.inverse()` is `Some`, and
    /// panics otherwise.
    #[inline]
    fn div(mut self, other: &Self) -> Self {
        self.mul_assign(&other.inverse().unwrap());
        self
    }
}

impl<'a, 'b> Add<&'b Fp> for &'a Fp {
    type Output = Fp;

    #[inline]
    fn add(self, other: &'b Fp) -> Fp {
        let mut result = *self;
        result.add_assign(*other);
        result
    }
}

impl<'a, 'b> Sub<&'b Fp> for &'a Fp {
    type Output = Fp;

    #[inline]
    fn sub(self, other: &Fp) -> Fp {
        let mut result = *self;
        result.sub_assign(*other);
        result
    }
}

impl<'a, 'b> Mul<&'b Fp> for &'a Fp {
    type Output = Fp;

    #[inline]
    fn mul(self, other: &Fp) -> Fp {
        let mut result = *self;
        result.mul_assign(other);
        result
    }
}

impl<'a, 'b> Div<&'b Fp> for &'a Fp {
    type Output = Fp;

    #[inline]
    fn div(self, other: &Fp) -> Fp {
        let mut result = *self;
        result.div_assign(other);
        result
    }
}

impl<'a> AddAssign<&'a Self> for Fp {
    #[inline]
    fn add_assign(&mut self, other: &Self) {
        Self::add_assign(self, *other);
    }
}

impl<'a> SubAssign<&'a Self> for Fp {
    #[inline]
    fn sub_assign(&mut self, other: &Self) {
        Self::sub_assign(self, *other);
    }
}

impl<'a> AddAssign<&'a mut Self> for Fp {
    #[inline]
    fn add_assign(&mut self, other: &mut Self) {
        Self::add_assign(self, *other);
    }
}

impl<'a> SubAssign<&'a mut Self> for Fp {
    #[inline]
    fn sub_assign(&mut self, other: &mut Self) {
        Self::sub_assign(self, *other);
    }
}

impl AddAssign<Self> for Fp {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        Self::add_assign(self, other);
    }
}

impl SubAssign<Self> for Fp {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        Self::sub_assign(self, other);
    }
}

impl Mul<Self> for Fp {
    type Output = Self;

    #[inline]
    fn mul(mut self, other: Self) -> Self {
        self.mul_assign(&other);
        self
    }
}

impl Div<Self> for Fp {
    type Output = Self;

    #[inline]
    fn div(mut self, other: Self) -> Self {
        self.div_assign(&other);
        self
    }
}

impl Add<Self> for Fp {
    type Output = Self;

    #[inline]
    fn add(mut self, other: Self) -> Self {
        self.add_assign(other);
        self
    }
}

impl Sub<Self> for Fp {
    type Output = Self;

    #[inline]
    fn sub(mut self, other: Self) -> Self {
        self.sub_assign(other);
        self
    }
}

impl<'a> Add<&'a mut Self> for Fp {
    type Output = Self;

    #[inline]
    fn add(self, other: &'a mut Self) -> Self {
        let mut result = self;
        result.add_assign(*other);
        result
    }
}

impl<'a> Sub<&'a mut Self> for Fp {
    type Output = Self;

    #[inline]
    fn sub(self, other: &'a mut Self) -> Self {
        let mut result = self;
        result.sub_assign(*other);
        result
    }
}

impl<'a> Mul<&'a mut Self> for Fp {
    type Output = Self;

    #[inline]
    fn mul(mut self, other: &'a mut Self) -> Self {
        self.mul_assign(&*other);
        self
    }
}

impl<'a> Div<&'a mut Self> for Fp {
    type Output = Self;

    #[inline]
    fn div(mut self, other: &'a mut Self) -> Self {
        self.div_assign(&*other);
        self
    }
}

impl Product<Self> for Fp {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::one(), core::ops::Mul::mul)
    }
}

impl<'a> Product<&'a Self> for Fp {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::one(), Mul::mul)
    }
}

impl Sum<Self> for Fp {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), core::ops::Add::add)
    }
}

impl<'a> Sum<&'a Self> for Fp {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), core::ops::Add::add)
    }
}

impl MulAssign<Self> for Fp {
    #[inline]
    fn mul_assign(&mut self, other: Self) {
        self.mul_assign(&other);
    }
}

impl DivAssign<Self> for Fp {
    #[inline]
    fn div_assign(&mut self, other: Self) {
        self.div_assign(&other);
    }
}

impl<'a> MulAssign<&'a Self> for Fp {
    #[inline]
    fn mul_assign(&mut self, other: &'a Self) {
        *self = Self::mul(*self, *other);
    }
}

impl<'a> MulAssign<&'a mut Self> for Fp {
    #[inline]
    fn mul_assign(&mut self, other: &'a mut Self) {
        self.mul_assign(&*other);
    }
}

impl<'a> DivAssign<&'a mut Self> for Fp {
    #[inline]
    fn div_assign(&mut self, other: &'a mut Self) {
        self.div_assign(&*other);
    }
}

/// Computes `self *= other.inverse()` if `other.inverse()` is `Some`, and
/// panics otherwise.
impl<'a> DivAssign<&'a Self> for Fp {
    #[inline]
    fn div_assign(&mut self, other: &Self) {
        self.mul_assign(&other.inverse().unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::Fp as TestField;
    use ark_algebra_test_templates::test_field;

    test_field!(generated; TestField; prime);
}
