extern crate simple_units;

use simple_units::si_units::*;
use simple_units::conversion::*;

#[test]
fn test_units1() {
    // Values for Biotite
    let d0 = Meter2PerSecond(2.0e-13); // Diffusivity at infinite temperature
    let grain_size = Meter::from(Micrometer(500.0));
    let diff: PerSecond = d0 / (grain_size * grain_size );
    let energy = JoulePerMol(105.0);
    let univ_gas_const = JoulePerKelvinMol(8.3144598);

    let temperature1 = DegC(200.0);
    let temperature2 = DegC(300.0);

    let time1 = Second::from(MilYear(1.0));
    let time2 = Second::from(MilYear(2.0));

    let cooling_rate: KelvinPerSecond = Kelvin::from((temperature2 - temperature1)) / (time2 - time1);

    let temperature_k = Kelvin::from(temperature2);

    let tau: Second = univ_gas_const * (temperature_k * temperature_k) / (energy * cooling_rate);

    let geometry_factor = 27.0;

    let closure_temp: DegC = DegC::from(energy / ((geometry_factor * tau * diff).ln() * univ_gas_const));

    // assert_eq!(closure_temp, DegC(1.0));
}

