pub mod modulo;
pub mod base;
pub mod spline;
pub mod cg;
pub mod transform;
pub mod rotation;
pub mod projection;
pub mod hashvector;
pub mod vectorgraph;
pub mod num;
pub mod geom;
#[cfg(test)]
pub mod tests;

pub type Matrix2 = base::Matrix<2, 2>;
pub type Matrix3 = base::Matrix<3, 3>;
pub type Matrix4 = base::Matrix<4, 4>;
pub type Vector2 = base::Vector<2>;
pub type Vector3 = base::Vector<3>;
pub type Vector4 = base::Vector<4>;
pub type PerspectiveInfo = projection::PerspectiveInfo;
pub type RotationOrder = rotation::RotationOrder;
pub type EulerAxis = rotation::EulerAxis;
pub type Float2 = cg::Float2;
pub type Float3 = cg::Float3;
pub type Float4 = cg::Float4;