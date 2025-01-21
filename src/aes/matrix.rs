//
// EPITECH PROJECT, 2024
// matrix.rs
// File description:
// Matrix struct
//

#![allow(dead_code)]

use std::{fmt::Debug, ops::{Index, IndexMut}};

pub struct Matrix {
    pub col_size: usize,
    pub rows: usize,
    pub data: Vec<Vec<u8>>
}

impl Matrix {
    /// Creates a Matrix object filled with zeros with shape as size.</br>
    /// ### Params
    /// `shape` => `(rows, col_size)` `rows` represents the number of rows and `col_size` represents the number of columns.</br>
    /// ### Examples
    /// ```
    /// let matrix_2_2 = Matrix::new((2, 2));
    /// //  [[0, 0],
    /// //   [0, 0]]
    ///
    /// let matrix_1_3 = Matrix::new((1, 3));
    /// //  [[0, 0, 0]]
    ///
    /// let matrix_4_2 = Matrix::new((4, 2));
    /// //  [[0, 0],
    /// //   [0, 0],
    /// //   [0, 0],
    /// //   [0, 0]]
    /// ```
    pub fn new(shape: (usize, usize)) -> Matrix {
        Matrix {
            col_size: shape.1,
            rows: shape.0,
            data: vec![vec![0; shape.1]; shape.0]
        }
    }

    /// Creates an empty Matrix object.</br>
    /// ### Params
    /// `col_size` => size of the columns that will be added to the Matrix.
    pub fn empty(col_size: usize) -> Matrix {
        Matrix {
            col_size,
            rows: 0,
            data: vec![]
        }
    }

    /// Creates a 1-dimensional Matrix object from a 1-dimensional array.</br>
    /// ### Params
    /// `src` => `&[u8]` array of `u8` to make the 1-dimensional Matrix.</br>
    /// ### Return value
    /// Returns a new Matrix object containing `src` into a 1-dimensional Matrix.</br>
    /// `rows` equals `1` and `col_size` equals `src.len()`.
    /// ### Examples
    /// ```
    /// let matrix = Matrix::from(&[1, 2, 3, 4]);
    /// // [[1, 2, 3, 4]]
    /// ```
    pub fn from(src: &[u8]) -> Matrix {
        Matrix {
            col_size: src.len(),
            rows: 1,
            data: vec![src.to_vec()]
        }
    }

    /// Creates a Matrix object from a 1-dimension array matching shape.</br>
    /// ### Params
    /// `src` => `&[u8]` array of `u8` to make the 2-dimensional Matrix.</br>
    /// `shape` => `(rows, col_size)` `rows` represents the number of rows and `col_size` represents the number of columns.</br>
    /// ### Return value
    /// Returns a new Matrix object fitting everything in `src` into the size given, padding the rest with zeros.</br>
    /// If `rows * col_size > src.len()` the Matrix takes at most `src.len()` values from `src`.
    /// ### Examples
    /// ```
    /// let char_list = "Hello World!".as_bytes();
    ///
    /// let matrix_full = Matrix::from_shape(char_list, (3, 4));
    /// assert_eq!(matrix_full.data, [[72, 101, 108, 108], [111, 32, 87, 111], [114, 108, 100, 33]]);
    ///
    /// let matrix_reduced = Matrix::from_shape(char_list, (3, 3));
    /// assert_eq!(matrix_reduced.data, [[72, 101, 108], [108, 111, 32], [87, 111, 114]]);
    ///
    /// let matrix_filled = Matrix::from_shape(char_list, (4, 4));
    /// assert_eq!(matrix_filled.data, [[72, 101, 108, 108], [111, 32, 87, 111], [114, 108, 100, 33], [0, 0, 0, 0]]);
    /// ```
    pub fn from_shape(src: &[u8], shape: (usize, usize)) -> Matrix {
        let mut data = src.chunks(shape.1).into_iter().map(|v| {let mut vec = v.to_vec(); vec.resize(shape.1, 0); vec}).collect::<Vec<Vec<u8>>>();
        data.resize(shape.0, vec![0; shape.1]);
        Matrix {
            col_size: shape.1,
            rows: shape.0,
            data
        }
    }

