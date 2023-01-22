use std::{
    ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign}, 
    fmt, 
    fmt::{Display}, 
    cmp::{Ordering, PartialOrd}};
use crate::sqroot::Sqroot;
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Vector2<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    pub x: T,
    pub y: T
}
impl<T> Vector2<T> 
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
    pub fn per<F>(&mut self, f: F) where F: Fn(&mut T) {
        f(&mut self.x);
        f(&mut self.y);
    }
    pub fn cross(&self, other: Self) -> T {
        self.x*other.y - self.y*other.x
    }
}
impl<T> Display for Vector2<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy + Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}
impl<T> PartialOrd for Vector2<T>
where T : PartialEq + PartialOrd + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.square_sum().partial_cmp(&other.square_sum())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Vector3<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    pub x: T,
    pub y: T,
    pub z: T
}
impl<T> Vector3<T> 
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
    pub fn per<F>(&mut self, f: F) where F: Fn(&mut T) {
        f(&mut self.x);
        f(&mut self.y);
        f(&mut self.z);
    }
    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y*other.z - self.z*other.y,
            y: self.z*other.x - self.x*other.z,
            z: self.x*other.y - self.y*other.x
        }
    }
}
impl<T> Display for Vector3<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy + Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}
impl<T> PartialOrd for Vector3<T>
where T : PartialEq + PartialOrd + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.square_sum().partial_cmp(&other.square_sum())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Vector4<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T
}
impl<T> Vector4<T> 
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    pub fn new(x: T, y: T, z: T, w: T ) -> Self {
        Self { x, y, z, w }
    }
    pub fn per<F>(&mut self, f: F) where F: Fn(&mut T) {
        f(&mut self.x);
        f(&mut self.y);
        f(&mut self.z);
        f(&mut self.w);
    }
}
impl<T> Display for Vector4<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy + Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}, {}]", self.x, self.y, self.z, self.w)
    }
}
impl<T> PartialOrd for Vector4<T>
where T : PartialEq + PartialOrd + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.square_sum().partial_cmp(&other.square_sum())
    }
}
impl<T> Default for Vector4<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy + Default {
    fn default() -> Self { Self::new(T::default(), T::default(), T::default(), T::default()) }
}

pub trait VectorSum {
    type Component;
    fn sum(&self) -> Self::Component;
    fn square_sum(&self) -> Self::Component;
}

impl<T> VectorSum for Vector2<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy 
{
    type Component = T;
    fn sum(&self) -> T { self.x + self.y }
    fn square_sum(&self) -> T { self.x*self.x + self.y*self.y }
}

impl<T> VectorSum for Vector3<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy 
{
    type Component = T;
    fn sum(&self) -> T { self.x + self.y + self.z }
    fn square_sum(&self) -> T { self.x*self.x + self.y*self.y + self.z*self.z }
}

impl<T> VectorSum for Vector4<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy 
{
    type Component = T;
    fn sum(&self) -> T { self.x + self.y + self.z + self.w }
    fn square_sum(&self) -> T { self.x*self.x + self.y*self.y + self.z*self.z + self.w*self.w }
}

pub trait Dot : VectorSum 
{
    fn dot(&self, other: &Self) -> Self::Component;
}

impl<T> Dot for T 
where T : VectorSum + Mul<T, Output=T> + Copy
{
    fn dot(&self, other: &Self) -> Self::Component {
        (*self * *other).sum()
    }
}

pub trait Mag : VectorSum {
    fn mag(&self) -> Self::Component;
}
impl<T> Mag for T
where T : VectorSum, T::Component : Sqroot
{
    fn mag(&self) -> Self::Component { self.square_sum().sqroot() }
}

