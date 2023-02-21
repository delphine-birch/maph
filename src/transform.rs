use crate::base::*;
use crate::num::Identity;
use crate::rotation::*;
pub fn from_translate(t: Vector<3>) -> Matrix<4, 4> {
    let mut mat = Matrix::<4, 4>::identity();
    mat[0][3] = t[0];
    mat[1][3] = t[1];
    mat[2][3] = t[2];
    mat
}
pub fn from_scale(s: Vector<3>) -> Matrix<4, 4> {
    let mut mat = Matrix::<4, 4>::identity();
    mat[0][0] = s[0];
    mat[1][1] = s[1];
    mat[2][2] = s[2];
    mat
}
pub fn from_rotate(q: Vector<4>) -> Matrix<4, 4> {
    let mut mat = Matrix::<4, 4>::identity();
    let rot = quat_to_matrix(q);
    for i in 0..3 {
        for j in 0..3 {
            mat[i][j] = rot[i][j]
        }
    }
    mat
}
pub fn from_trs(t: Vector<3>, r: Vector<4>, s: Vector<3>) -> Matrix<4, 4> {
    from_translate(t)*from_rotate(r)*from_scale(s)
}