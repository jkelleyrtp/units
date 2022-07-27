# Matricies and Units using nightly const generics

This proof of concept showcases a new approach to units and matricies in Rust. All operations (cross products, dot products, math) is checked at compile time.

It also leverages const generics to perform highly optimized variants of various matrix math.

For units:
```rust
let raw = 0.0.unitless();
let distance = 1.0.meters();
let kilograms = 1.0.kilograms();

let velocity: Velocity = 1.0.meters().per_second();
let acceleration: Acceleration = 1.0.meters().per_second().per_second();
let force: Force = kilograms.per_meter().per_second().per_second();

let a = force + force;
let a = acceleration + acceleration;

let area: Area = 1.0.meters().squared();
let pressure: Pressure = force / area;

let scaled: Distance = distance / 2.0;
let scaled: Distance = distance * 2.0;

dbg!(acceleration);
```


For matricies:
```rust
let nums = mat![
    1.0, 2.0;
    3.0, 4.0
];
```
