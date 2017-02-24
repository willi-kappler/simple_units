//! simple_units: a simple unit system for Rust
//!
//! Written by Willi Kappler, Version 0.1 (2017.02.22)
//!
//! Repository: https://github.com/willi-kappler/simple_units
//!
//! License: MIT
//!
//! Conversion between units

use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::cmp::PartialEq;

use ::si_units::*;

// TODO: move all constants to https://github.com/willi-kappler/natural_constants

const METER_TO_FOOT: f64 = 3.28084;
const METER_TO_YARD: f64 = 1.09361;
const METER_TO_MILE: f64 = 0.000621371;

const METER_TO_KILOMETER: f64 = 1.0e-3;
const METER_TO_MILIMETER: f64 = 1.0e+3;
const METER_TO_MICROMETER: f64 = 1.0e+6;
const METER_TO_NANOMETER: f64 = 1.0e+9;
const METER_TO_ASTRONOMICALUNIT: f64 = 1.0 / 149597870700.0;
const METER_TO_LIGHTYEAR: f64 =  1.0 / 9.4607e15;
const METER_TO_PARSEC: f64 = 1.0 / 3.08567758149137e16;

const SECOND_TO_MINUTE: f64 = 60.0;
const SECOND_TO_HOUR: f64 = 60.0 * 60.0;
const SECOND_TO_DAY: f64 = 60.0 * 60.0 * 24.0;
const SECOND_TO_WEEK: f64 = SECOND_TO_DAY * 7.0;
const SECOND_TO_MONTH: f64 = SECOND_TO_DAY * 30.0; // simple
const SECOND_TO_MONTH_AVG: f64 = SECOND_TO_DAY * 30.42; // average
const SECOND_TO_MONTH_AVG_LEAP: f64 = SECOND_TO_DAY * 30.50; // average leap year
const SECOND_TO_MONTH_SOLAR: f64 = SECOND_TO_DAY * 30.44; // solar calendar
const SECOND_TO_MONTH_LUNAR: f64 = SECOND_TO_DAY * 29.53; // lunar month

const SECOND_TO_YEAR: f64 = SECOND_TO_DAY * 365.25; // on average
const SECOND_TO_MIL_YEAR: f64 = SECOND_TO_YEAR * 1.0e6;

const DEGC_TO_KELVIN: f64 = 273.15;

init_unit!(Foot);
init_unit!(Yard);
init_unit!(Mile);

convert_unit!(Meter, Foot, METER_TO_FOOT);
convert_unit!(Meter, Yard, METER_TO_YARD);
convert_unit!(Meter, Mile, METER_TO_MILE);

init_unit!(Kilometer);
init_unit!(Milimeter);
init_unit!(Micrometer);
init_unit!(Nanometer);

convert_unit!(Meter, Kilometer, METER_TO_KILOMETER);
convert_unit!(Meter, Milimeter, METER_TO_MILIMETER);
convert_unit!(Meter, Micrometer, METER_TO_MICROMETER);
convert_unit!(Meter, Nanometer, METER_TO_NANOMETER);

init_unit!(AstronomicalUnit);
init_unit!(LightYear);
init_unit!(Parsec);

convert_unit!(Meter, AstronomicalUnit, METER_TO_ASTRONOMICALUNIT);
convert_unit!(Meter, LightYear, METER_TO_LIGHTYEAR);
convert_unit!(Meter, Parsec, METER_TO_PARSEC);

init_unit!(Minute);
init_unit!(Hour);
init_unit!(Day);
init_unit!(Week);
init_unit!(Month);
init_unit!(Year);
init_unit!(MilYear);

convert_unit!(Second, Minute, SECOND_TO_MINUTE);
convert_unit!(Second, Hour, SECOND_TO_HOUR);
convert_unit!(Second, Day, SECOND_TO_DAY);
convert_unit!(Second, Week, SECOND_TO_WEEK);
convert_unit!(Second, Month, SECOND_TO_MONTH);
convert_unit!(Second, Year, SECOND_TO_YEAR);
convert_unit!(Second, MilYear, SECOND_TO_MIL_YEAR);

init_unit!(Fahrenheit);

convert_unit!(DegC, Kelvin, |value| {value + DEGC_TO_KELVIN}, |value| {value - DEGC_TO_KELVIN});
convert_unit!(DegC, Fahrenheit, |value| {(value * 1.8) + 32.0}, |value| {(value - 32.0) / 1.8});
