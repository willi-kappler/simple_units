[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

# simple_units

A simple unit system for Rust.

It allows you to write numeric code that is not only type safe but also "unit safe".

It works on stable Rust, but unfortunately that means it doesn't scale.
All units (and combination of them) must be written by hand.
Macros help (and maybe some more macro magic can help more) but it is still a lot of work.

For me it already payed off: I found a small bug in an old Fortran program we are using ;-)

So if your favorite unit is missing just let me know.

There already is a better unit system for Rust: [Dimensioned](https://github.com/paholg/dimensioned), needs at least Rust 1.15.

And another one here: [runits](https://github.com/jesse99/runits) and here: [uom](https://github.com/iliekturtles/uom)

There is also a crate to help you convert between units: [rink-rs](https://github.com/tiffany352/rink-rs/)

## Notes

There are some blog posts / notes about unit systems in Rust:

https://www.reddit.com/r/rust/comments/37qut9/typesafe_userdefined_units_of_measure_for_rust/

https://blog.mozilla.org/research/2014/06/23/static-checking-of-units-in-servo/

https://github.com/jaheba/stuff/blob/master/communicating_intent.md

https://www.reddit.com/r/rust/comments/5uacxs/types_units_and_quantities/

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

I have plans for a better unit system, based on *build.rs* and comments: Basically that means to scan all \*.rs Rust source files and evaluate the expressions that have a special comment:

```rust
fn main() {
    let length = 20.72; // #[m]
    let time = 12.39; // #[s]

    // Resulting type: #[m / s]
    // No need to specify, will be inferred
    let velocity = length / time;

    // build.rs will give an error:
    // let error = length + time;

    // Multiply by #[s] gives you #[m]:
    let duration = 35.0; // #[s]
    let distance = velocity * duration; // This would be #[m], but could be omitted
}
```

Note the special ```#[]``` notation to describe units. Between the comment begin token and the unit description only space and tabs will be allowed. After that you can place any comments. Otherwise the unit description will be ignored as in the example above.


This will work on stable Rust.
