//! Conversion between units

use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::cmp::PartialEq;

use ::si_units::*;

const METER_TO_FOOT: f64 = 3.28084;
const METER_TO_YARD: f64 = 1.09361;
const METER_TO_MILE: f64 = 0.000621371;

init_unit!(Foot);
init_unit!(Yard);
init_unit!(Mile);

impl From<Meter> for Foot {
    fn from(Meter(value): Meter) -> Self {
        Foot(value * METER_TO_FOOT)
    }
}

impl From<Meter> for Yard {
    fn from(Meter(value): Meter) -> Self {
        Yard(value * METER_TO_YARD)
    }
}

impl From<Meter> for Mile {
    fn from(Meter(value): Meter) -> Self {
        Mile(value * METER_TO_MILE)
    }
}
