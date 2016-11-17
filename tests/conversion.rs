extern crate simple_units;

use simple_units::si_units::*;
use simple_units::conversion::*;

#[test]
fn test_meter_to_foot() {
    assert_eq!((Foot::from(Meter(1.0))), Foot(3.28084));

    let value: Foot = Meter(1.0).into();
    assert_eq!(value, Foot(3.28084));

    let value = Foot(1.0) + Meter(1.0).into();
    assert_eq!(value, Foot(4.2808399999999995));
}

#[test]
fn test_foot_to_meter() {
    assert_eq!((Meter::from(Foot(3.28084))), Meter(1.0));

    let value: Meter = Foot(3.28084).into();
    assert_eq!(value, Meter(1.0));

    let value = Meter(1.0) + Foot(3.28084).into();
    assert_eq!(value, Meter(2.0));
}

#[test]
fn test_meter_to_yard() {
    assert_eq!((Yard::from(Meter(1.0))), Yard(1.09361));

    let value: Yard = Meter(1.0).into();
    assert_eq!(value, Yard(1.09361));

    let value = Yard(1.0) + Meter(1.0).into();
    assert_eq!(value, Yard(2.09361));
}

#[test]
fn test_yard_to_meter() {
    assert_eq!((Meter::from(Yard(1.09361))), Meter(1.0));

    let value: Meter = Yard(1.09361).into();
    assert_eq!(value, Meter(1.0));

    let value = Meter(1.0) + Yard(1.09361).into();
    assert_eq!(value, Meter(2.0));
}
