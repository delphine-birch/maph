pub trait ModuloAdd<T, Output> {
    fn mod_add(&self, other: T, boundaries: (T, T)) -> Output;
    fn mod_add_eq(&mut self, other: T, boundaries: (T, T));
}

impl<T> ModuloAdd<T, T> for T
where T : Copy + std::ops::Add<T, Output=T> + std::ops::Sub<T, Output=T> + PartialOrd {
    fn mod_add(&self, other: T, boundaries: (T, T)) -> T {
        let mut n = *self + other;
        while n > boundaries.1 { n = n - (boundaries.1 - boundaries.0); }
        while n < boundaries.0 { n = n + (boundaries.1 - boundaries.0); }
        n
    }
    fn mod_add_eq(&mut self, other: T, boundaries: (T, T)) {
        *self = self.mod_add(other, boundaries);
    }
}