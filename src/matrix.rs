use std::ops::Sub;

/// A matrix is a two-dimensional array of values.
///
/// It is not an n-dimensional array, but a two-dimensional array.
///
/// For N-dimensional arrays, see [`Array`], which uses a different, less ergonomic API for dealing with larger datasets.
#[derive(Clone, Copy)]
struct Matrix<
    const X: usize = 1,
    const Y: usize = 1,
    const LENGTH: i16 = 0,
    const MASS: i16 = 0,
    const TIME: i16 = 0,
    const KELVIN: i16 = 0,
> where
    [f64; X * Y]: Sized,
{
    vals: [f64; X * Y],
}

impl<
        const ROW: usize,
        const COL: usize,
        const LENGTH: i16,
        const MASS: i16,
        const TIME: i16,
        const KELVIN: i16,
    > std::fmt::Debug for Matrix<ROW, COL, LENGTH, MASS, TIME, KELVIN>
where
    [f64; ROW * COL]: Sized,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Print as regular number or print as matrix
        if ROW == 1 && COL == 1 {
            //
            write!(f, "{:.2}", self.vals[0])?;
        } else {
            write!(f, "[")?;

            for row in 0..ROW {
                write!(f, "[")?;
                for col in 0..COL {
                    write!(f, "{:.2}", self.vals[row * COL + col])?;
                    if col < Sub::sub(COL, 1) {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")?;
                if row < Sub::sub(ROW, 1) {
                    write!(f, ", ")?;
                }
            }

            write!(f, "]")?;
        }

        match (LENGTH, MASS, TIME, KELVIN) {
            (0, 0, 0, 0) => write!(f, "")?,
            (1, 0, 0, 0) => write!(f, " meters")?,
            (0, 1, 0, 0) => write!(f, " kilograms")?,
            (0, 0, 1, 0) => write!(f, " seconds")?,
            (0, 0, 0, 1) => write!(f, " kelvin")?,
            _ => write!(f, "")?,
        };

        Ok(())

        // f.debug_struct("Matrix").field("vals", &self.vals).finish()
    }
}

impl<
        const X: usize,
        const Y: usize,
        const LENGTH: i16,
        const MASS: i16,
        const TIME: i16,
        const KELVIN: i16,
    > Matrix<X, Y, LENGTH, MASS, TIME, KELVIN>
where
    [f64; X * Y]: Sized,
{
    fn new(vals: [f64; X * Y]) -> Self {
        Self { vals }
    }

    fn sin(self) -> Self {
        todo!()
    }
    fn cos(self) -> Self {
        todo!()
    }

    fn per_second(&self) -> Matrix<X, Y, LENGTH, MASS, { TIME - 1 }, KELVIN> {
        todo!()
    }

    fn per_meter(&self) -> Matrix<X, Y, { LENGTH - 1 }, MASS, TIME, KELVIN> {
        todo!()
    }

    fn per_hour(&self) -> Matrix<X, Y, LENGTH, MASS, { TIME - 1 }, KELVIN> {
        todo!()
    }

    fn squared(&self) -> Matrix<X, Y, { LENGTH * 2 }, { MASS * 2 }, { TIME * 2 }, { KELVIN * 2 }> {
        todo!()
    }

    fn magnitude(&self) -> Matrix<1, 1, LENGTH, MASS, TIME, KELVIN> {
        let mut out = 0.0;

        for i in 0..X * Y {
            out += self.vals[i] * self.vals[i];
        }

        out = out.sqrt();

        Matrix::new([out])
    }
}

// 1d
pub type Unitless = Matrix<1, 1, 0, 0, 0, 0>;
pub type Mass = Matrix<1, 1, 0, 1, 0, 0>;
pub type Time = Matrix<1, 1, 0, 0, 1, 0>;
pub type Length = Matrix<1, 1, 1, 0, 0, 0>;
pub type Area = Matrix<1, 1, 2, 0, 0, 0>;
pub type Pressure = Matrix<1, 1, -3, 1, -2, 0>;
pub type Distance = Matrix<1, 1, 1, 0, 0, 0>;
pub type Velocity = Matrix<1, 1, 1, 0, -1, 0>;
pub type Acceleration = Matrix<1, 1, 1, 0, -2, 0>;
pub type Force = Matrix<1, 1, -1, 1, -2, 0>;

