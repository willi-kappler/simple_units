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
fn test_power3_unit() {
    assert_eq!(Meter(2.5) * Meter(3.0), Meter2(7.5));
    assert_eq!(Meter(2.0) * Meter2(3.0), Meter3(6.0));

    assert_eq!(Meter2(1.0) * Meter(3.0), Meter3(3.0));

    assert_eq!(PerMeter(2.5) * PerMeter(3.0), PerMeter2(7.5));
    assert_eq!(PerMeter(1.5) * PerMeter2(3.0), PerMeter3(4.5));

    assert_eq!(PerMeter2(2.0) * PerMeter(3.0), PerMeter3(6.0));

    assert_eq!(Meter(10.0) / Meter(5.0), 2.0);
    assert_eq!(Meter(8.0) / Meter2(4.0), PerMeter(2.0));
    assert_eq!(Meter(10.0) / Meter3(4.0), PerMeter2(2.5));

    assert_eq!(Meter2(1.0) / Meter(2.0), Meter(0.5));
    assert_eq!(Meter2(3.0) / Meter2(3.0), 1.0);
    assert_eq!(Meter2(4.5) / Meter3(1.5), PerMeter(3.0));

    assert_eq!(Meter3(1.0) / Meter(1.0), Meter2(1.0));
    assert_eq!(Meter3(5.0) / Meter2(2.0), Meter(2.5));
    assert_eq!(Meter3(8.0) / Meter3(4.0), 2.0);

    assert_eq!(Meter(5.0) * PerMeter(2.0), 10.0);
    assert_eq!(Meter(2.0) * PerMeter2(3.0), PerMeter(6.0));
    assert_eq!(Meter(3.1) * PerMeter3(2.0), PerMeter2(6.2));

    assert_eq!(Meter2(1.5) * PerMeter(2.0), Meter(3.0));
    assert_eq!(Meter2(1.8) * PerMeter2(2.0), 3.6);
    assert_eq!(Meter2(5.0) * PerMeter3(3.0), PerMeter(15.0));

    assert_eq!(Meter3(3.5) * PerMeter(2.0), Meter2(7.0));
    assert_eq!(Meter3(3.0) * PerMeter2(2.0), Meter(6.0));
    assert_eq!(Meter3(6.0) * PerMeter3(3.0), 18.0);

    assert_eq!(Meter(16.0) / PerMeter(4.0), Meter2(4.0));
    assert_eq!(Meter(9.0) / PerMeter2(3.0), Meter3(3.0));

    assert_eq!(Meter2(7.0) / PerMeter(2.0), Meter3(3.5));

    assert_eq!(PerMeter(1.2) * Meter(2.0), 2.4);
    assert_eq!(PerMeter(2.2) * Meter2(2.0), Meter(4.4));
    assert_eq!(PerMeter(3.2) * Meter3(2.0), Meter2(6.4));

    assert_eq!(PerMeter2(1.2) * Meter(2.0), PerMeter(2.4));
    assert_eq!(PerMeter2(2.2) * Meter2(2.0), 4.4);
    assert_eq!(PerMeter2(3.2) * Meter3(2.0), Meter(6.4));

    assert_eq!(PerMeter3(1.2) * Meter(2.0), PerMeter2(2.4));
    assert_eq!(PerMeter3(2.2) * Meter2(2.0), PerMeter(4.4));
    assert_eq!(PerMeter3(3.2) * Meter3(2.0), 6.4);

    assert_eq!(PerMeter(12.0) / Meter(6.0), PerMeter2(2.0));
    assert_eq!(PerMeter(12.0) / Meter2(3.0), PerMeter3(4.0));

    assert_eq!(PerMeter2(12.0) / Meter(4.0), PerMeter3(3.0));

    assert_eq!(PerMeter(18.0) / PerMeter(6.0), 3.0);
    assert_eq!(PerMeter(18.0) / PerMeter2(2.0), Meter(9.0));
    assert_eq!(PerMeter(20.0) / PerMeter3(5.0), Meter2(4.0));

    assert_eq!(PerMeter2(20.0) / PerMeter(10.0), PerMeter(2.0));
    assert_eq!(PerMeter2(24.0) / PerMeter2(6.0), 4.0);
    assert_eq!(PerMeter2(24.0) / PerMeter3(12.0), Meter(2.0));

    assert_eq!(PerMeter3(3.0) / PerMeter(3.0), PerMeter2(1.0));
    assert_eq!(PerMeter3(6.0) / PerMeter2(2.0), PerMeter(3.0));
    assert_eq!(PerMeter3(30.0) / PerMeter3(3.0), 10.0);
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
