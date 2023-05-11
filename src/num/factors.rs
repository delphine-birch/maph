const FAC_ITER: usize = 100;

pub fn rat_approx_32(f: f32, max: f32) -> (f32, f32) {
    let frac = f.fract();
    let int = f - frac;
    if int > max || int < -max { panic!("Tried to convert an f32 out of bounds for r32, overflowed."); }
    if frac <= 0.0 {
        return (int, 1.0);
    }
    let dmax = max/int;
    assert!(0.0 < frac && frac < 1.0);
    let (mut nl, mut dl) : (f32, f32) = (0.0, 1.0);
    let (mut nr, mut dr) : (f32, f32) = (1.0, 1.0);
    let (mut nm, mut dm) : (i64, i64) = (0, 1);
    let mut loc = None;
    while (dl <= dmax) && (dr <= dmax) {
        let ntmp = nl - frac * dl;
        let dtmp = frac * dr - nr;
        let mut br = (ntmp/dtmp).floor() as u32;
        let mut bl = (dtmp/ntmp).floor() as u32;
        let mut side = 0;
        if bl == 0 { bl = 1; side = -1; }
        if br == 0 { br = 1; side = 1; }
        dm = dl as i64 * bl as i64 + dr as i64 * br as i64;
        
        if dm > dmax as i64 {
            if side == -1 { br = 1.max(((dmax - dl) / dr).floor() as u32); }
            else if side == 1 { bl = 1.max(((dmax - dr) / dl).floor() as u32); }
        }

        nm = nl as i64 * bl as i64 + nr as i64 * br as i64;
        dm = dl as i64 * bl as i64 + dr as i64 * br as i64;
        if dm > dmax as i64 { break; }
        let med = (nm as f32)/(dm as f32);

        if frac == med {
            loc = Some(0);
            break;
        } else if frac < med {
            loc = Some(-1);
            (nr, dr) = (nm as f32, dm as f32);
        } else if frac > med {
            loc = Some(1);
            (nl, dl) = (nm as f32, dm as f32);
        }
    }

    let (mut n, d);

    match loc {
        Some(0) => {
            (n, d) = (nm, dm);
        },
        _ => {
            let errl = (frac - (nl as f32/dl as f32)).abs();
            let errr = (frac - (nr as f32 / dr as f32)).abs();
            if errl <= errr {
                (n, d) = (nl as i64, dl as i64);
            } else {
                (n, d) = (nr as i64, dr as i64);
            }
        },
    }



    n += int as i64 * d as i64;
    (n as f32, d as f32)
}

pub fn rat_approx_64(f: f64, max: f64) -> (f64, f64) {
    let frac = f.fract();
    let int = f - frac;
    if int > max || int < -max { panic!("Tried to convert an f32 out of bounds for r32, overflowed."); }
    if frac <= 0.0 {
        return (int, 1.0);
    }
    let dmax = max/int;
    assert!(0.0 < frac && frac < 1.0);
    let (mut nl, mut dl) : (f64, f64) = (0.0, 1.0);
    let (mut nr, mut dr) : (f64, f64) = (1.0, 1.0);
    let (mut nm, mut dm) : (i128, i128) = (0, 1);
    let mut loc = None;
    while (dl <= dmax) && (dr <= dmax) {
        let ntmp = nl - frac * dl;
        let dtmp = frac * dr - nr;
        let mut br = (ntmp/dtmp).floor() as u32;
        let mut bl = (dtmp/ntmp).floor() as u32;
        let mut side = 0;
        if bl == 0 { bl = 1; side = -1; }
        if br == 0 { br = 1; side = 1; }
        dm = dl as i128 * bl as i128 + dr as i128 * br as i128;
        
        if dm > dmax as i128 {
            if side == -1 { br = 1.max(((dmax - dl) / dr).floor() as u32); }
            else if side == 1 { bl = 1.max(((dmax - dr) / dl).floor() as u32); }
        }

        nm = nl as i128 * bl as i128 + nr as i128 * br as i128;
        dm = dl as i128 * bl as i128 + dr as i128 * br as i128;
        if dm > dmax as i128 { break; }
        let med = (nm as f64)/(dm as f64);

        if frac == med {
            loc = Some(0);
            break;
        } else if frac < med {
            loc = Some(-1);
            (nr, dr) = (nm as f64, dm as f64);
        } else if frac > med {
            loc = Some(1);
            (nl, dl) = (nm as f64, dm as f64);
        }
    }

    let (mut n, d);

    match loc {
        Some(0) => {
            (n, d) = (nm, dm);
        },
        _ => {
            let errl = (frac - (nl as f64/dl as f64)).abs();
            let errr = (frac - (nr as f64 / dr as f64)).abs();
            if errl <= errr {
                (n, d) = (nl as i128, dl as i128);
            } else {
                (n, d) = (nr as i128, dr as i128);
            }
        },
    }



    n += int as i128 * d as i128;
    (n as f64, d as f64)
}

///Returns the greatest common denominator of two u16s.
pub fn gcd16(a: u16, b: u16) -> u16 {
    let (mut a0, mut b0) = (a, b);
    while a0 != 0 {
        let a1 = b0 % a0;
        b0 = a0;
        a0 = a1;    
    }
    return b0;
}
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
pub fn lcm16(a: u16, b: u16) -> u16 { (a * b) / gcd16(a, b) }
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