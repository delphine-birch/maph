use crate::base::*;
use std::collections::VecDeque;
use std::ops::{Deref};

#[derive(Clone, Debug)]
pub struct SplineSegment<const N: usize>
{
    mat: Box<Matrix<4, 4>>,
    points: Matrix<4, N>
}
impl<const N: usize> SplineSegment<N>
{
    pub fn calc(&self, t: f32) -> Vector<N> {
        let tv = Vector::<4>::new([1.0, t, t*t, t*t*t]);
        let mv = tv * *self.mat.deref();
        mv * self.points
    }
}

#[derive(Clone, Debug)]
pub struct Spline<const N: usize>
{
    mat: Matrix<4, 4>,
    segments: VecDeque<SplineSegment<N>>
}
impl<const N: usize> Spline<N>
{
    pub fn new(&self, mat: Matrix<4, 4>, points: Vec<Vector<N>>) -> Self {
        let mut segments = VecDeque::new();
        for i in 0..points.len()/4 {
            if let (Some(a), Some(b), Some(c), Some(d)) = (points.get(i*4), points.get(i*4 + 1), points.get(i*4 + 2), points.get(i*4 + 3)) {
                let mut data = [[0.0; N]; 4];
                for i in 0..N {
                    data[0][i] = a[i];
                    data[1][i] = b[i];
                    data[2][i] = c[i];
                    data[3][i] = d[i];
                }
                segments.push_back(SplineSegment::<N> {
                    mat: Box::new(mat),
                    points: Matrix::<4, N>::new(data)
                })
            }
        }
        Self { mat, segments }
    }
    pub fn calc(&self, t: f32) -> Option<Vector<N>> {
        let f = t.floor();
        let t0 = t - f;
        match self.segments.get(f as usize) {
            None => None,
            Some(seg) => Some(seg.calc(t0))
        }
    }
    pub fn mat(&self) -> Matrix<4, 4> { self.mat }
    pub fn pop_last(&mut self) -> Option<SplineSegment<N>> { self.segments.pop_back() }
    pub fn pop_first(&mut self) -> Option<SplineSegment<N>> { self.segments.pop_front() }
}