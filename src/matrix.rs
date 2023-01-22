use std::ops::{Add, Sub, Mul, MulAssign, Div, Index, IndexMut};
use std::fmt; 
use std::fmt::Display;
use crate::identity::Identity;
use crate::vector::*;
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Mat2<T> 
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    data: [[T; 2]; 2]
}
impl<T> Mat2<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    pub fn new(rows: [Vector2<T>; 2]) -> Self {
        Self { data: [[rows[0].x, rows[1].x], [rows[0].y, rows[1].y]] }
    }
    pub fn row(&self, index: usize) -> Vector2<T> {
        Vector2::new(self[0][index], self[1][index])
    }
    pub fn col(&self, index: usize) -> Vector2<T> {
        Vector2::new(self[index][0], self[index][1])
    }
}
impl<T> Default for Mat2<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn default() -> Self { Self { data: [[T::default(); 2]; 2] }}
}
impl<T> Display for Mat2<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy + Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.row(0), self.row(1))
    }
}
impl<T> Mat2<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy + Default + Identity
{
    pub fn lu(&self) -> (Self, Self) {
        let mut l = Self::default(); let mut u = Self::default();
        for i in 0..2 {
            for k in i..2 {
                let mut sum = T::default();
                for j in 0..i {
                    sum = sum + l[j][i] * u[k][j];
                }
                u[k][i] = self[k][i] - sum;
            }
            for k in i..2 {
                if i == k { l[i][i] = T::identity() }
                else {
                    let mut sum = T::default();
                    for j in 0..i {
                        sum = sum + l[j][k]* u[i][j];
                    }
                    l[i][k] = (self[i][k] - sum)/u[i][i];
                }
            }
        }
        (l, u)
    }
    pub fn det(&self) -> T {
        let u = self.lu().1;
        u[0][0]*u[1][1]
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Mat3<T> 
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    data: [[T; 3]; 3]
}
impl<T> Mat3<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    pub fn new(rows: [Vector3<T>; 3]) -> Self {
        Self { data: [[rows[0].x, rows[1].x, rows[2].x], [rows[0].y, rows[1].y, rows[2].y], [rows[0].z, rows[1].z, rows[2].z]] }
    }
    pub fn row(&self, index: usize) -> Vector3<T> {
        Vector3::new(self[0][index], self[1][index], self[2][index])
    }
    pub fn col(&self, index: usize) -> Vector3<T> {
        Vector3::new(self[index][0], self[index][1], self[index][2])
    }
}
impl<T> Default for Mat3<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn default() -> Self { Self { data: [[T::default(); 3]; 3] }}
}
impl<T> Display for Mat3<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy + Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.row(0), self.row(1), self.row(2))
    }
}
impl<T> Mat3<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy + Default + Identity
{
    pub fn lu(&self) -> (Self, Self) {
        let mut l = Self::default(); let mut u = Self::default();
        for i in 0..3 {
            for k in i..3 {
                let mut sum = T::default();
                for j in 0..i {
                    sum = sum + l[j][i] * u[k][j];
                }
                u[k][i] = self[k][i] - sum;
            }
            for k in i..3 {
                if i == k { l[i][i] = T::identity() }
                else {
                    let mut sum = T::default();
                    for j in 0..i {
                        sum = sum + l[j][k]* u[i][j];
                    }
                    l[i][k] = (self[i][k] - sum)/u[i][i];
                }
            }
        }
        (l, u)
    }
    pub fn det(&self) -> T {
        let u = self.lu().1;
        u[0][0]*u[1][1]*u[2][2]
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Mat4<T> 
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    data: [[T; 4]; 4]
}
impl<T> Mat4<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    pub fn new(rows: [Vector4<T>; 4]) -> Self {
        Self { data: [[rows[0].x, rows[1].x, rows[2].x, rows[3].x], [rows[0].y, rows[1].y, rows[2].y, rows[3].y], [rows[0].z, rows[1].z, rows[2].z, rows[3].z], [rows[0].w, rows[1].w, rows[2].w, rows[3].w]] }
    }
    pub fn row(&self, index: usize) -> Vector4<T> {
        Vector4::new(self[0][index], self[1][index], self[2][index], self[3][index])
    }
    pub fn col(&self, index: usize) -> Vector4<T> {
        Vector4::new(self[index][0], self[index][1], self[index][2], self[index][3])
    }
}
impl<T> Default for Mat4<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn default() -> Self { Self { data: [[T::default(); 4]; 4] }}
}
impl<T> Display for Mat4<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy + Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}, {}]", self.row(0), self.row(1), self.row(2), self.row(3))
    }
}
impl<T> Mat4<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy + Default + Identity
{
    pub fn lu(&self) -> (Self, Self) {
        let mut l = Self::default(); let mut u = Self::default();
        for i in 0..4 {
            for k in i..4 {
                let mut sum = T::default();
                for j in 0..i {
                    sum = sum + l[j][i] * u[k][j];
                }
                u[k][i] = self[k][i] - sum;
            }
            for k in i..4 {
                if i == k { l[i][i] = T::identity() }
                else {
                    let mut sum = T::default();
                    for j in 0..i {
                        sum = sum + l[j][k]* u[i][j];
                    }
                    l[i][k] = (self[i][k] - sum)/u[i][i];
                }
            }
        }
        (l, u)
    }
    pub fn det(&self) -> T {
        let u = self.lu().1;
        u[0][0]*u[1][1]*u[2][2]*u[3][3]
    }
}

