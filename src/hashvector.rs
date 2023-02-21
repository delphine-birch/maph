use crate::base::Vector;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HashVector<const N: usize, const P: usize> {
    data: [i64; N],
}

impl<const N: usize, const P: usize> HashVector<N, P> {
    pub fn f32_to_i64(v: f32) -> i64 { (v*(10.0_f32.powf(P as f32)).floor()) as i64 }
    pub fn i64_to_f32(v: i64) -> f32 { v as f32/(10.0_f32.powf(P as f32)) }
    pub fn new(vec: Vector<N>) -> Self {
        let mut data = [0; N];
        for i in 0..N {
            data[i] = Self::f32_to_i64(vec.data[i]);
        }
        Self { data }
    }
    pub fn as_vector(&self) -> Vector<N> {
        let mut data = [0.0; N];
        for i in 0..N {
            data[i] = Self::i64_to_f32(self.data[i]);
        }
        Vector::<N>::new(data)
    }
}