use crate::{vector::*, matrix::*};

pub fn scale_matrix(s: Vector3<f32>) -> Mat4<f32> {
    Mat4::new([
        Vector4::new(s.x, 0.0, 0.0, 0.0),
        Vector4::new(0.0, s.y, 0.0, 0.0),
        Vector4::new(0.0, 0.0, s.z, 0.0),
        Vector4::new(0.0, 0.0, 0.0, 1.0)
    ])
}

pub fn translate_matrix(t: Vector3<f32>) -> Mat4<f32> {
    Mat4::new([
        Vector4::new(0.0, 0.0, 0.0, 0.0),
        Vector4::new(0.0, 0.0, 0.0, 0.0),
        Vector4::new(0.0, 0.0, 0.0, 0.0),
        Vector4::new(t.x, t.y, t.z, 1.0)
    ])
}

pub fn x_rotation_matrix(a: f32) -> Mat4<f32> {
    Mat4::new([
        Vector4::new(1.0, 0.0, 0.0, 0.0),
        Vector4::new(0.0, a.cos(), a.sin(), 0.0),
        Vector4::new(0.0, -a.sin(), a.cos(), 0.0),
        Vector4::new(0.0, 0.0, 0.0, 1.0)
    ])
}

pub fn y_rotation_matrix(a: f32) -> Mat4<f32> {
    Mat4::new([
        Vector4::new(a.cos(), 0.0, -a.sin(), 0.0),
        Vector4::new(0.0, 1.0, 0.0, 0.0),
        Vector4::new(a.sin(), 0.0, a.cos(), 0.0),
        Vector4::new(0.0, 0.0, 0.0, 1.0)
    ])
}

pub fn z_rotation_matrix(a: f32) -> Mat4<f32> {
    Mat4::new([
        Vector4::new(a.cos(), a.sin(), 0.0, 0.0),
        Vector4::new(-a.sin(), a.cos(), 0.0, 0.0),
        Vector4::new(0.0, 0.0, 1.0, 0.0),
        Vector4::new(0.0, 0.0, 0.0, 1.0),
    ])
}

pub fn euler_rotation_matrix(a: Vector3<f32>) -> Mat4<f32> {
    x_rotation_matrix(a.x)*y_rotation_matrix(a.y)*z_rotation_matrix(a.z)
}

pub fn rotation_matrix(q: Vector4<f32>) -> Mat4<f32> {
    Mat4::new([
        Vector4::new(1.0 - 2.0*q.x*q.x - 2.0*q.y*q.y, 2.0*q.y*q.z + 2.0*q.x*q.w, 2.0*q.z*q.z - 2.0*q.y*q.w, 0.0),
        Vector4::new(2.0*q.y*q.z - 2.0*q.x*q.w, 1.0 - 2.0*q.x*q.x - 2.0*q.z*q.z, 2.0*q.x*q.y + 2.0*q.z*q.w, 0.0),
        Vector4::new(2.0*q.x*q.z + 2.0*q.y*q.w, 2.0*q.x*q.y - 2.0*q.z*q.w, 1.0 - 2.0*q.y*q.y - 2.0*q.z*q.z, 0.0),
        Vector4::new(0.0, 0.0, 0.0, 1.0)
    ])
}

pub fn perpective_matrix(fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Mat4<f32> {
    let s = 1.0/(fov/2.0).tan();
    Mat4::new([
        Vector4::new(s/aspect_ratio, 0.0, 0.0, 0.0),
        Vector4::new(0.0, s, 0.0, 0.0),
        Vector4::new(0.0, 0.0, (-near-far)/(near-far), (2.0*near*far)/(near-far)),
        Vector4::new(0.0, 0.0, 1.0, 0.0)
    ])
}

pub fn orthographic_matrix(top: f32, bottom: f32, left: f32, right: f32, near: f32, far: f32) -> Mat4<f32> {
    Mat4::new([
        Vector4::new(2.0/(right-left), 0.0, 0.0, (-right-left)/(right-left)),
        Vector4::new(0.0, 2.0/(top-bottom), 0.0, (-top-bottom)/(top-bottom)),
        Vector4::new(0.0, 0.0, -2.0/(far-near), (-far-near)/(far-near)),
        Vector4::new(0.0, 0.0, 0.0, 1.0)
    ])
}