impl<T> Index<usize> for Mat2<T> 
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = [T; 2];
    fn index(&self, index: usize) -> &[T; 2] {
        &self.data[index]
    }
}
impl<T> IndexMut<usize> for Mat2<T> 
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn index_mut(&mut self, index: usize) -> &mut [T; 2] {
        &mut self.data[index]
    }
}
impl<T> Index<usize> for Mat3<T> 
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = [T; 3];
    fn index(&self, index: usize) -> &[T; 3] {
        &self.data[index]
    }
}
impl<T> IndexMut<usize> for Mat3<T> 
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn index_mut(&mut self, index: usize) -> &mut [T; 3] {
        &mut self.data[index]
    }
}
impl<T> Index<usize> for Mat4<T> 
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = [T; 4];
    fn index(&self, index: usize) -> &[T; 4] {
        &self.data[index]
    }
}
impl<T> IndexMut<usize> for Mat4<T> 
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn index_mut(&mut self, index: usize) -> &mut [T; 4] {
        &mut self.data[index]
    }
}

impl<T> Mul<Self> for Mat4<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let mut data = [[T::default(); 4]; 4];
        for row in 0..4 {
            for col in 0..4 {
                data[col][row] = self.row(row).dot(&other.col(col));
            }
        }
        Self { data }
    }
}
impl<T> Mul<&Self> for Mat4<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Self;
    fn mul(self, other: &Self) -> Self {
        let mut data = [[T::default(); 4]; 4];
        for row in 0..4 {
            for col in 0..4 {
                data[col][row] = self.row(row).dot(&other.col(col));
            }
        }
        Self { data }
    }
}
impl<T> Mul<&Self> for &Mat4<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Mat4<T>;
    fn mul(self, other: &Self) -> Mat4<T> {
        let mut data = [[T::default(); 4]; 4];
        for row in 0..4 {
            for col in 0..4 {
                data[col][row] = self.row(row).dot(&other.col(col));
            }
        }
        Mat4::<T> { data }
    }
}
impl<T> Mul<Self> for &Mat4<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Mat4<T>;
    fn mul(self, other: Self) -> Mat4<T> {
        let mut data = [[T::default(); 4]; 4];
        for row in 0..4 {
            for col in 0..4 {
                data[col][row] = self.row(row).dot(&other.col(col));
            }
        }
        Mat4::<T> { data }
    }
}
impl<T> Mul<T> for Mat4<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Self;
    fn mul(self, other: T) -> Self {
        let mut data = [[T::default(); 4]; 4];
        for row in 0..4 {
            for col in 0..4 {
                data[col][row] = self[col][row] * other;
            }
        }
        Self { data }
    }
}
impl<T> Mul<T> for &Mat4<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Mat4<T>;
    fn mul(self, other: T) -> Mat4<T> {
        let mut data = [[T::default(); 4]; 4];
        for row in 0..4 {
            for col in 0..4 {
                data[col][row] = self[col][row] * other;
            }
        }
        Mat4::<T> { data }
    }
}
impl<T> Mul<Vector4<T>> for Mat4<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector4<T>;
    fn mul(self, other: Vector4<T>) -> Vector4<T> {
        Vector4::<T>::new(
            other.dot(&self.row(0)),
            other.dot(&self.row(1)),
            other.dot(&self.row(2)),
            other.dot(&self.row(3))
        )
    }
}
impl<T> Mul<Vector4<T>> for &Mat4<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector4<T>;
    fn mul(self, other: Vector4<T>) -> Vector4<T> {
        Vector4::<T>::new(
            other.dot(&self.row(0)),
            other.dot(&self.row(1)),
            other.dot(&self.row(2)),
            other.dot(&self.row(3))
        )
    }
}
impl<T> Mul<&Vector4<T>> for &Mat4<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector4<T>;
    fn mul(self, other: &Vector4<T>) -> Vector4<T> {
        Vector4::<T>::new(
            other.dot(&self.row(0)),
            other.dot(&self.row(1)),
            other.dot(&self.row(2)),
            other.dot(&self.row(3))
        )
    }
}
impl<T> Mul<&Vector4<T>> for Mat4<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector4<T>;
    fn mul(self, other: &Vector4<T>) -> Vector4<T> {
        Vector4::<T>::new(
            other.dot(&self.row(0)),
            other.dot(&self.row(1)),
            other.dot(&self.row(2)),
            other.dot(&self.row(3))
        )
    }
}
impl<T> Mul<Mat4<T>> for Vector4<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector4<T>;
    fn mul(self, other: Mat4<T>) -> Vector4<T> {
        Vector4::<T>::new(
            self.dot(&other.col(0)),
            self.dot(&other.col(1)),
            self.dot(&other.col(2)),
            self.dot(&other.col(3))
        )
    }
}
impl<T> Mul<Mat4<T>> for &Vector4<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector4<T>;
    fn mul(self, other: Mat4<T>) -> Vector4<T> {
        Vector4::<T>::new(
            self.dot(&other.col(0)),
            self.dot(&other.col(1)),
            self.dot(&other.col(2)),
            self.dot(&other.col(3))
        )
    }
}
impl<T> Mul<&Mat4<T>> for Vector4<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector4<T>;
    fn mul(self, other: &Mat4<T>) -> Vector4<T> {
        Vector4::<T>::new(
            self.dot(&other.col(0)),
            self.dot(&other.col(1)),
            self.dot(&other.col(2)),
            self.dot(&other.col(3))
        )
    }
}
impl<T> Mul<&Mat4<T>> for &Vector4<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector4<T>;
    fn mul(self, other: &Mat4<T>) -> Vector4<T> {
        Vector4::<T>::new(
            self.dot(&other.col(0)),
            self.dot(&other.col(1)),
            self.dot(&other.col(2)),
            self.dot(&other.col(3))
        )
    }
}
impl<T> MulAssign<Self> for Mat4<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn mul_assign(&mut self, other: Self) {
        let mut data = [[T::default(); 4]; 4];
        for row in 0..4 {
            for col in 0..4 {
                data[col][row] = self.row(row).dot(&other.col(col));
            }
        }
        self.data = data;
    }
}
impl<T> MulAssign<&Self> for Mat4<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn mul_assign(&mut self, other: &Self) {
        let mut data = [[T::default(); 4]; 4];
        for row in 0..4 {
            for col in 0..4 {
                data[col][row] = self.row(row).dot(&other.col(col));
            }
        }
        self.data = data;
    }
}
impl<T> MulAssign<&Self> for &mut Mat4<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn mul_assign(&mut self, other: &Self) {
        let mut data = [[T::default(); 4]; 4];
        for row in 0..4 {
            for col in 0..4 {
                data[col][row] = self.row(row).dot(&other.col(col));
            }
        }
        self.data = data;
    }
}
impl<T> MulAssign<Self> for &mut Mat4<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn mul_assign(&mut self, other: Self) {
        let mut data = [[T::default(); 4]; 4];
        for row in 0..4 {
            for col in 0..4 {
                data[col][row] = self.row(row).dot(&other.col(col));
            }
        }
        self.data = data;
    }
}
impl<T> MulAssign<T> for Mat4<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn mul_assign(&mut self, other: T) {
        for row in 0..4 {
            for col in 0..4 {
                self[col][row] = self[col][row] * other;
            }
        }
    }
}
impl<T> MulAssign<T> for &mut Mat4<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn mul_assign(&mut self, other: T) {
        for row in 0..4 {
            for col in 0..4 {
                self[col][row] = self[col][row] * other;
            }
        }
    }
}

