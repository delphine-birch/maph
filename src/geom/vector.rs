use std::ops::{Add, Sub, Mul, Div, Index, IndexMut};
use std::fmt; 
use std::fmt::Display;
use std::collections::{HashMap, HashSet};
use std::f32::consts::PI;
use bimap::BiHashMap;
use crate::num::{Identity, Sqroot, rational::*};
use super::matrix::*;


///Vector Type - length of L, components are f32. Indexable by usize index of component.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector<const L: usize> {
    ///Array of f32 components.
    pub data: [f32; L],
}
impl<const L: usize> Vector<L> {
    ///Returns a new Vector of length L from an array of f32 with the same length.
    pub fn new(data: [f32; L]) -> Self { Self { data } }
    ///Dot product with another Vector of same length.
    pub fn dot(&self, other: Vector<L>) -> f32 {
        (0..L).map(|i| self.data[i]*other.data[i]).sum::<f32>()
    }
    ///Returns the Vector as a Matrix with 1 row and L columns - aka the Vector as a
    ///single row matrix.
    pub fn as_row(&self) -> Matrix<1, L> {
        Matrix::<1, L>::new([self.data])
    }
    ///Returns the Vector as a Matrix with L rows and 1 column - aka the Vector as a
    ///single column matrix.
    pub fn as_col(&self) -> Matrix<L, 1> {
        let mut columns = [[0.0]; L];
        for i in 0..L {
            columns[i][0] = self.data[i];
        }
        Matrix::<L, 1>::new(columns)
    }
    ///Utility function for adding vectors - used to implement std::ops.
    pub fn vec_add(&self, other: Self) -> Self {
        let mut data = [0.0; L];
        for i in 0..L {
            data[i] = self.data[i] + other.data[i];
        }
        Self { data }
    }
    ///Utility function for multiplying vectors - used to implement std::ops.
    pub fn vec_mul(&self, other: Self) -> Self {
        let mut data = [0.0; L];
        for i in 0..L {
            data[i] = self.data[i] * other.data[i];
        }
        Self { data }
    }
    ///Utility function for multiplying a vector by a float - used to implement std::ops.
    pub fn float_mul(&self, other: f32) -> Self {
        let mut data = [0.0; L];
        for i in 0..L {
            data[i] = self.data[i] * other;
        }
        Self { data }
    }
    ///Utility function for getting the reciprocal of the vector, component wise - used to implement std::ops.
    pub fn recip(&self) -> Self {
        let mut data = [0.0; L];
        for i in 0..L {
            data[i] = 1.0/self.data[i];
        }
        Self { data }
    }
    ///Returns the sum of the components of the vector.
    pub fn sum(&self) -> f32 {
        let mut sum = 0.0;
        for i in 0..L { sum += self[i] }
        sum
    }
    ///Returns the sum of the squared components of the vector, i.e. the squared magnitude of the vector.
    pub fn sq_sum(&self) -> f32 {
        let mut sum = 0.0;
        for i in 0..L { sum += self[i]*self[i] }
        sum
    }
    ///Returns the magnitude of the vector.
    pub fn mag(&self) -> f32 {
        self.sq_sum().sqrt()
    }
    ///Returns a normalised copy of the vector.
    pub fn normalised(&self) -> Self { *self/self.mag() }
}

impl<const L: usize> Display for Vector<L> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.data)
    }
}
impl<const L: usize> Default for Vector<L> {
    fn default() -> Vector<L> { Vector::<L>::new([0.0; L]) }
}
impl<const L: usize> Identity for Vector<L> {
    fn identity() -> Vector<L> { Vector::<L>::new([1.0; L]) }
}
impl<const L: usize> Index<usize> for Vector<L> {
    type Output = f32;
    fn index(&self, index: usize) -> &f32 { &self.data[index] }
}
impl<const L: usize> IndexMut<usize> for Vector<L> {
    fn index_mut(&mut self, index: usize) -> &mut f32 { &mut self.data[index] }
}
impl<const L: usize> Add<Vector<L>> for Vector<L> {
    type Output = Vector<L>;
    fn add(self, other: Vector<L>) -> Self { self.vec_add(other) }
}

impl<const L: usize> Mul<Vector<L>> for Vector<L> {
    type Output = Vector<L>;
    fn mul(self, other: Vector<L>) -> Self { self.vec_mul(other) }
}

impl<const L: usize> Mul<f32> for Vector<L> {
    type Output = Vector<L>;
    fn mul(self, other: f32) -> Self { self.float_mul(other) }
}

impl<const L: usize> Mul<Vector<L>> for f32 {
    type Output = Vector<L>;
    fn mul(self, other: Vector<L>) -> Vector<L> { other.float_mul(self) }
}

