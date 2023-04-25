use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::num::{ParseIntError};
use std::str::FromStr;
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg, BitXor};
use super::surd::{surd32, surd64};
use super::{Identity, Sqroot, Magnitude};

///Rational number type based on two u32s and a bool. Contains sign, numerator and denominator.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct r32 {
    ///Sign of the number.
    pub sign: bool,
    ///Numerator.
    pub n: u32,
    ///Denominator.
    pub d: u32
}
impl r32 {
    ///Returns a new r32 based on sign, numerator and denominator - doesn't try to simplify.
    pub fn new_raw(sign: bool, n: u32, d: u32) -> Self { Self { sign, n, d } }
    ///Returns a new r32 based on sign, numerator and denominator - will simplify down.
    pub fn new(sign: bool, n: u32, d: u32) -> Self {
        if d == 0 { panic!("Denominator can never be zero - tried to create fraction {}/{}", n, d); }
        let g = super::factors::gcd32(n, d);
        Self { sign, n: n/g, d: d/g } 
    }
    ///Tries to return a new r32 based on sign and two strings, the integer and fractional parts.
    pub fn int_dec(sign: bool, int: &str, dec: &str) -> Result<Self, ParseIntError> {
        let total = int.to_owned() + dec;
        Ok(Self::new(sign, total.parse::<u32>()?, 10_u32.pow(dec.len() as u32)))
    }
    ///Absolute value of the r32.
    pub fn abs(&self) -> r32 { r32::new(true, self.n, self.d) }
    ///Returns 1 if positive, -1 if negative.
    pub fn signum(&self) -> i32 { match self.sign { true => 1, false => -1 } }
    ///Returns the numerator as an i32 - will be negative if sign is negative.
    pub fn numerator_i32(&self) -> i32 { match self.sign { true => self.n as i32, false => -1 * self.n as i32 } }
    ///Returns the denominator as an i32 - will always be positive, though.
    pub fn denominator_i32(&self) -> i32 { self.d as i32 }
    ///Returns numerator as a u32.
    pub fn numerator_u32(&self) -> u32 { self.n }
    ///Returns denominator as a u32.
    pub fn denominator_u32(&self) -> u32 { self.d }
    ///Returns true if positive, false if negative.
    pub fn is_positive(&self) -> bool { self.sign }
    ///Returns true if negative, false if positive.
    pub fn is_negative(&self) -> bool { !self.sign }
    ///Returns reciprocal r32.
    pub fn reciprocal(&self) -> r32 { Self::new(self.sign, self.d, self.n) }
    ///Returns square root as a surd.
    pub fn surd_sqrt(&self) -> super::surd::surd32 {
        super::surd::surd32::new(Self::new(true, 1, self.d), self.n*self.d)
    }
}
impl Default for r32 {
    fn default() -> Self { Self::new(true, 0, 1) }
}
impl Identity for r32 {
    fn identity() -> Self { Self::new(true, 1, 1) }
}
impl Sqroot for r32 {
    type Output = surd32;
    fn sqroot(&self) -> surd32 {
        self.surd_sqrt()
    }
}
impl Magnitude for r32 {
    type Output = r32;
    fn mag(&self) -> r32 { self.abs() }
}
impl PartialOrd for r32 {
    fn partial_cmp(&self, other: &r32) -> Option<Ordering> {
        if self == other { return Some(Ordering::Equal); }
        else {
            let d = super::factors::lcm32(self.d, other.d);
            let (n0, n1) = ((self.n * (d/self.d)), (other.n * (d/other.d)));
            match (self.sign, !other.sign) {
                (true, true) => Some(Ordering::Greater),
                (true, false) => match n1 > n0 {
                    true => Some(Ordering::Less),
                    false => Some(Ordering::Greater),
                },
                (false, true) => match n0 > n1 {
                    true => Some(Ordering::Less),
                    false => Some(Ordering::Greater),
                },
                (false, false) => Some(Ordering::Less)
            }
        }
    }
}
impl Ord for r32 {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other { return Ordering::Equal; }
        else {
            match (self.sign, !other.sign) {
                (true, true) => Ordering::Greater,
                (false, false) => Ordering::Less,
                (true, false) => {
                    let d = super::factors::lcm32(self.d, other.d);
                    let (n0, n1) = ((self.n * (d/self.d)), (other.n * (d/other.d)));
                    match n1 > n0 {
                        true => Ordering::Less,
                        false => Ordering::Greater,
                    }
                },
                (false, true) => {
                    let d = super::factors::lcm32(self.d, other.d);
                    let (n0, n1) = ((self.n * (d/self.d)), (other.n * (d/other.d)));
                    match n0 > n1 {
                        true => Ordering::Less,
                        false => Ordering::Greater,
                    }
                },
            }
        }
    }
}
impl From<f32> for r32 {
    fn from(f: f32) -> r32 {
        let frac = f.fract();
        let (sign, dec) = match frac >= 0.0 {
            true => (true, frac),
            false => (false, -frac),
        };
        let n = dec.to_string().len().min(u16::MAX as usize);
        let t = 10_u32.pow(n as u32);
        r32::new(sign, (f*(t as f32)).round() as u32, t)
    }
}
impl From<(i32, i32)> for r32 {
    fn from(nd: (i32, i32)) -> r32 {
        let s = !nd.0.is_positive().bitxor(nd.1.is_positive());
        r32::new(s, nd.0.abs() as u32, nd.1.abs() as u32)
    }
}
impl From<u32> for r32 {
    fn from(u: u32) -> r32 {
        r32::new(true, u, 1)
    }
}
impl From<i32> for r32 {
    fn from(i: i32) -> r32 {
        r32::new(i.is_positive(), i.abs() as u32, 1)
    }
}
impl From<r32> for f32 {
    fn from(r: r32) -> f32 {
        match r.sign { true => r.n as f32 / r.d as f32, false => -(r.n as f32) / r.d as f32 }
    }
}
impl From<r32> for f64 {
    fn from(r: r32) -> f64 {
        match r.sign { true => r.n as f64 / r.d as f64, false => -(r.n as f64) / r.d as f64 }
    }
}
impl FromStr for r32 {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (sign, string) = match s.strip_prefix('-') {
            Some(s0) => (false, s0),
            None => (true, s)
        };
        let parts: Vec<&str> = string.split('.').collect();
        match (parts.get(0), parts.get(1)) {
            (Some(int), Some(dec)) => r32::int_dec(sign, int, dec),
            _ => r32::int_dec(sign, string, "")
        }
    }
}
impl Display for r32 {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let sign = match self.sign { true => "", false => "-" };
        write!(f, "{}{}/{}", sign, self.n, self.d)
    }
}
impl Add<r32> for r32 {
    type Output = r32;
    fn add(self, other: r32) -> r32 {
        let d = super::factors::lcm32(self.d, other.d);
        let (n0, n1) = ((self.n * (d/self.d)), (other.n * (d/other.d)));
        match (self.sign, other.sign) {
            (true, true) => Self::new(true, n0 + n1, d),
            (true, false) => match n1 > n0 {
                true => Self::new(false, n1 - n0, d),
                false => Self::new(true, n0 - n1, d),
            },
            (false, true) => match n0 > n1 {
                true => Self::new(false, n0 - n1, d),
                false => Self::new(true, n1 - n0, d),
            },
            (false, false) => Self::new(false, n0 + n1, d)
        }
    }
}
impl Neg for r32 {
    type Output = r32;
    fn neg(self) -> r32 { r32::new(!self.sign, self.n, self.d) } 
}
impl Sub<r32> for r32 {
    type Output = r32;
    fn sub(self, other: r32) -> r32 {
        let d = super::factors::lcm32(self.d, other.d);
        let (n0, n1) = ((self.n * (d/self.d)), (other.n * (d/other.d)));
        match (self.sign, !other.sign) {
            (true, true) => Self::new(true, n0 + n1, d),
            (true, false) => match n1 > n0 {
                true => Self::new(false, n1 - n0, d),
                false => Self::new(true, n0 - n1, d),
            },
            (false, true) => match n0 > n1 {
                true => Self::new(false, n0 - n1, d),
                false => Self::new(true, n1 - n0, d),
            },
            (false, false) => Self::new(false, n0 + n1, d)
        }
    }
}
impl Mul<r32> for r32 {
    type Output = r32;
    fn mul(self, other: r32) -> r32 { r32::new(!self.sign.bitxor(other.sign), self.n*other.n, self.d*other.d) }
}
impl Div<r32> for r32 {
    type Output = r32;
    fn div(self, other: r32) -> r32 { r32::new(!self.sign.bitxor(other.sign), self.n*other.d, self.d*other.n) }
}
impl Mul<i32> for r32 {
    type Output = r32;
    fn mul(self, other: i32) -> r32 { r32::new(!other.is_positive().bitxor(self.sign), self.n*other.abs() as u32, self.d) }
}
impl Mul<u32> for r32 {
    type Output = r32;
    fn mul(self, other: u32) -> r32 { r32::new(self.sign, self.n*other, self.d) }
}
impl Div<i32> for r32 {
    type Output = r32;
    fn div(self, other: i32) -> r32 { r32::new(!other.is_positive().bitxor(self.sign), self.n, self.d*other.abs() as u32) }
}
impl Div<u32> for r32 {
    type Output = r32;
    fn div(self, other: u32) -> r32 { r32::new(self.sign, self.n, self.d*other) }
}
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

