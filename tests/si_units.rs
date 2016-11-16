extern crate simple_units;

use simple_units::si_units::*;

#[test]
fn test_meter_add() {
    assert_eq!(Meter(2.3) + Meter(7.5), Meter(9.8));
}

#[test]
fn test_meter_sub() {
    assert_eq!(Meter(8.1) - Meter(5.6), Meter(2.5));
}

#[test]
fn test_meter_mul_meter() {
    assert_eq!(Meter(2.5) * Meter(4.0), Meter2(10.0));
}

#[test]
fn test_meter_div_meter() {
    assert_eq!(Meter(10.0) / Meter(5.0), 2.0);
}

#[test]
fn test_meter_div_number() {
    assert_eq!(Meter(10.0) / 2.0, Meter(5.0));
}

#[test]
fn meter2_div_meter() {
    assert_eq!(Meter2(12.0) / Meter(3.0), Meter(4.0));
}

#[test]
fn meter3_div_meter() {
    assert_eq!(Meter3(18.0) / Meter(3.0), Meter2(6.0));
}

#[test]
fn meter3_div_meter2() {
    assert_eq!(Meter3(24.0) / Meter2(4.0), Meter(6.0));
}

#[test]
fn test_meter_div_second() {
    assert_eq!(Meter(15.0) / Second(3.0), MeterPerSecond(5.0));
}

#[test]
fn test_meter_per_second_mul_second() {
    assert_eq!(MeterPerSecond(3.0) * Second(2.0), Meter(6.0));
}

#[test]
fn test_meter_div_meter_per_second() {
    assert_eq!(Meter(10.0) / MeterPerSecond(2.0), Second(5.0));
}
