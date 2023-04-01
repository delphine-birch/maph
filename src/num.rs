pub mod rational;
pub mod factors;
pub mod surd;

///Square Root trait for convenience.
pub trait Sqroot {
    type Output;
    fn sqroot(&self) -> Self::Output;
}

impl Sqroot for f64 { type Output = f64; fn sqroot(&self) -> Self { self.sqrt() } }
impl Sqroot for f32 { type Output = f32; fn sqroot(&self) -> Self { self.sqrt() } }
impl Sqroot for u128 { type Output = u128; fn sqroot(&self) -> Self { (*self as f64).sqrt().floor() as u128 }}
impl Sqroot for u64 { type Output = u64; fn sqroot(&self) -> Self { (*self as f64).sqrt().floor() as u64 }}
impl Sqroot for u32 { type Output = u32; fn sqroot(&self) -> Self { (*self as f64).sqrt().floor() as u32 }}
impl Sqroot for u16 { type Output = u16; fn sqroot(&self) -> Self { (*self as f64).sqrt().floor() as u16 }}
impl Sqroot for u8 { type Output = u8; fn sqroot(&self) -> Self { (*self as f64).sqrt().floor() as u8 }}
impl Sqroot for i128 { type Output = i128; fn sqroot(&self) -> Self { (*self as f64).sqrt().floor() as i128 }}
impl Sqroot for i64 { type Output = i64; fn sqroot(&self) -> Self { (*self as f64).sqrt().floor() as i64 }}
impl Sqroot for i32 { type Output = i32; fn sqroot(&self) -> Self { (*self as f64).sqrt().floor() as i32 }}
impl Sqroot for i16 { type Output = i16; fn sqroot(&self) -> Self { (*self as f64).sqrt().floor() as i16 }}
impl Sqroot for i8 { type Output = i8; fn sqroot(&self) -> Self { (*self as f64).sqrt().floor() as i8 }}

///Square Root trait for convenience.
pub trait Magnitude {
    type Output;
    fn mag(&self) -> Self::Output;
}

///Identity trait - should return a valid identity for structs it is implemented for. Implemented to return 1 for most number types.
pub trait Identity {
    fn identity() -> Self;
}

impl Identity for f64 { fn identity() -> Self { 1.0 } }
impl Identity for f32 { fn identity() -> Self { 1.0 } }
impl Identity for u128 { fn identity() -> Self { 1 }}
impl Identity for u64 { fn identity() -> Self { 1 }}
impl Identity for u32 { fn identity() -> Self { 1 }}
impl Identity for u16 { fn identity() -> Self { 1 }}
impl Identity for u8 { fn identity() -> Self { 1 }}
impl Identity for i128 { fn identity() -> Self { 1 }}
impl Identity for i64 { fn identity() -> Self { 1 }}
impl Identity for i32 { fn identity() -> Self { 1 }}
impl Identity for i16 { fn identity() -> Self { 1 }}
impl Identity for i8 { fn identity() -> Self { 1 }}

///Trait for mod_add operation - this adds two items and then modulates them to the range provided by the boundaries argument (inclusive at both ends).
///Implemented by default for anything that implements Copy, PartialOrd and Add and Sub for itself.
pub trait ModuloAdd<T, Output> {
    fn mod_add(&self, other: T, boundaries: (T, T)) -> Output;
    fn mod_add_assign(&mut self, other: T, boundaries: (T, T));
}

impl<T> ModuloAdd<T, T> for T
where T : Copy + std::ops::Add<T, Output=T> + std::ops::Sub<T, Output=T> + PartialOrd {
    fn mod_add(&self, other: T, boundaries: (T, T)) -> T {
        let mut n = *self + other;
        while n > boundaries.1 { n = n - (boundaries.1 - boundaries.0); }
        while n < boundaries.0 { n = n + (boundaries.1 - boundaries.0); }
        n
    }
    fn mod_add_assign(&mut self, other: T, boundaries: (T, T)) {
        *self = self.mod_add(other, boundaries);
    }
}

