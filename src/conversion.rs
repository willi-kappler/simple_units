//! Conversion between units

#[derive(Debug)]
pub struct Foot(pub f64);

#[derive(Debug)]
pub struct Yard(pub f64);

#[derive(Debug)]
pub struct Mile(pub f64);

impl From<Meter> for Foot {
    fn from(Meter(value): Meter) -> Self {
        Foot(value * 3.28084)
    }
}

impl From<Meter> for Yard {
    fn from(Meter(value): Meter) -> Self {
        Yard(value * 1.09361)
    }
}

impl From<Meter> for Mile {
    fn from(Meter(value): Meter) -> Self {
        Mile(value * 0.000621371)
    }
}
