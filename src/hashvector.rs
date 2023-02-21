use crate::base::Vector;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HashVector<const N: usize, const P: usize> {
    data: [u64; N],
}

impl<const N: usize, const P: usize> HashVector<N, P> {
    pub fn f32_to_u64(v: f32) -> u64 { (v*(10.0_f32.powf(P as f32)).floor()) as u64 }
    pub fn u64_to_f32(v: u64) -> f32 { (v/(10_u64.pow(P as u32))) as f32 }
    pub fn new(vec: Vector<N>) -> Self {
        let mut data = [0; N];
        for i in 0..N {
            data[i] = Self::f32_to_u64(vec.data[i]);
        }
        Self { data }
    }
    pub fn as_vector(&self) -> Vector<N> {
        let mut data = [0.0; N];
        for i in 0..N {
            data[i] = Self::u64_to_f32(self.data[i]);
        }
        Vector::<N>::new(data)
    }
}