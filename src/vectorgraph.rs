use std::collections::HashMap;
use bimap::BiHashMap;
use crate::base::Vector;
use crate::hashvector::HashVector;

pub struct VectorGraph<const N: usize, const P: usize> {
    points: BiHashMap<usize, HashVector<N, P>>,
    connections: HashMap<usize, Vec<usize>>,
    counter: usize,
    free: Vec<usize>,
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
        self.points.remove_by_right(&HashVector::new(point)).is_some()
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
    pub fn triangulate(&mut self) {
        self.connections = HashMap::new();
        let mut av = Vector::<2>::default();
        let mut max = 0.0;
        for (_id, point) in &self.points {
            let v = point.as_vector();
            av = av + v;
        }
        av = av / self.points.len() as f32;
        for (_id, point) in &self.points {
            let v = point.as_vector();
            let m = (v - av).mag();
            if m > max { max = m; }
        }
        max += 1.0;
        let sup_tri = crate::geom::unit_geom::<3>(max);
        let mut tris = vec![sup_tri];
        for (_id, point) in &self.points {
            let mut new_tris = Vec::new();
            let mut bad_tris = Vec::new();
            let v = point.as_vector();
            for tri in &tris {
                let circum = crate::geom::circum_centre(tri[0], tri[1], tri[2]);
                let rad = (tri[0] - circum).mag();
                let dist = (v - circum).mag();
                match dist < rad {
                    true => { bad_tris.push(*tri); },
                    false => { new_tris.push(*tri); }
                }
            }
            for tri in &bad_tris {
                new_tris.push([tri[0], tri[1], v]);
                new_tris.push([tri[1], tri[2], v]);
                new_tris.push([tri[2], tri[3], v]);
                
            }
            tris = new_tris;
        }
        for tri in &tris {
            let mut n = 0;
            if tri[0] == sup_tri[0] || tri[0] == sup_tri[1] || tri[0] == sup_tri[2] { n += 1; }
            if tri[1] == sup_tri[0] || tri[1] == sup_tri[1] || tri[1] == sup_tri[2] { n += 1; }
            if tri[2] == sup_tri[0] || tri[2] == sup_tri[1] || tri[2] == sup_tri[2] { n += 1; }
            if n < 2 {
                let hva = HashVector::<2, P>::new(tri[0]);
                let hvb = HashVector::<2, P>::new(tri[1]);
                let hvc = HashVector::<2, P>::new(tri[2]);
                match (self.points.get_by_right(&hva), self.points.get_by_right(&hvb), self.points.get_by_right(&hvc)) {
                    (Some(a), Some(b), Some(c)) => {
                        if let Some(al) = self.connections.get_mut(a) { al.push(*b); al.push(*c); }
                        if let Some(bl) = self.connections.get_mut(a) { bl.push(*a); bl.push(*c); }
                        if let Some(cl) = self.connections.get_mut(a) { cl.push(*a); cl.push(*b); }
                    },
                    _ => {},
                }
            }
        }
    }
}