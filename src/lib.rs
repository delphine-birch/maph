pub mod geom;
pub mod num;
pub mod cg;
#[cfg(test)]
pub mod tests;

pub type Matrix2 = geom::matrix::Matrix<2, 2>;
pub type Matrix3 = geom::matrix::Matrix<3, 3>;
pub type Matrix4 = geom::matrix::Matrix<4, 4>;
pub type Vector2 = geom::vector::Vector<2>;
pub type Vector3 = geom::vector::Vector<3>;
pub type Vector4 = geom::vector::Vector<4>;
pub type VectorGraph<const T: usize> = geom::vector::VectorGraph<T>;
pub type Matrix<const T: usize, const Q: usize> = geom::matrix::Matrix<T, Q>;
pub type Vector<const T: usize> = geom::vector::Vector<T>;
pub type MatrixPrecise2 = geom::matrix::MatrixPrecise<2, 2>;
pub type MatrixPrecise3 = geom::matrix::MatrixPrecise<3, 3>;
pub type MatrixPrecise4 = geom::matrix::MatrixPrecise<4, 4>;
pub type VectorPrecise2 = geom::vector::VectorPrecise<2>;
pub type VectorPrecise3 = geom::vector::VectorPrecise<3>;
pub type VectorPrecise4 = geom::vector::VectorPrecise<4>;
pub type MatrixPrecise<const T: usize, const Q: usize> = geom::matrix::MatrixPrecise<T, Q>;
pub type VectorPrecise<const T: usize> = geom::vector::VectorPrecise<T>;
pub type PerspectiveInfo = geom::transforms::PerspectiveInfo;
pub type RotationOrder = geom::transforms::RotationOrder;
pub type EulerAxis = geom::transforms::EulerAxis;
pub type Float2 = cg::Float2;
pub type Float3 = cg::Float3;
pub type Float4 = cg::Float4;
#[allow(non_camel_case_types)]
pub type r32 = num::rational::r32;
#[allow(non_camel_case_types)]
pub type r64 = num::rational::r64;