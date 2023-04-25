use crate::geom::{vector::*, matrix::*};
use std::f32::consts::PI;

///Utility functions for generating 4x4 Transformation Matrices.
pub mod transform_mat {
    use crate::geom::quaternion::Quaternion;
    use crate::geom::{vector::*, matrix::*};
    use crate::num::Identity;
    ///Returns a transformation matrix based on a 3D vector translation.
    pub fn from_translate(t: Vector<3>) -> Matrix<4, 4> {
        let mut mat = Matrix::<4, 4>::identity();
        mat[0][3] = t[0];
        mat[1][3] = t[1];
        mat[2][3] = t[2];
        mat
    }
    ///Returns a transformation matrix based on a 3D vector scaling.
    pub fn from_scale(s: Vector<3>) -> Matrix<4, 4> {
        let mut mat = Matrix::<4, 4>::identity();
        mat[0][0] = s[0];
        mat[1][1] = s[1];
        mat[2][2] = s[2];
        mat
    }
    ///Returns a transformation matrix based on a Quaternion Rotation.
    pub fn from_rotate(q: Quaternion) -> Matrix<4, 4> {
        let mut mat = Matrix::<4, 4>::identity();
        let rot = q.quat_to_matrix();
        for i in 0..3 {
            for j in 0..3 {
                mat[i][j] = rot[i][j]
            }
        }
        mat
    }
    ///Returns a transformation matrix based on a 3D Translation Vector, a Quaternion Rotation and a
    ///3D Scaling Vector.
    pub fn from_trs(t: Vector<3>, r: Quaternion, s: Vector<3>) -> Matrix<4, 4> {
        from_translate(t)*from_rotate(r)*from_scale(s)
    }
}

///Utility enum for the 3 dimensional axes.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EulerAxis {
    X,
    Y,
    Z
}

///Defines a rotation order.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RotationOrder(pub [EulerAxis; 3]);
impl RotationOrder {
    ///Returns the rotation order XYZ.
    pub fn xyz() -> Self { Self([EulerAxis::X, EulerAxis::Y, EulerAxis::Z]) }
    ///Returns the rotation order ZYX.
    pub fn zyx() -> Self { Self([EulerAxis::Z, EulerAxis::Y, EulerAxis::X]) }
}

///Cross product of two 3D Vectors.
pub fn cross(a: Vector<3>, b: Vector<3>) -> Vector<3> {
    Vector::<3>::new([
        a[1]*b[2] - a[2]*b[1],
        a[2]*b[0] - a[0]*b[2],
        a[0]*b[1] - a[1]*b[0]
    ])
}

///Converts a right-handed Z up coordinate system to a right-handed Y down coordinate system aka Camera Space.
pub fn to_camera_space() -> Matrix<4, 4> {
    Matrix::<4, 4>::new(
        [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, -1.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ]
    )
}

///Defines a perspective projection.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PerspectiveInfo {
    aspect_ratio: f32,
    fovy: f32,
    near: f32,
    far: f32,
}
impl PerspectiveInfo {
    ///Returns perspective info based on aspect_ration, field of view angle and near and far planes.
    pub fn new(
        aspect_ratio: f32,
        fovy: f32,
        near: f32,
        far: f32,
    ) -> Self
    {
        Self { aspect_ratio, fovy, near, far }
    }
}
impl Default for PerspectiveInfo {
    fn default() -> Self { Self::new(1920.0/1080.0, PI/2.0, 0.001, 100.0) }
}

///Returns a 4 by 4 perspective matrix based on perspective info.
pub fn perspective(p: PerspectiveInfo) -> Matrix<4, 4> {
    let iar = 1.0/p.aspect_ratio;
    let t = (p.fovy/2.0).tan();
    let d = p.far - p.near;
    Matrix::<4, 4>::new(
        [
            [iar/t, 0.0, 0.0, 0.0],
            [0.0, 1.0/t, 0.0, 0.0],
            [0.0, 0.0, p.far/d, -p.near*d],
            [0.0, 0.0, 1.0, 0.0]
        ]
    )
}

///Tries to return a matrix for a camera with a transformation matrix v and perspective info p - will
///fail if the transformation matrix cannot be succesfully inverted.
pub fn camera_matrix(v: Matrix<4, 4>, p: PerspectiveInfo) -> Option<Matrix<4, 4>> {
    match v.inverse() {
        Some(vi) => Some(to_camera_space() * perspective(p) * vi),
        None => None,
    }
}