pub mod spline;
pub mod transforms;
pub mod vector;
pub mod matrix;
pub mod quaternion;

use std::f32::consts::TAU;

use crate::geom::{vector::*};

pub fn cross(a: Vector<3>, b: Vector<3>) -> Vector<3> {
    let x = a[1]*b[2] - a[2]*b[1];
    let y = a[2]*b[0] - a[0]*b[2];
    let z = a[0]*b[1] - a[1]*b[0];
    Vector::<3>::new([x, y, z])
}

pub fn circum_centre(a: Vector<2>, b: Vector<2>, c: Vector<2>) -> Vector<2> {
    let d = (a[0]*(b[1]-c[1]) + b[0]*(c[1]-a[1]) + c[0]*(a[1]-b[1])) * 2.0;
    let x = (a.sq_sum()*(b[1]-c[1]) + b.sq_sum()*(c[1]-a[1]) + c.sq_sum()*(a[1]-b[1]))/d;
    let y = (a.sq_sum()*(c[0]-b[0]) + b.sq_sum()*(a[0]-c[0]) + c.sq_sum()*(b[0]-a[0]))/d;
    Vector::<2>::new([x, y])
}

pub fn heading(v: Vector<2>) -> f32 { v[1].atan2(v[0]) }

pub fn rotate(v: Vector<2>, a: f32) -> Vector<2> { 
    let r = heading(v) + a;
    let m = v.mag();
    Vector::<2>::new([m*r.cos(), m*r.sin()])
}

pub fn unit_geom<const N: usize>(size: f32) -> [Vector<2>; N] {
    let mut out = [Vector::<2>::default(); N];
    for i in 0..N {
        let a = i as f32 * (TAU/(N as f32));
        out[i] = Vector::<2>::new([size*a.cos(), size*a.sin()]);
    }
    out
}

pub fn av_vector<const N: usize>(v: &[Vector<N>]) -> Vector<N> {
    let mut out = Vector::<N>::default();
    for p in v {
        out = out + *p;
    }
    out = out / v.len() as f32;
    out
}

pub fn bounds<const N: usize>(v: &[Vector<N>]) -> (Vector<N>, Vector<N>) {
    let mut min = [0.0; N];
    let mut max = [0.0; N];
    for i in 0..N {
        min[i] = v.iter().map(|p| p[i]).fold(f32::INFINITY, |a, b| a.min(b));
        max[i] = v.iter().map(|p| p[i]).fold(f32::NEG_INFINITY, |a, b| a.max(b));
    }
    (Vector::<N>::new(min), Vector::<N>::new(max))
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Orientation { Colinear, ClockWise, CounterClockWise }
pub fn orientation(a: Vector<2>, b: Vector<2>, c: Vector<2>) -> Orientation {
    let val = (b[1] - a[1]) * (c[0] - b[0]) - (b[0] - a[0]) * (c[1] - a[1]);
    if val == 0.0 { return Orientation::Colinear }
    else if val > 0.0 { return Orientation::ClockWise }
    else { return Orientation::CounterClockWise }
}

pub fn on_segment(p: Vector<2>, q1: Vector<2>, q2: Vector<2>) -> bool {
    p[0] <= q1[0].max(q2[0]) &&
    p[0] >= q1[0].min(q2[0]) &&
    p[1] >= q1[1].max(q2[1]) &&
    p[1] <= q1[1].min(q2[1])
}

pub fn line_intersect(p1: Vector<2>, p2: Vector<2>, q1: Vector<2>, q2: Vector<2>) -> bool {
    let o1 = orientation(p1, p2, q1);
    let o2 = orientation(p1, p2, q2);
    let o3 = orientation(q1, q2, p1);
    let o4 = orientation(q1, q2, p2);

    if o1 != o2 && o3  != o4 { return true; }
    if o1 == Orientation::Colinear && on_segment(q1, p1, p2) { return true; }
    if o2 == Orientation::Colinear && on_segment(q2, p1, p2) { return true; }
    if o3 == Orientation::Colinear && on_segment(p1, q1, q2) { return true; }
    if o4 == Orientation::Colinear && on_segment(p2, q1, q2) { return true; }
    
    false 
}

pub fn point_in_poly(p: Vector<2>, v: &[Vector<2>]) -> bool {
    let bounds = bounds(v);
    let (p1, p2) = (p, Vector::<2>::new([bounds.1[0]*10.0, p[1]]));
    let mut num_intersect = 0;
    for (i, &q1) in v.iter().enumerate() {
        let q2i = match i + 1 > v.len() { true => i + 1, false => 0 };
        let q2 = v[q2i];
        if line_intersect(p1, p2, q1, q2) { num_intersect += 1; }
    }
    num_intersect % 2 == 0
}

pub fn dist_to_plane(point: Vector<3>, plane_point: Vector<3>, plane_normal: Vector<3>) -> f32 {
    (point - plane_point).dot(plane_normal)
}

