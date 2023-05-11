fn main() {
    let mut rats = [maph::num::rational::r64::new(1, 1).unwrap(); 1000];
    let mut rats0 = [maph::num::rational::r64::new(1, 1).unwrap(); 1000];
    loop {
        for i in 1..1000 {
            let ar = maph::num::rational::r64::new_unchecked(i, i + 1);
            let br = maph::num::rational::r64::new_unchecked(i + 2, i + 3);
            rats[i as usize] = ar - br; rats0[i as usize] = br / ar;
        }
    }
}