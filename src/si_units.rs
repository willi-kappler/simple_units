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
    ($unit:ident) => {
        #[derive(Debug,Clone,Copy)]
        pub struct $unit(pub f64);

        impl PartialEq for $unit {
            fn eq(self: &$unit, &$unit(rhs): &$unit) -> bool {
                let $unit(lhs) = *self;
                lhs == rhs
            }
        }

        impl Add for $unit {
            type Output = $unit;

            fn add(self: $unit, $unit(rhs): $unit) -> $unit {
                let $unit(lhs) = self;
                $unit(lhs + rhs)
            }
        }

        impl Sub for $unit {
            type Output = $unit;

            fn sub(self: $unit, $unit(rhs): $unit) -> $unit {
                let $unit(lhs) = self;
                $unit(lhs - rhs)
            }
        }

        impl Mul<f64> for $unit {
            type Output = $unit;

            fn mul(self: $unit, rhs: f64) -> $unit {
                let $unit(lhs) = self;
                $unit(lhs * rhs)
            }
        }

        impl Mul<$unit> for f64 {
            type Output = $unit;

            fn mul(self: f64, $unit(rhs): $unit) -> $unit {
                $unit(self * rhs)
            }
        }

        impl Div<f64> for $unit {
            type Output = $unit;

            fn div(self: $unit, rhs: f64) -> $unit {
                let $unit(lhs) = self;
                $unit(lhs / rhs)
            }
        }

        impl Div<$unit> for $unit {
            type Output = f64;

            fn div(self: $unit, $unit(rhs): $unit) -> f64 {
                let $unit(lhs) = self;
                lhs / rhs
            }
        }
    }
}

macro_rules! mul_div_unit {
    // $unit1 * $unit2 = $unit3
    // $unit2 * $unit1 = $unit3
    // $unit1 = $unit3 / $unit2
    // $unit2 = $unit3 / $unit1
    ($unit1:ident, $unit2:ident, $unit3:ident) => {
        impl Mul<$unit2> for $unit1 {
            type Output = $unit3;

            fn mul(self: $unit1, $unit2(rhs): $unit2) -> $unit3 {
                let $unit1(lhs) = self;
                $unit3(lhs * rhs)
            }
        }

        impl Mul<$unit1> for $unit2 {
            type Output = $unit3;

            fn mul(self: $unit2, $unit1(rhs): $unit1) -> $unit3 {
                let $unit2(lhs) = self;
                $unit3(lhs * rhs)
            }
        }

        impl Div<$unit2> for $unit3 {
            type Output = $unit1;

            fn div(self: $unit3, $unit2(rhs): $unit2) -> $unit1 {
                let $unit3(lhs) = self;
                $unit1(lhs / rhs)
            }
        }

        impl Div<$unit1> for $unit3 {
            type Output = $unit2;

            fn div(self: $unit3, $unit1(rhs): $unit1) -> $unit2 {
                let $unit3(lhs) = self;
                $unit2(lhs / rhs)
            }
        }
    };

    // $unit1 * $unit1 = $unit2
    // $unit1 = $unit2 / $unit1
    ($unit1:ident, $unit2:ident) => {
        impl Mul<$unit1> for $unit1 {
            type Output = $unit2;

            fn mul(self: $unit1, $unit1(rhs): $unit1) -> $unit2 {
                let $unit1(lhs) = self;
                $unit2(lhs * rhs)
            }
        }

        impl Div<$unit1> for $unit2 {
            type Output = $unit1;

            fn div(self: $unit2, $unit1(rhs): $unit1) -> $unit1 {
                let $unit2(lhs) = self;
                $unit1(lhs / rhs)
            }
        }
    }
}

