use num::{
    bigint::{BigInt, ToBigInt},
    BigRational, ToPrimitive, Num,
};
use std::{fmt, ops, str::FromStr};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Number {
    Exact(BigRational),
    Approx(f64),
}

impl Number {
    pub fn new(integer: i64) -> Self {
        Exact(BigRational::new(
            integer.to_bigint().unwrap(),
            1.to_bigint().unwrap(),
        ))
    }

    /// Convert something like "123.2" into 1232/10
    pub fn from_decimal_str(s: &str) -> Self {
        match s.split_once('.') {
            Some((integer, decimal)) => Exact(BigRational::new(
                BigInt::from_str(&format!("{integer}{decimal}")).unwrap(),
                10.to_bigint().unwrap().pow(decimal.chars().count() as u32),
            )),
            None => Number::new(s.parse::<i64>().expect("Could not parse as a number")),
        }
    }

    /// Convert a string written in engineering/scientific form 1.5e3
    pub fn from_scientific_str(decimal: &str, exp: &str, is_negative: bool) -> Self {
        let decimal = Self::from_decimal_str(decimal);

        // 10 ^ exp
        let exp = u32::from_str_radix(exp, 10).unwrap();
        let number = 10.to_bigint().unwrap().pow(exp);

        let scaling = Exact(if is_negative {
            // 1 / 10^number
            BigRational::new(1.to_bigint().unwrap(), number)
        } else {
            // 10^number / 1
            BigRational::new(number, 1.to_bigint().unwrap())
        });

        decimal * scaling
    }

    /// Convert a binary string like "01010" into a Number
    pub fn from_binary_str(s: &str) -> Self {
        Self::from_radix_str(s, 2)
    }

    /// Convert a hex string like "12ABC" into a Number
    pub fn from_hex_str(s: &str) -> Self {
        Self::from_radix_str(s, 16)
    }

    /// Convert a string in a given base to a Number
    fn from_radix_str(s: &str, radix: u32) -> Self {
        Exact(BigRational::new(
            BigInt::from_str_radix(s, radix).expect("Not a base 2 number"),
            1.to_bigint().unwrap(),
        ))
    }

    pub fn one() -> Self {
        Self::new(1)
    }

    pub fn into_approx(self) -> Self {
        if let Exact(n) = self {
            Self::Approx(n.to_f64().expect("Cannot represent number as f64"))
        } else {
            self
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Exact(n) => write!(f, "{}", n),
            Approx(n) => write!(f, "{}", n),
        }
    }
}

use Number::*;

impl ops::Add for Number {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Exact(a), Exact(b)) => Exact(a + b),
            (Approx(a), Approx(b)) => Approx(a + b),
            // If they both are not of the same form, convert the number into approximate form
            (a, b) => a.into_approx() + b.into_approx(),
        }
    }
}

impl ops::Sub for Number {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Exact(a), Exact(b)) => Exact(a - b),
            (Approx(a), Approx(b)) => Approx(a - b),
            // If they both are not of the same form, convert the number into approximate form
            (a, b) => a.into_approx() - b.into_approx(),
        }
    }
}

impl ops::Neg for Number {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Exact(a) => Exact(-a),
            Approx(a) => Approx(-a),
        }
    }
}

impl ops::Mul for Number {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Exact(a), Exact(b)) => Exact(a * b),
            (Approx(a), Approx(b)) => Approx(a * b),
            // If they both are not of the same form, convert the number into approximate form
            (a, b) => a.into_approx() * b.into_approx(),
        }
    }
}

impl ops::Div for Number {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Exact(a), Exact(b)) => Exact(a / b),
            (Approx(a), Approx(b)) => Approx(a / b),
            // If they both are not of the same form, convert the number into approximate form
            (a, b) => a.into_approx() / b.into_approx(),
        }
    }
}
