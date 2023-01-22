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