impl<T> Add for Vector4<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Self;
    fn add(self, other: Self) -> Self { Self::new(self.x + other.x, self.y + other.y, self.z + other.z, self.w + other.w) }
}
impl<T> Add for &Vector4<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector4<T>;
    fn add(self, other: Self) -> Vector4<T> { Vector4::new(self.x + other.x, self.y + other.y, self.z + other.z, self.w + other.w) }
}
impl<T> Add<&Self> for Vector4<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Self;
    fn add(self, other: &Self) -> Self { Self::new(self.x + other.x, self.y + other.y, self.z + other.z, self.w + other.w) }
}
impl<T> Add<&Self> for &Vector4<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector4<T>;
    fn add(self, other: &Self) -> Vector4<T> { Vector4::new(self.x + other.x, self.y + other.y, self.z + other.z, self.w + other.w) }
}
impl<T> Sub for Vector4<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Self;
    fn sub(self, other: Self) -> Self { Self::new(self.x - other.x, self.y - other.y, self.z - other.z, self.w - other.w) }
}
impl<T> Sub for &Vector4<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector4<T>;
    fn sub(self, other: Self) -> Vector4<T> { Vector4::new(self.x - other.x, self.y - other.y, self.z - other.z, self.w - other.w) }
}
impl<T> Sub<&Self> for Vector4<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Self;
    fn sub(self, other: &Self) -> Self { Self::new(self.x - other.x, self.y - other.y, self.z - other.z, self.w - other.w) }
}
impl<T> Sub<&Self> for &Vector4<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector4<T>;
    fn sub(self, other: &Self) -> Vector4<T> { Vector4::new(self.x - other.x, self.y - other.y, self.z - other.z, self.w - other.w) }
}
impl<T> Mul for Vector4<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Self;
    fn mul(self, other: Self) -> Self { Self::new(self.x * other.x, self.y * other.y, self.z * other.z, self.w * other.w) }
}
impl<T> Mul for &Vector4<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector4<T>;
    fn mul(self, other: Self) -> Vector4<T> { Vector4::new(self.x * other.x, self.y * other.y, self.z * other.z, self.w * other.w) }
}
impl<T> Mul<&Self> for Vector4<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Self;
    fn mul(self, other: &Self) -> Self { Self::new(self.x * other.x, self.y * other.y, self.z * other.z, self.w * other.w) }
}
impl<T> Mul<&Self> for &Vector4<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector4<T>;
    fn mul(self, other: &Self) -> Vector4<T> { Vector4::new(self.x * other.x, self.y * other.y, self.z * other.z, self.w * other.w) }
}
impl<T> Mul<T> for Vector4<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Self;
    fn mul(self, other: T) -> Self { Self::new(self.x * other, self.y * other, self.z * other, self.w * other) }
}
impl<T> Mul<T> for &Vector4<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector4<T>;
    fn mul(self, other: T) -> Vector4<T> { Vector4::new(self.x * other, self.y * other, self.z * other, self.w * other) }
}
impl<T> Div for Vector4<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Self;
    fn div(self, other: Self) -> Self { Self::new(self.x / other.x, self.y / other.y, self.z / other.z, self.w / other.w) }
}
impl<T> Div for &Vector4<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector4<T>;
    fn div(self, other: Self) -> Vector4<T> { Vector4::new(self.x / other.x, self.y / other.y, self.z / other.z, self.w / other.w) }
}
impl<T> Div<&Self> for Vector4<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Self;
    fn div(self, other: &Self) -> Self { Self::new(self.x / other.x, self.y / other.y, self.z / other.z, self.w / other.w) }
}
impl<T> Div<&Self> for &Vector4<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector4<T>;
    fn div(self, other: &Self) -> Vector4<T> { Vector4::new(self.x / other.x, self.y / other.y, self.z / other.z, self.w / other.w) }
}
impl<T> Div<T> for Vector4<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Self;
    fn div(self, other: T) -> Self { Self::new(self.x / other, self.y / other, self.z / other, self.w / other) }
}
impl<T> Div<T> for &Vector4<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector4<T>;
    fn div(self, other: T) -> Vector4<T> { Vector4::new(self.x / other, self.y / other, self.z / other, self.w / other) }
}
impl<T> AddAssign for Vector4<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn add_assign(&mut self, other: Self) { *self = *self + other; }
}
impl<T> AddAssign<&Self> for Vector4<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn add_assign(&mut self, other: &Self) { *self = *self + other; }
}
impl<T> SubAssign for Vector4<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn sub_assign(&mut self, other: Self) { *self = *self - other; }
}
impl<T> SubAssign<&Self> for Vector4<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn sub_assign(&mut self, other: &Self) { *self = *self - other; }
}
impl<T> MulAssign for Vector4<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn mul_assign(&mut self, other: Self) { *self = *self * other; }
}
impl<T> MulAssign<&Self> for Vector4<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn mul_assign(&mut self, other: &Self) { *self = *self * other; }
}
impl<T> MulAssign<T> for Vector4<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn mul_assign(&mut self, other: T) { *self = *self * other; }
}
impl<T> DivAssign for Vector4<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn div_assign(&mut self, other: Self) { *self = *self * other; }
}
impl<T> DivAssign<&Self> for Vector4<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn div_assign(&mut self, other: &Self) { *self = *self * other; }
}
impl<T> DivAssign<T> for Vector4<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn div_assign(&mut self, other: T) { *self = *self * other; }
}

