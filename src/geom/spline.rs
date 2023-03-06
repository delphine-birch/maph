use crate::geom::{vector::*, matrix::*};
use std::collections::VecDeque;
use std::ops::{Deref};

///Spline Segment - set of four control points with the matrix for calculating output from t:0.0 - 1.0.
///N Dimensional.
#[derive(Clone, Debug)]
pub struct SplineSegment<const N: usize>
{
    mat: Box<Matrix<4, 4>>,
    points: Matrix<4, N>
}
impl<const N: usize> SplineSegment<N>
{
    ///Returns a new segment based on 4 N dimensional points and a 4 by 4 matrix.
    pub fn new(points: [Vector<N>; 4], spline_matrix: Matrix<4, 4>) -> Self {
        let mut data = [[0.0; N]; 4];
        for i in 0..N {
            data[0][i] = points[0][i];
            data[1][i] = points[1][i];
            data[2][i] = points[2][i];
            data[3][i] = points[3][i];
        }
        Self { mat: Box::new(spline_matrix), points: Matrix::<4, N>::new(data) }
    }
    ///Calculates point (Vector<N>) based on t-value - should be between 0.0 and 1.0 but will behave
    ///as you would expect a quadratic spline to for values outside that range. 
    pub fn calc(&self, t: f32) -> Vector<N> {
        let tv = Vector::<4>::new([1.0, t, t*t, t*t*t]);
        let mv = tv * *self.mat.deref();
        mv * self.points
    }
}

///N dimensional Quadratic Spline type - contains a 4 by 4 matrix defining spline behaviour and a set of segments,
///each defining a set of four control points (although the last point of a segment and the first point
///of the next should usually overlap).  
#[derive(Clone, Debug)]
pub struct Spline<const N: usize>
{
    ///Matrix defining Quadratic Spline behaviour.
    pub mat: Matrix<4, 4>,
    ///VecDeque of segments - allows for easy removal at both beginning and end of spline.
    pub segments: VecDeque<SplineSegment<N>>
}
impl<const N: usize> Spline<N>
{
    ///Returns a new spline from a matrix defining its behaviour and a list of points.
    ///This will generate segments based on consecutive sets of 4 points. i.e. :
    ///List of points [A, B, C, D, E, F, G, H] will generate two segments, ABCD and EFGH.
    pub fn new(mat: Matrix<4, 4>, points: Vec<Vector<N>>) -> Self {
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
    ///Returns a new spline from a matrix defining its behaviour and a list of points.
    ///This will generate segments based on overlapping sets of 4 points. i.e. :
    ///List of points [A, B, C, D, E, F, G, H, I, J] will generate three segments, ABCD, DEFG and GHIJ.
    pub fn new_overlap(mat: Matrix<4, 4>, points: Vec<Vector<N>>) -> Self {
        let mut segments = VecDeque::new();
        for i in 0..points.len()/3 {
            if let (Some(a), Some(b), Some(c), Some(d)) = (points.get(i*3), points.get(i*3 + 1), points.get(i*3 + 2), points.get(i*3 + 3)) {
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
    ///Calculates an N-dimensional points based on a t-value - should not be less than 0.0, otherwise value n.0
    ///will calculate from segment index n. Will return None if t value is higher than number of segments would allow.
    pub fn calc(&self, t: f32) -> Option<Vector<N>> {
        let f = t.floor();
        let t0 = t - f;
        match self.segments.get(f as usize) {
            None => None,
            Some(seg) => Some(seg.calc(t0))
        }
    }
    ///Returns the matrix defining the quadratic spline behaviour.
    pub fn mat(&self) -> Matrix<4, 4> { self.mat }
    ///Removes and returns the last spline segment.
    pub fn pop_last(&mut self) -> Option<SplineSegment<N>> { self.segments.pop_back() }
    ///Removes and returns the first spline segment.
    pub fn pop_first(&mut self) -> Option<SplineSegment<N>> { self.segments.pop_front() }
}