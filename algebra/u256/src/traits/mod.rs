mod binary;
use crate::Zero;

pub trait SubFromAssign<Rhs = Self> {
    fn sub_from_assign(&mut self, rhs: Rhs);
}

pub trait DivRem<Rhs> {
    type Quotient;
    type Remainder;

    fn div_rem(self, rhs: Rhs) -> Option<(Self::Quotient, Self::Remainder)>;
}

pub trait InvMod: Sized {
    fn inv_mod(&self, modulus: &Self) -> Option<Self>;
}

pub trait GCD: Sized {
    fn gcd(a: &Self, b: &Self) -> Self;

    fn gcd_extended(a: &Self, b: &Self) -> (Self, Self, Self, bool);

    // TODO: LCM
}

pub trait SquareInline: Sized {
    /// **Note.** Implementers *must* add the `#[inline(always)]` attribute
    fn square_full_inline(&self) -> (Self, Self);

    /// **Note.** Implementers *must* add the `#[inline(always)]` attribute
    // Default implementation to be overridden
    #[inline(always)]
    fn square_inline(&self) -> Self {
        self.square_full_inline().0
    }

    // Optionally-inline version
    #[cfg_attr(feature = "inline", inline(always))]
    fn square_full(&self) -> (Self, Self) {
        self.square_full_inline()
    }

    // Optionally-inline version
    #[cfg_attr(feature = "inline", inline(always))]
    fn square(&self) -> Self {
        self.square_inline()
    }

    // TODO: Square_assign
}

pub trait MulInline<Rhs>: Sized {
    type High;

    /// **Note.** Implementers *must* add the `#[inline(always)]` attribute
    fn mul_full_inline(&self, rhs: Rhs) -> (Self, Self::High);

    /// **Note.** Implementers *must* add the `#[inline(always)]` attribute
    // Default implementation to be overridden
    #[inline(always)]
    fn mul_inline(&self, rhs: Rhs) -> Self {
        self.mul_full_inline(rhs).0
    }

    // Optionally-inline version
    #[cfg_attr(feature = "inline", inline(always))]
    fn mul_full(&self, rhs: Rhs) -> (Self, Self::High) {
        self.mul_full_inline(rhs)
    }
}

// TODO: Automatically derive Mul<..> traits. Maybe also MulAssign<..>

pub trait MontgomeryParameters<T: Sized> {
    /// The modulus to implement in Montgomery form
    const MODULUS: T;

    /// M64 = -MODULUS^(-1) mod 2^64
    const M64: u64;

    // R1 = 2^256 mod MODULUS
    const R1: T;

    // R2 = 2^512 mod MODULUS
    const R2: T;

    // R3 = 2^768 mod MODULUS
    const R3: T;
}

pub trait Montgomery: Zero {
    fn to_montgomery<M: MontgomeryParameters<Self>>(&self) -> Self {
        self.mul_redc::<M>(&M::R2)
    }

    fn from_montgomery<M: MontgomeryParameters<Self>>(&self) -> Self {
        // Use inline version to propagate the zeros
        Self::redc_inline::<M>(&Self::zero(), self)
    }

    /// **Note.** Implementers *must* add the `#[inline(always)]` attribute
    fn redc_inline<M: MontgomeryParameters<Self>>(lo: &Self, hi: &Self) -> Self;

    /// **Note.** Implementers *must* add the `#[inline(always)]` attribute
    fn square_redc_inline<M: MontgomeryParameters<Self>>(&self) -> Self;

    /// **Note.** Implementers *must* add the `#[inline(always)]` attribute
    fn mul_redc_inline<M: MontgomeryParameters<Self>>(&self, rhs: &Self) -> Self;

    fn inv_redc<M: MontgomeryParameters<Self>>(&self) -> Option<Self>;

    fn square_redc<M: MontgomeryParameters<Self>>(&self) -> Self {
        self.square_redc_inline::<M>()
    }

    fn mul_redc<M: MontgomeryParameters<Self>>(&self, rhs: &Self) -> Self {
        self.mul_redc_inline::<M>(rhs)
    }
}

// TODO: Mega-trait for binary rings like U256 that PrimeField can use

pub use binary::{Binary, BinaryAssignRef};

pub trait BinaryRing: Binary {}

// TODO: Factorial, Totient, Carmichael, Jacobi, Legendre, Binomial, etc.
// See https://gmplib.org/manual/Number-Theoretic-Functions.html
