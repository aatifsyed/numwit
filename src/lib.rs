//! Witness types for numbers being [`Positive`] or [`Negative`].
//!
//! | Operation | LHS | RHS | Output | Assignable? |
//! |-----------|-----|-----|--------|-------------|
//! | +         | +   | +   | +      | Y           |
//! |           | -   | -   | -      | Y           |
//! |           | +   | -   | ?      | N           |
//! |           | -   | +   | ?      | N           |
//! | -         | +   | +   | ?      | N           |
//! |           | -   | -   | ?      | N           |
//! |           | +   | -   | +      | Y           |
//! |           | -   | +   | -      | Y           |
//! | *         | +   | +   | +      | Y           |
//! |           | -   | -   | +      | N           |
//! |           | +   | -   | -      | N           |
//! |           | -   | +   | -      | Y           |
//! | /         | +   | +   | +      | Y           |
//! |           | -   | -   | +      | N           |
//! |           | +   | -   | -      | N           |
//! |           | -   | +   | -      | Y           |

use std::{fmt, ops};

/////////////////
// Positive<T> //
/////////////////

/// A guarantee that `T > 0`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, derive_more::AsRef)]
#[repr(transparent)]
pub struct Positive<T>(T);

impl<T> Positive<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
    pub fn new_unchecked(value: T) -> Self {
        Self(value)
    }
    pub fn map_unchecked(self, mut f: impl FnMut(T) -> T) -> Self {
        Self(f(self.0))
    }
    pub fn mut_unchecked(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> Positive<T>
where
    T: num::Zero + PartialOrd,
{
    pub fn new(value: T) -> Result<Self, NotPositive<T>> {
        match value > T::zero() {
            true => Ok(Self(value)),
            false => Err(NotPositive(value)),
        }
    }
    pub fn map(self, mut f: impl FnMut(T) -> T) -> Result<Self, NotPositive<T>> {
        Self::new(f(self.0))
    }
}

impl<T> num::One for Positive<T>
where
    T: num::One,
{
    fn one() -> Self {
        Self::new_unchecked(T::one())
    }
}

#[derive(Debug)]
pub struct NotPositive<T>(pub T);

impl<T: fmt::Display> fmt::Display for NotPositive<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("The value {} was not positive", self.0))
    }
}

impl<T: fmt::Display + fmt::Debug> std::error::Error for NotPositive<T> {}

impl<T> PartialEq<T> for Positive<T>
where
    T: PartialEq<T>,
{
    fn eq(&self, other: &T) -> bool {
        self.0.eq(other)
    }
}

/////////////////
// Negative<T> //
/////////////////

/// A guarantee that `T < 0`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, derive_more::AsRef)]
#[repr(transparent)]
pub struct Negative<T>(T);

impl<T> Negative<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
    pub fn new_unchecked(value: T) -> Self {
        Self(value)
    }
    pub fn map_unchecked(self, mut f: impl FnMut(T) -> T) -> Self {
        Self(f(self.0))
    }
    pub fn mut_unchecked(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> Negative<T>
where
    T: num::Zero + PartialOrd,
{
    pub fn new(value: T) -> Result<Self, NotNegative<T>> {
        match value < T::zero() {
            true => Ok(Self(value)),
            false => Err(NotNegative(value)),
        }
    }
    pub fn map(self, mut f: impl FnMut(T) -> T) -> Result<Self, NotNegative<T>> {
        Self::new(f(self.0))
    }
}

impl<T> Negative<T>
where
    T: num::One + ops::Neg<Output = T>,
{
    pub fn one() -> Self {
        Self::new_unchecked(-T::one())
    }
}

#[derive(Debug)]
pub struct NotNegative<T>(pub T);

impl<T: fmt::Display> fmt::Display for NotNegative<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("The value {} was not negative", self.0))
    }
}

impl<T: fmt::Display + fmt::Debug> std::error::Error for NotNegative<T> {}

impl<T> PartialEq<T> for Negative<T>
where
    T: PartialEq<T>,
{
    fn eq(&self, other: &T) -> bool {
        self.0.eq(other)
    }
}

// | Operation | LHS | RHS | Output |
// |-----------|-----|-----|--------|
// | +         | +   | +   | +      |
// |           | -   | -   | -      |
// |           | +   | -   | ?      |
// |           | -   | +   | ?      |

impl<LhsT, RhsT, OutT> ops::Add<Positive<RhsT>> for Positive<LhsT>
where
    LhsT: ops::Add<RhsT, Output = OutT>,
{
    type Output = Positive<OutT>;

    fn add(self, rhs: Positive<RhsT>) -> Self::Output {
        Self::Output::new_unchecked(self.0 + rhs.0)
    }
}

