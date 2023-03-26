use std::ops::{Mul, Index, IndexMut};
use std::fmt; 
use std::fmt::Display;
use crate::num::{Identity, rational::*};
use super::vector::*;

///Matrix Type - R rows, C columns, components are f32. Stored in row-major format,
///indexable by usize indices - row then column.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Matrix<const R: usize, const C: usize> {
    ///2D Array of f32 data.
    pub data: [[f32; C]; R],
}
impl<const R: usize, const C: usize> Matrix<R, C> {
    ///Returns a new Matrix with R rows and C columns from a correctly shaped array of f32.
    pub fn new(data: [[f32; C]; R]) -> Self { Self { data } }
    ///Returns R, the number of rows in this Matrix Type (i.e. returns 3 for a Matrix<3, 4>).
    pub fn num_rows() -> usize { R }
    ///Returns R, the length of a column in this Matrix Type (i.e. returns 3 for a Matrix<3, 4>).
    pub fn col_len() -> usize { R }
    ///Returns C, the number of columns in this Matrix Type (i.e. returns 4 for a Matrix<3, 4>).
    pub fn num_cols() -> usize { C }
    ///Returns C, the length of a row in this Matrix Type (i.e. returns 4 for a Matrix<3, 4>).
    pub fn row_len() -> usize { R }
    ///Returns a single row of the Matrix as a Vector<C>, indexed by usize.
    pub fn row(&self, index: usize) -> Vector<C> {
        if index >= R { panic!("Out of bounds! Tried to access Row {}/{}", index, R); }
        Vector::<C> { data: self.data[index] }
    }
    ///Returns a single column of the Matrix as a Vector<R>, indexed by usize.
    pub fn col(&self, index: usize) -> Vector<R> {
        if index >= C { panic!("Out of bounds! Tried to access Column {}/{}", index, C); }
        let mut column = [0.0; R];
        for i in 0..R {
            column[i] = self.data[i][index];
        }
        Vector::<R> { data: column }
    }
    ///Utility function for multiplying matrices - as per standard matrix multiplication, to multiply
    ///two matrices of dimensions (A, B) and (C, D), B must equal C. Used for implementing std::ops.
    pub fn multiply<const D: usize>(&self, other: Matrix<C, D>) -> Matrix<R, D> {
        let mut out = [[0.0; D]; R];
        for i in 0..R {
            for j in 0..D {
                out[i][j] = self.row(i).dot(other.col(j));
            }
        }
        Matrix::<R, D> { data: out }
    }
    pub fn transpose(&self) -> Matrix<C, R> {
        let mut data = [[0.0; R]; C];
        for i in 0..R {
            for j in 0..C {
                data[j][i] = self.data[i][j];
            }
        }
        Matrix::<C, R>::new(data)
    }
    pub fn change_dimensions<const R0: usize, const C0: usize>(&self) -> Matrix<R0, C0> {
        let mut data = [[0.0; C0]; R0];

        for row in 0..R0 {
            if row < R {
                for col in 0..C0 {
                    if col < C {
                        data[row][col] = self.data[row][col];
                    }
                }
            }
        }

        Matrix::<R0, C0>::new(data)
    }
    pub fn minor(&self, row: usize, col: usize) -> Option<Matrix<R, C>> {
        if R <= 1 || C <= 1 || row >= R || col >= C { return None; }
        let mut data = [[0.0; C]; R];
        let (mut rowc, mut colc) = (0, 0);
        for row0 in 0..R {
            if row0 != row {
                for col0 in 0..C {
                    if col0 != col {
                        data[rowc][colc] = self.data[row0][col0]; 
                        colc += 1; 
                    }
                }
                rowc += 1;
                colc = 0; 
            }
        }
        Some(Self::new(data))
    }
    pub fn to_data_vec(&self) -> Vec<f32> {
        let mut v = Vec::new();
        for i in 0..R { for j in 0..C { v.push(self.data[i][j]); } }
        v
    } 
    pub fn minor_vec(vec: &Vec<f32>, dim: (usize, usize), index: (usize, usize)) -> Vec<f32> {
        if dim.0 == 0 || dim.1 == 0 || index.0 >= dim.0 || index.1 >= dim.1 { return Vec::new(); }
        else if dim.0 == 1 && dim.1 == 1 { return vec.clone(); }
        let mut data = Vec::new();
        for row0 in 0..dim.0 {
            if row0 != index.0 {
                for col0 in 0..dim.1 {
                    if col0 != index.1 {
                        let index0 = row0 * dim.0 + col0;
                        data.push(vec[index0]);
                    }
                }
            }
        }
        data
    }
}
impl<const L: usize> Matrix<L, L> {
    pub fn det(&self) -> Option<f32> { Self::det_vec(&self.to_data_vec(), L) }
    pub fn det_vec(vec: &Vec<f32>, dim: usize) -> Option<f32> {
        let len = vec.len();
        if len != dim*dim { return None; }
        if len == 1 { return Some(vec[0]); }
        else {
            let mut cofactors = Vec::new();
            for i in 0..dim {
                let item = vec[i];
                let cofactor = Self::minor_vec(vec, (dim, dim), (0, i));
                cofactors.push((item, Self::det_vec(&cofactor, dim - 1).unwrap()));
            }
            let (mut add, mut sum) = (true, 0.0);
            for cf in cofactors {
                match add {
                    true => sum += cf.0*cf.1,
                    false => sum -= cf.0*cf.1,
                }
                add = !add;
            }
            Some(sum)
        }
    }
    pub fn cofactor(&self) -> Option<Matrix<L, L>> {
        if L <= 1 { return None; }
        let mut data = [[0.0; L]; L];
        let v = self.to_data_vec();
        for row in 0..L {
            for col in 0..L {
                let minor_matrix = Self::minor_vec(&v, (L, L), (row, col));
                let minor = Self::det_vec(&minor_matrix, L - 1).unwrap();
                data[row][col] = (-1.0_f32).powf(row as f32 + col as f32)*minor;
            }
        }
        Some(Matrix::<L, L>::new(data))        
    }
    pub fn adjoint(&self) -> Option<Matrix<L, L>> {
        match self.cofactor() {
            Some(m) => Some(m.transpose()),
            None => None,
        }
    }
    pub fn inverse(&self) -> Option<Matrix<L, L>> {
        match self.adjoint() {
            Some(adjoint) => {
                let diag = *self * adjoint;
                let det = diag[0][0];
                if det.abs() < 0.00001 { return None; }
                else { return Some(adjoint * (1.0/det)); }
            },
            None => None,
        }
    }
    pub fn cofactor_vec(vec: &Vec<f32>, dim: usize) -> Option<Vec<f32>> {
        let len = vec.len();
        if len != dim*dim { return None; }
        if len <= 1 { return None; }
        let mut v = Vec::new();
        let (mut row, mut col) = (0, 0);
        for _ in 0..len {
            let minor_matrix = Self::minor_vec(vec, (L, L), (row, col));
            col += 1;
            if col >= dim { col = 0; row += 1; }
            let minor = Self::det_vec(&minor_matrix, L - 1).unwrap();
            v.push((-1.0_f32).powf(row as f32 + col as f32)*minor);
        }
        Some(v)
    }
    ///LU decomposition - may not succeed, tries to triangulate into two triangular matrices, L and U.
    pub fn lu(&self) -> Option<(Self, Self)> {
        let mut l = Self::default(); let mut u = Self::default();
        for i in 0..L {
            for k in i..L {
                let mut sum = 0.0;
                for j in 0..i {
                    sum = sum + (l[i][j] * u[j][k]);
                }
                u[i][k] = self[i][k] - sum;
            }
            for k in i..L {
                if i == k { l[i][i] = 1.0 }
                else {
                    let mut sum = 0.0;
                    for j in 0..i {
                        sum = sum + (l[k][j] * u[j][i]);
                    }
                    if u[i][i] == 0.0 { return None; }
                    l[k][i] = (self[k][i] - sum)/u[i][i];
                }
            }
        }
        Some((l, u))
    }
    ///LUP decomposition - may not succeed, tries to decompose into two triangular matrices
    ///L and U, with a permutation matrix P.
    pub fn lup(&self) -> Option<(Self, Self, Self)> {
        let mut p = Self::identity();
        let mut check = [false; L];
        for i in 0..L {
            let col = self.col(i);
            let mut max = (0, 0.0);
            for j in 0..L {
                if !check[j] {
                    if check[max.0] || col[j] > max.1 { max.0 = j; max.1 = col[j]; }
                }
            }
            let mut p_row = [0.0; L];
            p_row[max.0] = 1.0;
            p[i] = p_row;
            check[max.0] = true;
        }
        match (p * *self).lu() {
            Some(lu) => Some((lu.0, lu.1, p)),
            None => None,
        }
    }
    ///Tries to calculate the determinant using LUP decomposition.
    pub fn lup_det(&self) -> Option<f32> {
        match self.lu() {
            Some(lup) => {
                let u = lup.1;
                let mut sum = 1.0;
                for i in 0..L { sum *= u[i][i]; }
                Some(sum)
            },
            None => None
        }
        
    }
    ///Forward substitution - solves for Lx = b where L is a lower triangular matrix.
    ///Takes target vector b and calculates x, where the matrix is L.
    pub fn forward_sub(&self, target: Vector<L>) -> Vector<L> {
        let mut out = Vector::<L>::default();
        out[0] = target[0];
        for i in 1..L {
            let row = self.row(i);
            let v = target[i] - row.dot(out);
            out[i] = v;
        }
        out
    }
    ///Backward substitution - solves for Ux = b where U is an upper triangular matrix.
    ///Takes target vector b and calculates x, where the matrix is U.
    pub fn back_sub(&self, target: Vector<L>) -> Vector<L> {
        let mut out = Vector::<L>::default();
        for ii in 0..L {
            let i = L - ii - 1;
            let mut v = target[i];
            for j in i+1..L {
                v -= self[i][j]*out[j];
            }
            out[i] = v/self[i][i];
        }
        out
    }
    ///LUP substitution - uses LUP decomposition to solve for Ax = b, where A is the matrix
    ///calling this, b is the target vector provided as an argument, and x is returned - dependent
    ///on successful LUP decomposition.
    pub fn lup_sub(&self, target: Vector<L>) -> Option<Vector<L>> {
        match self.lup() {
            Some(lup) => {
                let p_target = lup.2 * target;
                let forward = lup.0.forward_sub(p_target);
                let back = lup.1.back_sub(forward);
                Some(back)
            },
            None => None,
        }
    }
    ///Attempts to calculate an inverse matrix for this matrix using LUP substitution. This will
    ///fail if a LUP decomposition cannot be found for the matrix.
    pub fn lup_inverse(&self) -> Option<Matrix<L, L>> {
        if let Some(d) = self.lup_det() { if d.abs() < 0.00000001 { return None; } }
        let mut columns = [Vector::<L>::default(); L];
        let identity = Self::identity();
        for i in 0..L {
            let Some(ci) = self.lup_sub(identity.row(i)) else { return None; };
            columns[i] = ci; //USING ROWS FOR EFFICIENCY - ITS COLUMNS
        }
        let mut data = [[0.0; L]; L];
        for i in 0..L {
            for j in 0..L {
                data[i][j] = columns[j][i];
            }
        }
        Some(Self::new(data))
    }
}

