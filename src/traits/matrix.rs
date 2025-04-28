
use crate::core::option::Option;
use crate::core::marker::Copy;
use crate::{
    Matrix,
    MatrixConstructor,
};

// 2x2

impl<T: Copy> Matrix<T, 2> for ((T, T), (T, T)) {
    #[inline]
    fn get_unchecked( &self, row: usize, col: usize ) -> T {
        match (row, col) {
            (0, 0) => self.0.0,
            (0, 1) => self.0.1,
            (1, 0) => self.1.0,
            (1, 1) => self.1.1,
            _ => crate::core::panic!("Out of index operation! Got ({row}, {col}), accepting at most (1, 1)")
        }
    }

    #[inline]
    fn get( &self, row: usize, col: usize ) -> Option<T> {
        use Option::{Some, None};
        match (row, col) {
            (0, 0) => Some(self.0.0),
            (0, 1) => Some(self.0.1),
            (1, 0) => Some(self.1.0),
            (1, 1) => Some(self.1.1),
            _ => None
        }
    }

    #[inline]
    fn to_array( &self ) -> [[T; 2]; 2] {
        [
            [self.0.0, self.0.1],
            [self.1.0, self.1.1],
        ]
    }
}

impl<T: Copy> Matrix<T, 2> for ([T; 2], [T; 2]) {
    #[inline]
    fn get_unchecked( &self, row: usize, col: usize ) -> T {
        match (row, col) {
            (0, 0) => self.0[0],
            (0, 1) => self.0[1],
            (1, 0) => self.1[0],
            (1, 1) => self.1[1],
            _ => crate::core::panic!("Out of index operation! Got ({row}, {col}), accepting at most (1, 1)")
        }
    }

    #[inline]
    fn get( &self, row: usize, col: usize ) -> Option<T> {
        use Option::{Some, None};
        match (row, col) {
            (0, 0) => Some(self.0[0]),
            (0, 1) => Some(self.0[1]),
            (1, 0) => Some(self.1[0]),
            (1, 1) => Some(self.1[1]),
            _ => None
        }
    }

    #[inline]
    fn to_array( &self ) -> [[T; 2]; 2] {
        [
            [self.0[0], self.0[1]],
            [self.1[0], self.1[1]],
        ]
    }
}

impl<T: Copy> Matrix<T, 2> for [(T, T); 2] {
    #[inline]
    fn get_unchecked( &self, row: usize, col: usize ) -> T {
        match (row, col) {
            (0, 0) => self[0].0,
            (0, 1) => self[0].1,
            (1, 0) => self[1].0,
            (1, 1) => self[1].1,
            _ => crate::core::panic!("Out of index operation! Got ({row}, {col}), accepting at most (1, 1)")
        }
    }

    #[inline]
    fn get( &self, row: usize, col: usize ) -> Option<T> {
        use Option::{Some, None};
        match (row, col) {
            (0, 0) => Some(self[0].0),
            (0, 1) => Some(self[0].1),
            (1, 0) => Some(self[1].0),
            (1, 1) => Some(self[1].1),
            _ => None
        }
    }

    #[inline]
    fn to_array( &self ) -> [[T; 2]; 2] {
        [
            [self[0].0, self[0].1],
            [self[1].0, self[1].1],
        ]
    }
}

impl<T: Copy> Matrix<T, 2> for [T; 4] {
    #[inline]
    fn get_unchecked( &self, row: usize, col: usize ) -> T {
        #[cfg(not(debug_assertions))]
        return self[row * 2 + col];
        #[cfg(debug_assertions)]
        match (row, col) {
            (..2, ..2) => self[row * 2 + col],
            _ => crate::core::panic!("Out of index operation! Got ({row}, {col}), accepting at most (1, 1)")
        }
    }

    #[inline]
    fn get( &self, row: usize, col: usize ) -> Option<T> {
        use Option::{Some, None};
        match (row, col) {
            (..2, ..2) => Some(self[row * 2 + col]),
            _ => None
        }
    }

