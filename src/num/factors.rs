const FAC_ITER: usize = 100;

///Returns the greatest common denominator of two u32s.
pub fn gcd32(a: u32, b: u32) -> u32 {
    let (mut a0, mut b0) = (a, b);
    while a0 != 0 {
        let a1 = b0 % a0;
        b0 = a0;
        a0 = a1;    
    }
    return b0;
}
///Returns the greatest common denominator of two u64s.
pub fn gcd64(a: u64, b: u64) -> u64 {
    let (mut a0, mut b0) = (a, b);
    while a0 != 0 {
        let a1 = b0 % a0;
        b0 = a0;
        a0 = a1;    
    }
    return b0;
}
///Returns the lowest common multiple of two u32s.
pub fn lcm32(a: u32, b: u32) -> u32 { (a * b) / gcd32(a, b) }
///Returns the lowest common multiple of two u64s.
pub fn lcm64(a: u64, b: u64) -> u64 { (a * b) / gcd64(a, b) }
///Returns a list of prime factors for a u32.
pub fn fac32(n: u32) -> Vec<u32> {
    let mut v = vec!(n);
    let mut running = true;
    while running {
        running = false;
        let mut v0 = Vec::new();
        for n0 in &v {
            match prho32(*n0) {
                (0, 0) => { v0.push(*n0); },
                (_, 1) => { v0.push(*n0); },
                (1, _) => { v0.push(*n0); },
                (a, b) => { v0.push(a); v0.push(b); running = true; }
            }
        }
        v = v0;
    }
    v
}
///Pollard Rho algorithm for a u32 - returns a non-trivial factor of a u32 and the original number
///with the factor removed.
pub fn prho32(n: u32) -> (u32, u32) {
    let (mut x, mut y, mut d);
    let (mut count, mut attempt) = (0, 0);
    while count < FAC_ITER {
        count += 1;
        if n % 2 == 0 { return (2, n/2); }
        else if n % 3 == 0 { return (3, n/3); }
        else if n % 5 == 0 { return (5, n/5); }
        else {
            x = 2_i32;
            y = 2_i32;
            d = 1_i32;
            while d == 1 {
                x = (x*x + attempt) % n as i32;
                y = (y*y + attempt) % n as i32;
                y = (y*y + attempt) % n as i32;
                d = gcd32((x - y).abs() as u32, n) as i32;
            }
            if d != n as i32 { return (d as u32, n/d as u32); }
            else { attempt += 1; }
        }
    }
    (0, 0)
}
///Returns a list of prime factors for a u64.
pub fn fac64(n: u64) -> Vec<u64> {
    let mut v = vec!(n);
    let mut running = true;
    while running {
        running = false;
        let mut v0 = Vec::new();
        for n0 in &v {
            match prho64(*n0) {
                (0, 0) => { v0.push(*n0); },
                (_, 1) => { v0.push(*n0); },
                (1, _) => { v0.push(*n0); },
                (a, b) => { v0.push(a); v0.push(b); running = true; }
            }
        }
        v = v0;
    }
    v
}
///Pollard Rho algorithm for a u64 - returns a non-trivial factor of a u64 and the original number
///with the factor removed.
pub fn prho64(n: u64) -> (u64, u64) {
    let (mut x, mut y, mut d);
    let (mut count, mut attempt) = (0, 0);
    while count < FAC_ITER {
        count += 1;
        if n % 2 == 0 { return (2, n/2); }
        else if n % 3 == 0 { return (3, n/3); }
        else if n % 5 == 0 { return (5, n/5); }
        else {
            x = 2_i64;
            y = 2_i64;
            d = 1_i64;
            while d == 1 {
                x = (x*x + attempt) % n as i64;
                y = (y*y + attempt) % n as i64;
                y = (y*y + attempt) % n as i64;
                d = gcd64((x - y).abs() as u64, n) as i64;
            }
            if d != n as i64 { return (d as u64, n/d as u64); }
            else { attempt += 1; }
        }
    }
    (0, 0)
}

//Returns a vector of square factors for a u32 - each item is a tuple of the unsquared factor and the squared factor.
pub fn sqfac32(n: u32) -> Vec<(u32, u32)> {
    let f = fac32(n);
    let mut v = Vec::new();
    for fac in f {
        let mut found = false;
        for (fac0, n) in &mut v {
            if fac == *fac0 { *n += 1; found = true; }
        }
        if !found { v.push((fac, 1)); }
    }
    v.iter().filter(|(_f, n)| n % 2 == 0).map(|(f, n)| (f.pow(*n/2), f.pow(*n))).collect::<Vec<_>>()
}

//Returns a vector of square factors for a u64 - each item is a tuple of the unsquared factor and the squared factor.
pub fn sqfac64(n: u64) -> Vec<(u64, u64)> {
    let f = fac64(n);
    let mut v = Vec::new();
    for fac in f {
        let mut found = false;
        for (fac0, n) in &mut v {
            if fac == *fac0 { *n += 1; found = true; }
        }
        if !found { v.push((fac, 1)); }
    }
    v.iter().filter(|(_f, n)| n % 2 == 0).map(|(f, n)| (f.pow(*n/2), f.pow(*n))).collect::<Vec<_>>()
}