impl<const A: usize, const B: usize> Display for Matrix<A, B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rows = [Vector::<B>::default(); A];
        for i in 0..A { rows[i] = self.row(i); }
        write!(f, "{:?}", rows)
    }
}
impl<const A: usize, const B: usize> Default for Matrix<A, B> {
    fn default() -> Self {
        Self::new([[0.0; B]; A])
    }
}
impl<const A: usize> Identity for Matrix<A, A> {
    fn identity() -> Self {
        let mut data = [[0.0; A]; A];
        for i in 0..A {
            data[i][i] = 1.0;
        }
        Self::new(data)
    }
}
impl<const A: usize, const B: usize> Index<usize> for Matrix<A, B> {
    type Output = [f32; B];
    fn index(&self, index: usize) -> &[f32; B] { &self.data[index] }
}
impl<const A: usize, const B: usize> IndexMut<usize> for Matrix<A, B> {
    fn index_mut(&mut self, index: usize) -> &mut [f32; B] { &mut self.data[index] }
}
impl<const A: usize, const B: usize, const C: usize> Mul<Matrix<B, C>> for Matrix<A, B> {
    type Output = Matrix<A, C>;
    fn mul(self, other: Matrix<B, C>) -> Matrix<A, C> { self.multiply(other) }
}
impl<const A: usize, const B: usize> Mul<Vector<B>> for Matrix<A, B> {
    type Output = Vector<A>;
    fn mul(self, other: Vector<B>) -> Vector<A> {
        self.mul(other.as_col()).col(0)
    }
}
impl<const A: usize, const B: usize> Mul<Matrix<A, B>> for Vector<A> {
    type Output = Vector<B>;
    fn mul(self, other: Matrix<A, B>) -> Vector<B> {
        self.as_row().mul(other).row(0)
    }
}
impl<const A: usize, const B: usize> Mul<f32> for Matrix<A, B> {
    type Output = Matrix<A, B>;
    fn mul(self, other: f32) -> Matrix<A, B> {
        let mut data = self.data;
        for x in 0..A { for y in 0..B { data[x][y] *= other; } }
        Self::new(data)
    }
}
impl<const A: usize, const B: usize> Mul<Matrix<A, B>> for f32 {
    type Output = Matrix<A, B>;
    fn mul(self, other: Matrix<A, B>) -> Matrix<A, B> {
        let mut data = other.data;
        for x in 0..A { for y in 0..B { data[x][y] *= self; } }
        Matrix::<A, B>::new(data)
    }
}