impl<T> Add for Vector3<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Self;
    fn add(self, other: Self) -> Self { Self::new(self.x + other.x, self.y + other.y, self.z + other.z, ) }
}
impl<T> Add for &Vector3<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector3<T>;
    fn add(self, other: Self) -> Vector3<T> { Vector3::new(self.x + other.x, self.y + other.y, self.z + other.z, ) }
}
impl<T> Add<&Self> for Vector3<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Self;
    fn add(self, other: &Self) -> Self { Self::new(self.x + other.x, self.y + other.y, self.z + other.z, ) }
}
impl<T> Add<&Self> for &Vector3<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector3<T>;
    fn add(self, other: &Self) -> Vector3<T> { Vector3::new(self.x + other.x, self.y + other.y, self.z + other.z, ) }
}
impl<T> Sub for Vector3<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Self;
    fn sub(self, other: Self) -> Self { Self::new(self.x - other.x, self.y - other.y, self.z - other.z, ) }
}
impl<T> Sub for &Vector3<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector3<T>;
    fn sub(self, other: Self) -> Vector3<T> { Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z, ) }
}
impl<T> Sub<&Self> for Vector3<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Self;
    fn sub(self, other: &Self) -> Self { Self::new(self.x - other.x, self.y - other.y, self.z - other.z, ) }
}
impl<T> Sub<&Self> for &Vector3<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector3<T>;
    fn sub(self, other: &Self) -> Vector3<T> { Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z, ) }
}
impl<T> Mul for Vector3<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Self;
    fn mul(self, other: Self) -> Self { Self::new(self.x * other.x, self.y * other.y, self.z * other.z, ) }
}
impl<T> Mul for &Vector3<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector3<T>;
    fn mul(self, other: Self) -> Vector3<T> { Vector3::new(self.x * other.x, self.y * other.y, self.z * other.z, ) }
}
impl<T> Mul<&Self> for Vector3<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Self;
    fn mul(self, other: &Self) -> Self { Self::new(self.x * other.x, self.y * other.y, self.z * other.z, ) }
}
impl<T> Mul<&Self> for &Vector3<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector3<T>;
    fn mul(self, other: &Self) -> Vector3<T> { Vector3::new(self.x * other.x, self.y * other.y, self.z * other.z, ) }
}
impl<T> Mul<T> for Vector3<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Self;
    fn mul(self, other: T) -> Self { Self::new(self.x * other, self.y * other, self.z * other) }
}
impl<T> Mul<T> for &Vector3<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector3<T>;
    fn mul(self, other: T) -> Vector3<T> { Vector3::new(self.x * other, self.y * other, self.z * other) }
}
impl<T> Div for Vector3<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Self;
    fn div(self, other: Self) -> Self { Self::new(self.x / other.x, self.y / other.y, self.z / other.z, ) }
}
impl<T> Div for &Vector3<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector3<T>;
    fn div(self, other: Self) -> Vector3<T> { Vector3::new(self.x / other.x, self.y / other.y, self.z / other.z, ) }
}
impl<T> Div<&Self> for Vector3<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Self;
    fn div(self, other: &Self) -> Self { Self::new(self.x / other.x, self.y / other.y, self.z / other.z, ) }
}
impl<T> Div<&Self> for &Vector3<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector3<T>;
    fn div(self, other: &Self) -> Vector3<T> { Vector3::new(self.x / other.x, self.y / other.y, self.z / other.z, ) }
}
impl<T> Div<T> for Vector3<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Self;
    fn div(self, other: T) -> Self { Self::new(self.x / other, self.y / other, self.z / other) }
}
impl<T> Div<T> for &Vector3<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector3<T>;
    fn div(self, other: T) -> Vector3<T> { Vector3::new(self.x / other, self.y / other, self.z / other) }
}
impl<T> AddAssign for Vector3<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn add_assign(&mut self, other: Self) { *self = *self + other; }
}
impl<T> AddAssign<&Self> for Vector3<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn add_assign(&mut self, other: &Self) { *self = *self + other; }
}
impl<T> SubAssign for Vector3<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn sub_assign(&mut self, other: Self) { *self = *self - other; }
}
impl<T> SubAssign<&Self> for Vector3<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn sub_assign(&mut self, other: &Self) { *self = *self - other; }
}
impl<T> MulAssign for Vector3<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn mul_assign(&mut self, other: Self) { *self = *self * other; }
}
impl<T> MulAssign<&Self> for Vector3<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn mul_assign(&mut self, other: &Self) { *self = *self * other; }
}
impl<T> MulAssign<T> for Vector3<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn mul_assign(&mut self, other: T) { *self = *self * other; }
}
impl<T> DivAssign for Vector3<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn div_assign(&mut self, other: Self) { *self = *self * other; }
}
impl<T> DivAssign<&Self> for Vector3<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn div_assign(&mut self, other: &Self) { *self = *self * other; }
}
impl<T> DivAssign<T> for Vector3<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn div_assign(&mut self, other: T) { *self = *self * other; }
}

