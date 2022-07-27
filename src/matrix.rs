struct Matrix<const DIMA: usize> {
    vals: [f64; DIMA],
}

impl<const DIMA: usize> Matrix<DIMA> {
    fn new(vals: [f64; DIMA]) -> Self {
        Self { vals }
    }
}

macro_rules! mat {
    (
        $(
            $val:literal
        ),*
    ) => {
        {
            Matrix::new([
                $( $val, )*
            ])
        }
    };
}

#[test]
fn demo() {
    let a = mat![1.0, 2.0, 3.0];
}
