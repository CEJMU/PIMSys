use super::vector::{F16x1, F16x16};
use core::fmt::Display;
use nalgebra::{SMatrix, SVector};

#[repr(C, align(65536))]
#[derive(Debug)]
pub struct Matrix<const X16R: usize, const X16C: usize>(pub [SMatrix<F16x16, 16, X16C>; X16R]);

#[repr(C, align(1024))]
#[derive(Debug)]
pub struct Vector<const R: usize>(pub SVector<F16x1, R>);

impl<const X16R: usize, const X16C: usize> Display for Matrix<X16R, X16C> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for block in self.0.iter() {
            block.fmt(f)?
        }
        Ok(())
    }
}

impl<const R: usize, const X16R: usize, const C: usize, const X16C: usize>
    From<SMatrix<F16x1, R, C>> for Matrix<X16R, X16C>
{
    fn from(matrix: SMatrix<F16x1, R, C>) -> Self {
        Self(core::array::from_fn(|i| {
            SMatrix::from_row_iterator(
                matrix
                    .fixed_rows::<16>(i * 16)
                    .transpose()
                    .iter()
                    .map(|e| *e)
                    .array_chunks::<16>()
                    .map(|chunk| F16x16(chunk)),
            )
        }))
    }
}
