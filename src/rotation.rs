use crate::{base::*, num::Identity};

pub fn quaternion_identity() -> Vector<4> { Vector::<4>::new([0.0, 0.0, 0.0, 1.0]) }

pub fn compose(r: Vector<4>, s: Vector<4>) -> Vector<4> {
    Vector::<4>::new([
        r[3]*s[0] - r[2]*s[1] + r[1]*s[2] + r[0]*s[3],
        r[2]*s[0] + r[3]*s[1] - r[0]*s[2] + r[1]*s[3],
        -r[1]*s[0] + r[0]*s[1] + r[3]*s[2] + r[2]*s[3],
        -r[0]*s[0] - r[1]*s[1] - r[2]*s[2] + r[3]*s[3],
    ])
}

pub fn cross(a: Vector<3>, b: Vector<3>) -> Vector<3> {
    Vector::<3>::new([
        a[1]*b[2] - a[2]*b[1],
        a[2]*b[0] - a[0]*b[2],
        a[0]*b[1] - a[1]*b[0]
    ])
}

pub fn from_axis_angle(axis: Vector<3>, angle: f32) -> Vector<4> {
    let half_angle = angle/2.0;
    Vector::<4>::new([
        axis[0]*half_angle.sin(),
        axis[1]*half_angle.sin(),
        axis[2]*half_angle.sin(),
        half_angle.cos(),
    ])
}

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

pub fn from_x_angle(angle: f32) -> Vector<4> {
    let half_angle = angle/2.0;
    Vector::<4>::new([
        half_angle.sin(),
        0.0,
        0.0,
        half_angle.cos(),
    ])
}
pub fn from_y_angle(angle: f32) -> Vector<4> {
    let half_angle = angle/2.0;
    Vector::<4>::new([
        0.0,
        half_angle.sin(),
        0.0,
        half_angle.cos(),
    ])
}
pub fn from_z_angle(angle: f32) -> Vector<4> {
    let half_angle = angle/2.0;
    Vector::<4>::new([
        0.0,
        0.0,
        half_angle.sin(),
        half_angle.cos(),
    ])
}
pub fn from_euler(x: f32, y: f32, z: f32, axis_order: RotationOrder) -> Vector<4> {
    let mut quats = [Vector::<4>::identity(); 3];
    for i in 0..3 { 
        quats[i] = match axis_order.0[i] {
            EulerAxis::X => from_x_angle(-x),
            EulerAxis::Y => from_y_angle(-y),
            EulerAxis::Z => from_z_angle(-z)
        }
    }
    compose(quats[2], compose(quats[1], quats[0]))
}

pub fn look_at(from: Vector<3>, to: Vector<3>, facing: Vector<3>, up: Vector<3>, forward: Vector<3>) -> Vector<4> {
    let diff = (to - from).normalised();
    let mut rot_axis = cross(facing, diff).normalised();
    if rot_axis.sq_sum() == 0.0 { rot_axis = up }
    let dot = forward.dot(to);
    let ang = dot.acos();
    from_axis_angle(rot_axis, ang)
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EulerAxis {
    X,
    Y,
    Z
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RotationOrder([EulerAxis; 3]);
impl RotationOrder {
    pub fn xyz() -> Self { Self([EulerAxis::X, EulerAxis::Y, EulerAxis::Z]) }
    pub fn zyx() -> Self { Self([EulerAxis::Z, EulerAxis::Y, EulerAxis::X]) }
}

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