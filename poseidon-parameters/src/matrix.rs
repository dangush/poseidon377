use core::convert::TryInto;
use core::ops::Mul;

use crate::error::PoseidonParameterError;
use crate::matrix_ops::{MatrixOperations, SquareMatrixOperations};
use decaf377::Fq;

/// Represents a matrix over `PrimeField` elements.
///
/// This matrix can be used to represent row or column
/// vectors.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Matrix<const N_ROWS: usize, const N_COLS: usize, const N_ELEMENTS: usize> {
    /// Elements of the matrix, stored in a fixed-size array.
    ///
    pub elements: [Fq; N_ELEMENTS],
}

impl<const N_ROWS: usize, const N_COLS: usize, const N_ELEMENTS: usize> MatrixOperations
    for Matrix<N_ROWS, N_COLS, N_ELEMENTS>
{
    fn new(elements: &[Fq]) -> Self {
        // Note: We use a third const generic to denote the number of elements in the
        // matrix here due to `generic_const_exprs` being an unstable Rust feature at
        // the time of writing.
        if N_ELEMENTS != N_ROWS * N_COLS {
            panic!("Matrix has an insufficient number of elements")
        }

        let elements: [Fq; N_ELEMENTS] = elements
            .try_into()
            .expect("Matrix has the correct number of elements");

        Self { elements }
    }

    fn elements(&self) -> &[Fq] {
        &self.elements
    }

    fn get_element(&self, i: usize, j: usize) -> Fq {
        self.elements[i * N_COLS + j]
    }

    fn set_element(&mut self, i: usize, j: usize, val: Fq) {
        self.elements[i * N_COLS + j] = val
    }

    fn rows(&self) -> &[&[Fq]] {
        // self.elements.chunks(self.n_cols()).collect()
        todo!()
    }

    fn n_rows(&self) -> usize {
        N_ROWS
    }

    fn n_cols(&self) -> usize {
        N_COLS
    }

    fn transpose(&self) -> Self {
        let mut transposed_elements = [Fq::default(); N_ELEMENTS];

        let mut index = 0;
        for j in 0..self.n_cols() {
            for i in 0..self.n_rows() {
                transposed_elements[index] = self.get_element(i, j);
                index += 1;
            }
        }
        Self::new(&transposed_elements)
    }

    fn hadamard_product(&self, rhs: &Self) -> Result<Self, PoseidonParameterError>
    where
        Self: Sized,
    {
        if self.n_rows() != rhs.n_rows() || self.n_cols() != rhs.n_cols() {
            return Err(PoseidonParameterError::InvalidMatrixDimensions);
        }

        let mut new_elements = [Fq::default(); N_ELEMENTS];
        let mut index = 0;
        for i in 0..self.n_rows() {
            for j in 0..self.n_cols() {
                new_elements[index] = self.get_element(i, j) * rhs.get_element(i, j);
                index += 1;
            }
        }

        Ok(Self::new(&new_elements))
    }
}

/// Multiply scalar by Matrix
impl<const N_ROWS: usize, const N_COLS: usize, const N_ELEMENTS: usize> Mul<Fq>
    for Matrix<N_ROWS, N_COLS, N_ELEMENTS>
{
    type Output = Matrix<N_ROWS, N_COLS, N_ELEMENTS>;

    fn mul(self, rhs: Fq) -> Self::Output {
        let elements = self.elements();
        let mut new_elements = [Fq::default(); N_ELEMENTS];
        for (i, &element) in elements.iter().enumerate() {
            new_elements[i] = element * rhs;
        }
        Self::new(&new_elements)
    }
}

impl<const N_ROWS: usize, const N_COLS: usize, const N_ELEMENTS: usize>
    Matrix<N_ROWS, N_COLS, N_ELEMENTS>
{
    /// Get row vector at a specified row index
    pub fn row_vector(&self, i: usize) -> Matrix<1, N_COLS, N_ELEMENTS> {
        let mut row_elements = [Fq::default(); N_COLS];
        for j in 0..N_COLS {
            row_elements[j] = self.get_element(i, j);
        }
        Matrix::new(&row_elements)
    }
}

/// Represents a square matrix over `PrimeField` elements
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SquareMatrix<const N_ROWS: usize, const N_COLS: usize, const N_ELEMENTS: usize>(
    pub Matrix<N_ROWS, N_COLS, N_ELEMENTS>,
);