impl<LhsT, RhsT, OutT> ops::Add<Negative<RhsT>> for Negative<LhsT>
where
    LhsT: ops::Add<RhsT, Output = OutT>,
{
    type Output = Negative<OutT>;

    fn add(self, rhs: Negative<RhsT>) -> Self::Output {
        Self::Output::new_unchecked(self.0 + rhs.0)
    }
}

impl<LhsT, RhsT, OutT> ops::Add<Negative<RhsT>> for Positive<LhsT>
where
    LhsT: ops::Add<RhsT, Output = OutT>,
{
    type Output = OutT;

    fn add(self, rhs: Negative<RhsT>) -> Self::Output {
        self.0 + rhs.0
    }
}

impl<LhsT, RhsT, OutT> ops::Add<Positive<RhsT>> for Negative<LhsT>
where
    LhsT: ops::Add<RhsT, Output = OutT>,
{
    type Output = OutT;

    fn add(self, rhs: Positive<RhsT>) -> Self::Output {
        self.0 + rhs.0
    }
}

// | Operation | LHS | RHS | Output |
// |-----------|-----|-----|--------|
// | -         | +   | +   | ?      |
// |           | -   | -   | ?      |
// |           | +   | -   | +      |
// |           | -   | +   | -      |

impl<LhsT, RhsT, OutT> ops::Sub<Positive<RhsT>> for Positive<LhsT>
where
    LhsT: ops::Sub<RhsT, Output = OutT>,
{
    type Output = OutT;

    fn sub(self, rhs: Positive<RhsT>) -> Self::Output {
        self.0 - rhs.0
    }
}

impl<LhsT, RhsT, OutT> ops::Sub<Negative<RhsT>> for Negative<LhsT>
where
    LhsT: ops::Sub<RhsT, Output = OutT>,
{
    type Output = OutT;

    fn sub(self, rhs: Negative<RhsT>) -> Self::Output {
        self.0 - rhs.0
    }
}

impl<LhsT, RhsT, OutT> ops::Sub<Negative<RhsT>> for Positive<LhsT>
where
    LhsT: ops::Sub<RhsT, Output = OutT>,
{
    type Output = Positive<OutT>;

    fn sub(self, rhs: Negative<RhsT>) -> Self::Output {
        Self::Output::new_unchecked(self.0 - rhs.0)
    }
}

impl<LhsT, RhsT, OutT> ops::Sub<Positive<RhsT>> for Negative<LhsT>
where
    LhsT: ops::Sub<RhsT, Output = OutT>,
{
    type Output = Negative<OutT>;

    fn sub(self, rhs: Positive<RhsT>) -> Self::Output {
        Self::Output::new_unchecked(self.0 - rhs.0)
    }
}

// | Operation | LHS | RHS | Output |
// |-----------|-----|-----|--------|
// | *         | +   | +   | +      |
// |           | -   | -   | +      |
// |           | +   | -   | -      |
// |           | -   | +   | -      |

impl<LhsT, RhsT, OutT> ops::Mul<Positive<RhsT>> for Positive<LhsT>
where
    LhsT: ops::Mul<RhsT, Output = OutT>,
{
    type Output = Positive<OutT>;

    fn mul(self, rhs: Positive<RhsT>) -> Self::Output {
        Self::Output::new_unchecked(self.0 * rhs.0)
    }
}

impl<LhsT, RhsT, OutT> ops::Mul<Negative<RhsT>> for Negative<LhsT>
where
    LhsT: ops::Mul<RhsT, Output = OutT>,
{
    type Output = Positive<OutT>;

    fn mul(self, rhs: Negative<RhsT>) -> Self::Output {
        Self::Output::new_unchecked(self.0 * rhs.0)
    }
}

impl<LhsT, RhsT, OutT> ops::Mul<Negative<RhsT>> for Positive<LhsT>
where
    LhsT: ops::Mul<RhsT, Output = OutT>,
{
    type Output = Negative<OutT>;

    fn mul(self, rhs: Negative<RhsT>) -> Self::Output {
        Self::Output::new_unchecked(self.0 * rhs.0)
    }
}

impl<LhsT, RhsT, OutT> ops::Mul<Positive<RhsT>> for Negative<LhsT>
where
    LhsT: ops::Mul<RhsT, Output = OutT>,
{
    type Output = Negative<OutT>;

    fn mul(self, rhs: Positive<RhsT>) -> Self::Output {
        Self::Output::new_unchecked(self.0 * rhs.0)
    }
}

// | Operation | LHS | RHS | Output |
// |-----------|-----|-----|--------|
// | /         | +   | +   | +      |
// |           | -   | -   | +      |
// |           | +   | -   | -      |
// |           | -   | +   | -      |

