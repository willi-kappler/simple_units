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
fn test_meter_mul_number() {
    assert_eq!(Meter(3.0) * 4.0, Meter(12.0));
}

#[test]
fn test_number_mul_meter() {
    assert_eq!(4.0 * Meter(3.0), Meter(12.0));
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
fn test_inverse() {
    assert_eq!(Meter(2.4) * PerMeter(2.0), 4.8);
    assert_eq!(5.0 / Meter(2.0), PerMeter(2.5));
}

#[test]
fn test_power3() {
    assert_eq!(Meter(2.5) * Meter(2.0), Meter2(5.0));
    assert_eq!(Meter2(1.5) * Meter(2.0), Meter3(3.0));
    assert_eq!(Meter(1.5) * Meter2(2.0), Meter3(3.0));

    assert_eq!(Meter2(9.0) / Meter(3.0), Meter(3.0));
    assert_eq!(Meter3(15.0) / Meter(2.0), Meter2(7.5));
    assert_eq!(Meter3(8.4) / Meter2(2.0), Meter(4.2));

    assert_eq!(PerMeter(2.5) * PerMeter(2.0), PerMeter2(5.0));
    assert_eq!(PerMeter2(1.5) * PerMeter(2.0), PerMeter3(3.0));
    assert_eq!(PerMeter(1.5) * PerMeter2(2.0), PerMeter3(3.0));



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