///Precise Matrix Type using rational components - R rows, C columns, components are r32. Stored in row-major format,
///indexable by usize indices - row then column. 
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MatrixPrecise<const R: usize, const C: usize> {
    ///2D Array of r32 data.
    pub data: [[r32; C]; R],
}
impl<const R: usize, const C: usize> MatrixPrecise<R, C> {
    ///Returns a new Matrix with R rows and C columns from a correctly shaped array of r32.
    pub fn new(data: [[r32; C]; R]) -> Self { Self { data } }
    ///Returns R, the number of rows in this Matrix Type (i.e. returns 3 for a Matrix<3, 4>).
    pub fn num_rows() -> usize { R }
    ///Returns R, the length of a column in this Matrix Type (i.e. returns 3 for a Matrix<3, 4>).
    pub fn col_len() -> usize { R }
    ///Returns C, the number of columns in this Matrix Type (i.e. returns 4 for a Matrix<3, 4>).
    pub fn num_cols() -> usize { C }
    ///Returns C, the length of a row in this Matrix Type (i.e. returns 4 for a Matrix<3, 4>).
    pub fn row_len() -> usize { R }
    ///Returns a single row of the Matrix as a VectorPrecise<C>, indexed by usize.
    pub fn row(&self, index: usize) -> VectorPrecise<C> {
        if index >= R { panic!("Out of bounds! Tried to access Row {}/{}", index, R); }
        VectorPrecise::<C> { data: self.data[index] }
    }
    ///Returns a single column of the Matrix as a VectorPrecise<R>, indexed by usize.
    pub fn col(&self, index: usize) -> VectorPrecise<R> {
        if index >= C { panic!("Out of bounds! Tried to access Column {}/{}", index, C); }
        let mut column = [r32::default(); R];
        for i in 0..R {
            column[i] = self.data[i][index];
        }
        VectorPrecise::<R> { data: column }
    }
    ///Utility function for multiplying matrices - as per standard matrix multiplication, to multiply
    ///two matrices of dimensions (A, B) and (C, D), B must equal C. Used for implementing std::ops.
    pub fn multiply<const D: usize>(&self, other: MatrixPrecise<C, D>) -> MatrixPrecise<R, D> {
        let mut out = [[r32::default(); D]; R];
        for i in 0..R {
            for j in 0..D {
                out[i][j] = self.row(i).dot(other.col(j));
            }
        }
        MatrixPrecise::<R, D> { data: out }
    }
    pub fn transpose(&self) -> MatrixPrecise<C, R> {
        let mut data = [[r32::default(); R]; C];
        for i in 0..R {
            for j in 0..C {
                data[j][i] = self.data[i][j];
            }
        }
        MatrixPrecise::<C, R>::new(data)
    }
    pub fn change_dimensions<const R0: usize, const C0: usize>(&self) -> MatrixPrecise<R0, C0> {
        let mut data = [[r32::default(); C0]; R0];

        for row in 0..R0 {
            if row < R {
                for col in 0..C0 {
                    if col < C {
                        data[row][col] = self.data[row][col];
                    }
                }
            }
        }
        MatrixPrecise::<R0, C0>::new(data)
    }
    pub fn minor(&self, row: usize, col: usize) -> Option<MatrixPrecise<R, C>> {
        if R <= 1 || C <= 1 || row >= R || col >= C { return None; }
        let mut data = [[r32::default(); C]; R];
        let (mut rowc, mut colc) = (0, 0);
        for row0 in 0..R {
            if row0 != row {
                for col0 in 0..C {
                    if col0 != col {
                        data[rowc][colc] = self.data[row0][col0]; 
                        colc += 1; 
                    }
                }
                rowc += 1;
                colc = 0; 
            }
        }
        Some(Self::new(data))
    }
    pub fn to_data_vec(&self) -> Vec<r32> {
        let mut v = Vec::new();
        for i in 0..R { for j in 0..C { v.push(self.data[i][j]); } }
        v
    } 
    pub fn minor_vec(vec: &Vec<r32>, dim: (usize, usize), index: (usize, usize)) -> Vec<r32> {
        if dim.0 == 0 || dim.1 == 0 || index.0 >= dim.0 || index.1 >= dim.1 { return Vec::new(); }
        else if dim.0 == 1 && dim.1 == 1 { return vec.clone(); }
        let mut data = Vec::new();
        for row0 in 0..dim.0 {
            if row0 != index.0 {
                for col0 in 0..dim.1 {
                    if col0 != index.1 {
                        let index0 = row0 * dim.0 + col0;
                        data.push(vec[index0]);
                    }
                }
            }
        }
        data
    }
}
impl<const R: usize, const C: usize> From<Matrix<R, C>> for MatrixPrecise<R, C> {
    fn from(v: Matrix<R, C>) -> Self {
        let mut data = [[r32::default(); C]; R];
        for i in 0..R {
            for j in 0..C {
                data[i][j] = r32::from(v.data[i][j]);
            }
        }
        Self::new(data)
    }
}
impl<const R: usize, const C: usize> From<MatrixPrecise<R, C>> for Matrix<R, C> {
    fn from(v: MatrixPrecise<R, C>) -> Self {
        let mut data = [[0.0; C]; R];
        for i in 0..R {
            for j in 0..C {
                data[i][j] = f32::from(v.data[i][j]);
            }
        }
        Self::new(data)
    }
}
impl<const L: usize> MatrixPrecise<L, L> {
    pub fn det(&self) -> Option<r32> { Self::det_vec(&self.to_data_vec(), L) }
    pub fn det_vec(vec: &Vec<r32>, dim: usize) -> Option<r32> {
        let len = vec.len();
        if len != dim*dim { return None; }
        if len == 1 { return Some(vec[0]); }
        else {
            let mut cofactors = Vec::new();
            for i in 0..dim {
                let item = vec[i];
                let cofactor = Self::minor_vec(vec, (dim, dim), (0, i));
                cofactors.push((item, Self::det_vec(&cofactor, dim - 1).unwrap()));
            }
            let (mut add, mut sum) = (true, r32::default());
            for cf in cofactors {
                match add {
                    true => sum += cf.0*cf.1,
                    false => sum -= cf.0*cf.1,
                }
                add = !add;
            }
            Some(sum)
        }
    }
    pub fn cofactor(&self) -> Option<MatrixPrecise<L, L>> {
        if L <= 1 { return None; }
        let mut data = [[r32::default(); L]; L];
        let v = self.to_data_vec();
        for row in 0..L {
            for col in 0..L {
                let minor_matrix = Self::minor_vec(&v, (L, L), (row, col));
                let minor = Self::det_vec(&minor_matrix, L - 1).unwrap();
                data[row][col] = r32::from((-1.0_f32).powf(row as f32 + col as f32))*minor;
            }
        }
        Some(MatrixPrecise::<L, L>::new(data))        
    }
    pub fn adjoint(&self) -> Option<MatrixPrecise<L, L>> {
        match self.cofactor() {
            Some(m) => Some(m.transpose()),
            None => None,
        }
    }
    pub fn inverse(&self) -> Option<MatrixPrecise<L, L>> {
        match self.adjoint() {
            Some(adjoint) => {
                let diag = *self * adjoint;
                let det = diag[0][0];
                if det == r32::default() { return None; }
                else { return Some(adjoint * det.reciprocal()); }
            },
            None => None,
        }
    }
    pub fn cofactor_vec(vec: &Vec<r32>, dim: usize) -> Option<Vec<r32>> {
        let len = vec.len();
        if len != dim*dim { return None; }
        if len <= 1 { return None; }
        let mut v = Vec::new();
        let (mut row, mut col) = (0, 0);
        for _ in 0..len {
            let minor_matrix = Self::minor_vec(vec, (L, L), (row, col));
            col += 1;
            if col >= dim { col = 0; row += 1; }
            let minor = Self::det_vec(&minor_matrix, L - 1).unwrap();
            v.push(r32::from((-1.0_f32).powf(row as f32 + col as f32))*minor);
        }
        Some(v)
    }
    ///LU decomposition - may not succeed, tries to triangulate into two triangular matrices, L and U.
    pub fn lu(&self) -> Option<(Self, Self)> {
        let mut l = Self::default(); let mut u = Self::default();
        for i in 0..L {
            for k in i..L {
                let mut sum = r32::default();
                for j in 0..i {
                    sum = sum + (l[i][j] * u[j][k]);
                }
                u[i][k] = self[i][k] - sum;
            }
            for k in i..L {
                if i == k { l[i][i] = r32::identity() }
                else {
                    let mut sum = r32::default();
                    for j in 0..i {
                        sum = sum + (l[k][j] * u[j][i]);
                    }
                    if u[i][i] == r32::default() { return None; }
                    l[k][i] = (self[k][i] - sum)/u[i][i];
                }
            }
        }
        Some((l, u))
    }
    ///LUP decomposition - may not succeed, tries to decompose into two triangular matrices
    ///L and U, with a permutation matrix P.
    pub fn lup(&self) -> Option<(Self, Self, Self)> {
        let mut p = Self::identity();
        let mut check = [false; L];
        for i in 0..L {
            let col = self.col(i);
            let mut max = (0, r32::default());
            for j in 0..L {
                if !check[j] {
                    if check[max.0] || col[j] > max.1 { max.0 = j; max.1 = col[j]; }
                }
            }
            let mut p_row = [r32::default(); L];
            p_row[max.0] = r32::identity();
            p[i] = p_row;
            check[max.0] = true;
        }
        match (p * *self).lu() {
            Some(lu) => Some((lu.0, lu.1, p)),
            None => None,
        }
    }
    ///Tries to calculate the determinant using LUP decomposition.
    pub fn lup_det(&self) -> Option<r32> {
        match self.lu() {
            Some(lu) => {
                let u = lu.1;
                let mut sum = r32::identity();
                for i in 0..L { sum *= u[i][i]; }
                Some(sum)
            },
            None => None
        }
        
    }
    ///Forward substitution - solves for Lx = b where L is a lower triangular matrix.
    ///Takes target vector b and calculates x, where the matrix is L.
    pub fn forward_sub(&self, target: VectorPrecise<L>) -> VectorPrecise<L> {
        let mut out = VectorPrecise::<L>::default();
        out[0] = target[0];
        for i in 1..L {
            let row = self.row(i);
            let v = target[i] - row.dot(out);
            out[i] = v;
        }
        out
    }
    ///Backward substitution - solves for Ux = b where U is an upper triangular matrix.
    ///Takes target vector b and calculates x, where the matrix is U.
    pub fn back_sub(&self, target: VectorPrecise<L>) -> VectorPrecise<L> {
        let mut out = VectorPrecise::<L>::default();
        for ii in 0..L {
            let i = L - ii - 1;
            let mut v = target[i];
            for j in i+1..L {
                v -= self[i][j]*out[j];
            }
            out[i] = v/self[i][i];
        }
        out
    }
    ///LUP substitution - uses LUP decomposition to solve for Ax = b, where A is the matrix
    ///calling this, b is the target vector provided as an argument, and x is returned - dependent
    ///on successful LUP decomposition.
    pub fn lup_sub(&self, target: VectorPrecise<L>) -> Option<VectorPrecise<L>> {
        match self.lup() {
            Some(lup) => {
                let p_target = lup.2 * target;
                let forward = lup.0.forward_sub(p_target);
                let back = lup.1.back_sub(forward);
                Some(back)
            },
            None => None,
        }
    }
    ///Attempts to calculate an inverse matrix for this matrix using LUP substitution. This will
    ///fail if a LUP decomposition cannot be found for the matrix.
    pub fn lup_inverse(&self) -> Option<MatrixPrecise<L, L>> {
        if let Some(d) = self.lup_det() { if d.abs() < r32::default() { return None; } }
        let mut columns = [VectorPrecise::<L>::default(); L];
        let identity = Self::identity();
        for i in 0..L {
            let Some(ci) = self.lup_sub(identity.row(i)) else { return None; };
            columns[i] = ci; //USING ROWS FOR EFFICIENCY - ITS COLUMNS
        }
        let mut data = [[r32::default(); L]; L];
        for i in 0..L {
            for j in 0..L {
                data[i][j] = columns[j][i];
            }
        }
        Some(Self::new(data))
    }
}