    #[inline]
    fn to_array( &self ) -> [[T; 2]; 2] {
        [
            [self[0], self[1]],
            [self[2], self[3]],
        ]
    }
}

impl<T: Copy> MatrixConstructor<T, 2> for ((T, T), (T, T)) {
    #[inline]
    fn new_matrix(matrix: [[T; 2]; 2]) -> Self {
        (
            (
                matrix[0][0],
                matrix[0][1],
            ),
            (
                matrix[1][0],
                matrix[1][1],
            ),
        )
    }
}

impl<T: Copy> MatrixConstructor<T, 2> for ([T; 2], [T; 2]) {
    #[inline]
    fn new_matrix(matrix: [[T; 2]; 2]) -> Self {
        (
            [
                matrix[0][0],
                matrix[0][1],
            ],
            [
                matrix[1][0],
                matrix[1][1],
            ],
        )
    }
}

impl<T: Copy> MatrixConstructor<T, 2> for [(T, T); 2] {
    #[inline]
    fn new_matrix(matrix: [[T; 2]; 2]) -> Self {
        [
            (
                matrix[0][0],
                matrix[0][1],
            ),
            (
                matrix[1][0],
                matrix[1][1],
            ),
        ]
    }
}

impl<T: Copy> MatrixConstructor<T, 2> for [T; 4] {
    #[inline]
    fn new_matrix(matrix: [[T; 2]; 2]) -> Self {
        [
            (matrix[0][0]),
            (matrix[0][1]),
            (matrix[1][0]),
            (matrix[1][1]),
        ]
    }
}

// 3x3

impl<T: Copy> Matrix<T, 3> for ((T, T, T), (T, T, T), (T, T, T)) {
    fn get_unchecked( &self, row: usize, col: usize ) -> T {
        match (row, col) {
            (0, 0) => self.0.0,
            (0, 1) => self.0.1,
            (0, 2) => self.0.2,
            (1, 0) => self.1.0,
            (1, 1) => self.1.1,
            (1, 2) => self.1.2,
            (2, 0) => self.2.0,
            (2, 1) => self.2.1,
            (2, 2) => self.2.2,
            _ => crate::core::panic!("Out of index operation! Got ({row}, {col}), accepting at most (2, 2)"),
        }
    }

    fn get( &self, row: usize, col: usize ) -> Option<T> {
        use Option::{Some, None};
        match (row, col) {
            (0, 0) => Some(self.0.0),
            (0, 1) => Some(self.0.1),
            (0, 2) => Some(self.0.2),
            (1, 0) => Some(self.1.0),
            (1, 1) => Some(self.1.1),
            (1, 2) => Some(self.1.2),
            (2, 0) => Some(self.2.0),
            (2, 1) => Some(self.2.1),
            (2, 2) => Some(self.2.2),
            _ => None,
        }
    }

    #[inline]
    fn to_array( &self ) -> [[T; 3]; 3] {
        [
            [self.0.0, self.0.1, self.0.2],
            [self.1.0, self.1.1, self.1.2],
            [self.2.0, self.2.1, self.2.2],
        ]
    }
}

impl<T: Copy> Matrix<T, 3> for ([T; 3], [T; 3], [T; 3]) {
    fn get_unchecked( &self, row: usize, col: usize ) -> T {
        match (row, col) {
            (0, 0) => self.0[0],
            (0, 1) => self.0[1],
            (0, 2) => self.0[2],
            (1, 0) => self.1[0],
            (1, 1) => self.1[1],
            (1, 2) => self.1[2],
            (2, 0) => self.2[0],
            (2, 1) => self.2[1],
            (2, 2) => self.2[2],
            _ => crate::core::panic!("Out of index operation! Got ({row}, {col}), accepting at most (2, 2)"),
        }
    }

