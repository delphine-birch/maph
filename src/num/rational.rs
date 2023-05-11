use std::{cmp::Ordering};
use std::num::ParseIntError;
use std::ops::*;
use super::surd::surd64;
use super::{Identity, Absolute, surd::surd32, Sqroot};

#[derive(Clone, Debug)]
pub enum Error {
    ParseInt(ParseIntError),
    DivideByZero,
}
impl From<ParseIntError> for Error {
    fn from(other: ParseIntError) -> Self { Self::ParseInt(other) }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct r32 {
    n: i16,
    d: u16
}
impl r32 {
    ///Returns the numerator (top half of the fraction) of the r32.
    pub fn numerator(&self) -> i16 { self.n }
    ///Returns the denominator (bottom half of the fraction) of the r32.
    pub fn denominator(&self) -> u16 { self.d }
    ///Returns the signum of the numerator
    pub fn signum(&self) -> i16 { self.n.signum() }
    ///Checks if positive
    pub fn is_positive(&self) -> bool { self.n > 0 } 
    ///Returns a new r32 from the raw i16 and u16
    pub fn new_raw(n: i16, d: u16) -> Self { Self { n, d } }
    ///Returns a new r32 based on numerator and denominator - will simplify down if possible, will panic if dividing by zero.
    pub fn new_unchecked(numerator: i16, denominator: i16) -> Self {
        if denominator == 0 { eprintln!("{}/{}", numerator, denominator); panic!("Denominator of rational type can never be zero!"); }
        let g = super::factors::gcd16(numerator.abs() as u16, denominator.abs() as u16);
        let mut n = numerator as i32 / g as i32;
        let d = denominator as i32 / g as i32;
        if d.is_negative() { n *= -1 }

        Self::new_raw(n as i16, d.abs() as u16)
    }
    ///Returns a new r32 based on numerator and denominator, wrapped in a result in case of division by zero - will simplify down if possible.
    pub fn new(numerator: i16, denominator: i16) -> Result<Self, Error> {
        if denominator == 0 { return Err(Error::DivideByZero); }
        let g = super::factors::gcd16(numerator.abs() as u16, denominator.abs() as u16);
        let mut n = numerator as i32 / g as i32;
        let d = denominator as i32 / g as i32;
        if d.is_negative() { n *= -1 }

        Ok(Self::new_raw(n as i16, d.abs() as u16))
    }
    ///Tries to return a new r32 based on sign and two strings, the integer and fractional parts.
    pub fn int_dec(sign: bool, int: &str, dec: &str) -> Result<Self, Error> {
        let total = int.to_owned() + dec;
        let mut n = total.parse::<i16>()?;
        if !sign { n *= -1; }
        Ok(Self::new(n, 10_i16.pow(dec.len() as u32))?)
    }
    pub fn reciprocal(&self) -> Result<Self, Error> {
        if self.n == 0 { return Err(Error::DivideByZero) }
        else {
            let sign = self.n.signum();
            Ok(Self::new_unchecked(self.d as i16 * sign, self.n.abs()))
        }
    }
    pub fn surd_sqrt(&self) -> surd32 {
        surd32::new(r32::new_unchecked(self.n.signum(), self.d as i16), self.n.abs() as u32 * self.d as u32)
    }
}

impl std::fmt::Display for r32 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} / {}", self.n, self.d)
    }
}

impl From<f32> for r32 {
    fn from(other: f32) -> Self {
        let (mut n, mut d) = super::factors::rat_approx_32(other, i16::MAX as f32);
        while n.abs() > i16::MAX as f32 || d > u16::MAX as f32 {
            n /= 2.0; n = n.round(); d /= 2.0; d = d.round();
        }
        Self::new_raw(n as i16, d as u16)
    }
}

impl From<r32> for f32 {
    fn from(other: r32) -> Self { other.n as f32 / other.d as f32 }
}

impl Identity for r32 {
    fn identity() -> Self { Self::new_raw(1, 1) }
}

impl Default for r32 {
    fn default() -> Self { Self::new_raw(0, 1) }
}

impl Absolute for r32 {
    type Output = r32;
    fn absolute(&self) -> Self { Self::new_raw(self.n.abs(), self.d) }
}

impl Sqroot for r32 {
    type Output = surd32;
    fn sqroot(&self) -> surd32 { self.surd_sqrt() }
}

