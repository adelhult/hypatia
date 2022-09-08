use lazy_static::lazy_static;
use std::collections::HashMap;

/// Long and short name
#[derive(PartialEq, Eq, Hash)]
pub struct PrefixName(pub String, pub String);

/// Base and exponent
#[derive(PartialEq, Eq, Hash)]
pub struct PrefixScale(pub u32, pub i32);

lazy_static! {
    pub static ref PREFIXES: HashMap<PrefixName, PrefixScale> = {
        let mut map = HashMap::new();
        map.insert(
            PrefixName("yotta".to_string(), "Y".to_string()),
            PrefixScale(10, 3),
        );

        map.insert(
            PrefixName("zetta".to_string(), "Z".to_string()),
            PrefixScale(10, 21),
        );

        map.insert(
            PrefixName("exa".to_string(), "E".to_string()),
            PrefixScale(10, 18),
        );

        map.insert(
            PrefixName("peta".to_string(), "P".to_string()),
            PrefixScale(10, 15),
        );

        map.insert(
            PrefixName("tera".to_string(), "T".to_string()),
            PrefixScale(10, 12),
        );

        map.insert(
            PrefixName("giga".to_string(), "G".to_string()),
            PrefixScale(10, 9),
        );

        map.insert(
            PrefixName("mega".to_string(), "M".to_string()),
            PrefixScale(10, 6),
        );

        map.insert(
            PrefixName("kilo".to_string(), "k".to_string()),
            PrefixScale(10, 3),
        );

        map.insert(
            PrefixName("hecto".to_string(), "h".to_string()),
            PrefixScale(10, 2),
        );

        map.insert(
            PrefixName("deca".to_string(), "da".to_string()),
            PrefixScale(10, 1),
        );

        map.insert(
            PrefixName("deci".to_string(), "d".to_string()),
            PrefixScale(10, -1),
        );

        map.insert(
            PrefixName("centi".to_string(), "c".to_string()),
            PrefixScale(10, -2),
        );

        map.insert(
            PrefixName("milli".to_string(), "m".to_string()),
            PrefixScale(10, -3),
        );

        map.insert(
            PrefixName("micro".to_string(), "μ".to_string()),
            PrefixScale(10, -6),
        );

        map.insert(
            PrefixName("nano".to_string(), "n".to_string()),
            PrefixScale(10, -9),
        );

        map.insert(
            PrefixName("pico".to_string(), "p".to_string()),
            PrefixScale(10, -12),
        );

        map.insert(
            PrefixName("femto".to_string(), "f".to_string()),
            PrefixScale(10, -15),
        );

        map.insert(
            PrefixName("atto".to_string(), "a".to_string()),
            PrefixScale(10, -18),
        );

        map.insert(
            PrefixName("zepto".to_string(), "z".to_string()),
            PrefixScale(10, -21),
        );

        map.insert(
            PrefixName("yocto".to_string(), "y".to_string()),
            PrefixScale(10, -24),
        );

        map.insert(
            PrefixName("kibi".to_string(), "Ki".to_string()),
            PrefixScale(2, 10),
        );

        map.insert(
            PrefixName("mibi".to_string(), "Mi".to_string()),
            PrefixScale(2, 20),
        );

        map.insert(
            PrefixName("gibi".to_string(), "Gi".to_string()),
            PrefixScale(2, 30),
        );

        map.insert(
            PrefixName("tebi".to_string(), "Ti".to_string()),
            PrefixScale(2, 40),
        );

        map.insert(
            PrefixName("pebi".to_string(), "Pi".to_string()),
            PrefixScale(2, 50),
        );

        map.insert(
            PrefixName("exbi".to_string(), "Ei".to_string()),
            PrefixScale(2, 60),
        );

        map.insert(
            PrefixName("zebi".to_string(), "Zi".to_string()),
            PrefixScale(2, 70),
        );

        map.insert(
            PrefixName("yobi".to_string(), "Yi".to_string()),
            PrefixScale(2, 80),
        );

        map
    };
}
