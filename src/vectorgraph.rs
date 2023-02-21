use std::collections::{HashMap, HashSet};
use std::f32::consts::PI;
use bimap::BiHashMap;
use crate::base::Vector;
use crate::hashvector::HashVector;

pub struct VectorGraph<const N: usize, const P: usize> {
    pub points: BiHashMap<usize, HashVector<N, P>>,
    pub connections: HashMap<usize, Vec<usize>>,
    pub counter: usize,
    pub free: Vec<usize>,
}

impl<const N: usize, const P: usize> VectorGraph<N, P> {
    pub fn get_id(&mut self) -> usize {
        match self.free.pop() {
            Some(id) => id,
            None => { self.counter += 1; self.counter - 1 }
        }
    }
    pub fn new() -> Self {
        Self {
            points: BiHashMap::new(),
            connections: HashMap::new(),
            counter: 0,
            free: Vec::new(),
        }
    }
    pub fn insert(&mut self, point: Vector<N>) {
        let id = self.get_id();
        self.points.insert(id, HashVector::new(point));
        self.connections.insert(id, Vec::new());
    }
    pub fn remove(&mut self, point: Vector<N>) -> bool {
        if let Some((id, _hv)) = self.points.remove_by_right(&HashVector::new(point)) {
            self.free.push(id); return true;
        } else { return false; }
    }
    pub fn connect(&mut self, point_a: Vector<N>, point_b: Vector<N>, bidir: bool) -> bool {
        match (self.points.get_by_right(&HashVector::new(point_a)), self.points.get_by_right(&HashVector::new(point_b))) {
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
    pub fn neighbours(&self, point: Vector<N>) -> Option<Vec<Vector<N>>> {
        match self.points.get_by_right(&HashVector::new(point)) {
            Some(id) => {
                match self.connections.get(id) {
                    Some(points) => {
                        Some(points.iter()
                            .map(|i| self.points.get_by_left(i))
                            .filter(|o| o.is_some())
                            .map(|o| o.unwrap())
                            .map(|hv| hv.as_vector())
                            .collect::<Vec<_>>()    
                        )
                    },
                    None => None,
                }
            },
            None => None,
        }
    }
    pub fn points(&self) -> Vec<Vector<N>> {
        self.points.right_values().map(|hv| hv.as_vector()).collect::<Vec<_>>()
    }
    
}

impl<const P: usize> VectorGraph<2, P> {
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
                let a = HashVector::<2, 5>::new(tri[0]);
                let b = HashVector::<2, 5>::new(tri[1]);
                let c = HashVector::<2, 5>::new(tri[2]);
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
                good.push([edge.0.as_vector(), edge.1.as_vector(), *point])
            }
            tris = good;
        }
        let mut points = HashSet::new();
        for tri in &tris {
            points.insert(HashVector::<2, 5>::new(tri[0]));
            points.insert(HashVector::<2, 5>::new(tri[1]));
            points.insert(HashVector::<2, 5>::new(tri[2]));
        }
        eprintln!("Found Supra Tri 0? {}", points.remove(&HashVector::<2, 5>::new(supra_tri[0])));
        eprintln!("Found Supra Tri 1? {}", points.remove(&HashVector::<2, 5>::new(supra_tri[1])));
        eprintln!("Found Supra Tri 2? {}", points.remove(&HashVector::<2, 5>::new(supra_tri[2])));

        for point in points { vg.insert(point.as_vector()); }
        for tri in &tris {
            vg.connect(tri[0], tri[1], true);
            vg.connect(tri[1], tri[2], true);
            vg.connect(tri[2], tri[0], true);
        }

        vg
    }
}