impl<const N_ROWS: usize, const N_COLS: usize, const N_ELEMENTS: usize> MatrixOperations
    for SquareMatrix<N_ROWS, N_COLS, N_ELEMENTS>
{
    fn new(elements: &[Fq]) -> Self {
        Self(Matrix::new(elements))
    }

    fn elements(&self) -> &[Fq] {
        self.0.elements()
    }

    fn get_element(&self, i: usize, j: usize) -> Fq {
        self.0.get_element(i, j)
    }

    fn set_element(&mut self, i: usize, j: usize, val: Fq) {
        self.0.set_element(i, j, val)
    }

    fn rows(&self) -> &[&[Fq]] {
        todo!()
    }

    fn n_rows(&self) -> usize {
        N_ROWS
    }

    fn n_cols(&self) -> usize {
        N_COLS
    }

    fn transpose(&self) -> Self {
        Self(self.0.transpose())
    }

    fn hadamard_product(&self, rhs: &Self) -> Result<Self, PoseidonParameterError>
    where
        Self: Sized,
    {
        Ok(Self(self.0.hadamard_product(&rhs.0)?))
    }
}

// impl SquareMatrixOperations for SquareMatrix {
//     /// Compute the inverse of the matrix
//     fn inverse(&self) -> Result<Self, PoseidonParameterError> {
//         let identity = Self::identity(self.n_rows());

//         if self.n_rows() == 1 {
//             let mut elements = Vec::<Fq, MAX_DIMENSION>::new();
//             elements
//                 .push(
//                     self.get_element(0, 0)
//                         .inverse()
//                         .expect("inverse of single element must exist for 1x1 matrix"),
//                 )
//                 .expect("capacity should not be exceeded");
//             return Ok(Self::from_vec(elements));
//         }

//         let determinant = self.determinant();
//         if determinant == Fq::zero() {
//             return Err(PoseidonParameterError::NoMatrixInverse);
//         }

//         let minors = self.minors();
//         let cofactor_matrix = self.cofactors();
//         let signed_minors = minors
//             .hadamard_product(&cofactor_matrix)
//             .expect("minor and cofactor matrix have correct dimensions");
//         let adj = signed_minors.transpose();
//         let matrix_inverse = adj * (Fq::one() / determinant);

//         debug_assert_eq!(
//             mat_mul(self, &matrix_inverse)
//                 .expect("matrix and its inverse should have same dimensions"),
//             identity
//         );
//         Ok(matrix_inverse)
//     }

//     /// Construct a dim x dim identity matrix
//     fn identity(dim: usize) -> Self {
//         let mut elements = Vec::<Fq, MAX_DIMENSION>::new();
//         for _ in 0..dim {
//             for _ in 0..dim {
//                 elements
//                     .push(Fq::zero())
//                     .expect("capacity should not be exceeded");
//             }
//         }
//         let mut m = Self::from_vec(elements);

//         // Set diagonals to 1
//         for i in 0..dim {
//             m.set_element(i, i, Fq::one());
//         }

//         m
//     }

//     /// Compute the (unsigned) minors of this matrix
//     fn minors(&self) -> Self {
//         match self.n_cols() {
//             0 => panic!("matrix has no elements!"),
//             1 => {
//                 let mut elements = Vec::<Fq, MAX_DIMENSION>::new();
//                 elements
//                     .push(self.get_element(0, 0))
//                     .expect("capacity should not be exceeded");
//                 Self::from_vec(elements)
//             }
//             2 => {
//                 let mut elements = Vec::<Fq, MAX_DIMENSION>::new();
//                 let a = self.get_element(0, 0);
//                 let b = self.get_element(0, 1);
//                 let c = self.get_element(1, 0);
//                 let d = self.get_element(1, 1);
//                 elements
//                     .extend_from_slice(&[d, c, b, a])
//                     .expect("capacity should not be exceeded");
//                 Self::from_vec(elements)
//             }
//             _ => {
//                 let dim = self.n_rows();
//                 let mut minor_matrix_elements = Vec::<Fq, MAX_DIMENSION>::new();
//                 for i in 0..dim {
//                     for j in 0..dim {
//                         let mut elements: Vec<Fq, MAX_DIMENSION> = Vec::new();
//                         for k in 0..i {
//                             for l in 0..j {
//                                 elements
//                                     .push(self.get_element(k, l))
//                                     .expect("capacity should not be exceeded");
//                             }
//                             for l in (j + 1)..dim {
//                                 elements
//                                     .push(self.get_element(k, l))
//                                     .expect("capacity should not be exceeded");
//                             }
//                         }
//                         for k in i + 1..dim {
//                             for l in 0..j {
//                                 elements
//                                     .push(self.get_element(k, l))
//                                     .expect("capacity should not be exceeded");
//                             }
//                             for l in (j + 1)..dim {
//                                 elements
//                                     .push(self.get_element(k, l))
//                                     .expect("capacity should not be exceeded");
//                             }
//                         }
//                         let minor = Self::from_vec(elements);
//                         minor_matrix_elements
//                             .push(minor.determinant())
//                             .expect("capacity should not be exceeded");
//                     }
//                 }
//                 Self::from_vec(minor_matrix_elements)
//             }
//         }
//     }