impl<const L: usize> Div<f32> for Vector<L> {
    type Output = Vector<L>;
    fn div(self, other: f32) -> Self { self * (1.0/other) }
}

impl<const L: usize> Sub<Vector<L>> for Vector<L> {
    type Output = Vector<L>;
    fn sub(self, other: Vector<L>) -> Self { self + (other*-1.0) }
}

impl<const L: usize> Div<Vector<L>> for Vector<L> {
    type Output = Vector<L>;
    fn div(self, other: Vector<L>) -> Self { self * other.recip() }
}


///Vector type using rational components - allows for higher precision + implementation of
///Eq and Hash. Length of L, components are r32, indexable by usize index.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct VectorPrecise<const L: usize> {
    ///Array of r32 components.
    pub data: [r32; L],
}
impl<const L: usize> VectorPrecise<L> {
    ///Returns a new Vector of length L from an array of f32 with the same length.
    pub fn new(data: [r32; L]) -> Self { Self { data } }
    ///Dot product with another Vector of same length.
    pub fn dot(&self, other: VectorPrecise<L>) -> r32 {
        (0..L).map(|i| self.data[i]*other.data[i]).fold(r32::default(), |a, e| a + e)
    }
    ///Returns the Vector as a Matrix with 1 row and L columns - aka the Vector as a
    ///single row matrix.
    pub fn as_row(&self) -> MatrixPrecise<1, L> {
        MatrixPrecise::<1, L>::new([self.data])
    }
    ///Returns the Vector as a Matrix with L rows and 1 column - aka the Vector as a
    ///single column matrix.
    pub fn as_col(&self) -> MatrixPrecise<L, 1> {
        let mut columns = [[r32::default()]; L];
        for i in 0..L {
            columns[i][0] = self.data[i];
        }
        MatrixPrecise::<L, 1>::new(columns)
    }
    ///Utility function for adding vectors - used to implement std::ops.
    pub fn vec_add(&self, other: Self) -> Self {
        let mut data = [r32::default(); L];
        for i in 0..L {
            data[i] = self.data[i] + other.data[i];
        }
        Self { data }
    }
    ///Utility function for multiplying vectors - used to implement std::ops.
    pub fn vec_mul(&self, other: Self) -> Self {
        let mut data = [r32::default(); L];
        for i in 0..L {
            data[i] = self.data[i] * other.data[i];
        }
        Self { data }
    }
    ///Utility function for multiplying a vector by a rational - used to implement std::ops.
    pub fn rational_mul(&self, other: r32) -> Self {
        let mut data = [r32::default(); L];
        for i in 0..L {
            data[i] = self.data[i] * other;
        }
        Self { data }
    }
    ///Utility function for getting the reciprocal of the vector, component wise - used to implement std::ops.
    pub fn recip(&self) -> Self {
        let mut data = [r32::default(); L];
        for i in 0..L {
            data[i] = r32::identity()/self.data[i];
        }
        Self { data }
    }
    ///Returns the sum of the components of the vector.
    pub fn sum(&self) -> r32 {
        let mut sum = r32::default();
        for i in 0..L { sum += self[i] }
        sum
    }
    ///Returns the sum of the squared components of the vector, i.e. the squared magnitude of the vector.
    pub fn sq_sum(&self) -> r32 {
        let mut sum = r32::default();
        for i in 0..L { sum += self[i]*self[i] }
        sum
    }
    ///Returns the magnitude of the vector.
    pub fn mag(&self) -> r32 {
        self.sq_sum().sqroot()
    }
    ///Returns a normalised copy of the vector.
    pub fn normalised(&self) -> Self { *self/self.mag() }
}
impl<const L: usize> From<Vector<L>> for VectorPrecise<L> {
    fn from(v: Vector<L>) -> Self {
        let mut data = [r32::default(); L];
        for i in 0..L {
            data[i] = r32::from(v.data[i]);
        }
        Self::new(data)
    }
}
impl<const L: usize> From<VectorPrecise<L>> for Vector<L> {
    fn from(v: VectorPrecise<L>) -> Self {
        let mut data = [0.0; L];
        for i in 0..L {
            data[i] = f32::from(v.data[i]);
        }
        Self::new(data)
    }
}
impl<const L: usize> Display for VectorPrecise<L> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.data)
    }
}
impl<const L: usize> Default for VectorPrecise<L> {
    fn default() -> VectorPrecise<L> { VectorPrecise::<L>::new([r32::default(); L]) }
}
impl<const L: usize> Identity for VectorPrecise<L> {
    fn identity() -> VectorPrecise<L> { VectorPrecise::<L>::new([r32::identity(); L]) }
}
impl<const L: usize> Index<usize> for VectorPrecise<L> {
    type Output = r32;
    fn index(&self, index: usize) -> &r32 { &self.data[index] }
}
impl<const L: usize> IndexMut<usize> for VectorPrecise<L> {
    fn index_mut(&mut self, index: usize) -> &mut r32 { &mut self.data[index] }
}
impl<const L: usize> Add<VectorPrecise<L>> for VectorPrecise<L> {
    type Output = VectorPrecise<L>;
    fn add(self, other: VectorPrecise<L>) -> Self { self.vec_add(other) }
}