    fn get( &self, row: usize, col: usize ) -> Option<T> {
        use Option::{Some, None};
        match (row, col) {
            (0, 0) => Some(self.0[0]),
            (0, 1) => Some(self.0[1]),
            (0, 2) => Some(self.0[2]),
            (1, 0) => Some(self.1[0]),
            (1, 1) => Some(self.1[1]),
            (1, 2) => Some(self.1[2]),
            (2, 0) => Some(self.2[0]),
            (2, 1) => Some(self.2[1]),
            (2, 2) => Some(self.2[2]),
            _ => None,
        }
    }

    #[inline]
    fn to_array( &self ) -> [[T; 3]; 3] {
        [
            [self.0[0], self.0[1], self.0[2]],
            [self.1[0], self.1[1], self.1[2]],
            [self.2[0], self.2[1], self.2[2]],
        ]
    }
}

impl<T: Copy> Matrix<T, 3> for [(T, T, T); 3] {
    fn get_unchecked( &self, row: usize, col: usize ) -> T {
        match (row, col) {
            (0, 0) => self[0].0,
            (0, 1) => self[0].1,
            (0, 2) => self[0].2,
            (1, 0) => self[1].0,
            (1, 1) => self[1].1,
            (1, 2) => self[1].2,
            (2, 0) => self[2].0,
            (2, 1) => self[2].1,
            (2, 2) => self[2].2,
            _ => crate::core::panic!("Out of index operation! Got ({row}, {col}), accepting at most (2, 2)"),
        }
    }

    fn get( &self, row: usize, col: usize ) -> Option<T> {
        use Option::{Some, None};
        match (row, col) {
            (0, 0) => Some(self[0].0),
            (0, 1) => Some(self[0].1),
            (0, 2) => Some(self[0].2),
            (1, 0) => Some(self[1].0),
            (1, 1) => Some(self[1].1),
            (1, 2) => Some(self[1].2),
            (2, 0) => Some(self[2].0),
            (2, 1) => Some(self[2].1),
            (2, 2) => Some(self[2].2),
            _ => None,
        }
    }

    #[inline]
    fn to_array( &self ) -> [[T; 3]; 3] {
        [
            [self[0].0, self[0].1, self[0].2],
            [self[1].0, self[1].1, self[1].2],
            [self[2].0, self[2].1, self[2].2],
        ]
    }
}

impl<T: Copy> Matrix<T, 3> for [T; 9] {
    #[inline]
    fn get_unchecked( &self, row: usize, col: usize ) -> T {
        #[cfg(not(debug_assertions))]
        return self[row * 3 + col];
        #[cfg(debug_assertions)]
        match (row, col) {
            (..3, ..3) => self[row * 3 + col],
            _ => crate::core::panic!("Out of index operation! Got ({row}, {col}), accepting at most (2, 2)")
        }
    }

    #[inline]
    fn get( &self, row: usize, col: usize ) -> Option<T> {
        use Option::{Some, None};
        match (row, col) {
            (..3, ..3) => Some(self[row * 3 + col]),
            _ => None
        }
    }

    #[inline]
    fn to_array( &self ) -> [[T; 3]; 3] {
        [
            [self[0], self[1], self[2]],
            [self[3], self[4], self[5]],
            [self[6], self[7], self[8]],
        ]
    }
}

impl<T: Copy> MatrixConstructor<T, 3> for ((T, T, T), (T, T, T), (T, T, T)) {
    #[inline]
    fn new_matrix(matrix: [[T; 3]; 3]) -> Self {
        (
            (
                (matrix[0][0]),
                (matrix[0][1]),
                (matrix[0][2]),
            ),
            (
                (matrix[1][0]),
                (matrix[1][1]),
                (matrix[1][2]),
            ),
            (
                (matrix[2][0]),
                (matrix[2][1]),
                (matrix[2][2]),
            ),
        )
    }
}

impl<T: Copy> MatrixConstructor<T, 3> for ([T; 3], [T; 3], [T; 3]) {
    #[inline]
    fn new_matrix(matrix: [[T; 3]; 3]) -> Self {
        (
            [
                (matrix[0][0]),
                (matrix[0][1]),
                (matrix[0][2]),
            ],
            [
                (matrix[1][0]),
                (matrix[1][1]),
                (matrix[1][2]),
            ],
            [
                (matrix[2][0]),
                (matrix[2][1]),
                (matrix[2][2]),
            ],
        )
    }
}

