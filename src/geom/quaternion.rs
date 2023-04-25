use std::ops::{Mul, Add, MulAssign, AddAssign};

use crate::{Vector, num::{Identity, Magnitude}, Matrix};

use super::transforms::transform_mat::from_trs;

///Quaternion type - analogous to Vector<4>, although keeps its components individually rather than in an array - convert to and from for
///quaternion operations vs. vector operations.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl From<Vector<4>> for Quaternion {
    /// Implements Vectors as [x, y, z, w], not [w, x, y, z].
    fn from(other: Vector<4>) -> Self { Self { x: other[0], y: other[1], z: other[2], w: other[3] } }
}
impl From<Quaternion> for Vector<4> {
    /// Implements Vectors as [x, y, z, w], not [w, x, y, z].
    fn from(other: Quaternion) -> Self { Self::new([other.x, other.y, other.z, other.w]) }
}
impl Default for Quaternion {
    fn default() -> Self { Self::new(0.0, 0.0, 0.0, 0.0) }
}
impl Identity for Quaternion {
    fn identity() -> Self { Self::new(0.0, 0.0, 0.0, 1.0) }
}

impl Quaternion {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self { Self { x, y, z, w } }
    ///Returns the Conjugate of the Quaternion.
    pub fn conjugate(&self) -> Self { Self::new(-self.x, -self.y, -self.z, self.w) }
    ///Returns a Quaternion from a 3D vector axis and a float angle.
    pub fn from_axis_angle(axis: Vector<3>, angle: f32) -> Quaternion {
        let half_angle = angle/2.0;
        Quaternion::new(
            axis[0]*half_angle.sin(),
            axis[1]*half_angle.sin(),
            axis[2]*half_angle.sin(),
            half_angle.cos(),
        )
    }
    ///Returns a 3D Vector axis and a float angle from a Quaternion.
    pub fn to_axis_angle(&self) -> (Vector<3>, f32) {
        if self.w == 1.0 {
            return (Vector::<3>::new([1.0, 0.0, 0.0]), 0.0);
        }
        let angle = self.w.acos()*2.0;
        let half_angle = angle/2.0;
        (Vector::<3>::new([
            self.x/half_angle.sin(), self.y/half_angle.sin(), self.z/half_angle.sin()
        ]), angle)
    }
    ///Returns a Quaternion from a rotation around the X axis.
    pub fn from_x_angle(angle: f32) -> Quaternion {
        let half_angle = angle/2.0;
        Quaternion::new(
            half_angle.sin(),
            0.0,
            0.0,
            half_angle.cos(),
        )
    }

    ///Returns a Quaternion from a rotation around the Y axis.
    pub fn from_y_angle(angle: f32) -> Quaternion {
        let half_angle = angle/2.0;
        Quaternion::new(
            0.0,
            half_angle.sin(),
            0.0,
            half_angle.cos(),
        )
    }

    ///Returns a Quaternion from a rotation around the Z axis.
    pub fn from_z_angle(angle: f32) -> Quaternion {
        let half_angle = angle/2.0;
        Quaternion::new(
            0.0,
            0.0,
            half_angle.sin(),
            half_angle.cos(),
        )
    }

    ///Returns a Quaternion from an euler rotation (x, y and z angles) and a defined rotation order.
    pub fn from_euler(x: f32, y: f32, z: f32, axis_order: crate::RotationOrder) -> Quaternion {
        let mut quats = [Quaternion::identity(); 3];
        for i in 0..3 { 
            quats[i] = match axis_order.0[i] {
                crate::EulerAxis::X => Self::from_x_angle(-x),
                crate::EulerAxis::Y => Self::from_y_angle(-y),
                crate::EulerAxis::Z => Self::from_z_angle(-z)
            }
        }
        quats[2] * (quats[1] * quats[0])
    }

    ///Returns a quaternion defining a look at rotation - takes the vector we are looking from, the vector we are looking towards,
    ///the direction we are currently facing, an upwards vector and a forwards vector.
    pub fn look_at(from: Vector<3>, to: Vector<3>, facing: Vector<3>, up: Vector<3>, forward: Vector<3>) -> Quaternion {
        let diff = (to - from).normalised();
        let mut rot_axis = super::cross(facing, diff).normalised();
        if rot_axis.sq_sum() == 0.0 { rot_axis = up }
        let dot = forward.dot(to);
        let ang = dot.acos();
        Self::from_axis_angle(rot_axis, ang)
    }

