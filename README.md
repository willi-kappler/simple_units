[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

# simple_units

A simple unit system for Rust.

It allows you to write numeric code that is not only type save but also "unit save".

It works on stable Rust, but unfortunately that means it doesn't scale.
All units (and combination of them) must we written by hand.
Macros help (and maybe some more macro magic can help more) but it is still a lot of work.

For me it already payed off: I found a small bug in an old Fortran program the we are using ;-)

So if your favorite unit is missing just let me know.

There already is a better unit system for Rust: [Dimensioned](https://github.com/paholg/dimensioned) but it needs the unstable nightly compiler version.

## Examples:

```rust
extern crate simple_units;
use simple_units::si_units::*;

fn main() {
    let length = Meter(20.72);
    let time = Second(12.39);

    // Resulting type: MeterPerSecond
    let velocity = length / time;

    // This will not compile:
    // let error = length + time;

    // Multiply by Second gives you Meter:
    let duration = Second(35.0);
    let distance: Meter = velocity * duration;
    // Type (= unit) not needed, will be inferred:
    let distance2 = velocity * duration;
}
```

Conversion is also supported:

```rust
extern crate simple_units;
use simple_units::si_units::*;
use simple_units::conversion::*;

fn main() {
    let length_in_m = Meter(20.72);
    // Explicit conversion, unit implements ``from``
    let length_in_foot = Foot::from(length_in_m);

    // Compile error:
    // let length_sum = length_in_m + length_in_foot;

    // But this works:
    let length_sum = length_in_m + length_in_foot.into();

    let temperature_in_degc = DegC(20.7);
    // You must provide the type (= unit) here
    let temperature_in_k: Kelvin = temperature_in_degc.into();
}
```

It would be nice if Rust would allow to implement the ``as`` operator. Then you could write s.th. like this:

```rust
extern crate simple_units;
use simple_units::si_units::*;
use simple_units::conversion::*;

fn main() {
    let length_in_m = Meter(20.72);
    let length_in_foot = Foot(12.56);

    // Does not work (yet ?)
    // let length_sum = length_in_m + (length_in_foot as Meter);
}
```