macro_rules! inverse_unit {
    ($unit1:ident, $unit2:ident) => {
        // A * B = 1
        impl Mul<$unit2> for $unit1 {
            type Output = f64;

            fn mul(self: $unit1, $unit2(rhs): $unit2) -> f64 {
                let $unit1(lhs) = self;
                lhs * rhs
            }
        }

        // B * A = 1
        impl Mul<$unit1> for $unit2 {
            type Output = f64;

            fn mul(self: $unit2, $unit1(rhs): $unit1) -> f64 {
                let $unit2(lhs) = self;
                lhs * rhs
            }
        }

        // 1 / A = B
        impl Div<$unit1> for f64 {
            type Output = $unit2;

            fn div(self: f64, $unit1(rhs): $unit1) -> $unit2 {
                $unit2(self / rhs)
            }
        }

        // 1 / B = A
        impl Div<$unit2> for f64 {
            type Output = $unit1;

            fn div(self: f64, $unit2(rhs): $unit2) -> $unit1 {
                $unit1(self / rhs)
            }
        }
    }
}

macro_rules! init_unit_and_inverse {
    ($unit1:ident, $per_unit1:ident) => {
        init_unit!($unit1);
        init_unit!($per_unit1);
        inverse_unit!($unit1, $per_unit1);
    }
}

macro_rules! power3_unit {
    ($unit1:ident, $unit2:ident, $unit3:ident, $per_unit1:ident, $per_unit2:ident, $per_unit3:ident) => {
        mul_div_unit!($unit1, $unit2);
        mul_div_unit!($unit1, $unit2, $unit3);

        mul_div_unit!($per_unit1, $per_unit2);
        mul_div_unit!($per_unit1, $per_unit2, $per_unit3);

        mul_div_unit!($unit1, $per_unit2, $per_unit1);
        mul_div_unit!($unit1, $per_unit3, $per_unit2);

        mul_div_unit!($unit2, $per_unit1, $unit1);
        mul_div_unit!($unit2, $per_unit3, $per_unit1);

        mul_div_unit!($unit3, $per_unit1, $unit2);
        mul_div_unit!($unit3, $per_unit2, $unit1);
    }
}

macro_rules! combine_unit {
    ($unit1:ident, $per_unit1:ident, $unit2:ident, $per_unit2:ident, $unit1_unit2:ident, $unit1_per_unit2:ident, $unit2_per_unit1:ident, $per_unit1_unit2:ident) => {
        mul_div_unit!($unit1, $unit2, $unit1_unit2);
        mul_div_unit!($unit1, $per_unit2, $unit1_per_unit2);

        mul_div_unit!($unit1_per_unit2, $unit2, $unit1);
        mul_div_unit!($unit1_per_unit2, $per_unit1, $per_unit2);

        mul_div_unit!($unit2_per_unit1, $unit1, $unit2);
        mul_div_unit!($unit2_per_unit1, $per_unit2, $per_unit1);

        mul_div_unit!($unit2, $per_unit1, $unit2_per_unit1);
        mul_div_unit!($per_unit1, $per_unit2, $per_unit1_unit2);

        mul_div_unit!($unit1_unit2, $per_unit1, $unit2);
        mul_div_unit!($unit1_unit2, $per_unit2, $unit1);

        mul_div_unit!($per_unit1_unit2, $unit1, $per_unit2);
        mul_div_unit!($per_unit1_unit2, $unit2, $per_unit1);
    }
}

init_unit_and_inverse!(Meter, PerMeter);
init_unit_and_inverse!(Meter2, PerMeter2);
init_unit_and_inverse!(Meter3, PerMeter3);
power3_unit!(Meter, Meter2, Meter3, PerMeter, PerMeter2, PerMeter3);

init_unit_and_inverse!(Second, PerSecond);
init_unit_and_inverse!(Second2, PerSecond2);
init_unit_and_inverse!(Second3, PerSecond3);
power3_unit!(Second, Second2, Second3, PerSecond, PerSecond2, PerSecond3);

init_unit_and_inverse!(MeterPerSecond, SecondPerMeter);
init_unit_and_inverse!(MeterPerSecond2, Second2PerMeter);
init_unit_and_inverse!(Meter2PerSecond, SecondPerMeter2);
init_unit_and_inverse!(Meter2PerSecond2, Second2PerMeter2);
init_unit_and_inverse!(MeterSecond, PerMeterSecond);
init_unit_and_inverse!(Meter2Second, PerMeter2Second);
init_unit_and_inverse!(MeterSecond2, PerMeterSecond2);
init_unit_and_inverse!(Meter2Second2, PerMeter2Second2);