    ///Returns a 3 by 3 rotation matrix from a Quaternion.
    pub fn quat_to_matrix(&self) -> crate::Matrix<3, 3> {
        let q0 = self.w;
        let q1 = self.x;
        let q2 = self.y;
        let q3 = self.z;
        crate::Matrix::<3, 3>::new([
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
    pub fn normalise(&self) -> Self {
        self * (1.0/self.mag())
    }
}
impl Magnitude for Quaternion {
    type Output = f32;
    fn mag(&self) -> f32 { Vector::<4>::from(*self).mag() }
}

impl Mul<Quaternion> for Quaternion {
    type Output = Quaternion;
    fn mul(self, other: Quaternion) -> Self {
        Quaternion::new(
            self.w*other.x - self.z*other.y + self.y*other.z + self.x*other.w,
            self.z*other.x + self.w*other.y - self.x*other.z + self.y*other.w,
            -self.y*other.x + self.x*other.y + self.w*other.z + self.z*other.w,
            -self.x*other.x - self.y*other.y - self.z*other.z + self.w*other.w,
        )
    }
}
impl Mul<&Quaternion> for Quaternion {
    type Output = Quaternion;
    fn mul(self, other: &Quaternion) -> Quaternion { self * *other }
}
impl Mul<Quaternion> for &Quaternion {
    type Output = Quaternion;
    fn mul(self, other: Quaternion) -> Quaternion { *self * other }
}
impl Mul<&Quaternion> for &Quaternion {
    type Output = Quaternion;
    fn mul(self, other: &Quaternion) -> Quaternion { *self * *other }
}
impl Mul<f32> for Quaternion {
    type Output = Quaternion;
    fn mul(self, other: f32) -> Self {
        Self::new(self.x*other, self.y*other, self.z*other, self.w*other)
    }
}
impl Mul<&f32> for Quaternion {
    type Output = Quaternion;
    fn mul(self, other: &f32) -> Quaternion { self * *other }
}
impl Mul<f32> for &Quaternion {
    type Output = Quaternion;
    fn mul(self, other: f32) -> Quaternion { *self * other }
}
impl Mul<&f32> for &Quaternion {
    type Output = Quaternion;
    fn mul(self, other: &f32) -> Quaternion { *self * *other }
}
impl Mul<Quaternion> for f32 {
    type Output = Quaternion;
    fn mul(self, other: Quaternion) -> Quaternion {
        Quaternion::new(other.x*self, other.y*self, other.z*self, other.w*self)
    }
}
impl Mul<&Quaternion> for f32 {
    type Output = Quaternion;
    fn mul(self, other: &Quaternion) -> Quaternion { self * *other }
}
impl Mul<Quaternion> for &f32 {
    type Output = Quaternion;
    fn mul(self, other: Quaternion) -> Quaternion { *self * other }
}
impl Mul<&Quaternion> for &f32 {
    type Output = Quaternion;
    fn mul(self, other: &Quaternion) -> Quaternion { *self * *other }
}
impl Add<Quaternion> for Quaternion {
    type Output = Quaternion;
    fn add(self, other: Quaternion) -> Self {
        Quaternion::new(
            self.x + other.x, self.y + other.y, self.z + other.z, self.w + other.w
        )
    }
}
impl Add<&Quaternion> for Quaternion {
    type Output = Quaternion;
    fn add(self, other: &Quaternion) -> Quaternion { self + *other }
}
impl Add<Quaternion> for &Quaternion {
    type Output = Quaternion;
    fn add(self, other: Quaternion) -> Quaternion { *self + other }
}
impl Add<&Quaternion> for &Quaternion {
    type Output = Quaternion;
    fn add(self, other: &Quaternion) -> Quaternion { *self + *other }
}
impl MulAssign<Quaternion> for Quaternion {
    fn mul_assign(&mut self, other: Quaternion) { *self = *self * other }
}
impl MulAssign<Quaternion> for &mut Quaternion {
    fn mul_assign(&mut self, other: Quaternion) { **self = **self * other }
}
impl MulAssign<&Quaternion> for Quaternion {
    fn mul_assign(&mut self, other: &Quaternion) { *self = *self * other }
}
impl MulAssign<&Quaternion> for &mut Quaternion {
    fn mul_assign(&mut self, other: &Quaternion) { **self = **self * other }
}
impl MulAssign<f32> for Quaternion {
    fn mul_assign(&mut self, other: f32) { *self = *self * other }
}
impl MulAssign<f32> for &mut Quaternion {
    fn mul_assign(&mut self, other: f32) { **self = **self * other }
}
impl MulAssign<&f32> for Quaternion {
    fn mul_assign(&mut self, other: &f32) { *self = *self * other }
}
impl MulAssign<&f32> for &mut Quaternion {
    fn mul_assign(&mut self, other: &f32) { **self = **self * other }
}
impl AddAssign<Quaternion> for Quaternion {
    fn add_assign(&mut self, other: Quaternion) { *self = *self + other }
}
impl AddAssign<Quaternion> for &mut Quaternion {
    fn add_assign(&mut self, other: Quaternion) { **self = **self + other }
}
impl AddAssign<&Quaternion> for Quaternion {
    fn add_assign(&mut self, other: &Quaternion) { *self = *self + other }
}
impl AddAssign<&Quaternion> for &mut Quaternion {
    fn add_assign(&mut self, other: &Quaternion) { **self = **self + other }
}

///Dual Quaternion type - these can represent all rigid 3D combinations of a translation and a rotation. Useful for 
///transformations that do not require scaling, such as camera transformations or skeletal animation. See https://cs.gmu.edu/~jmlien/teaching/cs451/uploads/Main/dual-quaternion.pdf
///and https://faculty.sites.iastate.edu/jia/files/inline-files/dual-quaternion.pdf for the resources I used to research + build this.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DualQuaternion {
    pub real: Quaternion,
    pub dual: Quaternion
}
impl DualQuaternion {
    pub fn new(data: [f32; 8]) -> Self {
        Self { 
            real: Quaternion::new(data[0], data[1], data[2], data[3]), 
            dual: Quaternion::new(data[4], data[5], data[6], data[7])
        }
    }
    ///Returns a new Dual Quaternion from two separate Quaternions.
    pub fn from_quats(a: Quaternion, b: Quaternion) -> Self {
        Self { real: a, dual: b }
    }
    ///Adds two Dual Quaternions - helper function for implementing ops.
    pub fn adds(&self, other: DualQuaternion) -> Self {
        Self { real: self.real + other.real, dual: self.dual + other.dual }
    }
    ///Multiplies a Dual Quaternion by a Scalar f32 - helper function for implementing ops.
    pub fn scalar_mul(&self, other: f32) -> Self {
        Self { real: self.real * other, dual: self.dual * other }
    }
    ///Multiplies two Dual Quaternions - helper function for implementing ops.
    pub fn muls(&self, other: DualQuaternion) -> Self {
        Self {
            real: self.real * other.real,
            dual: self.real * other.dual + self.dual * other.real
        }
    }
    pub fn inverse(&self) -> Self {
        Self::from_quats(
            self.real.conjugate(),
            -1.0 * self.real.conjugate() * self.dual * self.real.conjugate()
        ).normalise()
    }
    ///Returns the Conjugate of the Dual Quaternion. Note - there are three different ways of taking the conjugate of a
    ///dual quaternion, but this is the most common and combines the other two, which are individually also available as dual_conjugate() and quaternion_conjugate().
    ///For a Dual Quaternion Q + eR, this is Q* - eR* i.e. both quaternions are conjugated and the sign of the dual part is flipped.
    pub fn conjugate(&self) -> Self {
        Self {
            real: self.real.conjugate(),
            dual: -1.0 * self.dual.conjugate()
        }
    }
    ///Returns the Dual Conjugate of the Dual Quaternion. For a Dual Quaternion Q + eR, this is Q - eR - i.e. the sign of the dual part is flipped.
    pub fn dual_conjugate(&self) -> Self {
        Self { real: self.real, dual: -1.0 * self.dual }
    }
    ///Returns the Quaternion Conjugate of the Dual Quaternion. For a Dual Quaternion Q + eR, this is Q* + eR* - i.e. both quaternions are conjugated.
    pub fn quaternion_conjugate(&self) -> Self {
        Self { real: self.real.conjugate(), dual: self.dual.conjugate() }
    }
    ///Returns the magnitude of the Dual Quaternion - this is calculated as the product of the Dual Quaternion and its quaternion conjugate.
    ///This returns a dual scalar - a Dual Quaternion is unit if the first part is 1 and the second part is 0.
    pub fn magnitude(&self) -> (f32, f32) {
        let v4 = Vector::<4>::from(self.real);
        let d4 = Vector::<4>::from(self.dual);
        (v4.mag(), 2.0 * v4.dot(d4))
    }
    pub fn normalise(&self) -> Self {
        let mag = self.magnitude().0;
        Self::from_quats(self.real*(1.0/mag), self.dual*(1.0/mag))
    }
    ///Returns a Dual Quaternion from a Quaternion rotation
    pub fn from_quat(q: Quaternion) -> Self {
        Self::from_quats(q, Quaternion::default())
    }
    ///Returns a Dual Quaternion from a Vector<3> translation.
    pub fn from_translate(t: Vector<3>) -> Self {
        Self::from_quats(
            Quaternion::identity(),
            Quaternion::new(t[0]/2.0, t[1]/2.0, t[2]/2.0, 0.0)
        )
    }
    ///Returns a Dual Quaternion representing a Quaternion rotation followed by a Vector<3> translation.
    pub fn from_rotate_translate(q: Quaternion, t: Vector<3>) -> Self {
        Self::from_quats(
            q,
            0.5 * (Quaternion::new(t[0], t[1], t[2], 1.0) * q)
        )
    }
    ///Returns a Dual Quaternion representing a Vector<3> translation followed by a Quaternion rotation.
    pub fn from_translate_rotate(t: Vector<3>, q: Quaternion) -> Self {
        Self::from_quats(
            q,
            0.5 * (q * Quaternion::new(t[0], t[1], t[2], 1.0))
        )
    }
    pub fn to_matrix(&self) -> Matrix::<4, 4> { 
        let t = self.translation();
        from_trs(
            Vector::<3>::new([t[0], t[1], t[2]]),
            self.rotation(),
            Vector::<3>::new([1.0, 1.0, 1.0])
        )
    }
    ///Returns the rotation only of a Dual Quaternion.
    pub fn rotation(&self) -> Quaternion {
        self.real
    }
    ///Returns the translation only of a Dual Quaternion.
    pub fn translation(&self) -> Vector<4> {
        Vector::<4>::from(2.0 * (self.dual * self.real.conjugate()))
    }
    ///Applies the Dual Quaternion to a Vector<4> point.
    pub fn transform(&self, point: Vector<4>) -> Vector<4> {
        let dual = Self::from_quats(Quaternion::identity(), Quaternion::from(point));
        //eprintln!("Made Dual: {:?}", dual);
        //eprintln!("New Dual: {:?}", self.mul(dual));
        //eprintln!("Conjugate: {:?}", self.conjugate());
        let new = self.mul(dual).mul(self.conjugate());
        Vector::<4>::from(new.dual)
    }
}

impl Magnitude for DualQuaternion {
    type Output = (f32, f32);
    fn mag(&self) -> (f32, f32) { self.magnitude() }
}

impl Mul<DualQuaternion> for DualQuaternion {
    type Output = DualQuaternion;
    fn mul(self, other: DualQuaternion) -> Self {
        self.muls(other)
    }
}
impl Mul<&DualQuaternion> for DualQuaternion {
    type Output = DualQuaternion;
    fn mul(self, other: &DualQuaternion) -> DualQuaternion { self * *other }
}
impl Mul<DualQuaternion> for &DualQuaternion {
    type Output = DualQuaternion;
    fn mul(self, other: DualQuaternion) -> DualQuaternion { *self * other }
}
impl Mul<&DualQuaternion> for &DualQuaternion {
    type Output = DualQuaternion;
    fn mul(self, other: &DualQuaternion) -> DualQuaternion { *self * *other }
}
impl Mul<f32> for DualQuaternion {
    type Output = DualQuaternion;
    fn mul(self, other: f32) -> Self {
        self.scalar_mul(other)
    }
}
impl Mul<&f32> for DualQuaternion {
    type Output = DualQuaternion;
    fn mul(self, other: &f32) -> DualQuaternion { self * *other }
}
impl Mul<f32> for &DualQuaternion {
    type Output = DualQuaternion;
    fn mul(self, other: f32) -> DualQuaternion { *self * other }
}
impl Mul<&f32> for &DualQuaternion {
    type Output = DualQuaternion;
    fn mul(self, other: &f32) -> DualQuaternion { *self * *other }
}
impl Mul<DualQuaternion> for f32 {
    type Output = DualQuaternion;
    fn mul(self, other: DualQuaternion) -> DualQuaternion {
        other.scalar_mul(self)
    }
}
impl Mul<&DualQuaternion> for f32 {
    type Output = DualQuaternion;
    fn mul(self, other: &DualQuaternion) -> DualQuaternion { self * *other }
}
impl Mul<DualQuaternion> for &f32 {
    type Output = DualQuaternion;
    fn mul(self, other: DualQuaternion) -> DualQuaternion { *self * other }
}
impl Mul<&DualQuaternion> for &f32 {
    type Output = DualQuaternion;
    fn mul(self, other: &DualQuaternion) -> DualQuaternion { *self * *other }
}
impl Add<DualQuaternion> for DualQuaternion {
    type Output = DualQuaternion;
    fn add(self, other: DualQuaternion) -> Self {
        self.adds(other)
    }
}
impl Add<&DualQuaternion> for DualQuaternion {
    type Output = DualQuaternion;
    fn add(self, other: &DualQuaternion) -> DualQuaternion { self + *other }
}
impl Add<DualQuaternion> for &DualQuaternion {
    type Output = DualQuaternion;
    fn add(self, other: DualQuaternion) -> DualQuaternion { *self + other }
}
impl Add<&DualQuaternion> for &DualQuaternion {
    type Output = DualQuaternion;
    fn add(self, other: &DualQuaternion) -> DualQuaternion { *self + *other }
}
impl MulAssign<DualQuaternion> for DualQuaternion {
    fn mul_assign(&mut self, other: DualQuaternion) { *self = *self * other }
}
impl MulAssign<DualQuaternion> for &mut DualQuaternion {
    fn mul_assign(&mut self, other: DualQuaternion) { **self = **self * other }
}
impl MulAssign<&DualQuaternion> for DualQuaternion {
    fn mul_assign(&mut self, other: &DualQuaternion) { *self = *self * other }
}
impl MulAssign<&DualQuaternion> for &mut DualQuaternion {
    fn mul_assign(&mut self, other: &DualQuaternion) { **self = **self * other }
}
impl MulAssign<f32> for DualQuaternion {
    fn mul_assign(&mut self, other: f32) { *self = *self * other }
}
impl MulAssign<f32> for &mut DualQuaternion {
    fn mul_assign(&mut self, other: f32) { **self = **self * other }
}
impl MulAssign<&f32> for DualQuaternion {
    fn mul_assign(&mut self, other: &f32) { *self = *self * other }
}
impl MulAssign<&f32> for &mut DualQuaternion {
    fn mul_assign(&mut self, other: &f32) { **self = **self * other }
}
impl AddAssign<DualQuaternion> for DualQuaternion {
    fn add_assign(&mut self, other: DualQuaternion) { *self = *self + other }
}
impl AddAssign<DualQuaternion> for &mut DualQuaternion {
    fn add_assign(&mut self, other: DualQuaternion) { **self = **self + other }
}
impl AddAssign<&DualQuaternion> for DualQuaternion {
    fn add_assign(&mut self, other: &DualQuaternion) { *self = *self + other }
}
impl AddAssign<&DualQuaternion> for &mut DualQuaternion {
    fn add_assign(&mut self, other: &DualQuaternion) { **self = **self + other }
}