impl<const L: usize> Mul<VectorPrecise<L>> for VectorPrecise<L> {
    type Output = VectorPrecise<L>;
    fn mul(self, other: VectorPrecise<L>) -> Self { self.vec_mul(other) }
}

impl<const L: usize> Mul<r32> for VectorPrecise<L> {
    type Output = VectorPrecise<L>;
    fn mul(self, other: r32) -> Self { self.rational_mul(other) }
}

impl<const L: usize> Mul<VectorPrecise<L>> for r32 {
    type Output = VectorPrecise<L>;
    fn mul(self, other: VectorPrecise<L>) -> VectorPrecise<L> { other.rational_mul(self) }
}

impl<const L: usize> Div<r32> for VectorPrecise<L> {
    type Output = VectorPrecise<L>;
    fn div(self, other: r32) -> Self { self * (r32::identity()/other) }
}

impl<const L: usize> Sub<VectorPrecise<L>> for VectorPrecise<L> {
    type Output = VectorPrecise<L>;
    fn sub(self, other: VectorPrecise<L>) -> Self { self + (other*-r32::identity()) }
}

impl<const L: usize> Div<VectorPrecise<L>> for VectorPrecise<L> {
    type Output = VectorPrecise<L>;
    fn div(self, other: VectorPrecise<L>) -> Self { self * other.recip() }
}

///A basic graph type based on N-dimensional vectors - uses VectorPrecise for Hashable
///vector coordinates.
///Automatically uses distance between vectors for weighting.
///Uses running usize count of vectors for internal indexing but 
///these shouldn't be required for external interaction.
pub struct VectorGraph<const N: usize> {
    ///BiHashMap between usize indices and precise vectors.
    pub points: BiHashMap<usize, VectorPrecise<N>>,
    //HashMap between each index and the list of indices it's connected to.
    pub connections: HashMap<usize, Vec<usize>>,
    counter: usize,
    free: Vec<usize>,
}

impl<const N: usize> VectorGraph<N> {
    fn get_id(&mut self) -> usize {
        match self.free.pop() {
            Some(id) => id,
            None => { self.counter += 1; self.counter - 1 }
        }
    }
    ///Returns a new VectorGraph using vectors of length N.
    pub fn new() -> Self {
        Self {
            points: BiHashMap::new(),
            connections: HashMap::new(),
            counter: 0,
            free: Vec::new(),
        }
    }
    ///Inserts a vector point into the graph.
    pub fn insert(&mut self, point: Vector<N>) {
        let id = self.get_id();
        self.points.insert(id, VectorPrecise::from(point));
        self.connections.insert(id, Vec::new());
    }
    ///Removes a vector point from the graph.
    pub fn remove(&mut self, point: Vector<N>) -> bool {
        if let Some((id, _hv)) = self.points.remove_by_right(&VectorPrecise::from(point)) {
            self.free.push(id); return true;
        } else { return false; }
    }
    ///Creates a connection between two points in the graph. If bidir is true, this connection is
    ///bidirectional - otherwise, point A gets a one-way connection to point B. Returns true if the
    ///points were present in the graph to be connected, false otherwise.
    pub fn connect(&mut self, point_a: Vector<N>, point_b: Vector<N>, bidir: bool) -> bool {
        match (self.points.get_by_right(&VectorPrecise::from(point_a)), self.points.get_by_right(&VectorPrecise::from(point_b))) {
            (Some(a), Some(b)) => {
                match self.connections.get_mut(a) {
                    Some(al) => {
                        al.push(*b);
                    },
                    _ => {}
                }
                if bidir {
                    match self.connections.get_mut(b) {
                        Some(bl) => {
                            bl.push(*a);
                        },
                        _ => {}
                    }
                }
                true
            },
            _ => false,
        }
    }
    ///Disconnects two points in the graph. If bidir is true, this will remove any connections between
    ///points A and B, but otherwise it will only remove connections going from point A to point B. Returns
    ///true if the points were in the graph and a connection was removed.
    pub fn disconnect(&mut self, point_a: Vector<N>, point_b: Vector<N>, bidir: bool) -> bool {
        match (self.points.get_by_right(&VectorPrecise::from(point_a)), self.points.get_by_right(&VectorPrecise::from(point_b))) {
            (Some(a), Some(b)) => {
                let mut found = false;
                match self.connections.get_mut(a) {
                    Some(al) => {
                        match al.iter().position(|i| *i == *b) {
                            Some(i) => { al.remove(i); found = true; },
                            None => {},
                        }
                    },
                    _ => {}
                }
                if bidir {
                    match self.connections.get_mut(b) {
                        Some(bl) => {
                            match bl.iter().position(|i| *i == *a) {
                                Some(i) => { bl.remove(i); found = true; },
                                None => {},
                            }
                        },
                        _ => {}
                    }
                }
                found
            },
            _ => false,
        }
    }
    ///Returns an option containing a vector of all points connected to the point. This will return
    ///None if the point is not found in the graph, but will still return an option containing an empty vector
    ///if the point is found and simply has no connections.
    pub fn neighbours(&self, point: Vector<N>) -> Option<Vec<Vector<N>>> {
        match self.points.get_by_right(&VectorPrecise::from(point)) {
            Some(id) => {
                match self.connections.get(id) {
                    Some(points) => {
                        Some(points.iter()
                            .map(|i| self.points.get_by_left(i))
                            .filter(|o| o.is_some())
                            .map(|o| o.unwrap())
                            .map(|hv| Vector::from(*hv))
                            .collect::<Vec<_>>()    
                        )
                    },
                    None => Some(Vec::new())
                }
            },
            None => None,
        }
    }
    ///Returns a vector containing all points in the graph.
    pub fn points(&self) -> Vec<Vector<N>> {
        self.points.right_values().map(|hv| Vector::from(*hv)).collect::<Vec<_>>()
    }
    
}

