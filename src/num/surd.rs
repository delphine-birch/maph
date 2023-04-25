use super::{rational::*};
use std::{ops::{Mul, MulAssign, Div, DivAssign, Neg}, fmt::{Display, Formatter}};

///Surd Type using 32 bit components - composed of a rational coefficient and a u32 radicand - i.e.
///for coefficient A and radicand B, this is A * sqrt(B); Cannot be Added or Subtracted, but can be multiplied and divided.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct surd32 {
    ///Rational coefficient.
    pub coef: r32,
    ///Radicand.
    pub radicand: u32,
}
impl surd32 {
    ///Returns a new, simplified surd based on coefficient and radicand.
    pub fn new(coef: r32, radicand: u32) -> Self {
        if radicand == 0 { panic!("Radicand for a Surd should never be 0."); }
        let factors = super::factors::sqfac32(radicand);
        let sq_sum = factors.iter().map(|(s, _sq)| s).product::<u32>();
        Self { coef: coef * sq_sum, radicand: radicand / (sq_sum * sq_sum) }
    }
    ///Returns the coefficient of the surd.
    pub fn coef(&self) -> r32 { self.coef }
    ///Returns the radicand of the surd.
    pub fn radicand(&self) -> u32 { self.radicand }
    ///Tries to convert the surd to a rational number - only possible if the radicand is 1.
    pub fn rational(&self) -> Option<r32> { match self.radicand == 1 {
        true => Some(self.coef), false => None
    }}
    ///Returns the squared surd as a rational number - surds squared are always rational.
    pub fn squared(&self) -> r32 { (self * self).rational().expect("A squared surd should always return a rational number.") }
}
impl From<f32> for surd32 {
    fn from(other: f32) -> Self {
        Self::new(r32::from(other), 1)
    }   
}
impl From<surd32> for f32 {
    fn from(other: surd32) -> f32 {
        f32::from(other.coef) * (other.radicand as f32).sqrt()
    }
}
impl From<r32> for surd32 {
    fn from(other: r32) -> Self {
        Self::new(other, 1)
    }
}
impl From<surd32> for r32 {
    fn from(other: surd32) -> Self {
        other.coef * r32::from((other.radicand as f32).sqrt())
    }
}
impl Display for surd32 {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({})*sqrt({})", self.coef, self.radicand)
    }
}
impl Mul<surd32> for surd32 {
    type Output = surd32;
    fn mul(self, other: surd32) -> surd32 {
        Self::new(self.coef * other.coef, self.radicand * other.radicand)
    }
}
impl Div<surd32> for surd32 {
    type Output = surd32;
    fn div(self, other: surd32) -> surd32 {
        Self::new(self.coef / other.coef, self.radicand / other.radicand)
    }
}
impl Mul<u32> for surd32 {
    type Output = surd32;
    fn mul(self, other: u32) -> surd32 { Self::new(self.coef * other, self.radicand) }
}
impl Mul<i32> for surd32 {
    type Output = surd32;
    fn mul(self, other: i32) -> surd32 { Self::new(self.coef * other, self.radicand) }
}
impl Div<u32> for surd32 {
    type Output = surd32;
    fn div(self, other: u32) -> surd32 { Self::new(self.coef / other, self.radicand) }
}
impl Div<i32> for surd32 {
    type Output = surd32;
    fn div(self, other: i32) -> surd32 { Self::new(self.coef / other, self.radicand) }
}
impl Neg for surd32 {
    type Output = surd32;
    fn neg(self) -> surd32 { Self::new(-self.coef, self.radicand) }
}
impl Mul<&surd32> for surd32 { type Output = surd32; fn mul(self, other: &surd32) -> surd32 { self * *other } }
impl Mul<surd32> for &surd32 { type Output = surd32; fn mul(self, other: surd32) -> surd32 { *self * other } }
impl Mul<&surd32> for &surd32 { type Output = surd32; fn mul(self, other: &surd32) -> surd32 { *self * *other } }
impl MulAssign<surd32> for surd32 { fn mul_assign(&mut self, other: surd32) { *self = *self * other; } }
impl MulAssign<&surd32> for surd32 { fn mul_assign(&mut self, other: &surd32) { *self = *self * other; } }
impl Div<&surd32> for surd32 { type Output = surd32; fn div(self, other: &surd32) -> surd32 { self / *other } }
impl Div<surd32> for &surd32 { type Output = surd32; fn div(self, other: surd32) -> surd32 { *self / other } }
impl Div<&surd32> for &surd32 { type Output = surd32; fn div(self, other: &surd32) -> surd32 { *self / *other } }
impl DivAssign<surd32> for surd32 { fn div_assign(&mut self, other: surd32) { *self = *self / other; } }
impl DivAssign<&surd32> for surd32 { fn div_assign(&mut self, other: &surd32) { *self = *self / other; } }
impl Mul<&i32> for surd32 { type Output = surd32; fn mul(self, other: &i32) -> surd32 { self * *other } }
impl Mul<i32> for &surd32 { type Output = surd32; fn mul(self, other: i32) -> surd32 { *self * other } }
impl Mul<&i32> for &surd32 { type Output = surd32; fn mul(self, other: &i32) -> surd32 { *self * *other } }
impl MulAssign<i32> for surd32 { fn mul_assign(&mut self, other: i32) { *self = *self * other; } }
impl MulAssign<&i32> for surd32 { fn mul_assign(&mut self, other: &i32) { *self = *self * other; } }
impl Div<&i32> for surd32 { type Output = surd32; fn div(self, other: &i32) -> surd32 { self / *other } }
impl Div<i32> for &surd32 { type Output = surd32; fn div(self, other: i32) -> surd32 { *self / other } }
impl Div<&i32> for &surd32 { type Output = surd32; fn div(self, other: &i32) -> surd32 { *self / *other } }
impl DivAssign<i32> for surd32 { fn div_assign(&mut self, other: i32) { *self = *self / other; } }
impl DivAssign<&i32> for surd32 { fn div_assign(&mut self, other: &i32) { *self = *self / other; } }
impl Mul<&u32> for surd32 { type Output = surd32; fn mul(self, other: &u32) -> surd32 { self * *other } }
impl Mul<u32> for &surd32 { type Output = surd32; fn mul(self, other: u32) -> surd32 { *self * other } }
impl Mul<&u32> for &surd32 { type Output = surd32; fn mul(self, other: &u32) -> surd32 { *self * *other } }
impl MulAssign<u32> for surd32 { fn mul_assign(&mut self, other: u32) { *self = *self * other; } }
impl MulAssign<&u32> for surd32 { fn mul_assign(&mut self, other: &u32) { *self = *self * other; } }
impl Div<&u32> for surd32 { type Output = surd32; fn div(self, other: &u32) -> surd32 { self / *other } }
impl Div<u32> for &surd32 { type Output = surd32; fn div(self, other: u32) -> surd32 { *self / other } }
impl Div<&u32> for &surd32 { type Output = surd32; fn div(self, other: &u32) -> surd32 { *self / *other } }
impl DivAssign<u32> for surd32 { fn div_assign(&mut self, other: u32) { *self = *self / other; } }
impl DivAssign<&u32> for surd32 { fn div_assign(&mut self, other: &u32) { *self = *self / other; } }