impl<T> Mul<Self> for Mat3<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let mut data = [[T::default(); 3]; 3];
        for row in 0..3 {
            for col in 0..3 {
                data[col][row] = self.row(row).dot(&other.col(col));
            }
        }
        Self { data }
    }
}
impl<T> Mul<&Self> for Mat3<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Self;
    fn mul(self, other: &Self) -> Self {
        let mut data = [[T::default(); 3]; 3];
        for row in 0..3 {
            for col in 0..3 {
                data[col][row] = self.row(row).dot(&other.col(col));
            }
        }
        Self { data }
    }
}
impl<T> Mul<&Self> for &Mat3<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Mat3<T>;
    fn mul(self, other: &Self) -> Mat3<T> {
        let mut data = [[T::default(); 3]; 3];
        for row in 0..3 {
            for col in 0..3 {
                data[col][row] = self.row(row).dot(&other.col(col));
            }
        }
        Mat3::<T> { data }
    }
}
impl<T> Mul<Self> for &Mat3<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Mat3<T>;
    fn mul(self, other: Self) -> Mat3<T> {
        let mut data = [[T::default(); 3]; 3];
        for row in 0..3 {
            for col in 0..3 {
                data[col][row] = self.row(row).dot(&other.col(col));
            }
        }
        Mat3::<T> { data }
    }
}
impl<T> Mul<T> for Mat3<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Self;
    fn mul(self, other: T) -> Self {
        let mut data = [[T::default(); 3]; 3];
        for row in 0..3 {
            for col in 0..3 {
                data[col][row] = self[col][row] * other;
            }
        }
        Self { data }
    }
}
impl<T> Mul<T> for &Mat3<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Mat3<T>;
    fn mul(self, other: T) -> Mat3<T> {
        let mut data = [[T::default(); 3]; 3];
        for row in 0..3 {
            for col in 0..3 {
                data[col][row] = self[col][row] * other;
            }
        }
        Mat3::<T> { data }
    }
}
impl<T> Mul<Vector3<T>> for Mat3<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector3<T>;
    fn mul(self, other: Vector3<T>) -> Vector3<T> {
        Vector3::<T>::new(
            other.dot(&self.col(0)),
            other.dot(&self.col(1)),
            other.dot(&self.col(2)),
        )
    }
}
impl<T> Mul<Vector3<T>> for &Mat3<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector3<T>;
    fn mul(self, other: Vector3<T>) -> Vector3<T> {
        Vector3::<T>::new(
            other.dot(&self.col(0)),
            other.dot(&self.col(1)),
            other.dot(&self.col(2)),
        )
    }
}
impl<T> Mul<&Vector3<T>> for &Mat3<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector3<T>;
    fn mul(self, other: &Vector3<T>) -> Vector3<T> {
        Vector3::<T>::new(
            other.dot(&self.col(0)),
            other.dot(&self.col(1)),
            other.dot(&self.col(2)),
        )
    }
}
impl<T> Mul<&Vector3<T>> for Mat3<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector3<T>;
    fn mul(self, other: &Vector3<T>) -> Vector3<T> {
        Vector3::<T>::new(
            other.dot(&self.col(0)),
            other.dot(&self.col(1)),
            other.dot(&self.col(2)),
        )
    }
}
impl<T> Mul<Mat3<T>> for Vector3<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector3<T>;
    fn mul(self, other: Mat3<T>) -> Vector3<T> {
        Vector3::<T>::new(
            self.dot(&other.row(0)),
            self.dot(&other.row(1)),
            self.dot(&other.row(2)),
        )
    }
}
impl<T> Mul<Mat3<T>> for &Vector3<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector3<T>;
    fn mul(self, other: Mat3<T>) -> Vector3<T> {
        Vector3::<T>::new(
            self.dot(&other.row(0)),
            self.dot(&other.row(1)),
            self.dot(&other.row(2)),
        )
    }
}
impl<T> Mul<&Mat3<T>> for Vector3<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector3<T>;
    fn mul(self, other: &Mat3<T>) -> Vector3<T> {
        Vector3::<T>::new(
            self.dot(&other.row(0)),
            self.dot(&other.row(1)),
            self.dot(&other.row(2)),
        )
    }
}
impl<T> Mul<&Mat3<T>> for &Vector3<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector3<T>;
    fn mul(self, other: &Mat3<T>) -> Vector3<T> {
        Vector3::<T>::new(
            self.dot(&other.row(0)),
            self.dot(&other.row(1)),
            self.dot(&other.row(2)),
        )
    }
}
impl<T> MulAssign<Self> for Mat3<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn mul_assign(&mut self, other: Self) {
        let mut data = [[T::default(); 3]; 3];
        for row in 0..3 {
            for col in 0..3 {
                data[col][row] = self.row(row).dot(&other.col(col));
            }
        }
        self.data = data;
    }
}
impl<T> MulAssign<&Self> for Mat3<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn mul_assign(&mut self, other: &Self) {
        let mut data = [[T::default(); 3]; 3];
        for row in 0..3 {
            for col in 0..3 {
                data[col][row] = self.row(row).dot(&other.col(col));
            }
        }
        self.data = data;
    }
}
impl<T> MulAssign<&Self> for &mut Mat3<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn mul_assign(&mut self, other: &Self) {
        let mut data = [[T::default(); 3]; 3];
        for row in 0..3 {
            for col in 0..3 {
                data[col][row] = self.row(row).dot(&other.col(col));
            }
        }
        self.data = data;
    }
}
impl<T> MulAssign<Self> for &mut Mat3<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn mul_assign(&mut self, other: Self) {
        let mut data = [[T::default(); 3]; 3];
        for row in 0..3 {
            for col in 0..3 {
                data[col][row] = self.row(row).dot(&other.col(col));
            }
        }
        self.data = data;
    }
}
impl<T> MulAssign<T> for Mat3<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn mul_assign(&mut self, other: T) {
        for row in 0..3 {
            for col in 0..3 {
                self[col][row] = self[col][row] * other;
            }
        }
    }
}
impl<T> MulAssign<T> for &mut Mat3<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn mul_assign(&mut self, other: T) {
        for row in 0..3 {
            for col in 0..3 {
                self[col][row] = self[col][row] * other;
            }
        }
    }
}

