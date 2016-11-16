extern crate simple_units;

use simple_units::si_units::*;
use simple_units::conversion::*;

#[test]
fn test_meter_to_foot() {
    assert_eq!((Foot::from(Meter(1.0))), Foot(3.28084));
    let value: Foot = Meter(1.0).into();
    assert_eq!(value, Foot(3.28084));
