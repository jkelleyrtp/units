// https://www.govinfo.gov/content/pkg/GOVPUB-C13-f10c2ff9e7af2091314396a2d53213e4/pdf/GOVPUB-C13-f10c2ff9e7af2091314396a2d53213e4.pdf

use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy)]
struct UnitValue<
    //
    const LENGTH: i16,
    const MASS: i16,
    const TIME: i16,
    const KELVIN: i16,
>(f64);

impl<const LENGTH: i16, const MASS: i16, const TIME: i16, const KELVIN: i16>
    UnitValue<LENGTH, MASS, TIME, KELVIN>
{
    fn per_second(&self) -> UnitValue<LENGTH, MASS, { TIME - 1 }, KELVIN> {
        todo!()
    }

    fn per_meter(&self) -> UnitValue<{ LENGTH - 1 }, MASS, TIME, KELVIN> {
        todo!()
    }

    fn per_hour(&self) -> UnitValue<LENGTH, MASS, { TIME - 1 }, KELVIN> {
        todo!()
    }

    fn squared(&self) -> UnitValue<{ LENGTH * 2 }, { MASS * 2 }, { TIME * 2 }, { KELVIN * 2 }> {
        todo!()
    }
}

pub type Unitless = UnitValue<0, 0, 0, 0>;
pub type Mass = UnitValue<0, 1, 0, 0>;
pub type Length = UnitValue<1, 0, 0, 0>;
pub type Area = UnitValue<2, 0, 0, 0>;
pub type Pressure = UnitValue<-3, 1, -2, 0>;
pub type Distance = UnitValue<1, 0, 0, 0>;
pub type Velocity = UnitValue<1, 0, -1, 0>;
pub type Acceleration = UnitValue<1, 0, -2, 0>;
pub type Force = UnitValue<-1, 1, -2, 0>;

impl<const LENGTH: i16, const MASS: i16, const TIME: i16, const KELVIN: i16> std::fmt::Debug
    for UnitValue<LENGTH, MASS, TIME, KELVIN>
{
    // regular print: in native units
    // custom print: in SI units
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // https://www.ldeo.columbia.edu/~martins/hydro/case_studies/units_dimensions.html
        let abbrev = match (LENGTH, MASS, TIME, KELVIN) {
            (1, 0, 0, 0) => "meters",
            (0, 1, 0, 0) => "kilograms",
            (0, 0, 1, 0) => "seconds",
            (0, 0, 0, 1) => "kelvin",
            _ => "",
        };

        write!(f, "{:.2} {}", self.0, abbrev)
    }
}

pub type Identitiy = UnitValue<0, 0, 0, 0>;

impl UnitValue<0, 0, 0, 0> {
    fn new(val: f64) -> Self {
        Self(val)
    }
}

trait United {
    fn unitless(&self) -> UnitValue<0, 0, 0, 0>;
    fn meters(&self) -> UnitValue<1, 0, 0, 0>;
    fn kilograms(&self) -> UnitValue<0, 1, 0, 0>;
    fn seconds(&self) -> UnitValue<0, 0, 1, 0>;
    fn kelvin(&self) -> UnitValue<0, 0, 0, 1>;
}

impl United for f64 {
    fn unitless(&self) -> UnitValue<0, 0, 0, 0> {
        todo!()
    }

    fn meters(&self) -> UnitValue<1, 0, 0, 0> {
        todo!()
    }

    fn kilograms(&self) -> UnitValue<0, 1, 0, 0> {
        todo!()
    }

    fn seconds(&self) -> UnitValue<0, 0, 1, 0> {
        todo!()
    }

    fn kelvin(&self) -> UnitValue<0, 0, 0, 1> {
        todo!()
    }
}

impl<const LENGTH: i16, const MASS: i16, const TIME: i16, const KELVIN: i16> std::ops::Add
    for UnitValue<LENGTH, MASS, TIME, KELVIN>
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        UnitValue(Add::<f64>::add(rhs.0, self.0))
    }
}

impl<const LENGTH: i16, const MASS: i16, const TIME: i16, const KELVIN: i16> std::ops::Sub
    for UnitValue<LENGTH, MASS, TIME, KELVIN>
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        UnitValue(Sub::<f64>::sub(rhs.0, self.0))
    }
}

impl<
        // LHS
        const LENGTH: i16,
        const MASS: i16,
        const TIME: i16,
        const KELVIN: i16,
        // RHS
        const R_LENGTH: i16,
        const R_MASS: i16,
        const R_TIME: i16,
        const R_KELVIN: i16,
    > std::ops::Mul<UnitValue<R_LENGTH, R_MASS, R_TIME, R_KELVIN>>
    for UnitValue<LENGTH, MASS, TIME, KELVIN>
where
    UnitValue<{ LENGTH + R_LENGTH }, { MASS + R_MASS }, { TIME + R_TIME }, { KELVIN + R_KELVIN }>:
        Sized,
{
    type Output = UnitValue<
        { LENGTH + R_LENGTH },
        { MASS + R_MASS },
        { TIME + R_TIME },
        { KELVIN + R_KELVIN },
    >;

    fn mul(self, rhs: UnitValue<R_LENGTH, R_MASS, R_TIME, R_KELVIN>) -> Self::Output {
        UnitValue(Mul::<f64>::mul(rhs.0, self.0))
    }
}

impl<
        // LHS
        const LENGTH: i16,
        const MASS: i16,
        const TIME: i16,
        const KELVIN: i16,
        // RHS
        const R_LENGTH: i16,
        const R_MASS: i16,
        const R_TIME: i16,
        const R_KELVIN: i16,
    > std::ops::Div<UnitValue<R_LENGTH, R_MASS, R_TIME, R_KELVIN>>
    for UnitValue<LENGTH, MASS, TIME, KELVIN>
where
    UnitValue<{ LENGTH - R_LENGTH }, { MASS - R_MASS }, { TIME - R_TIME }, { KELVIN - R_KELVIN }>:
        Sized,
{
    type Output = UnitValue<
        { LENGTH - R_LENGTH },
        { MASS - R_MASS },
        { TIME - R_TIME },
        { KELVIN - R_KELVIN },
    >;

    fn div(self, rhs: UnitValue<R_LENGTH, R_MASS, R_TIME, R_KELVIN>) -> Self::Output {
        UnitValue(Div::<f64>::div(rhs.0, self.0))
    }
}

impl<const LENGTH: i16, const MASS: i16, const TIME: i16, const KELVIN: i16> std::ops::Div<f64>
    for UnitValue<LENGTH, MASS, TIME, KELVIN>
where
    UnitValue<LENGTH, MASS, TIME, KELVIN>: Sized,
{
    type Output = UnitValue<LENGTH, MASS, TIME, KELVIN>;

    fn div(self, rhs: f64) -> Self::Output {
        UnitValue(Div::<f64>::div(rhs, self.0))
    }
}

impl<const LENGTH: i16, const MASS: i16, const TIME: i16, const KELVIN: i16> std::ops::Mul<f64>
    for UnitValue<LENGTH, MASS, TIME, KELVIN>
where
    UnitValue<LENGTH, MASS, TIME, KELVIN>: Sized,
{
    type Output = UnitValue<LENGTH, MASS, TIME, KELVIN>;

    fn mul(self, rhs: f64) -> Self::Output {
        UnitValue(Mul::<f64>::mul(rhs, self.0))
    }
}

#[test]
fn demo() {
    let raw: Unitless = 0.0.unitless();
    let distance: Distance = 1.0.meters();
    let kilograms: Mass = 1.0.kilograms();
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
}
