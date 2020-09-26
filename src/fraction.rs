//! provides a means to calculate with fractions.
//!
//! ```
//! # use camel_up::fraction::Fraction;
//! let f = Fraction::new(1,2);
//! let g = Fraction::new(1,3);
//!
//! let sum = f + g;
//!
//! assert_eq!(sum, Fraction::new(5,6));
//! ```

use std::cmp::Ordering;
use std::fmt::{self, Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Fraction::new(n, d) represents the rational number n/d.
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Fraction(i64, u64);

impl Fraction {
    /// Creates a fraction.
    ///
    /// The denominator should not be zero, panics otherwise.
    pub fn new(numerator: i64, denominator: u64) -> Self {
        if denominator == 0 {
            panic!("denominator should never be 0")
        }
        let gcd = gcd(numerator.abs() as u64, denominator);
        let numerator = numerator / (gcd as i64);
        let denominator = denominator / gcd;

        Fraction(numerator, denominator)
    }

    /// Returns 0/1
    pub fn zero() -> Self {
        Fraction(0, 1)
    }

    /// returns 1/1
    pub fn one() -> Self {
        Fraction(1, 1)
    }

    fn inverse(&self) -> Self {
        Self::new(self.0.signum() * (self.1 as i64), self.0.abs() as u64)
    }
}

impl Default for Fraction {
    fn default() -> Self {
        Fraction::zero()
    }
}

impl From<i64> for Fraction {
    fn from(numerator: i64) -> Self {
        Fraction::new(numerator as i64, 1)
    }
}

impl<F> Add<F> for Fraction
where
    F: Into<Fraction> + Sized,
{
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn add(self, other: F) -> Self::Output {
        let other = other.into();

        Fraction::new(
            self.0 * (other.1 as i64) + (self.1 as i64) * other.0,
            self.1 * other.1,
        )
    }
}

impl Neg for Fraction {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Fraction::new(-self.0, self.1)
    }
}

impl<F> Sub<F> for Fraction
where
    F: Into<Fraction> + Sized,
{
    type Output = Self;

    fn sub(self, other: F) -> Self::Output {
        let other = other.into();

        self + (-other)
    }
}

impl<F> Mul<F> for Fraction
where
    F: Into<Fraction> + Sized,
{
    type Output = Self;

    fn mul(self, other: F) -> Self::Output {
        let other = other.into();

        Fraction::new(self.0 * other.0, self.1 * other.1)
    }
}

impl<F> Div<F> for Fraction
where
    F: Into<Fraction> + Sized,
{
    type Output = Option<Self>;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, other: F) -> Self::Output {
        let other = other.into();

        if other != Fraction::zero() {
            Some(self * other.inverse())
        } else {
            None
        }
    }
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b > 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    a
}

impl Display for Fraction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let (n, d) = (self.0, self.1);
        if d != 1 {
            write!(f, "{}/{}", n, d)
        } else {
            write!(f, "{}", n)
        }
    }
}

impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.1 as i64 * self.0).cmp(&(self.1 as i64 * other.0))
    }
}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fmt::Write;

    #[test]
    fn fractions_should_add_other_fraction() {
        let s = Fraction::new(1, 2);
        let t = Fraction::new(1, 3);

        let answer = s + t;

        assert_eq!(answer, Fraction::new(5, 6));
    }

    #[test]
    fn fractions_should_add_other_number() {
        let s = Fraction::new(1, 2);
        let t = 1i64;

        let answer = s + t;

        assert_eq!(answer, Fraction::new(3, 2));
    }

    #[test]
    fn fractions_should_subtract_other_fraction() {
        let s = Fraction::new(1, 2);
        let t = Fraction::new(1, 3);

        let answer = s - t;

        assert_eq!(answer, Fraction::new(1, 6));
    }

    #[test]
    fn fractions_should_multiply_other_fractions() {
        let s = Fraction::new(1, 2);
        let t = Fraction::new(1, 3);

        let answer = s * t;

        assert_eq!(answer, Fraction::new(1, 6));
    }

    #[test]
    fn fractions_should_divide_other_fractions() {
        let s = Fraction::new(1, 2);
        let t = Fraction::new(1, 3);

        let answer = s / t;

        assert_eq!(answer, Some(Fraction::new(3, 2)));
    }

    #[test]
    fn fractions_are_in_lowest_terms() {
        let answer = Fraction::new(4, 6);

        assert_eq!(answer, Fraction::new(2, 3));
    }

    #[test]
    fn fractions_are_normalized() {
        let answer = Fraction::new(0, 6);

        assert_eq!(answer, Fraction::zero());
    }

    #[test]
    fn fractions_can_be_displayed() {
        let s = Fraction::new(2, 4);
        let mut output = String::new();
        write!(output, "{}", s).expect("to write");

        assert_eq!(output, "1/2".to_owned());
    }

    #[test]
    fn fractions_can_be_ordered() {
        let mut fractions = Vec::new();
        fractions.push(Fraction::new(2, 3));
        fractions.push(Fraction::new(1, 3));
        fractions.push(Fraction::new(1, 2));
        fractions.sort();

        assert_eq!(
            fractions,
            vec![
                Fraction::new(1, 3),
                Fraction::new(1, 2),
                Fraction::new(2, 3)
            ]
        )
    }
}
