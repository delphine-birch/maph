use crate::base::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Float2 {
    pub x: f32,
    pub y: f32,
}
impl Float2 {
    pub fn new(x: f32, y: f32) -> Self { Self { x, y } }
    pub fn mat2(m: Matrix<2, 2>) -> (Self, Self) {
        (Self::from(m.row(0)), Self::from(m.row(1)))
    }
}
impl Default for Float2 { fn default() -> Self { Self::new(0.0, 0.0) } }
impl From<Vector<2>> for Float2 { fn from(other: Vector<2>) -> Self { Self { x: other[0], y: other[1] } } }
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Float3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Float3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self { Self { x, y, z } }
    pub fn mat3(m: Matrix<3, 3>) -> (Self, Self, Self) {
        (Self::from(m.row(0)), Self::from(m.row(1)), Self::from(m.row(2)))
    }
}
impl Default for Float3 { fn default() -> Self { Self::new(0.0, 0.0, 0.0) } }
impl From<Vector<3>> for Float3 { fn from(other: Vector<3>) -> Self { Self { x: other[0], y: other[1], z: other[2] } } }
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Float4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
impl Float4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self { Self { x, y, z, w } }
    pub fn mat4(m: Matrix<4, 4>) -> (Self, Self, Self, Self) {
        (Self::from(m.row(0)), Self::from(m.row(1)), Self::from(m.row(2)), Self::from(m.row(3)))
    }
}
impl Default for Float4 { fn default() -> Self { Self::new(0.0, 0.0, 0.0, 0.0) } }
impl From<Vector<4>> for Float4 { fn from(other: Vector<4>) -> Self { Self { x: other[0], y: other[1], z: other[2], w: other[3] } } }
