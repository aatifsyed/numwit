/// Witness types for numbers.
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

#[derive(Debug)]
pub struct NotPositive<T>(pub T);

impl<T: fmt::Display> fmt::Display for NotPositive<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("The value {} was not positive", self.0))
    }
}

impl<T: fmt::Display + fmt::Debug> std::error::Error for NotPositive<T> {}

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

#[derive(Debug)]
pub struct NotNegative<T>(pub T);

impl<T: fmt::Display> fmt::Display for NotNegative<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("The value {} was not negative", self.0))
    }
}

impl<T: fmt::Display + fmt::Debug> std::error::Error for NotNegative<T> {}

////////////////////////////////
// Positive<T> +  Positive<T> //
// Positive<T> *  Positive<T> //
// Positive<T> += Positive<T> //
// Positive<T> *= Positive<T> //
////////////////////////////////

impl<T> ops::Add<Self> for Positive<T>
where
    T: ops::Add<T, Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        // - self is positive
        // - rhs is positive
        // - adding two positive integers will result in a positive integer
        Self::new_unchecked(self.0 + rhs.0)
    }
}

impl<T> ops::Mul<Self> for Positive<T>
where
    T: ops::Mul<T, Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        // - self is positive
        // - rhs is positive
        // - multiplying two positive integers will result in a positive integer
        Self::new_unchecked(self.0 * rhs.0)
    }
}

impl<T> ops::AddAssign<Self> for Positive<T>
where
    T: ops::AddAssign<T>,
{
    fn add_assign(&mut self, rhs: Self) {
        // - self is positive
        // - rhs is positive
        // - adding two positive integers will result in a positive integer
        self.mut_unchecked().add_assign(rhs.into_inner());
    }
}

impl<T> ops::MulAssign<Self> for Positive<T>
where
    T: ops::MulAssign<T>,
{
    fn mul_assign(&mut self, rhs: Self) {
        // - self is positive
        // - rhs is positive
        // - multiplying two positive integers will result in a positive integer
        self.mut_unchecked().mul_assign(rhs.into_inner())
    }
}

//////////////////////////////////////
// Positive<UnsignedT> +  UnsignedT //
// Positive<UnsignedT> += UnsignedT //
//////////////////////////////////////

impl<UnsignedT> ops::Add<UnsignedT> for Positive<UnsignedT>
where
    UnsignedT: num::Unsigned + ops::Add<UnsignedT, Output = UnsignedT>,
{
    type Output = Self;

    fn add(self, rhs: UnsignedT) -> Self::Output {
        // - self is positive
        // - rhs is positive or zero
        // - adding two positive integers will result in a positive integer
        // - adding zero to a positive integer will result in a positive integer
        Self::new_unchecked(self.0 + rhs)
    }
}

impl<T> ops::AddAssign<T> for Positive<T>
where
    T: num::Unsigned + ops::AddAssign<T>,
{
    fn add_assign(&mut self, rhs: T) {
        // - self is positive
        // - rhs is positive or zero
        // - adding two positive integers will result in a positive integer
        // - adding zero to a positive integer will result in a positive integer
        self.mut_unchecked().add_assign(rhs)
    }
}