combine_unit!(Meter, PerMeter, Second, PerSecond, MeterSecond, MeterPerSecond, SecondPerMeter, PerMeterSecond);
combine_unit!(Meter2, PerMeter2, Second, PerSecond, Meter2Second, Meter2PerSecond, SecondPerMeter2, PerMeter2Second);
combine_unit!(Meter, PerMeter, Second2, PerSecond2, MeterSecond2, MeterPerSecond2, Second2PerMeter, PerMeterSecond2);
combine_unit!(Meter2, PerMeter2, Second2, PerSecond2, Meter2Second2, Meter2PerSecond2, Second2PerMeter2, PerMeter2Second2);

mul_div_unit!(PerSecond2, Meter2Second, Meter2PerSecond);


init_unit_and_inverse!(Kilogram, PerKilogram);

 // momentum, impulse
init_unit_and_inverse!(KilogramMeterPerSecond, SecondPerKilogramMeter);

mul_div_unit!(Kilogram, MeterPerSecond, KilogramMeterPerSecond);

init_unit_and_inverse!(Newton, PerNewton);

mul_div_unit!(Kilogram, MeterPerSecond2, Newton);
mul_div_unit!(Newton, Second, KilogramMeterPerSecond);

init_unit_and_inverse!(Pascal, PerPascal);

mul_div_unit!(Pascal, Meter2, Newton);

init_unit_and_inverse!(Joule, PerJoule);

mul_div_unit!(Joule, Meter, Newton);

init_unit_and_inverse!(Watt, PerWatt);

mul_div_unit!(Watt, Second, Joule);

init_unit_and_inverse!(DegC, PerDegC);

init_unit_and_inverse!(Kelvin, PerKelvin);

init_unit_and_inverse!(Kelvin2, PerKelvin2);

mul_div_unit!(Kelvin, Kelvin2);

init_unit_and_inverse!(KelvinPerSecond, SecondPerKelvin);

mul_div_unit!(KelvinPerSecond, Second, Kelvin);

init_unit_and_inverse!(DegCPerSecond, SecondPerDegC);

mul_div_unit!(DegCPerSecond, Second, DegC);

init_unit!(JouleDegCPerSecond);

mul_div_unit!(DegCPerSecond, Joule, JouleDegCPerSecond);

init_unit!(JouleKelvinPerSecond);

mul_div_unit!(KelvinPerSecond, Joule, JouleKelvinPerSecond);

init_unit_and_inverse!(Mol, PerMol);

init_unit_and_inverse!(JoulePerKelvin, KelvinPerJoule);

init_unit_and_inverse!(JoulePerMol, MolPerJoule);

init_unit_and_inverse!(JoulePerKelvinMol, KelvinMolPerJoule);

init_unit_and_inverse!(JouleKelvinPerMol, MolPerJouleKelvin);

mul_div_unit!(JoulePerKelvinMol, Kelvin2, JouleKelvinPerMol);

init_unit_and_inverse!(SecondPerMol, MolPerSecond);

init_unit_and_inverse!(JouleKelvinPerMolSecond, MolSecondPerJouleKelvin);

mul_div_unit!(SecondPerMol, JouleKelvinPerSecond, JouleKelvinPerMol);

mul_div_unit!(Mol, SecondPerMol, Second);

mul_div_unit!(Mol, JoulePerKelvinMol, JoulePerKelvin);

mul_div_unit!(Kelvin, JoulePerKelvin, Joule);

mul_div_unit!(JoulePerMol, KelvinPerSecond, JouleKelvinPerMolSecond);

mul_div_unit!(Kelvin,  JoulePerKelvinMol, JoulePerMol);

mul_div_unit!(Second, JouleKelvinPerMolSecond, JouleKelvinPerMol);

