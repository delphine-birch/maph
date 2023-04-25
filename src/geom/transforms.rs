use crate::geom::{vector::*, matrix::*};
use std::f32::consts::PI;

///Utility functions for generating 4x4 Transformation Matrices.
pub mod transform_mat {
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
    pub fn from_rotate(q: Vector<4>) -> Matrix<4, 4> {
        let mut mat = Matrix::<4, 4>::identity();
        let rot = super::quaternion::quat_to_matrix(q);
        for i in 0..3 {
            for j in 0..3 {
                mat[i][j] = rot[i][j]
            }
        }
        mat
    }
    ///Returns a transformation matrix based on a 3D Translation Vector, a Quaternion Rotation and a
    ///3D Scaling Vector.
    pub fn from_trs(t: Vector<3>, r: Vector<4>, s: Vector<3>) -> Matrix<4, 4> {
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
pub struct RotationOrder([EulerAxis; 3]);
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

pub mod quaternion {
    use crate::geom::{vector::*, matrix::*};
    use crate::num::Identity;

    ///Returns the Identity Quaternion.
    pub fn quaternion_identity() -> Vector<4> { Vector::<4>::new([0.0, 0.0, 0.0, 1.0]) }

    ///Returns the Conjugate of a Quaternion.
    pub fn quaternion_conjugate(q: Vector<4>) -> Vector<4> {
        Vector::<4>::new([-q[0], -q[1], -q[2], q[3]])
    }

    ///Composes two Quaternions.
    pub fn compose(r: Vector<4>, s: Vector<4>) -> Vector<4> {
        Vector::<4>::new([
            r[3]*s[0] - r[2]*s[1] + r[1]*s[2] + r[0]*s[3],
            r[2]*s[0] + r[3]*s[1] - r[0]*s[2] + r[1]*s[3],
            -r[1]*s[0] + r[0]*s[1] + r[3]*s[2] + r[2]*s[3],
            -r[0]*s[0] - r[1]*s[1] - r[2]*s[2] + r[3]*s[3],
        ])
    }

    ///Returns a Quaternion from a 3D vector axis and a float angle.
    pub fn from_axis_angle(axis: Vector<3>, angle: f32) -> Vector<4> {
        let half_angle = angle/2.0;
        Vector::<4>::new([
            axis[0]*half_angle.sin(),
            axis[1]*half_angle.sin(),
            axis[2]*half_angle.sin(),
            half_angle.cos(),
        ])
    }

    ///Returns a 3D Vector axis and a float angle from a Quaternion.
    pub fn to_axis_angle(quat: Vector<4>) -> (Vector<3>, f32) {
        if quat[3] == 1.0 {
            return (Vector::<3>::new([1.0, 0.0, 0.0]), 0.0);
        }
        let angle = quat[3].acos()*2.0;
        let half_angle = angle/2.0;
        (Vector::<3>::new([
            quat[0]/half_angle.sin(), quat[1]/half_angle.sin(), quat[2]/half_angle.sin()
        ]), angle)
    }

    ///Returns a Quaternion from a rotation around the X axis.
    pub fn from_x_angle(angle: f32) -> Vector<4> {
        let half_angle = angle/2.0;
        Vector::<4>::new([
            half_angle.sin(),
            0.0,
            0.0,
            half_angle.cos(),
        ])
    }

    ///Returns a Quaternion from a rotation around the Y axis.
    pub fn from_y_angle(angle: f32) -> Vector<4> {
        let half_angle = angle/2.0;
        Vector::<4>::new([
            0.0,
            half_angle.sin(),
            0.0,
            half_angle.cos(),
        ])
    }

    ///Returns a Quaternion from a rotation around the Z axis.
    pub fn from_z_angle(angle: f32) -> Vector<4> {
        let half_angle = angle/2.0;
        Vector::<4>::new([
            0.0,
            0.0,
            half_angle.sin(),
            half_angle.cos(),
        ])
    }

    ///Returns a Quaternion from an euler rotation (x, y and z angles) and a defined rotation order.
    pub fn from_euler(x: f32, y: f32, z: f32, axis_order: super::RotationOrder) -> Vector<4> {
        let mut quats = [Vector::<4>::identity(); 3];
        for i in 0..3 { 
            quats[i] = match axis_order.0[i] {
                super::EulerAxis::X => from_x_angle(-x),
                super::EulerAxis::Y => from_y_angle(-y),
                super::EulerAxis::Z => from_z_angle(-z)
            }
        }
        compose(quats[2], compose(quats[1], quats[0]))
    }

    ///Returns a quaternion defining a look at rotation - takes the vector we are looking from, the vector we are looking towards,
    ///the direction we are currently facing, an upwards vector and a forwards vector.
    pub fn look_at(from: Vector<3>, to: Vector<3>, facing: Vector<3>, up: Vector<3>, forward: Vector<3>) -> Vector<4> {
        let diff = (to - from).normalised();
        let mut rot_axis = super::cross(facing, diff).normalised();
        if rot_axis.sq_sum() == 0.0 { rot_axis = up }
        let dot = forward.dot(to);
        let ang = dot.acos();
        from_axis_angle(rot_axis, ang)
    }

    ///Returns a 3 by 3 rotation matrix from a Quaternion.
    pub fn quat_to_matrix(q: Vector<4>) -> Matrix<3, 3> {
        let q0 = q[3];
        let q1 = q[0];
        let q2 = q[1];
        let q3 = q[2];
        Matrix::<3, 3>::new([
            [
                2.0*(q0*q0 + q1*q1) - 1.0,
                2.0*(q1*q2 - q0*q3),
                2.0*(q1*q3 + q0*q2)
            ],
            [
                2.0*(q1*q2 + q0*q3),
                2.0*(q0*q0 + q2*q2) - 1.0,
                2.0*(q2*q3 - q0*q1)
            ],
            [
                2.0*(q1*q3 - q0*q2),
                2.0*(q2*q3 + q0*q1),
                2.0*(q0*q0 + q3*q3) - 1.0
            ],
        ])
    }
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