///Rational number type based on two u64s and a bool. Contains sign, numerator and denominator.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct r64 {
    ///Sign of the number.
    pub sign: bool,
    ///Numerator.
    pub n: u64,
    ///Denominator.
    pub d: u64
}
impl r64 {
    ///Returns a new r64 based on sign, numerator and denominator - doesn't try to simplify.
    pub fn new_raw(sign: bool, n: u64, d: u64) -> Self { Self { sign, n, d } }
    ///Returns a new r64 based on sign, numerator and denominator - will simplify down.
    pub fn new(sign: bool, n: u64, d: u64) -> Self { 
        if d == 0 { panic!("Denominator can never be zero."); }
        let g = super::factors::gcd64(n, d);
        Self { sign, n: n/g, d: d/g } 
    }
    ///Tries to return a new r64 based on sign and two strings, the integer and fractional parts.
    pub fn int_dec(sign: bool, int: &str, dec: &str) -> Result<Self, ParseIntError> {
        let total = int.to_owned() + dec;
        Ok(Self::new(sign, total.parse::<u64>()?, 10_u64.pow(dec.len() as u32)))
    }
    ///Absolute value of the r64.
    pub fn abs(&self) -> r64 { r64::new(true, self.n, self.d) }
    ///Returns 1 if positive, -1 if negative.
    pub fn signum(&self) -> i64 { match self.sign { true => 1, false => -1 } }
    ///Returns the numerator as an i64 - will be negative if sign is negative.
    pub fn numerator_i64(&self) -> i64 { match self.sign { true => self.n as i64, false => -1 * self.n as i64 } }
    ///Returns the denominator as an i64 - will always be positive, though.
    pub fn denominator_i64(&self) -> i64 { self.d as i64 }
    ///Returns numerator as a u64.
    pub fn numerator_u64(&self) -> u64 { self.n }
    ///Returns denominator as a u64.
    pub fn denominator_u64(&self) -> u64 { self.d }
    ///Returns true if positive, false if negative.
    pub fn is_positive(&self) -> bool { self.sign }
    ///Returns true if negative, false if positive.
    pub fn is_negative(&self) -> bool { !self.sign }
    ///Returns reciprocal r64.
    pub fn reciprocal(&self) -> r64 { Self::new(self.sign, self.d, self.n) }
    ///Returns square root as a surd.
    pub fn surd_sqrt(&self) -> super::surd::surd64 {
        super::surd::surd64::new(Self::new(true, 1, self.d), self.n*self.d)
    }
}
impl Default for r64 {
    fn default() -> Self { Self::new(true, 0, 1) }
}
impl Identity for r64 {
    fn identity() -> Self { Self::new(true, 1, 1) }
}
impl Sqroot for r64 {
    type Output = surd64;
    fn sqroot(&self) -> surd64 {
        self.surd_sqrt()
    }
}
impl Magnitude for r64 {
    type Output = r64;
    fn mag(&self) -> r64 { self.abs() }
}
impl PartialOrd for r64 {
    fn partial_cmp(&self, other: &r64) -> Option<Ordering> {
        if self == other { return Some(Ordering::Equal); }
        else {
            let d = super::factors::lcm64(self.d, other.d);
            let (n0, n1) = ((self.n * (d/self.d)), (other.n * (d/other.d)));
            match (self.sign, !other.sign) {
                (true, true) => Some(Ordering::Greater),
                (true, false) => match n1 > n0 {
                    true => Some(Ordering::Less),
                    false => Some(Ordering::Greater),
                },
                (false, true) => match n0 > n1 {
                    true => Some(Ordering::Less),
                    false => Some(Ordering::Greater),
                },
                (false, false) => Some(Ordering::Less)
            }
        }
    }
}
impl Ord for r64 {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other { return Ordering::Equal; }
        else {
            match (self.sign, !other.sign) {
                (true, true) => Ordering::Greater,
                (false, false) => Ordering::Less,
                (true, false) => {
                    let d = super::factors::lcm64(self.d, other.d);
                    let (n0, n1) = ((self.n * (d/self.d)), (other.n * (d/other.d)));
                    match n1 > n0 {
                        true => Ordering::Less,
                        false => Ordering::Greater,
                    }
                },
                (false, true) => {
                    let d = super::factors::lcm64(self.d, other.d);
                    let (n0, n1) = ((self.n * (d/self.d)), (other.n * (d/other.d)));
                    match n0 > n1 {
                        true => Ordering::Less,
                        false => Ordering::Greater,
                    }
                },
            }
        }
    }
}
impl From<f64> for r64 {
    fn from(f: f64) -> r64 {
        let frac = f.fract();
        let (sign, dec) = match frac >= 0.0 {
            true => (true, frac),
            false => (false, -frac),
        };
        r64::new(sign, (f.abs() - dec) as u64, dec as u64)
    }
}
impl From<(i64, i64)> for r64 {
    fn from(nd: (i64, i64)) -> r64 {
        let s = !nd.0.is_positive().bitxor(nd.1.is_positive());
        r64::new(s, nd.0.abs() as u64, nd.1.abs() as u64)
    }
}
impl From<u64> for r64 {
    fn from(u: u64) -> r64 {
        r64::new(true, u, 1)
    }
}
impl From<i64> for r64 {
    fn from(i: i64) -> r64 {
        r64::new(i.is_positive(), i.abs() as u64, 1)
    }
}
impl From<r64> for f32 {
    fn from(r: r64) -> f32 {
        match r.sign { true => r.n as f32 / r.d as f32, false => -(r.n as f32) / r.d as f32 }
    }
}
impl From<r64> for f64 {
    fn from(r: r64) -> f64 {
        match r.sign { true => r.n as f64 / r.d as f64, false => -(r.n as f64) / r.d as f64 }
    }
}
impl FromStr for r64 {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (sign, string) = match s.strip_prefix('-') {
            Some(s0) => (false, s0),
            None => (true, s)
        };
        let parts: Vec<&str> = string.split('.').collect();
        match (parts.get(0), parts.get(1)) {
            (Some(int), Some(dec)) => r64::int_dec(sign, int, dec),
            _ => r64::int_dec(sign, string, "")
        }
    }
}
impl Display for r64 {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let sign = match self.sign { true => "", false => "-" };
        write!(f, "{}{}/{}", sign, self.n, self.d)
    }
}
impl Add<r64> for r64 {
    type Output = r64;
    fn add(self, other: r64) -> r64 {
        let d = super::factors::lcm64(self.d, other.d);
        let (n0, n1) = ((self.n * (d/self.d)), (other.n * (d/other.d)));
        match (self.sign, other.sign) {
            (true, true) => Self::new(true, n0 + n1, d),
            (true, false) => match n1 > n0 {
                true => Self::new(false, n1 - n0, d),
                false => Self::new(true, n0 - n1, d),
            },
            (false, true) => match n0 > n1 {
                true => Self::new(false, n0 - n1, d),
                false => Self::new(true, n1 - n0, d),
            },
            (false, false) => Self::new(false, n0 + n1, d)
        }
    }
}
impl Neg for r64 {
    type Output = r64;
    fn neg(self) -> r64 { r64::new(!self.sign, self.n, self.d) } 
}
impl Sub<r64> for r64 {
    type Output = r64;
    fn sub(self, other: r64) -> r64 {
        let d = super::factors::lcm64(self.d, other.d);
        let (n0, n1) = ((self.n * (d/self.d)), (other.n * (d/other.d)));
        match (self.sign, !other.sign) {
            (true, true) => Self::new(true, n0 + n1, d),
            (true, false) => match n1 > n0 {
                true => Self::new(false, n1 - n0, d),
                false => Self::new(true, n0 - n1, d),
            },
            (false, true) => match n0 > n1 {
                true => Self::new(false, n0 - n1, d),
                false => Self::new(true, n1 - n0, d),
            },
            (false, false) => Self::new(false, n0 + n1, d)
        }
    }
}
impl Mul<r64> for r64 {
    type Output = r64;
    fn mul(self, other: r64) -> r64 { r64::new(!self.sign.bitxor(other.sign), self.n*other.n, self.d*other.d) }
}
impl Div<r64> for r64 {
    type Output = r64;
    fn div(self, other: r64) -> r64 { r64::new(!self.sign.bitxor(other.sign), self.n*other.d, self.d*other.n) }
}
impl Mul<i64> for r64 {
    type Output = r64;
    fn mul(self, other: i64) -> r64 { r64::new(!other.is_positive().bitxor(self.sign), self.n*other.abs() as u64, self.d) }
}
impl Mul<u64> for r64 {
    type Output = r64;
    fn mul(self, other: u64) -> r64 { r64::new(self.sign, self.n*other, self.d) }
}
impl Div<i64> for r64 {
    type Output = r64;
    fn div(self, other: i64) -> r64 { r64::new(!other.is_positive().bitxor(self.sign), self.n, self.d*other.abs() as u64) }
}
impl Div<u64> for r64 {
    type Output = r64;
    fn div(self, other: u64) -> r64 { r64::new(self.sign, self.n, self.d*other) }
}
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
