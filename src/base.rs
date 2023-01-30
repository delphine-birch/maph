use std::ops::{Add, Sub, Mul, Div, Index, IndexMut};
use std::fmt; 
use std::fmt::Display;
use crate::identity::Identity;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector<const L: usize> {
    data: [f32; L],
}
impl<const L: usize> Vector<L> {
    pub fn new(data: [f32; L]) -> Self { Self { data } }
    pub fn dot(&self, other: Vector<L>) -> f32 {
        (0..L).map(|i| self.data[i]*other.data[i]).sum::<f32>()
    }
    pub fn as_row(&self) -> Matrix<1, L> {
        Matrix::<1, L>::new([self.data])
    }
    pub fn as_col(&self) -> Matrix<L, 1> {
        let mut columns = [[0.0]; L];
        for i in 0..L {
            columns[i][0] = self.data[i];
        }
        Matrix::<L, 1>::new(columns)
    }
    pub fn vec_add(&self, other: Self) -> Self {
        let mut data = [0.0; L];
        for i in 0..L {
            data[i] = self.data[i] + other.data[i];
        }
        Self { data }
    }
    pub fn vec_mul(&self, other: Self) -> Self {
        let mut data = [0.0; L];
        for i in 0..L {
            data[i] = self.data[i] * other.data[i];
        }
        Self { data }
    }
    pub fn float_mul(&self, other: f32) -> Self {
        let mut data = [0.0; L];
        for i in 0..L {
            data[i] = self.data[i] * other;
        }
        Self { data }
    }
    pub fn recip(&self) -> Self {
        let mut data = [0.0; L];
        for i in 0..L {
            data[i] = 1.0/self.data[i];
        }
        Self { data }
    }
    pub fn sum(&self) -> f32 {
        let mut sum = 0.0;
        for i in 0..L { sum += self[i] }
        sum
    }
    pub fn sq_sum(&self) -> f32 {
        let mut sum = 0.0;
        for i in 0..L { sum += self[i]*self[i] }
        sum
    }
    pub fn mag(&self) -> f32 {
        self.sq_sum().sqrt()
    }
    pub fn normalised(&self) -> Self { *self/self.mag() }
}

impl<const L: usize> Display for Vector<L> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.data)
    }
}
impl<const L: usize> Default for Vector<L> {
    fn default() -> Vector<L> { Vector::<L>::new([0.0; L]) }
}
impl<const L: usize> Identity for Vector<L> {
    fn identity() -> Vector<L> { Vector::<L>::new([1.0; L]) }
}
impl<const L: usize> Index<usize> for Vector<L> {
    type Output = f32;
    fn index(&self, index: usize) -> &f32 { &self.data[index] }
}
impl<const L: usize> IndexMut<usize> for Vector<L> {
    fn index_mut(&mut self, index: usize) -> &mut f32 { &mut self.data[index] }
}
impl<const L: usize> Add<Vector<L>> for Vector<L> {
    type Output = Vector<L>;
    fn add(self, other: Vector<L>) -> Self { self.vec_add(other) }
}

impl<const L: usize> Mul<Vector<L>> for Vector<L> {
    type Output = Vector<L>;
    fn mul(self, other: Vector<L>) -> Self { self.vec_mul(other) }
}

impl<const L: usize> Mul<f32> for Vector<L> {
    type Output = Vector<L>;
    fn mul(self, other: f32) -> Self { self.float_mul(other) }
}

impl<const L: usize> Mul<Vector<L>> for f32 {
    type Output = Vector<L>;
    fn mul(self, other: Vector<L>) -> Vector<L> { other.float_mul(self) }
}

impl<const L: usize> Div<f32> for Vector<L> {
    type Output = Vector<L>;
    fn div(self, other: f32) -> Self { self * (1.0/other) }
}

impl<const L: usize> Sub<Vector<L>> for Vector<L> {
    type Output = Vector<L>;
    fn sub(self, other: Vector<L>) -> Self { self + (other*-1.0) }
}

impl<const L: usize> Div<Vector<L>> for Vector<L> {
    type Output = Vector<L>;
    fn div(self, other: Vector<L>) -> Self { self * other.recip() }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Matrix<const R: usize, const C: usize> {
    data: [[f32; C]; R],
}
impl<const R: usize, const C: usize> Matrix<R, C> {
    pub fn new(data: [[f32; C]; R]) -> Self { Self { data } }
    pub fn num_rows() -> usize { R }
    pub fn col_len() -> usize { R }
    pub fn num_cols() -> usize { C }
    pub fn row_len() -> usize { R }
    pub fn row(&self, index: usize) -> Vector<C> {
        if index >= R { panic!("Out of bounds! Tried to access Row {}/{}", index, R); }
        Vector::<C> { data: self.data[index] }
    }
    pub fn col(&self, index: usize) -> Vector<R> {
        if index >= C { panic!("Out of bounds! Tried to access Column {}/{}", index, C); }
        let mut column = [0.0; R];
        for i in 0..R {
            column[i] = self.data[i][index];
        }
        Vector::<R> { data: column }
    }
    pub fn multiply<const D: usize>(&self, other: Matrix<C, D>) -> Matrix<R, D> {
        let mut out = [[0.0; D]; R];
        for i in 0..R {
            for j in 0..D {
                out[i][j] = self.row(i).dot(other.col(j));
            }
        }
        Matrix::<R, D> { data: out }
    }
}

impl<const L: usize> Matrix<L, L> {
    pub fn lu(&self) -> Option<(Self, Self)> {
        let mut l = Self::default(); let mut u = Self::default();
        for i in 0..L {
            for k in i..L {
                let mut sum = 0.0;
                for j in 0..i {
                    sum = sum + (l[i][j] * u[j][k]);
                }
                //eprintln!("u sum {} : {}", k, sum);
                u[i][k] = self[i][k] - sum;
                //eprintln!("setting u[{}][{}] to {}", i, k, self[i][k] - sum);
            }
            for k in i..L {
                if i == k { l[i][i] = 1.0 }
                else {
                    let mut sum = 0.0;
                    for j in 0..i {
                        sum = sum + (l[k][j] * u[j][i]);
                    }
                    //eprintln!("l sum {} : {}", k, sum);
                    if u[i][i] == 0.0 { return None; }
                    l[k][i] = (self[k][i] - sum)/u[i][i];
                    //eprintln!("setting l[{}][{}] to {}", i, k, (self[i][k] - sum)/u[i][i]);
                }
            }
        }
        Some((l, u))
    }
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
    pub fn det(&self) -> Option<f32> {
        match self.lu() {
            Some(lu) => {
                let u = lu.1;
                let mut sum = 1.0;
                for i in 0..L { sum *= u[i][i]; }
                Some(sum)
            },
            None => None
        }
        
    }
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
    pub fn lu_sub(&self, target: Vector<L>) -> Option<Vector<L>> {
        match self.lu() {
            Some(lu) => {
                let forward = lu.0.forward_sub(target);
                let back = lu.1.back_sub(forward);
                Some(back)
            },
            None => None,
        }
    }
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
    pub fn inverse(&self) -> Option<Matrix<L, L>> {
        if let Some(d) = self.det() { if d.abs() < 0.0001 { return None; } }
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