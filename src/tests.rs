use crate::base::*;
fn equal_ish(a: f32, b: f32, d: f32) -> bool {
    (a - b).abs() < d
}
fn vec_equal_ish<const L: usize>(a: Vector<L>, b: Vector<L>, d: f32) -> bool {
    for i in 0..L {
        if !equal_ish(a[i], b[i], d) { return false; }
    }
    true
}
fn mat_equal_ish<const R: usize, const C: usize>(a: Matrix<R, C>, b: Matrix<R, C>, d: f32) -> bool {
    for i in 0..R {
        if !vec_equal_ish(a.row(i), b.row(i), d) { return false; }
    }
    true
}
#[test]
fn quaternion_composition() {
    let a = Vector::<4>::new([0.4, 0.3, 0.2, 0.5]);
    let b = Vector::<4>::new([0.5, 0.4, 0.3, 0.6]);
    let c = Vector::<4>::new([0.5, 0.36, 0.28, -0.08]);
    eprintln!("{}", crate::rotation::compose(a, b));
    assert!(vec_equal_ish(crate::rotation::compose(a, b), c, 0.001));
}

/*//Vector Dot
#[test]
fn matrix_dot() {
    assert!(matrix::Vector::<2>::new(2, 3).dot(&matrix::Vector::<2>::new(4, 5)) == 23);
    assert!(matrix::Vector::<3>::new(2, 3, 4).dot(&matrix::Vector::<3>::new(4, 5, 6)) == 47);
    assert!(matrix::Vector::<4>::new(2, 3, 4, 5).dot(&matrix::Vector::<4>::new(4, 5, 6, 7)) == 82);
}
//Vector Cross
#[test]
fn matrix_cross() {
    assert!(matrix::Vector::<3>::new(0, 1, 0).cross(&matrix::Vector::<3>::new(1, 0, 0)) == matrix::Vector::<3>::new(0, 0, -1));
}
//Vector Mag
#[test]
fn matrix_mag() {
    assert!(matrix::Vector::<2>::new(2.0_f32, 3.0_f32).mag() == (13.0_f32).sqrt());
    assert!(matrix::Vector::<3>::new(2.0_f32, 3.0_f32, 4.0_f32).mag() == (29.0_f32).sqrt());
    assert!(matrix::Vector::<4>::new(2.0_f32, 3.0_f32, 4.0_f32, 5.0_f32).mag() == (54.0_f32).sqrt());
}
//Matrix Math Tests

//Matrix Mul
#[test]
fn matrix_mul() {
    let mat1 = matrix::Mat3::new([matrix::Vector::<3>::new(0, 1, 2), matrix::Vector::<3>::new(3, 4, 5), matrix::Vector::<3>::new(6, 7, 8)]);
    let mat2 = matrix::Mat3::new([matrix::Vector::<3>::new(9, 8, 7), matrix::Vector::<3>::new(6, 5, 4), matrix::Vector::<3>::new(3, 2, 1)]);
    let mat3 = matrix::Mat3::new([matrix::Vector::<3>::new(12, 9, 6), matrix::Vector::<3>::new(66, 54, 42), matrix::Vector::<3>::new(120, 99, 78)]);
    assert!(mat1 * mat2 == mat3)
}
*/
//Matrix LU
#[test]
fn matrix_lu() {
    {
        let lu = Matrix::<4, 4>::new([
            [1.0, 1.0, 1.0, -5.0], 
            [-2.0, -6.0, 1.0, 13.0], 
            [3.0, -17.0, 16.0, -2.0], 
            [5.0, -3.0, 9.0, -25.0]
        ]).lu();
        let lu0 = Matrix::<4, 4>::new([
            [1.0, 0.0, 0.0, 0.0], 
            [-2.0, 1.0, 0.0, 0.0], 
            [3.0, 5.0, 1.0, 0.0],
            [5.0, 2.0, 1.0, 1.0]]);
        let lu1 = Matrix::<4, 4>::new([
            [1.0, 1.0, 1.0, -5.0], 
            [0.0, -4.0, 3.0, 3.0], 
            [0.0, 0.0, -2.0, -2.0],
            [0.0, 0.0, 0.0, -4.0]]);
        assert!(mat_equal_ish(lu0, lu.unwrap().0, 0.001));
        assert!(mat_equal_ish(lu1, lu.unwrap().1, 0.001));
    }
}
#[test]
fn back_sub_test() {
    let u = Matrix::<3, 3>::new([
        [1.0, -2.0, 1.0],
        [0.0, 1.0, 6.0],
        [0.0, 0.0, 1.0],
    ]);
    let d = Vector::<3>::new([4.0, -1.0, 2.0]);
    let x = Vector::<3>::new([-24.0, -13.0, 2.0]);
    eprintln!("{}", u.back_sub(d));
    assert!(vec_equal_ish(u.back_sub(d), x, 0.001));
}
#[test]
fn sub_test() {
    let a = Matrix::<4, 4>::new([
        [1.0, 1.0, 1.0, -5.0], 
        [-2.0, -6.0, 1.0, 13.0], 
        [3.0, -17.0, 16.0, -2.0], 
        [5.0, -3.0, 9.0, -25.0]
    ]);
    let b = Vector::<4>::new([-14.0, 41.0, 9.0, -74.0]);
    let x = Vector::<4>::new([1.0, 2.0, 3.0, 4.0]);
    eprintln!("{}", a.lup_sub(b).unwrap());
    assert!(vec_equal_ish(a.lup_sub(b).unwrap(), x, 0.001));
}
#[test]
fn inverse_test() {
    let m = Matrix::<4, 4>::new([
        [1.0, 4.0, 5.0, -1.0],
        [-2.0, 3.0, -1.0, 0.0],
        [2.0, 1.0, 1.0, 0.0],
        [3.0, -1.0, 2.0, 1.0]
    ]);
    let m2 = Matrix::<4, 4>::new([
        [-0.1, -0.1, 0.6, -0.1],
        [0.0, 0.25, 0.25, 0.0],
        [0.2, -0.05, -0.45, 0.2],
        [-0.1, 0.65, -0.65, 0.9]
    ]);
    //eprintln!("{}", m.lu().unwrap().0);
    //eprintln!("{}", m.inverse().unwrap());
    assert!(mat_equal_ish(m.inverse().unwrap(), m2, 0.001));
}
/*
//Matrix Det
#[test]
fn matrix_det() {
    let mat = matrix::Mat3::new([
        matrix::Vector::<3>::new(22.0, 15.0, 5.0),
        matrix::Vector::<3>::new(2.0, 45.0, 3.0),
        matrix::Vector::<3>::new(42.0, 0.0, 10.0)
    ]);
    assert!((mat.det() - 2040.0_f32).abs() < 0.001);
    let mat2 = matrix::Mat4::new([
        matrix::Vector::<4>::new(1.0, 1.0, 1.0, -5.0), 
        matrix::Vector::<4>::new(-2.0, -6.0, 1.0, 13.0), 
        matrix::Vector::<4>::new(3.0, -17.0, 16.0, -2.0), 
        matrix::Vector::<4>::new(5.0, -3.0, 9.0, -25.0)
    ]);
    assert!((mat2.det() + 32.0_f32).abs() < 0.001);
}

//Matrix Vector Tests

//Matrix Vector Mul
#[test]
fn matrix_vec_mul() {
    let vec = matrix::Vector::<4>::new(1, 2, 3, 4);
    let mat = matrix::Mat4::new([
        matrix::Vector::<4>::new(1, 2, 3, 4),
        matrix::Vector::<4>::new(5, 6, 7, 8),
        matrix::Vector::<4>::new(9, 8, 7, 6),
        matrix::Vector::<4>::new(5, 4, 3, 2)
    ]);
    assert!(mat*vec == matrix::Vector::<4>::new(30, 70, 70, 30)); 
}

//Vector Matrix Mul
#[test]
fn vec_matrix_mul() {
    let vec = matrix::Vector::<4>::new(1, 2, 3, 4);
    let mat = matrix::Mat4::new([
        matrix::Vector::<4>::new(1, 2, 3, 4),
        matrix::Vector::<4>::new(5, 6, 7, 8),
        matrix::Vector::<4>::new(9, 8, 7, 6),
        matrix::Vector::<4>::new(5, 4, 3, 2)
    ]);
    assert!(vec*mat == matrix::Vector::<4>::new(58, 54, 50, 46)); 
}
//Quaternion Tests

//Quaternion To Euler
#[test]
fn quaternion_to_euler() {
    let euler = quaternion::quaternion_to_euler(matrix::Vector::<4>::new(0.7071068, 0.0, 0.7071068, 0.0));
    eprintln!("{}", euler);
    assert!((euler - matrix::Vector::<3>::new(3.1415927, -1.5707963, 0.0)).mag() < 0.01);
}
//Euler To Quaternion
#[test]
fn euler_to_quaternion() {
    let quat = quaternion::euler_to_quaternion(
        matrix::Vector::<3>::new(1.5707963, 1.5707963, 1.5707963)
    );
    assert!((quat - matrix::Vector::<4>::new(0.7071068, 0.0, 0.7071068, 0.0)).mag() < 0.01);
}
//Axis Angle

//Transform Tests
//TRS
#[test]
fn transformation_test() {
    let point = matrix::Vector::<4>::new(1.0, 0.0, 0.0, 1.0);
    let scale = matrix::Vector::<3>::new(2.0, 2.0, 2.0);
    let rotation = matrix::Vector::<3>::new(0.0, 0.0, PI/2.0);
    let translation = matrix::Vector::<3>::new(0.0, 2.0, 0.0);
    let trs = transform::translate_matrix(translation)*transform::euler_rotation_matrix(rotation)*transform::scale_matrix(scale);
    let new_point = trs * point;
    eprintln!("{}, {}", new_point, trs);
    assert!(matrix::Vector::<3>::new(new_point.x, new_point.y, new_point.z).mag() < 0.0001);
}

//Projection Tests

//Projection
//Orthographic

//CG Tests

//Create Vectors
//Create Matrices

#[test]
fn matrix_test() {
    let mat1 = matrix::Matrix::<3, 3>::new([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0 , 0.0, 1.0]]);
    let mat2 = matrix::Matrix::<3, 1>::new([[4.0], [5.0], [6.0]]);
    assert!(mat1.multiply(mat2) == mat2);
}*/