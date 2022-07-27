#![feature(generic_const_exprs)]

use std::marker::PhantomData;

mod matrix;
mod value;

struct Matrix<T> {
    dims: PhantomData<T>,
}

struct Unit<
    //
    const LENGTH: i16,
    const MASS: i16,
    const TIME: i16,
    const KELVIN: i16,
> {}

#[test]
fn demo() {}
