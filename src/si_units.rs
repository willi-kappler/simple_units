//! simple_units: a simple unit system for Rust
//!
//! Written by Willi Kappler, Version 0.1 (2017.02.22)
//!
//! Repository: https://github.com/willi-kappler/simple_units
//!
//! License: MIT
//!

use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::cmp::PartialEq;

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

// TODO: add more units...