impl PartialOrd for r32 {
    fn partial_cmp(&self, other: &r32) -> Option<Ordering> {
        match (self - other).signum() {
            1 => Some(Ordering::Greater),
            0 => Some(Ordering::Equal),
            -1 => Some(Ordering::Less),
            _ => None,
        }
    }
}
impl Ord for r32 {
    fn cmp(&self, other: &r32) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Add<r32> for r32 { type Output = r32; fn add(self, other: r32) -> Self {
    let d = super::factors::lcm16(self.d, other.d);
    //eprintln!("denom: {}", d);
    Self::new_unchecked((self.n * (d/self.d) as i16) + (other.n * (d/other.d) as i16), d as i16)    
} }
impl Sub<r32> for r32 { type Output = r32; fn sub(self, other: r32) -> Self { self + -other } }
impl Mul<r32> for r32 { type Output = r32; fn mul(self, other: r32) -> Self { Self::new_unchecked(self.n * other.n, (self.d * other.d) as i16) } }
impl Mul<i32> for r32 { type Output = r32; fn mul(self, other: i32) -> Self { Self::new_unchecked(self.n * other as i16, self.d as i16) } }
impl Mul<r32> for i32 { type Output = r32; fn mul(self, other: r32) -> r32 { r32::new_unchecked(other.n * self as i16, other.d as i16) } }
impl Mul<u32> for r32 { type Output = r32; fn mul(self, other: u32) -> Self { Self::new_unchecked(self.n * other as i16, self.d as i16) } }
impl Mul<r32> for u32 { type Output = r32; fn mul(self, other: r32) -> r32 { r32::new_unchecked(other.n * self as i16, other.d as i16) } }
impl Div<r32> for r32 { type Output = r32; fn div(self, other: r32) -> Self { self * other.reciprocal().unwrap() } }
impl Div<i32> for r32 { type Output = r32; fn div(self, other: i32) -> Self { self * Self::new_unchecked(1, other as i16) } }
impl Div<r32> for i32 { type Output = r32; fn div(self, other: r32) -> r32 { other * r32::new_unchecked(1, self as i16) } }
impl Div<u32> for r32 { type Output = r32; fn div(self, other: u32) -> Self { self * Self::new_unchecked(1, other as i16) } }
impl Div<r32> for u32 { type Output = r32; fn div(self, other: r32) -> r32 { other * r32::new_unchecked(1, self as i16) } }
impl Neg for r32 { type Output = r32; fn neg(self) -> Self { Self::new_raw(-self.n, self.d) } }
impl Add<&r32> for r32 { type Output = r32; fn add(self, other: &r32) -> r32 { self + *other } }
impl Add<r32> for &r32 { type Output = r32; fn add(self, other: r32) -> r32 { *self + other } }
impl Add<&r32> for &r32 { type Output = r32; fn add(self, other: &r32) -> r32 { *self + *other } }
impl AddAssign<r32> for r32 { fn add_assign(&mut self, other: r32) { *self = *self + other; } }
impl AddAssign<&r32> for r32 { fn add_assign(&mut self, other: &r32) { *self = *self + other; } }
impl Sub<&r32> for r32 { type Output = r32; fn sub(self, other: &r32) -> r32 { self - *other } }
impl Sub<r32> for &r32 { type Output = r32; fn sub(self, other: r32) -> r32 { *self - other } }
impl Sub<&r32> for &r32 { type Output = r32; fn sub(self, other: &r32) -> r32 { *self - *other } }
impl SubAssign<r32> for r32 { fn sub_assign(&mut self, other: r32) { *self = *self - other; } }
impl SubAssign<&r32> for r32 { fn sub_assign(&mut self, other: &r32) { *self = *self - other; } }
impl Mul<&r32> for r32 { type Output = r32; fn mul(self, other: &r32) -> r32 { self * *other } }
impl Mul<r32> for &r32 { type Output = r32; fn mul(self, other: r32) -> r32 { *self * other } }
impl Mul<&r32> for &r32 { type Output = r32; fn mul(self, other: &r32) -> r32 { *self * *other } }
impl MulAssign<r32> for r32 { fn mul_assign(&mut self, other: r32) { *self = *self * other; } }
impl MulAssign<&r32> for r32 { fn mul_assign(&mut self, other: &r32) { *self = *self * other; } }
impl Div<&r32> for r32 { type Output = r32; fn div(self, other: &r32) -> r32 { self / *other } }
impl Div<r32> for &r32 { type Output = r32; fn div(self, other: r32) -> r32 { *self / other } }
impl Div<&r32> for &r32 { type Output = r32; fn div(self, other: &r32) -> r32 { *self / *other } }
impl DivAssign<r32> for r32 { fn div_assign(&mut self, other: r32) { *self = *self / other; } }
impl DivAssign<&r32> for r32 { fn div_assign(&mut self, other: &r32) { *self = *self / other; } }
impl Mul<&i32> for r32 { type Output = r32; fn mul(self, other: &i32) -> r32 { self * *other } }
impl Mul<i32> for &r32 { type Output = r32; fn mul(self, other: i32) -> r32 { *self * other } }
impl Mul<&i32> for &r32 { type Output = r32; fn mul(self, other: &i32) -> r32 { *self * *other } }
impl MulAssign<i32> for r32 { fn mul_assign(&mut self, other: i32) { *self = *self * other; } }
impl MulAssign<&i32> for r32 { fn mul_assign(&mut self, other: &i32) { *self = *self * other; } }
impl Div<&i32> for r32 { type Output = r32; fn div(self, other: &i32) -> r32 { self / *other } }
impl Div<i32> for &r32 { type Output = r32; fn div(self, other: i32) -> r32 { *self / other } }
impl Div<&i32> for &r32 { type Output = r32; fn div(self, other: &i32) -> r32 { *self / *other } }
impl DivAssign<i32> for r32 { fn div_assign(&mut self, other: i32) { *self = *self / other; } }
impl DivAssign<&i32> for r32 { fn div_assign(&mut self, other: &i32) { *self = *self / other; } }
impl Mul<&u32> for r32 { type Output = r32; fn mul(self, other: &u32) -> r32 { self * *other } }
impl Mul<u32> for &r32 { type Output = r32; fn mul(self, other: u32) -> r32 { *self * other } }
impl Mul<&u32> for &r32 { type Output = r32; fn mul(self, other: &u32) -> r32 { *self * *other } }
impl MulAssign<u32> for r32 { fn mul_assign(&mut self, other: u32) { *self = *self * other; } }
impl MulAssign<&u32> for r32 { fn mul_assign(&mut self, other: &u32) { *self = *self * other; } }
impl Div<&u32> for r32 { type Output = r32; fn div(self, other: &u32) -> r32 { self / *other } }
impl Div<u32> for &r32 { type Output = r32; fn div(self, other: u32) -> r32 { *self / other } }
impl Div<&u32> for &r32 { type Output = r32; fn div(self, other: &u32) -> r32 { *self / *other } }
impl DivAssign<u32> for r32 { fn div_assign(&mut self, other: u32) { *self = *self / other; } }
impl DivAssign<&u32> for r32 { fn div_assign(&mut self, other: &u32) { *self = *self / other; } }

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct r64 {
    n: i32,
    d: u32
}
impl r64 {
    ///Returns the numerator (top half of the fraction) of the r64.
    pub fn numerator(&self) -> i32 { self.n }
    ///Returns the denominator (bottom half of the fraction) of the r64.
    pub fn denominator(&self) -> u32 { self.d }
    ///Returns the signum of the numerator
    pub fn signum(&self) -> i32 { self.n.signum() }
    ///Checks if positive
    pub fn is_positive(&self) -> bool { self.n > 0 } 
    ///Returns a new r64 from the raw i32 and u32
    pub fn new_raw(n: i32, d: u32) -> Self { Self { n, d } }
    ///Returns a new r64 based on numerator and denominator - will simplify down if possible, will panic if dividing by zero.
    pub fn new_unchecked(numerator: i32, denominator: i32) -> Self {
        if denominator == 0 { eprintln!("{}/{}", numerator, denominator); panic!("Denominator of rational type can never be zero!"); }
        let g = super::factors::gcd32(numerator.abs() as u32, denominator.abs() as u32);
        let mut n = numerator as i64 / g as i64;
        let d = denominator as i64 / g as i64;
        if d.is_negative() { n *= -1; }

        Self::new_raw(n as i32, d.abs() as u32)
    }
    ///Returns a new r64 based on numerator and denominator, wrapped in a result in case of division by zero - will simplify down if possible.
    pub fn new(numerator: i32, denominator: i32) -> Result<Self, Error> {
        if denominator == 0 { return Err(Error::DivideByZero); }
        let g = super::factors::gcd32(numerator.abs() as u32, denominator.abs() as u32);
        let mut n = numerator as i64 / g as i64;
        let d = denominator as i64 / g as i64;
        if d.is_negative() { n *= -1 }

        Ok(Self::new_raw(n as i32, d.abs() as u32))
    }
    ///Tries to return a new r64 based on sign and two strings, the integer and fractional parts.
    pub fn int_dec(sign: bool, int: &str, dec: &str) -> Result<Self, Error> {
        let total = int.to_owned() + dec;
        let mut n = total.parse::<i32>()?;
        if !sign { n *= -1; }
        Ok(Self::new(n, 10_i32.pow(dec.len() as u32))?)
    }
    pub fn reciprocal(&self) -> Result<Self, Error> {
        if self.n == 0 { return Err(Error::DivideByZero) }
        else {
            let sign = self.n.signum();
            Ok(Self::new_unchecked(self.d as i32 * sign, self.n.abs()))
        }
    }
    pub fn surd_sqrt(&self) -> surd64 {
        surd64::new(r64::new_unchecked(self.n.signum(), self.d as i32), self.n.abs() as u64 * self.d as u64 )
    }
}

impl std::fmt::Display for r64 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} / {}", self.n, self.d)
    }
}

