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
        #[derive(Debug,Clone,Copy)]
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

        impl Mul<$i> for f64 {
            type Output = $i;

            fn mul(self: f64, $i(rhs): $i) -> $i {
                $i(self * rhs)
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
    }
}

macro_rules! inverse_unit {
    ($i1:ident, $i2:ident) => {
        // A * B = 1
        impl Mul<$i2> for $i1 {
            type Output = f64;

            fn mul(self: $i1, $i2(rhs): $i2) -> f64 {
                let $i1(lhs) = self;
                lhs * rhs
            }
        }

        // B * A = 1
        impl Mul<$i1> for $i2 {
            type Output = f64;

            fn mul(self: $i2, $i1(rhs): $i1) -> f64 {
                let $i2(lhs) = self;
                lhs * rhs
            }
        }

        // 1 / A = B
        impl Div<$i1> for f64 {
            type Output = $i2;

            fn div(self: f64, $i1(rhs): $i1) -> $i2 {
                $i2(self / rhs)
            }
        }

        // 1 / B = A
        impl Div<$i2> for f64 {
            type Output = $i1;

            fn div(self: f64, $i2(rhs): $i2) -> $i1 {
                $i1(self / rhs)
            }
        }
    }
}

init_unit!(Meter);
init_unit!(PerMeter);
inverse_unit!(Meter, PerMeter);

init_unit!(Meter2);
init_unit!(PerMeter2);
inverse_unit!(Meter2, PerMeter2);

init_unit!(Meter3);
init_unit!(PerMeter3);
inverse_unit!(Meter3, PerMeter3);

mul_div_unit!(Meter, Meter2);
mul_div_unit!(Meter, Meter2, Meter3);

mul_div_unit!(PerMeter, PerMeter2);
mul_div_unit!(PerMeter, PerMeter2, PerMeter3);

init_unit!(Second);
init_unit!(PerSecond);
inverse_unit!(Second, PerSecond);

init_unit!(Second2);
init_unit!(PerSecond2);
inverse_unit!(Second2, PerSecond2);

init_unit!(Second3);
init_unit!(PerSecond3);
inverse_unit!(Second3, PerSecond3);

mul_div_unit!(Second, Second2);
mul_div_unit!(Second, Second2, Second3);

mul_div_unit!(PerSecond, PerSecond2);
mul_div_unit!(PerSecond, PerSecond2, PerSecond3);

mul_div_unit!(Second, PerSecond2, PerSecond);

init_unit!(MeterPerSecond);
init_unit!(SecondPerMeter);
inverse_unit!(MeterPerSecond, SecondPerMeter);

mul_div_unit!(MeterPerSecond, Second, Meter);
mul_div_unit!(Meter, PerSecond, MeterPerSecond);
mul_div_unit!(MeterPerSecond, PerMeter, PerSecond);

mul_div_unit!(SecondPerMeter, Meter, Second);
mul_div_unit!(Second, PerMeter, SecondPerMeter);

init_unit!(MeterPerSecond2);
init_unit!(Second2PerMeter);
inverse_unit!(MeterPerSecond2, Second2PerMeter);

mul_div_unit!(MeterPerSecond2, Second, MeterPerSecond);
mul_div_unit!(MeterPerSecond2, Second2, Meter);
mul_div_unit!(MeterPerSecond, PerSecond, MeterPerSecond2);

init_unit!(Meter2PerSecond);
init_unit!(SecondPerMeter2);
inverse_unit!(Meter2PerSecond, SecondPerMeter2);

mul_div_unit!(MeterPerSecond, Meter, Meter2PerSecond);
mul_div_unit!(Meter2, PerSecond, Meter2PerSecond);
mul_div_unit!(Meter2PerSecond, PerMeter, MeterPerSecond);

init_unit!(Meter2PerSecond2);
init_unit!(Second2PerMeter2);
inverse_unit!(Meter2PerSecond2, Second2PerMeter2);

mul_div_unit!(Meter2PerSecond2, Second, Meter2PerSecond);
mul_div_unit!(Meter2PerSecond2, Second2, Meter2);
mul_div_unit!(MeterPerSecond, Meter2PerSecond2);