pub trait Conjugate {
    fn conjugate(&self) -> Self;
}

pub trait Absolute {
    type Output;
    fn absolute(&self) -> Self::Output;
}
impl Absolute for f64 { type Output = f64; fn absolute(&self) -> Self { self.abs() } }
impl Absolute for f32 { type Output = f32; fn absolute(&self) -> Self { self.abs() } }
impl Absolute for u128 { type Output = u128; fn absolute(&self) -> Self { *self }}
impl Absolute for u64 { type Output = u64; fn absolute(&self) -> Self { *self }}
impl Absolute for u32 { type Output = u32; fn absolute(&self) -> Self { *self }}
impl Absolute for u16 { type Output = u16; fn absolute(&self) -> Self { *self }}
impl Absolute for u8 { type Output = u8; fn absolute(&self) -> Self { *self }}
impl Absolute for i128 { type Output = i128; fn absolute(&self) -> Self { self.abs() }}
impl Absolute for i64 { type Output = i64; fn absolute(&self) -> Self { self.abs() }}
impl Absolute for i32 { type Output = i32; fn absolute(&self) -> Self { self.abs() }}
impl Absolute for i16 { type Output = i16; fn absolute(&self) -> Self { self.abs() }}
impl Absolute for i8 { type Output = i8; fn absolute(&self) -> Self { self.abs() }}

impl<T, Q> Magnitude for T
where T: Absolute<Output=Q> {
    type Output = Q;
    fn mag(&self) -> Q { self.absolute() }
}

pub trait Round {
    type Output;
    fn tround(&self) -> Self::Output;
    fn tceil(&self) -> Self::Output;
    fn tfloor(&self) -> Self::Output;
}

impl Round for f64 { type Output = f64;   fn tround(&self) -> Self { self.round() } fn tceil(&self) -> Self { self.ceil() } fn tfloor(&self) -> Self { self.floor() } }
impl Round for f32 { type Output = f32;   fn tround(&self) -> Self { self.round() } fn tceil(&self) -> Self { self.ceil() } fn tfloor(&self) -> Self { self.floor() } }
impl Round for u128 { type Output = u128; fn tround(&self) -> Self { *self        } fn tceil(&self) -> Self { *self       } fn tfloor(&self) -> Self { *self        } }
impl Round for u64 { type Output = u64;   fn tround(&self) -> Self { *self        } fn tceil(&self) -> Self { *self       } fn tfloor(&self) -> Self { *self        } }
impl Round for u32 { type Output = u32;   fn tround(&self) -> Self { *self        } fn tceil(&self) -> Self { *self       } fn tfloor(&self) -> Self { *self        } }
impl Round for u16 { type Output = u16;   fn tround(&self) -> Self { *self        } fn tceil(&self) -> Self { *self       } fn tfloor(&self) -> Self { *self        } }
impl Round for u8 { type Output = u8;     fn tround(&self) -> Self { *self        } fn tceil(&self) -> Self { *self       } fn tfloor(&self) -> Self { *self        } }
impl Round for i128 { type Output = i128; fn tround(&self) -> Self { *self        } fn tceil(&self) -> Self { *self       } fn tfloor(&self) -> Self { *self        } }
impl Round for i64 { type Output = i64;   fn tround(&self) -> Self { *self        } fn tceil(&self) -> Self { *self       } fn tfloor(&self) -> Self { *self        } }
impl Round for i32 { type Output = i32;   fn tround(&self) -> Self { *self        } fn tceil(&self) -> Self { *self       } fn tfloor(&self) -> Self { *self        } }
impl Round for i16 { type Output = i16;   fn tround(&self) -> Self { *self        } fn tceil(&self) -> Self { *self       } fn tfloor(&self) -> Self { *self        } }
impl Round for i8 { type Output = i8;     fn tround(&self) -> Self { *self        } fn tceil(&self) -> Self { *self       } fn tfloor(&self) -> Self { *self        } }