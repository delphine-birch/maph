use crate::matrix::Mat4;
use crate::vector::{Vector4};
use std::collections::VecDeque;
use std::ops::{Add, Sub, Mul, Div, Deref};

#[derive(Clone, Debug)]
pub struct SplineSegment<T> 
where T : Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    mat: Box<Mat4<f32>>,
    points: Vector4<T>
}
impl<T> SplineSegment<T>
where T : Mul<f32, Output=T> + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy,
      Vector4<f32> : Mul<Mat4<f32>>
{
    pub fn calc(&self, t: f32) -> T {
        let tv = Vector4::new(1.0, t, t*t, t*t*t);
        let mv = tv * self.mat.deref();
        (self.points.x * mv.x) + (self.points.y * mv.y) + (self.points.z * mv.z) + (self.points.w * mv.w)
    }
}

#[derive(Clone, Debug)]
pub struct Spline<T>
where T : Mul<f32, Output=T> + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    mat: Mat4<f32>,
    segments: VecDeque<SplineSegment<T>>
}
impl<T> Spline<T>
where T : Mul<f32, Output=T> + Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T> + Copy
{
    pub fn new(&self, mat: Mat4<f32>, points: Vec<T>) -> Self {
        let mut segments = VecDeque::new();
        for i in 0..points.len()/4 {
            if let (Some(a), Some(b), Some(c), Some(d)) = (points.get(i*4), points.get(i*4 + 1), points.get(i*4 + 2), points.get(i*4 + 3)) {
                segments.push_back(SplineSegment::<T> {
                    mat: Box::new(mat),
                    points: Vector4::new(a.clone(), b.clone(), c.clone(), d.clone())
                })
            }
        }
        Self { mat, segments }
    }
    pub fn calc(&self, t: f32) -> Option<T> {
        let f = t.floor();
        let t0 = t - f;
        match self.segments.get(f as usize) {
            None => None,
            Some(seg) => Some(seg.calc(t0))
        }
    }
    pub fn mat(&self) -> Mat4<f32> { self.mat }
    pub fn pop_last(&mut self) -> Option<SplineSegment<T>> { self.segments.pop_back() }
    pub fn pop_first(&mut self) -> Option<SplineSegment<T>> { self.segments.pop_front() }
}