    /// Creates a Matrix object from a 1-dimension array matching shape, storing in column instead of rows.</br>
    /// /// ### Params
    /// `src` => `&[u8]` array of `u8` to make the 2-dimensional Matrix.</br>
    /// `shape` => `(rows, col_size)` `rows` represents the number of rows and `col_size` represents the number of columns.</br>
    /// ### Return value
    /// Returns a new Matrix object fitting everything in `src` into the size given, padding the rest with zeros.</br>
    /// If `rows * col_size > src.len()` the Matrix takes at most `src.len()` values from `src`.
    pub fn from_key(key: &[u8], shape: (usize, usize)) -> Matrix {
        let mut data: Vec<Vec<u8>> = vec![vec![0; shape.1]; shape.0];
        for i in 0..shape.0 {
            for j in 0..shape.1 {
                data[j][i] = key[(i * shape.0) + j];
            }
        }
        Matrix {
            col_size: shape.1,
            rows: shape.0,
            data
        }
    }

    /// Creates a Matrix object from an array.</br>
    /// ### Params
    /// `src` => `&[[u8; S]]` 2-dimensional array of `u8` to make the Matrix, `S` corresponds to the array size.</br>
    /// ### Return value
    /// Returns a new Matrix object containing `src` into a Matrix.</br>
    /// `rows` equals `src.len()` and `col_size` equals `src[0].len()`.
    /// ### Examples
    /// ```
    /// let matrix = Matrix::from_matrix_array(&[
    ///     [1, 2],
    ///     [3, 4],
    ///     [5, 6]
    /// ]);
    /// assert_eq!(matrix.data, &[[1, 2], [3, 4], [5, 6]]);
    /// ```
    pub fn from_matrix_array<const S: usize>(src: &[[u8; S]]) -> Matrix {
        Matrix {
            col_size: src[0].len(),
            rows: src.len(),
            data: src.to_vec().iter().map(|v| v.to_vec()).collect()
        }
    }

    /// Creates a Matrix object from a Vec.</br>
    /// ### Params
    /// `src` => `&Vec<Vec<u8>>` 2-dimensional vector of `u8` to make the Matrix.</br>
    /// ### Return value
    /// Returns a new Matrix object containing `src` into a Matrix.</br>
    /// `rows` equals `src.len()` and `col_size` equals `src[0].len()`.
    /// ### Examples
    /// ```
    /// let vec = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    /// let matrix = Matrix::from_matrix_vec(&vec);
    /// assert_eq!(matrix.data, vec);
    /// ```
    pub fn from_matrix_vec(src: &Vec<Vec<u8>>) -> Matrix {
        Matrix {
            col_size: src[0].len(),
            rows: src.len(),
            data: src.clone()
        }
    }

    /// Safe version of `Matrix[row][col]`.</br>
    /// Returns element at index `[row][col]` while checking if index is out of bounds.
    /// ### Return value
    /// On error returns `None`, else returns `Some(elem)`
    pub fn get(&mut self, row: usize, col: usize) -> Option<u8> {
        let val = self.data.get(row)?.get(col)?;
        Some(*val)
    }

    /// Returns a whole row of the Matrix at the `index` given in parameter.
    /// ### Return value
    /// On error returns `None`, else returns `Some(row)`
    pub fn get_row(&self, index: usize) -> Option<&Vec<u8>> {
        if index >= self.rows {
            return None;
        }
        Some(&self.data[index])
    }

    /// Returns a whole column of the Matrix at the `index` given in parameter.
    /// ### Return value
    /// On error returns `None`, else returns `Some(col)`
    pub fn get_col(&self, index: usize) -> Option<Vec<&u8>> {
        if index >= self.col_size {
            return None;
        }
        Some(self.data.iter().map(|row| &row[index]).collect())
    }

    /// Returns a whole row of the Matrix as a mutable value, at the `index` given in parameter.
    /// ### Return value
    /// On error returns `None`, else returns `Some(row)`
    pub fn get_row_mut(&mut self, index: usize) -> Option<&mut Vec<u8>> {
        if index >= self.rows {
            return None;
        }
        Some(&mut self.data[index])
    }