impl VectorGraph<2> {
    ///Delaunay Triangulation - takes an array of 2 dimensional vectors, returns a connected graph
    ///generated using the Delaunay Triangulation.
    pub fn triangulate(points: &[Vector<2>]) -> Self {
        let mut vg = Self::new();

        let mut p = crate::Vector2::default();
        for point in points {   
            if point.mag() > p.mag() { p = *point; }
        }
        let a = p*2.0;
        let b = crate::geom::rotate(a, 2.0 * PI/3.0);
        let c = crate::geom::rotate(a, -2.0 * PI/3.0);
        let supra_tri = [a, b, c];

        let mut tris = vec![supra_tri];
        for point in points {
            let (mut good, mut bad) = (Vec::new(), Vec::new());
            for tri in tris {
                let circum_centre = crate::geom::circum_centre(tri[0], tri[1], tri[2]);
                let rad = (tri[0] - circum_centre).mag();
                let dist = (*point - circum_centre).mag();
                match dist < rad {
                    true => { bad.push(tri); }
                    false => { good.push(tri); }
                }
            }
            let mut edges = Vec::new();
            for tri in bad {
                let a = VectorPrecise::<2>::from(tri[0]);
                let b = VectorPrecise::<2>::from(tri[1]);
                let c = VectorPrecise::<2>::from(tri[2]);
                let mut is_in = (None, None, None);
                for (i, &edge) in edges.iter().enumerate() {
                    if edge == (a, b) || edge == (b, a) { is_in.0 = Some(i); }
                    if edge == (b, c) || edge == (c, b) { is_in.1 = Some(i); }
                    if edge == (c, a) || edge == (a, c) { is_in.2 = Some(i); }
                }
                let mut rem = Vec::new();
                match is_in.0 {
                    Some(index) => { rem.push(index); }
                    None => { edges.push((a, b)); }
                }
                match is_in.1 {
                    Some(index) => { rem.push(index); }
                    None => { edges.push((b, c)); }
                }
                match is_in.2 {
                    Some(index) => { rem.push(index); }
                    None => { edges.push((c, a)); }
                }
                rem.sort();
                for (i, j) in rem.iter().enumerate() {
                    edges.remove(j - i);
                }
            }
            for edge in edges {
                good.push([Vector::from(edge.0), Vector::from(edge.1), *point])
            }
            tris = good;
        }
        let mut points = HashSet::new();
        for tri in &tris {
            points.insert(VectorPrecise::<2>::from(tri[0]));
            points.insert(VectorPrecise::<2>::from(tri[1]));
            points.insert(VectorPrecise::<2>::from(tri[2]));
        }
        eprintln!("Found Supra Tri 0? {}", points.remove(&VectorPrecise::<2>::from(supra_tri[0])));
        eprintln!("Found Supra Tri 1? {}", points.remove(&VectorPrecise::<2>::from(supra_tri[1])));
        eprintln!("Found Supra Tri 2? {}", points.remove(&VectorPrecise::<2>::from(supra_tri[2])));

        for point in points { vg.insert(Vector::from(point)); }
        for tri in &tris {
            vg.connect(tri[0], tri[1], true);
            vg.connect(tri[1], tri[2], true);
            vg.connect(tri[2], tri[0], true);
        }

        vg
    }
}

