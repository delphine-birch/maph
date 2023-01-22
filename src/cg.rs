use crate::vector::*;
use crate::matrix::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Float2 {
    x: f32,
    y: f32,
}
impl Float2 {
    pub fn new(x: f32, y: f32) -> Self { Self { x, y } }
    pub fn mat2(m: Mat2<f32>) -> (Self, Self) {
        (Self::from(m.row(0)), Self::from(m.row(1)))
    }
}
impl Default for Float2 { fn default() -> Self { Self::new(0.0, 0.0) } }
impl From<Vector2<f32>> for Float2 { fn from(other: Vector2<f32>) -> Self { Self { x: other.x, y: other.y } } }
impl From<Vector2<f64>> for Float2 { fn from(other: Vector2<f64>) -> Self { Self { x: other.x as f32, y: other.y as f32} } }
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Float3 {
    x: f32,
    y: f32,
    z: f32,
}
impl Float3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self { Self { x, y, z } }
    pub fn mat3(m: Mat3<f32>) -> (Self, Self, Self) {
        (Self::from(m.row(0)), Self::from(m.row(1)), Self::from(m.row(2)))
    }
}
impl Default for Float3 { fn default() -> Self { Self::new(0.0, 0.0, 0.0) } }
impl From<Vector3<f32>> for Float3 { fn from(other: Vector3<f32>) -> Self { Self { x: other.x, y: other.y, z: other.z } } }
impl From<Vector3<f64>> for Float3 { fn from(other: Vector3<f64>) -> Self { Self { x: other.x as f32, y: other.y as f32, z: other.z as f32} } }
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Float4 {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}
impl Float4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self { Self { x, y, z, w } }
    pub fn mat4(m: Mat4<f32>) -> (Self, Self, Self, Self) {
        (Self::from(m.row(0)), Self::from(m.row(1)), Self::from(m.row(2)), Self::from(m.row(3)))
    }
}
impl Default for Float4 { fn default() -> Self { Self::new(0.0, 0.0, 0.0, 0.0) } }
impl From<Vector4<f32>> for Float4 { fn from(other: Vector4<f32>) -> Self { Self { x: other.x, y: other.y, z: other.z, w: other.w } } }
impl From<Vector4<f64>> for Float4 { fn from(other: Vector4<f64>) -> Self { Self { x: other.x as f32, y: other.y as f32, z: other.z as f32, w: other.w as f32 } } }