init_unit!(Meter2Second);
init_unit!(PerMeter2Second);
inverse_unit!(Meter2Second, PerMeter2Second);

mul_div_unit!(Meter2, Second, Meter2Second);
mul_div_unit!(PerSecond2, Meter2Second, Meter2PerSecond);

init_unit!(Kilogram);
init_unit!(PerKilogram);
inverse_unit!(Kilogram, PerKilogram);

init_unit!(KilogramMeterPerSecond); // momentum, impulse
init_unit!(SecondPerKilogramMeter);
inverse_unit!(KilogramMeterPerSecond, SecondPerKilogramMeter);

mul_div_unit!(Kilogram, MeterPerSecond, KilogramMeterPerSecond);

init_unit!(Newton);
init_unit!(PerNewton);
inverse_unit!(Newton, PerNewton);

mul_div_unit!(Kilogram, MeterPerSecond2, Newton);
mul_div_unit!(Newton, Second, KilogramMeterPerSecond);

init_unit!(Pascal);
init_unit!(PerPascal);
inverse_unit!(Pascal, PerPascal);

mul_div_unit!(Pascal, Meter2, Newton);

init_unit!(Joule);
init_unit!(PerJoule);
inverse_unit!(Joule, PerJoule);

mul_div_unit!(Joule, Meter, Newton);

init_unit!(Watt);
init_unit!(PerWatt);
inverse_unit!(Watt, PerWatt);

mul_div_unit!(Watt, Second, Joule);

init_unit!(DegC);
init_unit!(PerDegC);
inverse_unit!(DegC, PerDegC);

init_unit!(Kelvin);
init_unit!(PerKelvin);
inverse_unit!(Kelvin, PerKelvin);

init_unit!(Kelvin2);
init_unit!(PerKelvin2);
inverse_unit!(Kelvin2, PerKelvin2);

mul_div_unit!(Kelvin, Kelvin2);

init_unit!(KelvinPerSecond);
init_unit!(SecondPerKelvin);
inverse_unit!(KelvinPerSecond, SecondPerKelvin);

mul_div_unit!(KelvinPerSecond, Second, Kelvin);

init_unit!(DegCPerSecond);
init_unit!(SecondPerDegC);
inverse_unit!(DegCPerSecond, SecondPerDegC);

mul_div_unit!(DegCPerSecond, Second, DegC);

init_unit!(JouleDegCPerSecond);

mul_div_unit!(DegCPerSecond, Joule, JouleDegCPerSecond);

init_unit!(JouleKelvinPerSecond);

mul_div_unit!(KelvinPerSecond, Joule, JouleKelvinPerSecond);

init_unit!(Mol);
init_unit!(PerMol);
inverse_unit!(Mol, PerMol);

init_unit!(JoulePerKelvin);
init_unit!(KelvinPerJoule);
inverse_unit!(JoulePerKelvin, KelvinPerJoule);

init_unit!(JoulePerMol);
init_unit!(MolPerJoule);
inverse_unit!(JoulePerMol, MolPerJoule);

init_unit!(JoulePerKelvinMol);
init_unit!(KelvinMolPerJoule);
inverse_unit!(JoulePerKelvinMol, KelvinMolPerJoule);

init_unit!(JouleKelvinPerMol);
init_unit!(MolPerJouleKelvin);
inverse_unit!(JouleKelvinPerMol, MolPerJouleKelvin);

mul_div_unit!(JoulePerKelvinMol, Kelvin2, JouleKelvinPerMol);

init_unit!(SecondPerMol);
init_unit!(MolPerSecond);
inverse_unit!(SecondPerMol, MolPerSecond);

mul_div_unit!(SecondPerMol, JouleKelvinPerSecond, JouleKelvinPerMol);

mul_div_unit!(Mol, SecondPerMol, Second);

mul_div_unit!(Mol, JoulePerKelvinMol, JoulePerKelvin);

mul_div_unit!(Kelvin, JoulePerKelvin, Joule);