impl<T> Mul<Self> for Mat2<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let mut data = [[T::default(); 2]; 2];
        for row in 0..2 {
            for col in 0..2 {
                data[col][row] = self.row(row).dot(&other.col(col));
            }
        }
        Self { data }
    }
}
impl<T> Mul<&Self> for Mat2<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Self;
    fn mul(self, other: &Self) -> Self {
        let mut data = [[T::default(); 2]; 2];
        for row in 0..2 {
            for col in 0..2 {
                data[col][row] = self.row(row).dot(&other.col(col));
            }
        }
        Self { data }
    }
}
impl<T> Mul<&Self> for &Mat2<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Mat2<T>;
    fn mul(self, other: &Self) -> Mat2<T> {
        let mut data = [[T::default(); 2]; 2];
        for row in 0..2 {
            for col in 0..2 {
                data[col][row] = self.row(row).dot(&other.col(col));
            }
        }
        Mat2::<T> { data }
    }
}
impl<T> Mul<Self> for &Mat2<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Mat2<T>;
    fn mul(self, other: Self) -> Mat2<T> {
        let mut data = [[T::default(); 2]; 2];
        for row in 0..2 {
            for col in 0..2 {
                data[col][row] = self.row(row).dot(&other.col(col));
            }
        }
        Mat2::<T> { data }
    }
}
impl<T> Mul<T> for Mat2<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Self;
    fn mul(self, other: T) -> Self {
        let mut data = [[T::default(); 2]; 2];
        for row in 0..2 {
            for col in 0..2 {
                data[col][row] = self[col][row] * other;
            }
        }
        Self { data }
    }
}
impl<T> Mul<T> for &Mat2<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Mat2<T>;
    fn mul(self, other: T) -> Mat2<T> {
        let mut data = [[T::default(); 2]; 2];
        for row in 0..2 {
            for col in 0..2 {
                data[col][row] = self[col][row] * other;
            }
        }
        Mat2::<T> { data }
    }
}
impl<T> Mul<Vector2<T>> for Mat2<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector2<T>;
    fn mul(self, other: Vector2<T>) -> Vector2<T> {
        Vector2::<T>::new(
            other.dot(&self.col(0)),
            other.dot(&self.col(1)),
        )
    }
}
impl<T> Mul<Vector2<T>> for &Mat2<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector2<T>;
    fn mul(self, other: Vector2<T>) -> Vector2<T> {
        Vector2::<T>::new(
            other.dot(&self.col(0)),
            other.dot(&self.col(1)),
        )
    }
}
impl<T> Mul<&Vector2<T>> for &Mat2<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector2<T>;
    fn mul(self, other: &Vector2<T>) -> Vector2<T> {
        Vector2::<T>::new(
            other.dot(&self.col(0)),
            other.dot(&self.col(1)),
        )
    }
}
impl<T> Mul<&Vector2<T>> for Mat2<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector2<T>;
    fn mul(self, other: &Vector2<T>) -> Vector2<T> {
        Vector2::<T>::new(
            other.dot(&self.col(0)),
            other.dot(&self.col(1)),
        )
    }
}
impl<T> Mul<Mat2<T>> for Vector2<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector2<T>;
    fn mul(self, other: Mat2<T>) -> Vector2<T> {
        Vector2::<T>::new(
            self.dot(&other.row(0)),
            self.dot(&other.row(1)),
        )
    }
}
impl<T> Mul<Mat2<T>> for &Vector2<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector2<T>;
    fn mul(self, other: Mat2<T>) -> Vector2<T> {
        Vector2::<T>::new(
            self.dot(&other.row(0)),
            self.dot(&other.row(1)),
        )
    }
}
impl<T> Mul<&Mat2<T>> for Vector2<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector2<T>;
    fn mul(self, other: &Mat2<T>) -> Vector2<T> {
        Vector2::<T>::new(
            self.dot(&other.row(0)),
            self.dot(&other.row(1)),
        )
    }
}
impl<T> Mul<&Mat2<T>> for &Vector2<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector2<T>;
    fn mul(self, other: &Mat2<T>) -> Vector2<T> {
        Vector2::<T>::new(
            self.dot(&other.row(0)),
            self.dot(&other.row(1)),
        )
    }
}
impl<T> MulAssign<Self> for Mat2<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn mul_assign(&mut self, other: Self) {
        let mut data = [[T::default(); 2]; 2];
        for row in 0..2 {
            for col in 0..2 {
                data[col][row] = self.row(row).dot(&other.col(col));
            }
        }
        self.data = data;
    }
}
impl<T> MulAssign<&Self> for Mat2<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn mul_assign(&mut self, other: &Self) {
        let mut data = [[T::default(); 2]; 2];
        for row in 0..2 {
            for col in 0..2 {
                data[col][row] = self.row(row).dot(&other.col(col));
            }
        }
        self.data = data;
    }
}
impl<T> MulAssign<&Self> for &mut Mat2<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn mul_assign(&mut self, other: &Self) {
        let mut data = [[T::default(); 2]; 2];
        for row in 0..2 {
            for col in 0..2 {
                data[col][row] = self.row(row).dot(&other.col(col));
            }
        }
        self.data = data;
    }
}
impl<T> MulAssign<Self> for &mut Mat2<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn mul_assign(&mut self, other: Self) {
        let mut data = [[T::default(); 2]; 2];
        for row in 0..2 {
            for col in 0..2 {
                data[col][row] = self.row(row).dot(&other.col(col));
            }
        }
        self.data = data;
    }
}
impl<T> MulAssign<T> for Mat2<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn mul_assign(&mut self, other: T) {
        for row in 0..2 {
            for col in 0..2 {
                self[col][row] = self[col][row] * other;
            }
        }
    }
}
impl<T> MulAssign<T> for &mut Mat2<T>
where T : Default + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn mul_assign(&mut self, other: T) {
        for row in 0..2 {
            for col in 0..2 {
                self[col][row] = self[col][row] * other;
            }
        }
    }
}


