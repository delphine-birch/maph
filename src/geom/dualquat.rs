use crate::geom::{vector::*};
use crate::geom::transforms::quaternion::{quaternion_conjugate, quaternion_identity, compose};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DualQuat {
    pub real: Vector<4>,
    pub dual: Vector<4>
}
impl DualQuat {
    pub fn new(data: [f32; 8]) -> Self {
        Self { real: Vector::<4>::new(
            [data[0], data[1], data[2], data[3]]), 
            dual: Vector::<4>::new([data[4], data[5], data[6], data[7]])
        }
    }
    pub fn from_quats(a: Vector<4>, b: Vector<4>) -> Self {
        Self { real: a, dual: b }
    }
    pub fn add(&self, other: DualQuat) -> Self {
        Self { real: self.real + other.real, dual: self.dual + other.dual }
    }
    pub fn scalar_mul(&self, other: f32) -> Self {
        Self { real: self.real * other, dual: self.dual * other }
    }
    pub fn mul(&self, other: DualQuat) -> Self {
        Self {
            real: compose(self.real, other.real),
            dual: compose(self.real, other.dual) + compose(self.dual, other.real)
        }
    }
    pub fn conjugate(&self) -> Self {
        Self {
            real: quaternion_conjugate(self.real),
            dual: -1.0 * quaternion_conjugate(self.dual)
        }
    }
    pub fn magnitude(&self) -> (f32, f32) {
        (self.real.mag(), 2.0 * self.real[3] * self.dual[3] * self.real.dot(self.dual))
    }
    pub fn from_quat(q: Vector<4>) -> Self {
        Self::from_quats(q, Vector::<4>::default())
    }
    pub fn from_translate(t: Vector<3>) -> Self {
        Self::from_quats(
            quaternion_identity(),
            Vector::<4>::new([t[0]/2.0, t[1]/2.0, t[2]/2.0, 0.0])
        )
    }
    pub fn from_rotate_translate(q: Vector<4>, t: Vector<3>) -> Self {
        Self::from_quats(
            q,
            0.5 * compose(Vector::<4>::new([t[0], t[1], t[2], 1.0]), q)
        )
    }

    pub fn from_translate_rotate(t: Vector<3>, q: Vector<4>) -> Self {
        Self::from_quats(
            q,
            0.5 * compose(q, Vector::<4>::new([t[0], t[1], t[2], 1.0]))
        )
    }
    
    pub fn rotation(&self) -> Vector<4> {
        self.real
    }
    pub fn translation(&self) -> Vector<4> {
        2.0 * compose(self.dual, quaternion_conjugate(self.real))
    }
    pub fn transform(&self, point: Vector<4>) -> Vector<4> {
        let dual = Self::from_quats(quaternion_identity(), point);
        eprintln!("Made Dual: {:?}", dual);
        eprintln!("New Dual: {:?}", self.mul(dual));
        eprintln!("Conjugate: {:?}", self.conjugate());
        let new = self.mul(dual).mul(self.conjugate());
        new.dual
    }
}