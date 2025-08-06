use super::vector::F16x16;
use crate::memory_config::NUMBER_OF_BANKS;
use nalgebra::SVector;

#[repr(C, align(512))]
#[derive(Debug)]
pub struct Vector<const X16R: usize>(pub [[F16x16; NUMBER_OF_BANKS]; X16R]);

impl<const X16R: usize> Default for Vector<X16R> {
    fn default() -> Self {
        Self([[F16x16::default(); NUMBER_OF_BANKS]; X16R])
    }
}

impl<const X16R: usize> From<SVector<F16x16, X16R>> for Vector<X16R> {
    fn from(input_vector: SVector<F16x16, X16R>) -> Self {
        let mut interleaved_vector = Self::default();

        for block_index in 0..X16R {
            let element = input_vector[block_index];
            for k in 0..NUMBER_OF_BANKS {
                interleaved_vector.0[block_index][k] = element;
            }
        }

        interleaved_vector
    }
}
