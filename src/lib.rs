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

////////////////////////////////
// Positive<T> +  Positive<U> //
// Positive<T> *  Positive<U> //
// Positive<T> += Positive<U> //
// Positive<T> *= Positive<U> //
////////////////////////////////

impl<T, U> ops::Add<Positive<U>> for Positive<T>
where
    T: ops::Add<U, Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Positive<U>) -> Self::Output {
        // - self is positive
        // - rhs is positive
        // - adding two positive numbers will result in a positive number
        Self::new_unchecked(self.0 + rhs.0)
    }
}

impl<T, U> ops::Mul<Positive<U>> for Positive<T>
where
    T: ops::Mul<U, Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: Positive<U>) -> Self::Output {
        // - self is positive
        // - rhs is positive
        // - multiplying two positive numbers will result in a positive number
        Self::new_unchecked(self.0 * rhs.0)
    }
}

impl<T, U> ops::AddAssign<Positive<U>> for Positive<T>
where
    T: ops::AddAssign<U>,
{
    fn add_assign(&mut self, rhs: Positive<U>) {
        // - self is positive
        // - rhs is positive
        // - adding two positive numbers will result in a positive number
        self.mut_unchecked().add_assign(rhs.into_inner());
    }
}

impl<T, U> ops::MulAssign<Positive<U>> for Positive<T>
where
    T: ops::MulAssign<U>,
{
    fn mul_assign(&mut self, rhs: Positive<U>) {
        // - self is positive
        // - rhs is positive
        // - multiplying two positive numbers will result in a positive number
        self.mut_unchecked().mul_assign(rhs.into_inner())
    }
}

////////////////////////////////
// Positive<T> -  Negative<U> //
// Positive<T> -= Negative<U> //
////////////////////////////////
impl<T, U> ops::Sub<Negative<U>> for Positive<T>
where
    T: ops::Sub<U, Output = T>,
{
    type Output = Positive<T>;

    fn sub(self, rhs: Negative<U>) -> Self::Output {
        // - self is positive
        // - rhs is negative
        // - subtracting a negative number is the same as adding a positive number
        // - adding two positive numbers will result in a positive number
        Self::new_unchecked(self.0 - rhs.0)
    }
}

impl<T, U> ops::SubAssign<Negative<U>> for Positive<T>
where
    T: ops::SubAssign<U>,
{
    fn sub_assign(&mut self, rhs: Negative<U>) {
        // - self is positive
        // - rhs is negative
        // - subtracting a negative number is the same as adding a positive number
        // - adding two positive numbers will result in a positive number
        self.mut_unchecked().sub_assign(rhs.0)
    }
}

//////////////////////////////
// Positive<T> +  UnsignedT //
// Positive<T> += UnsignedT //
//////////////////////////////

impl<T, UnsignedT> ops::Add<UnsignedT> for Positive<T>
where
    UnsignedT: num::Unsigned,
    T: ops::Add<UnsignedT, Output = T>,
{
    type Output = Self;

    fn add(self, rhs: UnsignedT) -> Self::Output {
        // - self is positive
        // - rhs is positive or zero
        // - adding two positive numbers will result in a positive number
        // - adding zero to a positive number will result in a positive number
        Self::new_unchecked(self.0 + rhs)
    }
}

impl<T, UnsignedT> ops::AddAssign<UnsignedT> for Positive<T>
where
    UnsignedT: num::Unsigned,
    T: ops::AddAssign<UnsignedT>,
{
    fn add_assign(&mut self, rhs: UnsignedT) {
        // - self is positive
        // - rhs is positive or zero
        // - adding two positive numbers will result in a positive number
        // - adding zero to a positive number will result in a positive number
        self.mut_unchecked().add_assign(rhs)
    }
}
