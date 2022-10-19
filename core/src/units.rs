use crate::{number::Number, Error};
use num::rational::Ratio;
use std::{collections::BTreeMap, fmt, ops};

#[derive(Clone, Debug, PartialEq)]
pub struct Quantity {
    pub number: Number,
    pub unit: Unit,
}

impl fmt::Display for Quantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let unit_str = self.unit.to_string();
        if unit_str.is_empty() {
            write!(f, "{}", self.number)
        } else {
            write!(f, "{} {}", self.number, self.unit)
        }
    }
}

impl Quantity {
    pub fn normalize(self) -> Self {
        Quantity {
            number: self.number * self.unit.0,
            unit: Unit(Number::one(), self.unit.1),
        }
    }

    pub fn try_convert(&self, target_unit: Unit) -> Option<Self> {
        if self.unit.1 != target_unit.1 {
            None
        } else {
            Some(Quantity {
                number: self.number.clone() * self.unit.0.clone() / target_unit.0.clone(),
                unit: target_unit,
            })
        }
    }
}

impl ops::Add for Quantity {
    type Output = Result<Self, Error>;

    fn add(self, rhs: Self) -> Self::Output {
        let Quantity {
            number: mag1,
            unit: Unit(scale1, powers1),
        } = self;

        let Quantity {
            number: mag2,
            unit: Unit(scale2, powers2),
        } = rhs;

        if powers1 != powers2 {
            return Err(Error::InvalidUnitOperation);
        }

        Ok(Quantity {
            // normalize to scale1
            number: mag1 + (mag2 * scale2 / scale1.clone()),
            unit: Unit(scale1, powers1),
        })
    }
}

impl ops::Sub for Quantity {
    type Output = Result<Self, Error>;

    fn sub(self, rhs: Self) -> Self::Output {
        let Quantity {
            number: mag1,
            unit: Unit(scale1, powers1),
        } = self;

        let Quantity {
            number: mag2,
            unit: Unit(scale2, powers2),
        } = rhs;

        if powers1 != powers2 {
            return Err(Error::InvalidUnitOperation);
        }

        Ok(Quantity {
            // normalize to scale1
            number: mag1 - (mag2 * scale2 / scale1.clone()),
            unit: Unit(scale1, powers1),
        })
    }
}

impl ops::Mul for Quantity {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let Quantity {
            number: mag1,
            unit: unit1,
        } = self;

        let Quantity {
            number: mag2,
            unit: unit2,
        } = rhs;

        Quantity {
            number: mag1 * mag2,
            unit: unit1 * unit2,
        }
    }
}

impl ops::Div for Quantity {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let Quantity {
            number: mag1,
            unit: unit1,
        } = self;

        let Quantity {
            number: mag2,
            unit: unit2,
        } = rhs;

        Quantity {
            number: mag1 / mag2,
            unit: unit1 / unit2,
        }
    }
}

impl ops::Neg for Quantity {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let Quantity { number: mag, unit } = self;
        Quantity { number: -mag, unit }
    }
}

/// Units is a derived unit with a scale and one or more base units with an exponent
/// Newton for example would be encoded as: scale 1000, [g:1, m:1, s:-2]
#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub struct Unit(pub Number, pub BTreeMap<BaseUnit, Ratio<i32>>);

impl Unit {
    pub fn unitless() -> Self {
        Self(Number::one(), BTreeMap::new())
    }

    pub fn rescaled(self, scale: Number) -> Self {
        Self(self.0 * scale, self.1)
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let magnitude = if self.0 == Number::one() {
            "".to_string()
        } else {
            format!("({}x) ", self.0)
        };

        if self.1.is_empty() {
            return write!(f, "{}", magnitude);
        }

        let positive = self
            .1
            .iter()
            .filter(|(_, ratio)| *ratio > &Ratio::new(0i32, 1i32))
            .map(|(base_unit, ratio)| {
                if *ratio == Ratio::new(1i32, 1i32) {
                    // if we have m^1, just display m
                    base_unit.to_string()
                } else {
                    format!("{}^{}", base_unit, ratio)
                }
            })
            .collect::<Vec<String>>()
            .join("");

        let pos_str = if positive.is_empty() {
            "1".into()
        } else {
            positive
        };

        let negative = self
            .1
            .iter()
            .filter(|(_, ratio)| *ratio < &Ratio::new(0i32, 1i32))
            .map(|(base_unit, ratio)| {
                if *ratio == Ratio::new(-1i32, 1i32) {
                    // if we have m^1, just display m
                    base_unit.to_string()
                } else {
                    format!("{}^{}", base_unit, -ratio)
                }
            })
            .collect::<Vec<String>>()
            .join("");

        let unit_str = if negative.is_empty() {
            pos_str
        } else {
            format!("{}/{}", pos_str, negative)
        };

        write!(f, "{}{}", magnitude, unit_str)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct BaseUnit(pub String, pub Option<String>);

impl fmt::Display for BaseUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            if let Some(short_name) = &self.1 {
                short_name
            } else {
                &self.0
            }
        )
    }
}

