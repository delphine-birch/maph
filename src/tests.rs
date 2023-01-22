use std::f32::consts::PI;

use crate::*;
use crate::vector::{Dot, Mag};
//Vector Math Tests

//Vector Add
#[test]
fn vector_add() {
    assert!(vector::Vector2::new(2, 3) + vector::Vector2::new(3, 4) == vector::Vector2::new(5, 7));
    assert!(vector::Vector3::new(2, 3, 4) + vector::Vector3::new(3, 4, 5) == vector::Vector3::new(5, 7, 9));
    assert!(vector::Vector4::new(2, 3, 4, 5) + vector::Vector4::new(3, 4, 5, 6) == vector::Vector4::new(5, 7, 9, 11));
}

//Vector Sub
#[test]
fn vector_sub() {
    assert!(vector::Vector2::new(2, 3) - vector::Vector2::new(3, 4) == vector::Vector2::new(-1, -1));
    assert!(vector::Vector3::new(2, 3, 4) - vector::Vector3::new(3, 4, 5) == vector::Vector3::new(-1, -1, -1));
    assert!(vector::Vector4::new(2, 3, 4, 5) - vector::Vector4::new(3, 4, 5, 6) == vector::Vector4::new(-1, -1, -1, -1));
}
//Vector-Vector Mul
#[test]
fn vector_mul() {
    assert!(vector::Vector2::new(2, 3) * vector::Vector2::new(3, 4) == vector::Vector2::new(6, 12));
    assert!(vector::Vector3::new(2, 3, 4) * vector::Vector3::new(3, 4, 5) == vector::Vector3::new(6, 12, 20));
    assert!(vector::Vector4::new(2, 3, 4, 5) * vector::Vector4::new(3, 4, 5, 6) == vector::Vector4::new(6, 12, 20, 30));
}
//Vector-Vector Div
#[test]
fn vector_div() {
    assert!(vector::Vector2::new(4, 6) / vector::Vector2::new(2, 3) == vector::Vector2::new(2, 2));
    assert!(vector::Vector3::new(4, 6, 8) / vector::Vector3::new(2, 3, 4) == vector::Vector3::new(2, 2, 2));
    assert!(vector::Vector4::new(4, 6, 8, 10) / vector::Vector4::new(2, 3, 4, 5) == vector::Vector4::new(2, 2, 2, 2));
}
//Vector-Float Mul
#[test]
fn vector_float_mul() {
    assert!(vector::Vector2::new(2, 3) * 4 == vector::Vector2::new(8, 12));
    assert!(vector::Vector3::new(2, 3, 4) * 4 == vector::Vector3::new(8, 12, 16));
    assert!(vector::Vector4::new(2, 3, 4, 5) * 4 == vector::Vector4::new(8, 12, 16, 20));
}
//Vector-Float Div
#[test]
fn vector_float_div() {
    assert!(vector::Vector2::new(8, 12) / 4 == vector::Vector2::new(2, 3));
    assert!(vector::Vector3::new(8, 12, 16) / 4 == vector::Vector3::new(2, 3, 4));
    assert!(vector::Vector4::new(8, 12, 16, 20) / 4 == vector::Vector4::new(2, 3, 4, 5));
}
//Vector Dot
#[test]
fn vector_dot() {
    assert!(vector::Vector2::new(2, 3).dot(&vector::Vector2::new(4, 5)) == 23);
    assert!(vector::Vector3::new(2, 3, 4).dot(&vector::Vector3::new(4, 5, 6)) == 47);
    assert!(vector::Vector4::new(2, 3, 4, 5).dot(&vector::Vector4::new(4, 5, 6, 7)) == 82);
}
//Vector Cross
#[test]
fn vector_cross() {
    assert!(vector::Vector3::new(0, 1, 0).cross(&vector::Vector3::new(1, 0, 0)) == vector::Vector3::new(0, 0, -1));
}
//Vector Mag
#[test]
fn vector_mag() {
    assert!(vector::Vector2::new(2.0_f32, 3.0_f32).mag() == (13.0_f32).sqrt());
    assert!(vector::Vector3::new(2.0_f32, 3.0_f32, 4.0_f32).mag() == (29.0_f32).sqrt());
    assert!(vector::Vector4::new(2.0_f32, 3.0_f32, 4.0_f32, 5.0_f32).mag() == (54.0_f32).sqrt());
}
//Matrix Math Tests