impl<LhsT, RhsT, OutT> ops::Div<Positive<RhsT>> for Positive<LhsT>
where
    LhsT: ops::Div<RhsT, Output = OutT>,
{
    type Output = Positive<OutT>;

    fn div(self, rhs: Positive<RhsT>) -> Self::Output {
        Self::Output::new_unchecked(self.0 / rhs.0)
    }
}

impl<LhsT, RhsT, OutT> ops::Div<Negative<RhsT>> for Negative<LhsT>
where
    LhsT: ops::Div<RhsT, Output = OutT>,
{
    type Output = Positive<OutT>;

    fn div(self, rhs: Negative<RhsT>) -> Self::Output {
        Self::Output::new_unchecked(self.0 / rhs.0)
    }
}

impl<LhsT, RhsT, OutT> ops::Div<Negative<RhsT>> for Positive<LhsT>
where
    LhsT: ops::Div<RhsT, Output = OutT>,
{
    type Output = Negative<OutT>;

    fn div(self, rhs: Negative<RhsT>) -> Self::Output {
        Self::Output::new_unchecked(self.0 / rhs.0)
    }
}

impl<LhsT, RhsT, OutT> ops::Div<Positive<RhsT>> for Negative<LhsT>
where
    LhsT: ops::Div<RhsT, Output = OutT>,
{
    type Output = Negative<OutT>;

    fn div(self, rhs: Positive<RhsT>) -> Self::Output {
        Self::Output::new_unchecked(self.0 / rhs.0)
    }
}

//////////////
// Negation //
//////////////

impl<T, U> ops::Neg for Positive<T>
where
    T: ops::Neg<Output = U>,
{
    type Output = Negative<U>;

    fn neg(self) -> Self::Output {
        Self::Output::new_unchecked(-self.0)
    }
}

impl<T, U> ops::Neg for Negative<T>
where
    T: ops::Neg<Output = U>,
{
    type Output = Positive<U>;

    fn neg(self) -> Self::Output {
        Self::Output::new_unchecked(-self.0)
    }
}

///////////////
// AddAssign //
///////////////

impl<LhsT, RhsT> ops::AddAssign<Positive<RhsT>> for Positive<LhsT>
where
    LhsT: ops::AddAssign<RhsT>,
{
    fn add_assign(&mut self, rhs: Positive<RhsT>) {
        self.mut_unchecked().add_assign(rhs.0)
    }
}

impl<LhsT, RhsT> ops::AddAssign<Negative<RhsT>> for Negative<LhsT>
where
    LhsT: ops::AddAssign<RhsT>,
{
    fn add_assign(&mut self, rhs: Negative<RhsT>) {
        self.mut_unchecked().add_assign(rhs.0)
    }
}

///////////////
// SubAssign //
///////////////

impl<LhsT, RhsT> ops::SubAssign<Negative<RhsT>> for Positive<LhsT>
where
    LhsT: ops::SubAssign<RhsT>,
{
    fn sub_assign(&mut self, rhs: Negative<RhsT>) {
        self.mut_unchecked().sub_assign(rhs.0)
    }
}

impl<LhsT, RhsT> ops::SubAssign<Positive<RhsT>> for Negative<LhsT>
where
    LhsT: ops::SubAssign<RhsT>,
{
    fn sub_assign(&mut self, rhs: Positive<RhsT>) {
        self.mut_unchecked().sub_assign(rhs.0)
    }
}

///////////////
// MulAssign //
///////////////

impl<LhsT, RhsT> ops::MulAssign<Positive<RhsT>> for Positive<LhsT>
where
    LhsT: ops::MulAssign<RhsT>,
{
    fn mul_assign(&mut self, rhs: Positive<RhsT>) {
        self.mut_unchecked().mul_assign(rhs.0)
    }
}

impl<LhsT, RhsT> ops::MulAssign<Positive<RhsT>> for Negative<LhsT>
where
    LhsT: ops::MulAssign<RhsT>,
{
    fn mul_assign(&mut self, rhs: Positive<RhsT>) {
        self.mut_unchecked().mul_assign(rhs.0)
    }
}

///////////////
// DivAssign //
///////////////

impl<LhsT, RhsT> ops::DivAssign<Positive<RhsT>> for Positive<LhsT>
where
    LhsT: ops::DivAssign<RhsT>,
{
    fn div_assign(&mut self, rhs: Positive<RhsT>) {
        self.mut_unchecked().div_assign(rhs.0)
    }
}

impl<LhsT, RhsT> ops::DivAssign<Positive<RhsT>> for Negative<LhsT>
where
    LhsT: ops::DivAssign<RhsT>,
{
    fn div_assign(&mut self, rhs: Positive<RhsT>) {
        self.mut_unchecked().div_assign(rhs.0)
    }
}
