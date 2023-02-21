pub trait Sqroot {
    fn sqroot(&self) -> Self;
}

impl Sqroot for f64 { fn sqroot(&self) -> Self { self.sqrt() } }
impl Sqroot for f32 { fn sqroot(&self) -> Self { self.sqrt() } }
impl Sqroot for u128 { fn sqroot(&self) -> Self { (*self as f64).sqrt().floor() as u128 }}
impl Sqroot for u64 { fn sqroot(&self) -> Self { (*self as f64).sqrt().floor() as u64 }}
impl Sqroot for u32 { fn sqroot(&self) -> Self { (*self as f64).sqrt().floor() as u32 }}
impl Sqroot for u16 { fn sqroot(&self) -> Self { (*self as f64).sqrt().floor() as u16 }}
impl Sqroot for u8 { fn sqroot(&self) -> Self { (*self as f64).sqrt().floor() as u8 }}
impl Sqroot for i128 { fn sqroot(&self) -> Self { (*self as f64).sqrt().floor() as i128 }}
impl Sqroot for i64 { fn sqroot(&self) -> Self { (*self as f64).sqrt().floor() as i64 }}
impl Sqroot for i32 { fn sqroot(&self) -> Self { (*self as f64).sqrt().floor() as i32 }}
impl Sqroot for i16 { fn sqroot(&self) -> Self { (*self as f64).sqrt().floor() as i16 }}
impl Sqroot for i8 { fn sqroot(&self) -> Self { (*self as f64).sqrt().floor() as i8 }}

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

pub trait ModuloAdd<T, Output> {
    fn mod_add(&self, other: T, boundaries: (T, T)) -> Output;
    fn mod_add_eq(&mut self, other: T, boundaries: (T, T));
}

impl<T> ModuloAdd<T, T> for T
where T : Copy + std::ops::Add<T, Output=T> + std::ops::Sub<T, Output=T> + PartialOrd {
    fn mod_add(&self, other: T, boundaries: (T, T)) -> T {
        let mut n = *self + other;
        while n > boundaries.1 { n = n - (boundaries.1 - boundaries.0); }
        while n < boundaries.0 { n = n + (boundaries.1 - boundaries.0); }
        n
    }
    fn mod_add_eq(&mut self, other: T, boundaries: (T, T)) {
        *self = self.mod_add(other, boundaries);
    }
}