    /// Returns a whole column of the Matrix as a mutable value, at the `index` given in parameter.
    /// ### Return value
    /// On error returns `None`, else returns `Some(col)`
    pub fn get_col_mut<'a>(&'a mut self, index: usize) -> Option<Vec<&'a mut u8>> {
        if index >= self.col_size {
            return None;
        }
        Some(self.data.iter_mut().map(|row| &mut row[index]).collect())
    }

    /// Performs a multiplication between `m1` and `m2` and returns `self` to make multiple operations.</br>
    /// `self` is affected by this operation.</br>
    /// `m1.col_size` should be equal to `m2.rows`.
    pub fn dot(&mut self, m2: &Matrix) -> &mut Matrix {
        assert_eq!(self.col_size, m2.rows);

        let mut res: Vec<Vec<u8>> = vec![vec![0; m2.col_size]; self.rows];

        for i in 0..self.rows {
            for j in 0..m2.col_size {
                res[i][j] = 0;
                for k in 0..m2.rows {
                    res[i][j] += self[i][k] * m2[k][j];
                }
            }
        }
        self.data = res;
        self
    }

    /// Performs a multiplication between `m1` and `m2` and returns a new Matrix containing the result.</br>
    /// `m1` and `m2` are unaffected by this operation.</br>
    /// `m1.col_size` should be equal to `m2.rows`.
    pub fn mul(m1: &Matrix, m2: &Matrix) -> Matrix {
        assert_eq!(m1.col_size, m2.rows);

        let mut res: Vec<Vec<u8>> = vec![vec![0; m2.col_size]; m1.rows];

        for i in 0..m1.rows {
            for j in 0..m2.col_size {
                res[i][j] = 0;
                for k in 0..m2.rows {
                    res[i][j] += m1[i][k] * m2[k][j];
                }
            }
        }
        Matrix::from_matrix_vec(&res)
    }

    /// Flattens the Matrix's data into a single vector.</br>
    /// It concatenates each row with each other.
    /// ### Examples
    /// ```
    /// let matrix = Matrix::from_shape(&[
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9]
    /// ], (3, 3));
    /// println!("{:?}", matrix.flatten_rows());
    /// >> [1, 2, 3, 4, 5, 6, 7, 8, 9]
    /// ```
    pub fn flatten_rows(&self) -> Vec<&u8> {
        self.data.iter().flatten().collect::<Vec<&u8>>()
    }

    /// Flattens the Matrix's data into a single vector.</br>
    /// It concatenates each column with each other.
    /// /// ### Examples
    /// ```
    /// let matrix = Matrix::from_shape(&[
    ///     [1, 2, 3],
    ///     [4, 5, 6],
    ///     [7, 8, 9]
    /// ], (3, 3));
    /// println!("{:?}", matrix.flatten_cols());
    /// >> [1, 4, 7, 2, 5, 8, 3, 6, 9]
    /// ```
    pub fn flatten_cols(&self) -> Vec<u8> {
        (0..self.col_size).flat_map(|a| self.data.clone().iter().map(|row| row[a]).collect::<Vec<u8>>()).collect()
    }

    /// Adds a row to the Matrix.
    pub fn push(&mut self, val: Vec<u8>) {
        assert_eq!(val.len(), self.col_size);
        self.data.push(val);
        self.rows += 1;
    }

    /// Deletes a row from the Matrix and returns it.
    /// ### Return value
    /// Returns `None` if the Matrix is empty or `Some(row)` otherwise.
    pub fn pop(&mut self) -> Option<Vec<u8>> {
        let res = self.data.pop()?;
        self.rows -= 1;
        Some(res)
    }
}

impl Clone for Matrix {
    fn clone(&self) -> Self {
        Self {
            col_size: self.col_size.clone(),
            rows: self.rows.clone(),
            data: self.data.clone()
        }
    }
}

impl Debug for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Matrix").field("col_size", &self.col_size).field("rows", &self.rows).field("data", &self.data).finish()
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.col_size == other.col_size && self.rows == other.rows && self.data == other.data
    }
}

impl Index<usize> for Matrix {
    type Output = Vec<u8>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

#[cfg(test)]
mod tests {
    use super::Matrix;

    #[test]
    fn test_new() {
        let matrix_0_2 = Matrix::new((0, 2));
        let test: Vec<Vec<u8>> = vec![];
        assert_eq!(matrix_0_2.data, test);

        let matrix_2_2 = Matrix::new((2, 2));
        assert_eq!(matrix_2_2.data, [[0, 0], [0, 0]]);

        let matrix_1_3 = Matrix::new((1, 3));
        assert_eq!(matrix_1_3.data, [[0, 0, 0]]);

        let matrix_4_2 = Matrix::new((4, 2));
        assert_eq!(matrix_4_2.data, [[0, 0], [0, 0], [0, 0], [0, 0]]);
    }