impl<const A: usize, const B: usize> Display for MatrixPrecise<A, B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rows = [VectorPrecise::<B>::default(); A];
        for i in 0..A { rows[i] = self.row(i); }
        write!(f, "{:?}", rows)
    }
}
impl<const A: usize, const B: usize> Default for MatrixPrecise<A, B> {
    fn default() -> Self {
        Self::new([[r32::default(); B]; A])
    }
}
impl<const A: usize> Identity for MatrixPrecise<A, A> {
    fn identity() -> Self {
        let mut data = [[r32::default(); A]; A];
        for i in 0..A {
            data[i][i] = r32::identity();
        }
        Self::new(data)
    }
}
impl<const A: usize, const B: usize> Index<usize> for MatrixPrecise<A, B> {
    type Output = [r32; B];
    fn index(&self, index: usize) -> &[r32; B] { &self.data[index] }
}
impl<const A: usize, const B: usize> IndexMut<usize> for MatrixPrecise<A, B> {
    fn index_mut(&mut self, index: usize) -> &mut [r32; B] { &mut self.data[index] }
}
impl<const A: usize, const B: usize, const C: usize> Mul<MatrixPrecise<B, C>> for MatrixPrecise<A, B> {
    type Output = MatrixPrecise<A, C>;
    fn mul(self, other: MatrixPrecise<B, C>) -> MatrixPrecise<A, C> { self.multiply(other) }
}
impl<const A: usize, const B: usize> Mul<VectorPrecise<B>> for MatrixPrecise<A, B> {
    type Output = VectorPrecise<A>;
    fn mul(self, other: VectorPrecise<B>) -> VectorPrecise<A> {
        self.mul(other.as_col()).col(0)
    }
}
impl<const A: usize, const B: usize> Mul<MatrixPrecise<A, B>> for VectorPrecise<A> {
    type Output = VectorPrecise<B>;
    fn mul(self, other: MatrixPrecise<A, B>) -> VectorPrecise<B> {
        self.as_row().mul(other).row(0)
    }
}
impl<const A: usize, const B: usize> Mul<r32> for MatrixPrecise<A, B> {
    type Output = MatrixPrecise<A, B>;
    fn mul(self, other: r32) -> MatrixPrecise<A, B> {
        let mut data = self.data;
        for x in 0..A { for y in 0..B { data[x][y] *= other; } }
        Self::new(data)
    }
}
impl<const A: usize, const B: usize> Mul<MatrixPrecise<A, B>> for r32 {
    type Output = MatrixPrecise<A, B>;
    fn mul(self, other: MatrixPrecise<A, B>) -> MatrixPrecise<A, B> {
        let mut data = other.data;
        for x in 0..A { for y in 0..B { data[x][y] *= self; } }
        MatrixPrecise::<A, B>::new(data)
    }
}