//Matrix Mul
#[test]
fn matrix_mul() {
    let mat1 = matrix::Mat3::new([vector::Vector3::new(0, 1, 2), vector::Vector3::new(3, 4, 5), vector::Vector3::new(6, 7, 8)]);
    let mat2 = matrix::Mat3::new([vector::Vector3::new(9, 8, 7), vector::Vector3::new(6, 5, 4), vector::Vector3::new(3, 2, 1)]);
    let mat3 = matrix::Mat3::new([vector::Vector3::new(12, 9, 6), vector::Vector3::new(66, 54, 42), vector::Vector3::new(120, 99, 78)]);
    assert!(mat1 * mat2 == mat3)
}
//Matrix LU
#[test]
fn matrix_lu() {
    {
        let lu = matrix::Mat3::new([vector::Vector3::new(2, -1, -2), vector::Vector3::new(-4, 6, 3), vector::Vector3::new(-4, -2, 8)]).lu();
        let lu0 = matrix::Mat3::new([vector::Vector3::new(1, 0, 0), vector::Vector3::new(-2, 1, 0), vector::Vector3::new(-2, -1, 1)]);
        let lu1 = matrix::Mat3::new([vector::Vector3::new(2, -1, -2), vector::Vector3::new(0, 4, -1), vector::Vector3::new(0, 0, 3)]);
        assert!(lu.0 == lu0);
        assert!(lu.1 == lu1);
    }
    {
        let lu = matrix::Mat4::new([
            vector::Vector4::new(1, 1, 1, -5), 
            vector::Vector4::new(-2, -6, 1, 13), 
            vector::Vector4::new(3, -17, 16, -2), 
            vector::Vector4::new(5, -3, 9, -25)
        ]).lu();
        let lu0 = matrix::Mat4::new([
            vector::Vector4::new(1, 0, 0, 0), 
            vector::Vector4::new(-2, 1, 0, 0), 
            vector::Vector4::new(3, 5, 1, 0),
            vector::Vector4::new(5, 2, 1, 1)]);
        let lu1 = matrix::Mat4::new([
            vector::Vector4::new(1, 1, 1, -5), 
            vector::Vector4::new(0, -4, 3, 3), 
            vector::Vector4::new(0, 0, -2, -2),
            vector::Vector4::new(0, 0, 0, -4)]);
        assert!(lu.0 == lu0);
        assert!(lu.1 == lu1);
    }
}
//Matrix Det
#[test]
fn matrix_det() {
    let mat = matrix::Mat3::new([
        vector::Vector3::new(22.0, 15.0, 5.0),
        vector::Vector3::new(2.0, 45.0, 3.0),
        vector::Vector3::new(42.0, 0.0, 10.0)
    ]);
    assert!((mat.det() - 2040.0_f32).abs() < 0.001);
    let mat2 = matrix::Mat4::new([
        vector::Vector4::new(1.0, 1.0, 1.0, -5.0), 
        vector::Vector4::new(-2.0, -6.0, 1.0, 13.0), 
        vector::Vector4::new(3.0, -17.0, 16.0, -2.0), 
        vector::Vector4::new(5.0, -3.0, 9.0, -25.0)
    ]);
    assert!((mat2.det() + 32.0_f32).abs() < 0.001);
}

//Matrix Vector Tests

//Matrix Vector Mul
#[test]
fn matrix_vec_mul() {
    let vec = vector::Vector4::new(1, 2, 3, 4);
    let mat = matrix::Mat4::new([
        vector::Vector4::new(1, 2, 3, 4),
        vector::Vector4::new(5, 6, 7, 8),
        vector::Vector4::new(9, 8, 7, 6),
        vector::Vector4::new(5, 4, 3, 2)
    ]);
    assert!(mat*vec == vector::Vector4::new(30, 70, 70, 30)); 
}

//Vector Matrix Mul
#[test]
fn vec_matrix_mul() {
    let vec = vector::Vector4::new(1, 2, 3, 4);
    let mat = matrix::Mat4::new([
        vector::Vector4::new(1, 2, 3, 4),
        vector::Vector4::new(5, 6, 7, 8),
        vector::Vector4::new(9, 8, 7, 6),
        vector::Vector4::new(5, 4, 3, 2)
    ]);
    assert!(vec*mat == vector::Vector4::new(58, 54, 50, 46)); 
}
//Quaternion Tests

//Quaternion To Euler
#[test]
fn quaternion_to_euler() {
    let euler = quaternion::quaternion_to_euler(vector::Vector4::new(0.7071068, 0.0, 0.7071068, 0.0));
    eprintln!("{}", euler);
    assert!((euler - vector::Vector3::new(3.1415927, -1.5707963, 0.0)).mag() < 0.01);
}
//Euler To Quaternion
#[test]
fn euler_to_quaternion() {
    let quat = quaternion::euler_to_quaternion(
        vector::Vector3::new(1.5707963, 1.5707963, 1.5707963)
    );
    assert!((quat - vector::Vector4::new(0.7071068, 0.0, 0.7071068, 0.0)).mag() < 0.01);
}
//Axis Angle

//Transform Tests
//TRS
#[test]
fn transformation_test() {
    let point = vector::Vector4::new(1.0, 0.0, 0.0, 1.0);
    let scale = vector::Vector3::new(2.0, 2.0, 2.0);
    let rotation = vector::Vector3::new(0.0, 0.0, PI/2.0);
    let translation = vector::Vector3::new(0.0, 2.0, 0.0);
    let trs = transform::euler_rotation_matrix(rotation)*transform::scale_matrix(scale);
    let new_point = trs * point;
    eprintln!("{}, {}", new_point, trs);
    assert!(vector::Vector3::new(new_point.x, new_point.y, new_point.z).mag() < 0.0001);
}

//Projection Tests

//Projection
//Orthographic

//CG Tests

//Create Vectors
//Create Matrices