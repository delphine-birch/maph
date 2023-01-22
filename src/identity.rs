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