impl<T: Copy> MatrixConstructor<T, 3> for [(T, T, T); 3] {
    #[inline]
    fn new_matrix(matrix: [[T; 3]; 3]) -> Self {
        [
            (
                (matrix[0][0]),
                (matrix[0][1]),
                (matrix[0][2]),
            ),
            (
                (matrix[1][0]),
                (matrix[1][1]),
                (matrix[1][2]),
            ),
            (
                (matrix[2][0]),
                (matrix[2][1]),
                (matrix[2][2]),
            ),
        ]
    }
}

impl<T: Copy> MatrixConstructor<T, 3> for [T; 9] {
    #[inline]
    fn new_matrix(matrix: [[T; 3]; 3]) -> Self {
        [
            (matrix[0][0]),
            (matrix[0][1]),
            (matrix[0][2]),
            (matrix[1][0]),
            (matrix[1][1]),
            (matrix[1][2]),
            (matrix[2][0]),
            (matrix[2][1]),
            (matrix[2][2]),
        ]
    }
}

// 4x4

impl<T: Copy> Matrix<T, 4> for ((T, T, T, T), (T, T, T, T), (T, T, T, T), (T, T, T, T)) {
    #[inline]
    fn get_unchecked( &self, row: usize, col: usize ) -> T {
        match (row, col) {
            (0, 0) => self.0.0,
            (0, 1) => self.0.1,
            (0, 2) => self.0.2,
            (0, 3) => self.0.3,
            (1, 0) => self.1.0,
            (1, 1) => self.1.1,
            (1, 2) => self.1.2,
            (1, 3) => self.1.3,
            (2, 0) => self.2.0,
            (2, 1) => self.2.1,
            (2, 2) => self.2.2,
            (2, 3) => self.2.3,
            (3, 0) => self.3.0,
            (3, 1) => self.3.1,
            (3, 2) => self.3.2,
            (3, 3) => self.3.3,
            _ => crate::core::panic!("Out of index operation! Got ({row}, {col}), accepting at most (3, 3)"),
        }
    }

    #[inline]
    fn get( &self, row: usize, col: usize ) -> Option<T> {
        use Option::{Some, None};
        match (row, col) {
            (0, 0) => Some(self.0.0),
            (0, 1) => Some(self.0.1),
            (0, 2) => Some(self.0.2),
            (0, 3) => Some(self.0.3),
            (1, 0) => Some(self.1.0),
            (1, 1) => Some(self.1.1),
            (1, 2) => Some(self.1.2),
            (1, 3) => Some(self.1.3),
            (2, 0) => Some(self.2.0),
            (2, 1) => Some(self.2.1),
            (2, 2) => Some(self.2.2),
            (2, 3) => Some(self.2.3),
            (3, 0) => Some(self.3.0),
            (3, 1) => Some(self.3.1),
            (3, 2) => Some(self.3.2),
            (3, 3) => Some(self.3.3),
            _ => None,
        }
    }

    #[inline]
    fn to_array( &self ) -> [[T; 4]; 4] {
        [
            [self.0.0, self.1.0, self.2.0, self.3.0],
            [self.0.1, self.1.1, self.2.1, self.3.1],
            [self.0.2, self.1.2, self.2.2, self.3.2],
            [self.0.3, self.1.3, self.2.3, self.3.3],
        ]
    }
}

impl<T: Copy> Matrix<T, 4> for ([T; 4], [T; 4], [T; 4], [T; 4]) {
    #[inline]
    fn get_unchecked( &self, row: usize, col: usize ) -> T {
        match (row, col) {
            (0, 0) => self.0[0],
            (0, 1) => self.0[1],
            (0, 2) => self.0[2],
            (0, 3) => self.0[3],
            (1, 0) => self.1[0],
            (1, 1) => self.1[1],
            (1, 2) => self.1[2],
            (1, 3) => self.1[3],
            (2, 0) => self.2[0],
            (2, 1) => self.2[1],
            (2, 2) => self.2[2],
            (2, 3) => self.2[3],
            (3, 0) => self.3[0],
            (3, 1) => self.3[1],
            (3, 2) => self.3[2],
            (3, 3) => self.3[3],
            _ => crate::core::panic!("Out of index operation! Got ({row}, {col}), accepting at most (3, 3)"),
        }
    }