impl From<f64> for r64 {
    fn from(other: f64) -> Self {
        let (mut n, mut d) = super::factors::rat_approx_64(other, i32::MAX as f64);
        while n.abs() > i32::MAX as f64 || d > u32::MAX as f64 {
            n /= 2.0; n = n.round(); d /= 2.0; d = d.round();
        }
        Self::new_raw(n as i32, d as u32)
    }
}

impl From<r64> for f64 {
    fn from(other: r64) -> Self { other.n as f64 / other.d as f64 }
}

impl Identity for r64 {
    fn identity() -> Self { Self::new_raw(1, 1) }
}

impl Default for r64 {
    fn default() -> Self { Self::new_raw(0, 1) }
}

impl Absolute for r64 {
    type Output = r64;
    fn absolute(&self) -> Self { Self::new_raw(self.n.abs(), self.d) }
}

impl Sqroot for r64 {
    type Output = surd64;
    fn sqroot(&self) -> surd64 { self.surd_sqrt() }
}

impl PartialOrd for r64 {
    fn partial_cmp(&self, other: &r64) -> Option<Ordering> {
        match (self - other).signum() {
            1 => Some(Ordering::Greater),
            0 => Some(Ordering::Equal),
            -1 => Some(Ordering::Less),
            _ => None,
        }
    }
}
impl Ord for r64 {
    fn cmp(&self, other: &r64) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Add<r64> for r64 { type Output = r64; fn add(self, other: r64) -> Self {
    let d = super::factors::lcm32(self.d, other.d);
    //eprintln!("denom: {}", d);
    Self::new_unchecked((self.n * (d/self.d) as i32) + (other.n * (d/other.d) as i32), d as i32)    
} }
impl Sub<r64> for r64 { type Output = r64; fn sub(self, other: r64) -> Self { self + -other } }
impl Mul<r64> for r64 { type Output = r64; fn mul(self, other: r64) -> Self { Self::new_unchecked(self.n * other.n, (self.d * other.d) as i32) } }
impl Mul<i64> for r64 { type Output = r64; fn mul(self, other: i64) -> Self { Self::new_unchecked(self.n * other as i32, self.d as i32) } }
impl Mul<r64> for i64 { type Output = r64; fn mul(self, other: r64) -> r64 { r64::new_unchecked(other.n * self as i32, other.d as i32) } }
impl Mul<u64> for r64 { type Output = r64; fn mul(self, other: u64) -> Self { Self::new_unchecked(self.n * other as i32, self.d as i32) } }
impl Mul<r64> for u64 { type Output = r64; fn mul(self, other: r64) -> r64 { r64::new_unchecked(other.n * self as i32, other.d as i32) } }
impl Div<r64> for r64 { type Output = r64; fn div(self, other: r64) -> Self { self * other.reciprocal().unwrap() } }
impl Div<i64> for r64 { type Output = r64; fn div(self, other: i64) -> Self { self * Self::new_unchecked(1, other as i32) } }
impl Div<r64> for i64 { type Output = r64; fn div(self, other: r64) -> r64 { other * r64::new_unchecked(1, self as i32) } }
impl Div<u64> for r64 { type Output = r64; fn div(self, other: u64) -> Self { self * Self::new_unchecked(1, other as i32) } }
impl Div<r64> for u64 { type Output = r64; fn div(self, other: r64) -> r64 { other * r64::new_unchecked(1, self as i32) } }
impl Neg for r64 { type Output = r64; fn neg(self) -> Self { Self::new_raw(-self.n, self.d) } }
impl Add<&r64> for r64 { type Output = r64; fn add(self, other: &r64) -> r64 { self + *other } }
impl Add<r64> for &r64 { type Output = r64; fn add(self, other: r64) -> r64 { *self + other } }
impl Add<&r64> for &r64 { type Output = r64; fn add(self, other: &r64) -> r64 { *self + *other } }
impl AddAssign<r64> for r64 { fn add_assign(&mut self, other: r64) { *self = *self + other; } }
impl AddAssign<&r64> for r64 { fn add_assign(&mut self, other: &r64) { *self = *self + other; } }
impl Sub<&r64> for r64 { type Output = r64; fn sub(self, other: &r64) -> r64 { self - *other } }
impl Sub<r64> for &r64 { type Output = r64; fn sub(self, other: r64) -> r64 { *self - other } }
impl Sub<&r64> for &r64 { type Output = r64; fn sub(self, other: &r64) -> r64 { *self - *other } }
impl SubAssign<r64> for r64 { fn sub_assign(&mut self, other: r64) { *self = *self - other; } }
impl SubAssign<&r64> for r64 { fn sub_assign(&mut self, other: &r64) { *self = *self - other; } }
impl Mul<&r64> for r64 { type Output = r64; fn mul(self, other: &r64) -> r64 { self * *other } }
impl Mul<r64> for &r64 { type Output = r64; fn mul(self, other: r64) -> r64 { *self * other } }
impl Mul<&r64> for &r64 { type Output = r64; fn mul(self, other: &r64) -> r64 { *self * *other } }
impl MulAssign<r64> for r64 { fn mul_assign(&mut self, other: r64) { *self = *self * other; } }
impl MulAssign<&r64> for r64 { fn mul_assign(&mut self, other: &r64) { *self = *self * other; } }
impl Div<&r64> for r64 { type Output = r64; fn div(self, other: &r64) -> r64 { self / *other } }
impl Div<r64> for &r64 { type Output = r64; fn div(self, other: r64) -> r64 { *self / other } }
impl Div<&r64> for &r64 { type Output = r64; fn div(self, other: &r64) -> r64 { *self / *other } }
impl DivAssign<r64> for r64 { fn div_assign(&mut self, other: r64) { *self = *self / other; } }
impl DivAssign<&r64> for r64 { fn div_assign(&mut self, other: &r64) { *self = *self / other; } }
impl Mul<&i64> for r64 { type Output = r64; fn mul(self, other: &i64) -> r64 { self * *other } }
impl Mul<i64> for &r64 { type Output = r64; fn mul(self, other: i64) -> r64 { *self * other } }
impl Mul<&i64> for &r64 { type Output = r64; fn mul(self, other: &i64) -> r64 { *self * *other } }
impl MulAssign<i64> for r64 { fn mul_assign(&mut self, other: i64) { *self = *self * other; } }
impl MulAssign<&i64> for r64 { fn mul_assign(&mut self, other: &i64) { *self = *self * other; } }
impl Div<&i64> for r64 { type Output = r64; fn div(self, other: &i64) -> r64 { self / *other } }
impl Div<i64> for &r64 { type Output = r64; fn div(self, other: i64) -> r64 { *self / other } }
impl Div<&i64> for &r64 { type Output = r64; fn div(self, other: &i64) -> r64 { *self / *other } }
impl DivAssign<i64> for r64 { fn div_assign(&mut self, other: i64) { *self = *self / other; } }
impl DivAssign<&i64> for r64 { fn div_assign(&mut self, other: &i64) { *self = *self / other; } }
impl Mul<&u64> for r64 { type Output = r64; fn mul(self, other: &u64) -> r64 { self * *other } }
impl Mul<u64> for &r64 { type Output = r64; fn mul(self, other: u64) -> r64 { *self * other } }
impl Mul<&u64> for &r64 { type Output = r64; fn mul(self, other: &u64) -> r64 { *self * *other } }
impl MulAssign<u64> for r64 { fn mul_assign(&mut self, other: u64) { *self = *self * other; } }
impl MulAssign<&u64> for r64 { fn mul_assign(&mut self, other: &u64) { *self = *self * other; } }
impl Div<&u64> for r64 { type Output = r64; fn div(self, other: &u64) -> r64 { self / *other } }
impl Div<u64> for &r64 { type Output = r64; fn div(self, other: u64) -> r64 { *self / other } }
impl Div<&u64> for &r64 { type Output = r64; fn div(self, other: &u64) -> r64 { *self / *other } }
impl DivAssign<u64> for r64 { fn div_assign(&mut self, other: u64) { *self = *self / other; } }
impl DivAssign<&u64> for r64 { fn div_assign(&mut self, other: &u64) { *self = *self / other; } }