///Surd Type using 64 bit components - composed of a rational coefficient and a u32 radicand - i.e.
///for coefficient A and radicand B, this is A * sqrt(B); Cannot be Added or Subtracted, but can be multiplied and divided.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct surd64 {
    ///Rational coefficient.
    pub coef: r64,
    ///Radicand.
    pub radicand: u64,
}
impl surd64 {
    ///Returns a new, simplified surd based on coefficient and radicand.
    pub fn new(coef: r64, radicand: u64) -> Self {
        if radicand == 0 { panic!("Radicand for a Surd should never be 0."); }
        let factors = super::factors::sqfac64(radicand);
        let sq_sum = factors.iter().map(|(s, _sq)| s).product::<u64>();
        Self { coef: coef * sq_sum, radicand: radicand / (sq_sum * sq_sum) }
    }
    ///Returns the coefficient of the surd.
    pub fn coef(&self) -> r64 { self.coef }
    ///Returns the radicand of the surd.
    pub fn radicand(&self) -> u64 { self.radicand }
    ///Tries to convert the surd to a rational number - only possible if the radicand is 1.
    pub fn rational(&self) -> Option<r64> { match self.radicand == 1 {
        true => Some(self.coef), false => None
    }}
    ///Returns the squared surd as a rational number - surds squared are always rational.
    pub fn squared(&self) -> r64 { (self * self).rational().expect("A squared surd should always return a rational number.") }
}
impl From<f64> for surd64 {
    fn from(other: f64) -> Self {
        Self::new(r64::from(other), 1)
    }   
}
impl From<surd64> for f64 {
    fn from(other: surd64) -> f64 {
        f64::from(other.coef) * (other.radicand as f64).sqrt()
    }
}
impl From<r64> for surd64 {
    fn from(other: r64) -> Self {
        Self::new(other, 1)
    }
}
impl From<surd64> for r64 {
    fn from(other: surd64) -> Self {
        other.coef * r64::from((other.radicand as f64).sqrt())
    }
}
impl Display for surd64 {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({})*sqrt({})", self.coef, self.radicand)
    }
}
impl Mul<surd64> for surd64 {
    type Output = surd64;
    fn mul(self, other: surd64) -> surd64 {
        Self::new(self.coef * other.coef, self.radicand * other.radicand)
    }
}
impl Div<surd64> for surd64 {
    type Output = surd64;
    fn div(self, other: surd64) -> surd64 {
        Self::new(self.coef / other.coef, self.radicand / other.radicand)
    }
}
impl Mul<u64> for surd64 {
    type Output = surd64;
    fn mul(self, other: u64) -> surd64 { Self::new(self.coef * other, self.radicand) }
}
impl Mul<i64> for surd64 {
    type Output = surd64;
    fn mul(self, other: i64) -> surd64 { Self::new(self.coef * other, self.radicand) }
}
impl Div<u64> for surd64 {
    type Output = surd64;
    fn div(self, other: u64) -> surd64 { Self::new(self.coef / other, self.radicand) }
}
impl Div<i64> for surd64 {
    type Output = surd64;
    fn div(self, other: i64) -> surd64 { Self::new(self.coef / other, self.radicand) }
}
impl Neg for surd64 {
    type Output = surd64;
    fn neg(self) -> surd64 { Self::new(-self.coef, self.radicand) }
}
impl Mul<&surd64> for surd64 { type Output = surd64; fn mul(self, other: &surd64) -> surd64 { self * *other } }
impl Mul<surd64> for &surd64 { type Output = surd64; fn mul(self, other: surd64) -> surd64 { *self * other } }
impl Mul<&surd64> for &surd64 { type Output = surd64; fn mul(self, other: &surd64) -> surd64 { *self * *other } }
impl MulAssign<surd64> for surd64 { fn mul_assign(&mut self, other: surd64) { *self = *self * other; } }
impl MulAssign<&surd64> for surd64 { fn mul_assign(&mut self, other: &surd64) { *self = *self * other; } }
impl Div<&surd64> for surd64 { type Output = surd64; fn div(self, other: &surd64) -> surd64 { self / *other } }
impl Div<surd64> for &surd64 { type Output = surd64; fn div(self, other: surd64) -> surd64 { *self / other } }
impl Div<&surd64> for &surd64 { type Output = surd64; fn div(self, other: &surd64) -> surd64 { *self / *other } }
impl DivAssign<surd64> for surd64 { fn div_assign(&mut self, other: surd64) { *self = *self / other; } }
impl DivAssign<&surd64> for surd64 { fn div_assign(&mut self, other: &surd64) { *self = *self / other; } }
impl Mul<&i64> for surd64 { type Output = surd64; fn mul(self, other: &i64) -> surd64 { self * *other } }
impl Mul<i64> for &surd64 { type Output = surd64; fn mul(self, other: i64) -> surd64 { *self * other } }
impl Mul<&i64> for &surd64 { type Output = surd64; fn mul(self, other: &i64) -> surd64 { *self * *other } }
impl MulAssign<i64> for surd64 { fn mul_assign(&mut self, other: i64) { *self = *self * other; } }
impl MulAssign<&i64> for surd64 { fn mul_assign(&mut self, other: &i64) { *self = *self * other; } }
impl Div<&i64> for surd64 { type Output = surd64; fn div(self, other: &i64) -> surd64 { self / *other } }
impl Div<i64> for &surd64 { type Output = surd64; fn div(self, other: i64) -> surd64 { *self / other } }
impl Div<&i64> for &surd64 { type Output = surd64; fn div(self, other: &i64) -> surd64 { *self / *other } }
impl DivAssign<i64> for surd64 { fn div_assign(&mut self, other: i64) { *self = *self / other; } }
impl DivAssign<&i64> for surd64 { fn div_assign(&mut self, other: &i64) { *self = *self / other; } }
impl Mul<&u64> for surd64 { type Output = surd64; fn mul(self, other: &u64) -> surd64 { self * *other } }
impl Mul<u64> for &surd64 { type Output = surd64; fn mul(self, other: u64) -> surd64 { *self * other } }
impl Mul<&u64> for &surd64 { type Output = surd64; fn mul(self, other: &u64) -> surd64 { *self * *other } }
impl MulAssign<u64> for surd64 { fn mul_assign(&mut self, other: u64) { *self = *self * other; } }
impl MulAssign<&u64> for surd64 { fn mul_assign(&mut self, other: &u64) { *self = *self * other; } }
impl Div<&u64> for surd64 { type Output = surd64; fn div(self, other: &u64) -> surd64 { self / *other } }
impl Div<u64> for &surd64 { type Output = surd64; fn div(self, other: u64) -> surd64 { *self / other } }
impl Div<&u64> for &surd64 { type Output = surd64; fn div(self, other: &u64) -> surd64 { *self / *other } }
impl DivAssign<u64> for surd64 { fn div_assign(&mut self, other: u64) { *self = *self / other; } }
impl DivAssign<&u64> for surd64 { fn div_assign(&mut self, other: &u64) { *self = *self / other; } }
