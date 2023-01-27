/*
NB: We are assuming Z up, right handed.
*/
use crate::base::*;

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
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PerspectiveInfo {
    aspect_ratio: f32,
    fovy: f32,
    near: f32,
    far: f32,
}
impl PerspectiveInfo {
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

pub fn camera_matrix(v: Matrix<4, 4>, p: PerspectiveInfo) -> Option<Matrix<4, 4>> {
    match v.inverse() {
        Some(vi) => Some(to_camera_space() * perspective(p) * vi),
        None => None,
    }
}