//! Data types and utility function for units:
//! Implemet a poor man's unit type system for now...
//!
//! Maybe we can have s.th. like this (F#) for Rust: https://en.wikibooks.org/wiki/F_Sharp_Programming/Units_of_Measure
//! Alternatively, see here:
//! https://www.reddit.com/r/rust/comments/37qut9/typesafe_userdefined_units_of_measure_for_rust/
//! https://blog.mozilla.org/research/2014/06/23/static-checking-of-units-in-servo/

use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::cmp::PartialEq;

macro_rules! init_unit {
    ($i:ident) => {
        #[derive(Debug)]
        pub struct $i(pub f64);

        impl PartialEq for $i {
            fn eq(self: &$i, &$i(rhs): &$i) -> bool {
                let $i(lhs) = *self;
                lhs == rhs
            }
        }

        impl Add for $i {
            type Output = $i;

            fn add(self: $i, $i(rhs): $i) -> $i {
                let $i(lhs) = self;
                $i(lhs + rhs)
            }
        }

        impl Sub for $i {
            type Output = $i;

            fn sub(self: $i, $i(rhs): $i) -> $i {
                let $i(lhs) = self;
                $i(lhs - rhs)
            }
        }

        impl Mul<f64> for $i {
            type Output = $i;

            fn mul(self: $i, rhs: f64) -> $i {
                let $i(lhs) = self;
                $i(lhs * rhs)
            }
        }

        impl Div<f64> for $i {
            type Output = $i;

            fn div(self: $i, rhs: f64) -> $i {
                let $i(lhs) = self;
                $i(lhs / rhs)
            }
        }

        impl Div<$i> for $i {
            type Output = f64;

            fn div(self: $i, $i(rhs): $i) -> f64 {
                let $i(lhs) = self;
                lhs / rhs
            }
        }
    }
}

macro_rules! mul_unit {
    ($i1:ident, $i2:ident, $i3:ident) => {
        impl Mul<$i2> for $i1 {
            type Output = $i3;

            fn mul(self: $i1, $i2(rhs): $i2) -> $i3 {
                let $i1(lhs) = self;
                $i3(lhs * rhs)
            }
        }
    }
}

macro_rules! div_unit {
    ($i1:ident, $i2:ident, $i3:ident) => {
        impl Div<$i2> for $i1 {
            type Output = $i3;

            fn div(self: $i1, $i2(rhs): $i2) -> $i3 {
                let $i1(lhs) = self;
                $i3(lhs / rhs)
            }
        }
    }
}

init_unit!(Meter);

init_unit!(Meter2);

mul_unit!(Meter, Meter, Meter2);

init_unit!(Second);

init_unit!(Second2);

mul_unit!(Second, Second, Second2);

init_unit!(MeterPerSecond);

div_unit!(Meter, Second, MeterPerSecond);

mul_unit!(MeterPerSecond, Second, Meter);