// 3d
pub type Unitless3 = Matrix<1, 3, 0, 0, 0, 0>;
pub type Mass3 = Matrix<1, 3, 0, 1, 0, 0>;
pub type Length3 = Matrix<1, 3, 1, 0, 0, 0>;
pub type Area3 = Matrix<1, 3, 2, 0, 0, 0>;
pub type Pressure3 = Matrix<1, 3, -3, 1, -2, 0>;
pub type Distance3 = Matrix<1, 3, 1, 0, 0, 0>;
pub type Velocity3 = Matrix<1, 3, 1, 0, -1, 0>;
pub type Acceleration3 = Matrix<1, 3, 1, 0, -2, 0>;
pub type Force3 = Matrix<1, 3, -1, 1, -2, 0>;

impl<
        const X: usize,
        const Y: usize,
        const LENGTH: i16,
        const MASS: i16,
        const TIME: i16,
        const KELVIN: i16,
    > std::ops::Mul<f64> for Matrix<X, Y, LENGTH, MASS, TIME, KELVIN>
where
    Matrix<X, Y, LENGTH, MASS, TIME, KELVIN>: Sized,
    [f64; X * Y]: Sized,
{
    type Output = Matrix<X, Y, LENGTH, MASS, TIME, KELVIN>;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut vals = self.vals;
        for val in vals.iter_mut() {
            *val *= rhs;
        }

        Matrix { vals }
    }
}

impl<
        // LHS
        const X: usize,
        const Y: usize,
        const LENGTH: i16,
        const MASS: i16,
        const TIME: i16,
        const KELVIN: i16,
        // RHS
        // const R_X: usize,
        const R_Y: usize,
        const R_LENGTH: i16,
        const R_MASS: i16,
        const R_TIME: i16,
        const R_KELVIN: i16,
    > std::ops::Mul<Matrix<Y, R_Y, R_LENGTH, R_MASS, R_TIME, R_KELVIN>>
    for Matrix<X, Y, LENGTH, MASS, TIME, KELVIN>
where
    Matrix<
        X,
        Y,
        { LENGTH + R_LENGTH },
        { MASS + R_MASS },
        { TIME + R_TIME },
        { KELVIN + R_KELVIN },
    >: Sized,
    [f64; X * Y]: Sized,
    [f64; Y * R_Y]: Sized,
    [f64; X * R_Y]: Sized,
{
    type Output = Matrix<
        X,
        R_Y,
        { LENGTH + R_LENGTH },
        { MASS + R_MASS },
        { TIME + R_TIME },
        { KELVIN + R_KELVIN },
    >;

    fn mul(self, rhs: Matrix<Y, R_Y, R_LENGTH, R_MASS, R_TIME, R_KELVIN>) -> Self::Output {
        let mut vals = [0.0; { X * R_Y }];

        // perform matrix multiplication
        for i in 0..X {
            for j in 0..R_Y {
                for k in 0..Y {
                    vals[i * R_Y + j] += self.vals[i * Y + k] * rhs.vals[k * R_Y + j];
                }
            }
        }

        Matrix { vals }
    }
}

#[test]
fn demo() {
    let a = Matrix::<1, 1>::new([1.0]);

    // a.sin();

    let b = Matrix::<3, 2>::new([5.0, 7.0, 12.0, 3.0, 6.0, 2.0]);
    let d = b * 5.0;

    let velocity = Velocity3::new([1.0, 2.0, 3.0]);
    let time = Time::new([5.0]);

    let displacement = time * velocity;
    let distance: Distance = displacement.magnitude();

    dbg!(distance);
}
