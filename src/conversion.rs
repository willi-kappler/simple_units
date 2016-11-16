//! Conversion between units

use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::cmp::PartialEq;

use ::si_units::*;

macro_rules! convert_unit {
    ($i1:ident, $i2:ident, $e1:expr) => {
        impl From<$i1> for $i2 {
            fn from($i1(value): $i1) -> Self {
                $i2(value * $e1)
            }
        }

        impl From<$i2> for $i1 {
            fn from($i2(value): $i2) -> Self {
                $i1(value / $e1)
            }
        }
    }
}


const METER_TO_FOOT: f64 = 3.28084;
const METER_TO_YARD: f64 = 1.09361;
const METER_TO_MILE: f64 = 0.000621371;

init_unit!(Foot);
init_unit!(Yard);
init_unit!(Mile);

convert_unit!(Meter, Foot, METER_TO_FOOT);
convert_unit!(Meter, Yard, METER_TO_YARD);
convert_unit!(Meter, Mile, METER_TO_MILE);