//     /// Compute the cofactor matrix, i.e. $C_{ij} = (-1)^{i+j}$
//     fn cofactors(&self) -> Self {
//         let dim = self.n_rows();
//         let mut elements = Vec::<Fq, MAX_DIMENSION>::new();

//         // TODO: non arkworks Fq::pow
//         use crate::StuffThatNeedsToGoInDecaf377;
//         for i in 0..dim {
//             for j in 0..dim {
//                 elements
//                     .push((-Fq::one()).pow([(i + j) as u64]))
//                     .expect("capacity should not be exceeded");
//             }
//         }
//         Self::from_vec(elements)
//     }

//     /// Compute the matrix determinant
//     fn determinant(&self) -> Fq {
//         match self.n_cols() {
//             0 => panic!("matrix has no elements!"),
//             1 => self.get_element(0, 0),
//             2 => {
//                 let a11 = self.get_element(0, 0);
//                 let a12 = self.get_element(0, 1);
//                 let a21 = self.get_element(1, 0);
//                 let a22 = self.get_element(1, 1);
//                 a11 * a22 - a21 * a12
//             }
//             3 => {
//                 let a11 = self.get_element(0, 0);
//                 let a12 = self.get_element(0, 1);
//                 let a13 = self.get_element(0, 2);
//                 let a21 = self.get_element(1, 0);
//                 let a22 = self.get_element(1, 1);
//                 let a23 = self.get_element(1, 2);
//                 let a31 = self.get_element(2, 0);
//                 let a32 = self.get_element(2, 1);
//                 let a33 = self.get_element(2, 2);

//                 a11 * (Self::new_2x2(a22, a23, a32, a33).determinant())
//                     - a12 * (Self::new_2x2(a21, a23, a31, a33).determinant())
//                     + a13 * (Self::new_2x2(a21, a22, a31, a32).determinant())
//             }
//             _ => {
//                 // Unoptimized, but MDS matrices are fairly small, so we do the naive thing
//                 let mut det = Fq::zero();
//                 let mut levi_civita = true;
//                 let dim = self.n_rows();

//                 for i in 0..dim {
//                     let mut elements: Vec<Fq, MAX_DIMENSION> = Vec::new();
//                     for k in 0..i {
//                         for l in 1..dim {
//                             elements
//                                 .push(self.get_element(k, l))
//                                 .expect("capacity should not be exceeded");
//                         }
//                     }
//                     for k in i + 1..dim {
//                         for l in 1..dim {
//                             elements
//                                 .push(self.get_element(k, l))
//                                 .expect("capacity should not be exceeded");
//                         }
//                     }
//                     let minor = Self::from_vec(elements);
//                     if levi_civita {
//                         det += self.get_element(i, 0) * minor.determinant();
//                     } else {
//                         det -= self.get_element(i, 0) * minor.determinant();
//                     }
//                     levi_civita = !levi_civita;
//                 }

//                 det
//             }
//         }
//     }
// }

/// Multiply scalar by SquareMatrix
impl<const N_ROWS: usize, const N_COLS: usize, const N_ELEMENTS: usize> Mul<Fq>
    for SquareMatrix<N_ROWS, N_COLS, N_ELEMENTS>
{
    type Output = SquareMatrix<N_ROWS, N_COLS, N_ELEMENTS>;

    fn mul(self, rhs: Fq) -> Self::Output {
        let elements = self.elements();
        let mut new_elements = [Fq::default(); N_ELEMENTS];
        for (i, &element) in elements.iter().enumerate() {
            new_elements[i] = element * rhs;
        }
        Self::new(&new_elements)
    }
}

impl<const N_ROWS: usize, const N_COLS: usize, const N_ELEMENTS: usize>
    SquareMatrix<N_ROWS, N_COLS, N_ELEMENTS>
{
    /// Get row vector at a specified row index.
    pub fn row_vector(&self, i: usize) -> Matrix<1, N_COLS, N_ELEMENTS> {
        self.0.row_vector(i)
    }

    /// Create a 2x2 `SquareMatrix` from four elements.
    pub fn new_2x2(a: Fq, b: Fq, c: Fq, d: Fq) -> SquareMatrix<2, 2, 4> {
        SquareMatrix::<2, 2, 4>::new(&[a, b, c, d])
    }
}
