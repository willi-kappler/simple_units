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

macro_rules! mul_div_unit {
    // $i1 * $i2 = $i3
    // $i2 * $i1 = $i3
    // $i1 = $i3 / $i2
    // $i2 = $i3 / $i1
    ($i1:ident, $i2:ident, $i3:ident) => {
        impl Mul<$i2> for $i1 {
            type Output = $i3;

            fn mul(self: $i1, $i2(rhs): $i2) -> $i3 {
                let $i1(lhs) = self;
                $i3(lhs * rhs)
            }
        }

        impl Mul<$i1> for $i2 {
            type Output = $i3;

            fn mul(self: $i2, $i1(rhs): $i1) -> $i3 {
                let $i2(lhs) = self;
                $i3(lhs * rhs)
            }
        }

        impl Div<$i2> for $i3 {
            type Output = $i1;

            fn div(self: $i3, $i2(rhs): $i2) -> $i1 {
                let $i3(lhs) = self;
                $i1(lhs / rhs)
            }
        }

        impl Div<$i1> for $i3 {
            type Output = $i2;

            fn div(self: $i3, $i1(rhs): $i1) -> $i2 {
                let $i3(lhs) = self;
                $i2(lhs / rhs)
            }
        }
    };

    // $i1 * $i1 = $i2
    // $i1 = $i2 / $i1
    ($i1:ident, $i2:ident) => {
        impl Mul<$i1> for $i1 {
            type Output = $i2;

            fn mul(self: $i1, $i1(rhs): $i1) -> $i2 {
                let $i1(lhs) = self;
                $i2(lhs * rhs)
            }
        }

        impl Div<$i1> for $i2 {
            type Output = $i1;

            fn div(self: $i2, $i1(rhs): $i1) -> $i1 {
                let $i2(lhs) = self;
                $i1(lhs / rhs)
            }
        }
    };
}

init_unit!(Meter);

init_unit!(Meter2);

init_unit!(Meter3);

mul_div_unit!(Meter, Meter2);

mul_div_unit!(Meter, Meter2, Meter3);

init_unit!(Second);

init_unit!(Second2);

init_unit!(Second3);

mul_div_unit!(Second, Second2);

mul_div_unit!(Second, Second2, Second3);

init_unit!(MeterPerSecond);

mul_div_unit!(MeterPerSecond, Second, Meter);

init_unit!(MeterPerSecond2);

mul_div_unit!(MeterPerSecond2, Second, MeterPerSecond);

mul_div_unit!(MeterPerSecond2, Second2, Meter);

init_unit!(Meter2PerSecond);

mul_div_unit!(MeterPerSecond, Meter, Meter2PerSecond);

init_unit!(Meter2PerSecond2);

mul_div_unit!(Meter2PerSecond2, Second, Meter2PerSecond);

mul_div_unit!(Meter2PerSecond2, Second2, Meter2);

mul_div_unit!(MeterPerSecond, Meter2PerSecond2);

init_unit!(KG);

init_unit!(Newton);

mul_div_unit!(KG, MeterPerSecond2, Newton);