impl<T> Add for Vector2<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Self;
    fn add(self, other: Self) -> Self { Self::new(self.x + other.x, self.y + other.y) }
}
impl<T> Add for &Vector2<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector2<T>;
    fn add(self, other: Self) -> Vector2<T> { Vector2::new(self.x + other.x, self.y + other.y) }
}
impl<T> Add<&Self> for Vector2<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Self;
    fn add(self, other: &Self) -> Self { Self::new(self.x + other.x, self.y + other.y) }
}
impl<T> Add<&Self> for &Vector2<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector2<T>;
    fn add(self, other: &Self) -> Vector2<T> { Vector2::new(self.x + other.x, self.y + other.y) }
}
impl<T> Sub for Vector2<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Self;
    fn sub(self, other: Self) -> Self { Self::new(self.x - other.x, self.y - other.y) }
}
impl<T> Sub for &Vector2<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector2<T>;
    fn sub(self, other: Self) -> Vector2<T> { Vector2::new(self.x - other.x, self.y - other.y) }
}
impl<T> Sub<&Self> for Vector2<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Self;
    fn sub(self, other: &Self) -> Self { Self::new(self.x - other.x, self.y - other.y) }
}
impl<T> Sub<&Self> for &Vector2<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector2<T>;
    fn sub(self, other: &Self) -> Vector2<T> { Vector2::new(self.x - other.x, self.y - other.y) }
}
impl<T> Mul for Vector2<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Self;
    fn mul(self, other: Self) -> Self { Self::new(self.x * other.x, self.y * other.y) }
}
impl<T> Mul for &Vector2<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector2<T>;
    fn mul(self, other: Self) -> Vector2<T> { Vector2::new(self.x * other.x, self.y * other.y) }
}
impl<T> Mul<&Self> for Vector2<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Self;
    fn mul(self, other: &Self) -> Self { Self::new(self.x * other.x, self.y * other.y) }
}
impl<T> Mul<&Self> for &Vector2<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector2<T>;
    fn mul(self, other: &Self) -> Vector2<T> { Vector2::new(self.x * other.x, self.y * other.y) }
}
impl<T> Mul<T> for Vector2<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Self;
    fn mul(self, other: T) -> Self { Self::new(self.x * other, self.y * other) }
}
impl<T> Mul<T> for &Vector2<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector2<T>;
    fn mul(self, other: T) -> Vector2<T> { Vector2::new(self.x * other, self.y * other) }
}
impl<T> Div for Vector2<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Self;
    fn div(self, other: Self) -> Self { Self::new(self.x / other.x, self.y / other.y) }
}
impl<T> Div for &Vector2<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector2<T>;
    fn div(self, other: Self) -> Vector2<T> { Vector2::new(self.x / other.x, self.y / other.y) }
}
impl<T> Div<&Self> for Vector2<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Self;
    fn div(self, other: &Self) -> Self { Self::new(self.x / other.x, self.y / other.y) }
}
impl<T> Div<&Self> for &Vector2<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector2<T>;
    fn div(self, other: &Self) -> Vector2<T> { Vector2::new(self.x / other.x, self.y / other.y) }
}
impl<T> Div<T> for Vector2<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Self;
    fn div(self, other: T) -> Self { Self::new(self.x / other, self.y / other) }
}
impl<T> Div<T> for &Vector2<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    type Output = Vector2<T>;
    fn div(self, other: T) -> Vector2<T> { Vector2::new(self.x / other, self.y / other) }
}
impl<T> AddAssign for Vector2<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn add_assign(&mut self, other: Self) { *self = *self + other; }
}
impl<T> AddAssign<&Self> for Vector2<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn add_assign(&mut self, other: &Self) { *self = *self + other; }
}
impl<T> SubAssign for Vector2<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn sub_assign(&mut self, other: Self) { *self = *self - other; }
}
impl<T> SubAssign<&Self> for Vector2<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn sub_assign(&mut self, other: &Self) { *self = *self - other; }
}
impl<T> MulAssign for Vector2<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn mul_assign(&mut self, other: Self) { *self = *self * other; }
}
impl<T> MulAssign<&Self> for Vector2<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn mul_assign(&mut self, other: &Self) { *self = *self * other; }
}
impl<T> MulAssign<T> for Vector2<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn mul_assign(&mut self, other: T) { *self = *self * other; }
}
impl<T> DivAssign for Vector2<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn div_assign(&mut self, other: Self) { *self = *self * other; }
}
impl<T> DivAssign<&Self> for Vector2<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn div_assign(&mut self, other: &Self) { *self = *self * other; }
}
impl<T> DivAssign<T> for Vector2<T>
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    fn div_assign(&mut self, other: T) { *self = *self * other; }
}