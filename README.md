# simple_units

A simple unit system for Rust

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
    let distance = velocity * duration;
}
```
