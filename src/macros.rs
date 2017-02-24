//! simple_units: a simple unit system for Rust
//!
//! Written by Willi Kappler, Version 0.1 (2017.02.22)
//!
//! Repository: https://github.com/willi-kappler/simple_units
//!
//! License: MIT
//!

/// This macro initializes a new unit
///
/// # Example:
///
/// ```
/// # #[macro_use] extern crate simple_units;
/// # use std::ops::Add;
/// # use std::ops::Sub;
/// # use std::ops::Mul;
/// # use std::ops::Div;
/// # use std::cmp::PartialEq;
/// # fn main() {
/// init_unit!(Meter);
/// # }
/// ```
#[macro_export] macro_rules! init_unit {
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

/// This macro implements multiplication and division for a given unit
///
/// # Example:
///
/// ```
/// # #[macro_use] extern crate simple_units;
/// # use std::ops::Add;
/// # use std::ops::Sub;
/// # use std::ops::Mul;
/// # use std::ops::Div;
/// # use std::cmp::PartialEq;
/// # fn main() {
/// init_unit!(Joule);
/// init_unit!(Meter);
/// init_unit!(Newton);
/// mul_div_unit!(Joule, Meter, Newton);
/// # }
/// ```
#[macro_export] macro_rules! mul_div_unit {
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

/// This macro implements the inverse of the given unit
///
/// # Example:
///
/// ```
/// # #[macro_use] extern crate simple_units;
/// # use std::ops::Add;
/// # use std::ops::Sub;
/// # use std::ops::Mul;
/// # use std::ops::Div;
/// # use std::cmp::PartialEq;
/// # fn main() {
/// init_unit!(Meter);
/// init_unit!(PerMeter);
/// inverse_unit!(Meter, PerMeter);
/// # }
/// ```
#[macro_export] macro_rules! inverse_unit {
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

/// This macro implements a new unit and the inverse unit
///
/// # Example:
///
/// ```
/// # #[macro_use] extern crate simple_units;
/// # use std::ops::Add;
/// # use std::ops::Sub;
/// # use std::ops::Mul;
/// # use std::ops::Div;
/// # use std::cmp::PartialEq;
/// # fn main() {
/// init_unit_and_inverse!(Meter, PerMeter);
/// # }
/// ```
#[macro_export] macro_rules! init_unit_and_inverse {
    ($unit1:ident, $per_unit1:ident) => {
        init_unit!($unit1);
        init_unit!($per_unit1);
        inverse_unit!($unit1, $per_unit1);
    }
}

/// This macro implements unit^1, unit^2, unit^3, 1/unit^1, 1/unit^2 and 1/unit^3
///
/// # Example:
///
/// ```
/// # #[macro_use] extern crate simple_units;
/// # use std::ops::Add;
/// # use std::ops::Sub;
/// # use std::ops::Mul;
/// # use std::ops::Div;
/// # use std::cmp::PartialEq;
/// # fn main() {
/// init_unit!(Second);
/// init_unit!(Second2);
/// init_unit!(Second3);
/// init_unit!(PerSecond);
/// init_unit!(PerSecond2);
/// init_unit!(PerSecond3);
/// power3_unit!(Second, Second2, Second3, PerSecond, PerSecond2, PerSecond3);
/// # }
/// ```
#[macro_export] macro_rules! power3_unit {
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

/// This macro combines two units and some permutation / inverse
///
/// # Example:
///
/// ```
/// # #[macro_use] extern crate simple_units;
/// # use std::ops::Add;
/// # use std::ops::Sub;
/// # use std::ops::Mul;
/// # use std::ops::Div;
/// # use std::cmp::PartialEq;
/// # fn main() {
/// init_unit!(Meter);
/// init_unit!(PerMeter);
/// init_unit!(Second);
/// init_unit!(PerSecond);
/// init_unit!(MeterSecond);
/// init_unit!(MeterPerSecond);
/// init_unit!(SecondPerMeter);
/// init_unit!(PerMeterSecond);
/// combine_unit!(Meter, PerMeter, Second, PerSecond, MeterSecond, MeterPerSecond, SecondPerMeter, PerMeterSecond);
/// # }
/// ```
#[macro_export] macro_rules! combine_unit {
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

/// This macro implements *From* for the given units.
/// It either uses a factor or a closure to calculate the conversion
///
/// # Example:
///
/// ```
/// # #[macro_use] extern crate simple_units;
/// # use std::ops::Add;
/// # use std::ops::Sub;
/// # use std::ops::Mul;
/// # use std::ops::Div;
/// # use std::cmp::PartialEq;
/// # fn main() {
/// const factor: f64 = 60.0;
/// init_unit!(Second);
/// init_unit!(Minute);
/// convert_unit!(Second, Minute, factor);
/// # }
/// ```
#[macro_export] macro_rules! convert_unit {
    // Convert from $i1 to $i2 using closure $e1
    // Convert from $i2 to $i1 using closure $e2
    ($i1:ident, $i2:ident, $e1:expr, $e2:expr) => {
        impl From<$i1> for $i2 {
            fn from($i1(value): $i1) -> Self {
                $i2($e1(value))
            }
        }

        impl From<$i2> for $i1 {
            fn from($i2(value): $i2) -> Self {
                $i1($e2(value))
            }
        }
    };

    // Convert from $i1 to $i2 using factor $i3
    // $i2 = $i1 * $i3
    ($i1:ident, $i2:ident, $i3:ident) => {
        impl From<$i1> for $i2 {
            fn from($i1(value): $i1) -> Self {
                $i2(value * $i3)
            }
        }

        impl From<$i2> for $i1 {
            fn from($i2(value): $i2) -> Self {
                $i1(value / $i3)
            }
        }
    };

}