    #[inline]
    fn get( &self, row: usize, col: usize ) -> Option<T> {
        use Option::{Some, None};
        match (row, col) {
            (0, 0) => Some(self.0[0]),
            (0, 1) => Some(self.0[1]),
            (0, 2) => Some(self.0[2]),
            (0, 3) => Some(self.0[3]),
            (1, 0) => Some(self.1[0]),
            (1, 1) => Some(self.1[1]),
            (1, 2) => Some(self.1[2]),
            (1, 3) => Some(self.1[3]),
            (2, 0) => Some(self.2[0]),
            (2, 1) => Some(self.2[1]),
            (2, 2) => Some(self.2[2]),
            (2, 3) => Some(self.2[3]),
            (3, 0) => Some(self.3[0]),
            (3, 1) => Some(self.3[1]),
            (3, 2) => Some(self.3[2]),
            (3, 3) => Some(self.3[3]),
            _ => None,
        }
    }

    #[inline]
    fn to_array( &self ) -> [[T; 4]; 4] {
        [
            [self.0[0], self.1[0], self.2[0], self.3[0]],
            [self.0[1], self.1[1], self.2[1], self.3[1]],
            [self.0[2], self.1[2], self.2[2], self.3[2]],
            [self.0[3], self.1[3], self.2[3], self.3[3]],
        ]
    }
}

impl<T: Copy> Matrix<T, 4> for [(T, T, T, T); 4] {
    #[inline]
    fn get_unchecked( &self, row: usize, col: usize ) -> T {
        match (row, col) {
            (0, 0) => self[0].0,
            (0, 1) => self[0].1,
            (0, 2) => self[0].2,
            (0, 3) => self[0].3,
            (1, 0) => self[1].0,
            (1, 1) => self[1].1,
            (1, 2) => self[1].2,
            (1, 3) => self[1].3,
            (2, 0) => self[2].0,
            (2, 1) => self[2].1,
            (2, 2) => self[2].2,
            (2, 3) => self[2].3,
            (3, 0) => self[3].0,
            (3, 1) => self[3].1,
            (3, 2) => self[3].2,
            (3, 3) => self[3].3,
            _ => crate::core::panic!("Out of index operation! Got ({row}, {col}), accepting at most (3, 3)"),
        }
    }

    #[inline]
    fn get( &self, row: usize, col: usize ) -> Option<T> {
        use Option::{Some, None};
        match (row, col) {
            (0, 0) => Some(self[0].0),
            (0, 1) => Some(self[0].1),
            (0, 2) => Some(self[0].2),
            (0, 3) => Some(self[0].3),
            (1, 0) => Some(self[1].0),
            (1, 1) => Some(self[1].1),
            (1, 2) => Some(self[1].2),
            (1, 3) => Some(self[1].3),
            (2, 0) => Some(self[2].0),
            (2, 1) => Some(self[2].1),
            (2, 2) => Some(self[2].2),
            (2, 3) => Some(self[2].3),
            (3, 0) => Some(self[3].0),
            (3, 1) => Some(self[3].1),
            (3, 2) => Some(self[3].2),
            (3, 3) => Some(self[3].3),
            _ => None,
        }
    }

    #[inline]
    fn to_array( &self ) -> [[T; 4]; 4] {
        [
            [self[0].0, self[1].0, self[2].0, self[3].0],
            [self[0].1, self[1].1, self[2].1, self[3].1],
            [self[0].2, self[1].2, self[2].2, self[3].2],
            [self[0].3, self[1].3, self[2].3, self[3].3],
        ]
    }
}

