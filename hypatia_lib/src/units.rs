use num::rational::Ratio;
use std::{collections::BTreeMap, fmt, ops};

#[derive(Clone, Debug)]
struct Quantity(f64, Unit);

impl fmt::Display for Quantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.0, self.1)
    }
}

impl ops::Add for Quantity {
    type Output = Option<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        let Quantity(mag1, Unit(scale1, powers1)) = self;
        let Quantity(mag2, Unit(scale2, powers2)) = rhs;

        if powers1 != powers2 {
            return None;
        }

        Some(Quantity(
            // normalize to scale1
            mag1 + (mag2 * scale2 / scale1),
            Unit(scale1, powers1),
        ))
    }
}

impl ops::Sub for Quantity {
    type Output = Option<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        let Quantity(mag1, Unit(scale1, powers1)) = self;
        let Quantity(mag2, Unit(scale2, powers2)) = rhs;

        if powers1 != powers2 {
            return None;
        }

        Some(Quantity(
            // normalize to scale1
            mag1 - (mag2 * scale2 / scale1),
            Unit(scale1, powers1),
        ))
    }
}

impl ops::Mul for Quantity {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let Quantity(mag1, unit1) = self;
        let Quantity(mag2, unit2) = rhs;

        Quantity(mag1 * mag2, unit1 * unit2)
    }
}

impl ops::Div for Quantity {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let Quantity(mag1, unit1) = self;
        let Quantity(mag2, unit2) = rhs;

        Quantity(mag1 / mag2, unit1 / unit2)
    }
}

/// Units is a derived unit with a scale and one or more base units with an exponent
/// Newton for example would be encoded as: scale 1000, [g:1, m:1, s:-2]
#[derive(PartialEq, PartialOrd, Clone, Debug)]
struct Unit(f64, BTreeMap<BaseUnit, Ratio<i32>>);

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}",
            if self.0 == 1.0 {
                "".to_string()
            } else {
                self.0.to_string()
            },
            self.1
                .iter()
                .map(|(base_unit, ratio)| format!("{}^{} ", base_unit, ratio))
                .collect::<String>()
        )
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
struct BaseUnit(String, String);

impl fmt::Display for BaseUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.1)
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
            ('m', BaseUnit("meter".to_string(), "m".to_string())),
            ('g', BaseUnit("gram".to_string(), "g".to_string())),
            ('s', BaseUnit("second".to_string(), "s".to_string()))
        ]);
        static ref UNITS: HashMap<char, Unit> = HashMap::from([
            (
                'm',
                Unit(
                    1.0,
                    [(BASE_UNITS.get(&'m').unwrap().clone(), Ratio::new(1, 1))].into()
                )
            ),
            (
                'g',
                Unit(
                    1.0,
                    [(BASE_UNITS.get(&'g').unwrap().clone(), Ratio::new(1, 1))].into()
                )
            ),
            (
                's',
                Unit(
                    1.0,
                    [(BASE_UNITS.get(&'s').unwrap().clone(), Ratio::new(1, 1))].into()
                )
            ),
            (
                'N',
                Unit(
                    1000.0,
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

    #[test]
    fn test() {
        // 10 000 g * 1 m / (4s*4s) + 20 N

        let m = Quantity(10_000.0, UNITS.get(&'g').unwrap().clone());
        let l = Quantity(1.0, UNITS.get(&'m').unwrap().clone());
        let t = Quantity(4.0, UNITS.get(&'s').unwrap().clone());
        let f = Quantity(20.0, UNITS.get(&'N').unwrap().clone());

        let tclone = t.clone();
        let tt = t * tclone;

        let result = m * l / tt + f;

        println!("{}", result.unwrap());
    }
}