impl ops::Mul for Unit {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let Unit(scale1, pow1) = self;
        let Unit(scale2, pow2) = rhs;

        let scale_res = scale1 * scale2;

        let powers_res = pow1
            .keys()
            .chain(pow2.keys())
            .map(|base| {
                let exp = pow1.get(base).unwrap_or(&Ratio::new(0i32, 1i32))
                    + pow2.get(base).unwrap_or(&Ratio::new(0i32, 1i32));
                (base.clone(), exp)
            })
            .collect();

        Self(scale_res, powers_res)
    }
}

impl ops::Div for Unit {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        let Unit(scale1, pow1) = self;
        let Unit(scale2, pow2) = rhs;

        let scale_res = scale1 / scale2;

        let powers_res = pow1
            .keys()
            .chain(pow2.keys())
            .map(|base| {
                let exp = pow1.get(base).unwrap_or(&Ratio::new(0i32, 1i32))
                    - pow2.get(base).unwrap_or(&Ratio::new(0i32, 1i32));
                (base.clone(), exp)
            })
            .collect();

        Self(scale_res, powers_res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;
    use std::collections::HashMap;

    lazy_static! {
        static ref BASE_UNITS: HashMap<char, BaseUnit> = HashMap::from([
            ('m', BaseUnit("meter".to_string(), Some("m".to_string()))),
            ('g', BaseUnit("gram".to_string(), Some("g".to_string()))),
            ('s', BaseUnit("second".to_string(), Some("s".to_string())))
        ]);
        static ref UNITS: HashMap<char, Unit> = HashMap::from([
            ('0', Unit(Number::one(), [].into())),
            (
                'm',
                Unit(
                    Number::one(),
                    [(BASE_UNITS.get(&'m').unwrap().clone(), Ratio::new(1, 1))].into()
                )
            ),
            (
                'g',
                Unit(
                    Number::one(),
                    [(BASE_UNITS.get(&'g').unwrap().clone(), Ratio::new(1, 1))].into()
                )
            ),
            (
                's',
                Unit(
                    Number::one(),
                    [(BASE_UNITS.get(&'s').unwrap().clone(), Ratio::new(1, 1))].into()
                )
            ),
            (
                'N',
                Unit(
                    Number::new(1000),
                    [
                        (BASE_UNITS.get(&'m').unwrap().clone(), Ratio::new(1, 1)),
                        (BASE_UNITS.get(&'g').unwrap().clone(), Ratio::new(1, 1)),
                        (BASE_UNITS.get(&'s').unwrap().clone(), Ratio::new(-2, 1))
                    ]
                    .into()
                )
            )
        ]);
    }

    fn unit(c: char) -> Unit {
        UNITS.get(&c).unwrap().clone()
    }

    #[test]
    fn simple_formatting() {
        let ten = Quantity {
            number: Number::new(10),
            unit: unit('0'),
        };
        let five_seconds = Quantity {
            number: Number::new(5),
            unit: unit('s'),
        };
        let div = ten.clone() / five_seconds.clone();

        assert_eq!(ten.to_string(), "10");
        assert_eq!(five_seconds.to_string(), "5 s");
        assert_eq!(div.to_string(), "2 1/s")
    }

    #[test]
    fn basic_arithmetic() {
        let m = Quantity {
            number: Number::new(10_000),
            unit: unit('g'),
        };
        let l = Quantity {
            number: Number::new(1),
            unit: unit('m'),
        };
        let t = Quantity {
            number: Number::new(4),
            unit: unit('s'),
        };
        let f = Quantity {
            number: Number::new(20),
            unit: unit('N'),
        };

        assert_eq!(&l.to_string(), "1 m");
        assert_eq!(&m.to_string(), "10000 g");
        assert_eq!(&f.to_string(), "20 (1000x) gm/s^2");
        assert_eq!(&f.clone().normalize().to_string(), "20000 gm/s^2");
        assert_eq!(
            &f.clone()
                .normalize()
                .try_convert(unit('N'))
                .unwrap()
                .to_string(),
            "20 (1000x) gm/s^2"
        );
        assert!(&f.try_convert(unit('s')).is_none());

        // 10 000 g * 1 m / (4s*4s) + 20 N = 625 gm/s^2 + 20 000 gm/s^2 = 20625 gm/s^2
        let result = m * l / (t.clone() * t) + f;

        assert_eq!(result.unwrap().to_string(), "20625 gm/s^2");
    }
}