impl<T: Copy> Matrix<T, 4> for [T; 16] {
    #[inline]
    fn get_unchecked( &self, row: usize, col: usize ) -> T {
        #[cfg(not(debug_assertions))]
        return self[row * 4 + col];
        #[cfg(debug_assertions)]
        match (row, col) {
            (..4, ..4) => self[row * 4 + col],
            _ => crate::core::panic!("Out of index operation! Got ({row}, {col}), accepting at most (3, 3)")
        }
    }

    #[inline]
    fn get( &self, row: usize, col: usize ) -> Option<T> {
        use Option::{Some, None};
        match (row, col) {
            (..4, ..4) => Some(self[row * 4 + col]),
            _ => None
        }
    }

    #[inline]
    fn to_array( &self ) -> [[T; 4]; 4] {
        [
            [self[00], self[01], self[02], self[03]],
            [self[04], self[05], self[06], self[07]],
            [self[08], self[09], self[10], self[11]],
            [self[12], self[13], self[14], self[15]],
        ]
    }
}

impl<T: Copy> MatrixConstructor<T, 4> for ((T, T, T, T), (T, T, T, T), (T, T, T, T), (T, T, T, T)) {
    #[inline]
    fn new_matrix(matrix: [[T; 4]; 4]) -> Self {
        (
            (
                (matrix[0][0]),
                (matrix[0][1]),
                (matrix[0][2]),
                (matrix[0][3]),
            ),
            (
                (matrix[1][0]),
                (matrix[1][1]),
                (matrix[1][2]),
                (matrix[1][3]),
            ),
            (
                (matrix[2][0]),
                (matrix[2][1]),
                (matrix[2][2]),
                (matrix[2][3]),
            ),
            (
                (matrix[3][0]),
                (matrix[3][1]),
                (matrix[3][2]),
                (matrix[3][3]),
            ),
        )
    }
}

impl<T: Copy> MatrixConstructor<T, 4> for ([T; 4], [T; 4], [T; 4], [T; 4]) {
    #[inline]
    fn new_matrix(matrix: [[T; 4]; 4]) -> Self {
        (
            [
                (matrix[0][0]),
                (matrix[0][1]),
                (matrix[0][2]),
                (matrix[0][3]),
            ],
            [
                (matrix[1][0]),
                (matrix[1][1]),
                (matrix[1][2]),
                (matrix[1][3]),
            ],
            [
                (matrix[2][0]),
                (matrix[2][1]),
                (matrix[2][2]),
                (matrix[2][3]),
            ],
            [
                (matrix[3][0]),
                (matrix[3][1]),
                (matrix[3][2]),
                (matrix[3][3]),
            ],
        )
    }
}

impl<T: Copy> MatrixConstructor<T, 4> for [(T, T, T, T); 4] {
    #[inline]
    fn new_matrix(matrix: [[T; 4]; 4]) -> Self {
        [
            (
                matrix[0][0],
                matrix[0][1],
                matrix[0][2],
                matrix[0][3],
            ),
            (
                matrix[1][0],
                matrix[1][1],
                matrix[1][2],
                matrix[1][3],
            ),
            (
                matrix[2][0],
                matrix[2][1],
                matrix[2][2],
                matrix[2][3],
            ),
            (
                matrix[3][0],
                matrix[3][1],
                matrix[3][2],
                matrix[3][3],
            ),
        ]
    }
}

impl<T: Copy> MatrixConstructor<T, 4> for [T; 16] {
    #[inline]
    fn new_matrix(matrix: [[T; 4]; 4]) -> Self {
        [
            matrix[0][0],
            matrix[0][1],
            matrix[0][2],
            matrix[0][3],
            matrix[1][0],
            matrix[1][1],
            matrix[1][2],
            matrix[1][3],
            matrix[2][0],
            matrix[2][1],
            matrix[2][2],
            matrix[2][3],
            matrix[3][0],
            matrix[3][1],
            matrix[3][2],
            matrix[3][3],
        ]
    }
}