    #[test]
    fn test_from_shape() {
        let char_list = "Hello World!".as_bytes();

        let matrix_full = Matrix::from_shape(char_list, (3, 4));
        assert_eq!(matrix_full.data, [[72, 101, 108, 108], [111, 32, 87, 111], [114, 108, 100, 33]]);

        let matrix_reduced = Matrix::from_shape(char_list, (3, 3));
        assert_eq!(matrix_reduced.data, [[72, 101, 108], [108, 111, 32], [87, 111, 114]]);

        let matrix_filled = Matrix::from_shape(char_list, (4, 4));
        assert_eq!(matrix_filled.data, [[72, 101, 108, 108], [111, 32, 87, 111], [114, 108, 100, 33], [0, 0, 0, 0]]);
    }

    #[test]
    fn test_from_matrix_array() {
        let matrix = Matrix::from_matrix_array(&[
            [1, 2],
            [3, 4],
            [5, 6]
        ]);
        assert_eq!(matrix.data, &[[1, 2], [3, 4], [5, 6]]);
    }

    #[test]
    fn test_from_matrix_vec() {
        let vec = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
        let matrix = Matrix::from_matrix_vec(&vec);
        assert_eq!(matrix.data, vec);
    }

    #[test]
    fn test_matrix_mul() {
        let m1 = Matrix::from_matrix_array(&[[1, 2, 3], [4, 5, 6]]);
        let m2 = Matrix::from_matrix_array(&[[7, 8], [9, 10], [11, 12]]);
        let res = Matrix::mul(&m1, &m2);
        assert_eq!(res.data, &[[58, 64], [139, 154]]);
    }

    #[test]
    fn test_matrix_dot() {
        let res: [[u8; 2]; 2] = [[19, 22], [43, 50]];
        let mut m1 = Matrix::from_matrix_array(&[[1, 2], [3, 4]]);
        let m2 = Matrix::from_matrix_array(&[[5, 6], [7, 8]]);
        assert_eq!(m1.dot(&m2).data, &res);
        assert_eq!(m1.data, &res);
    }

    #[test]
    fn test_push_pop() {
        let mut matrix = Matrix::from_matrix_array(&[
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9]
        ]);

        assert_eq!(matrix.pop(), Some(vec![7, 8, 9]));
        assert_eq!(matrix.data, [[1, 2, 3], [4, 5, 6]]);

        matrix.push(vec![7, 8, 9]);
        assert_eq!(matrix.data, [[1, 2, 3], [4, 5, 6], [7, 8, 9]]);

        let mut empty_matrix = Matrix::new((0, 0));
        assert_eq!(empty_matrix.pop(), None);
    }

    #[test]
    fn test_get_col() {
        let cmp: Vec<u8> = vec![3, 6, 9];
        let matrix = Matrix::from_matrix_array(&[
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9]
        ]);
        assert_eq!(matrix.get_col(2), Some(cmp.iter().collect()));
        assert_eq!(matrix.get_col(99), None);
    }

    #[test]
    fn test_get_col_mut() {
        let mut cmp: Vec<u8> = vec![3, 6, 9];
        let mut matrix = Matrix::from_matrix_array(&[
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9]
        ]);
        assert_eq!(matrix.get_col_mut(2), Some(cmp.iter_mut().collect()));
        assert_eq!(matrix.get_col_mut(99), None);
    }

    #[test]
    fn test_get_row() {
        let cmp: Vec<u8> = vec![7, 8, 9];
        let matrix = Matrix::from_matrix_array(&[
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9]
        ]);
        assert_eq!(matrix.get_row(2), Some(&cmp));
        assert_eq!(matrix.get_row(99), None);
    }

    #[test]
    fn test_get_row_mut() {
        let mut cmp: Vec<u8> = vec![7, 8, 9];
        let mut matrix = Matrix::from_matrix_array(&[
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9]
        ]);
        assert_eq!(matrix.get_row_mut(2), Some(&mut cmp));
        assert_eq!(matrix.get_row_mut(99), None);

        let tmp = matrix.get_row_mut(2).unwrap();
        cmp.rotate_left(1);
        tmp.rotate_left(1);
        assert_eq!(matrix.get_row_mut(2), Some(&mut cmp));
    }
}
