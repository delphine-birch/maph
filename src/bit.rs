pub struct Byte(u8);

pub fn sesp(f: f32) -> (bool, u32, u32) {
    eprintln!("{:b}", f.to_bits());
    let raw_bytes = f.to_be_bytes();
    let mut bytes = [
        Byte(raw_bytes[0]), Byte(raw_bytes[1]),
        Byte(raw_bytes[2]), Byte(raw_bytes[3])
    ];
    let sign = !bytes[0].get_bit(7);
    let mut exp = Byte(bytes[0].0 << 1);
    if bytes[1].get_bit(7) { exp.0 += 1; }

    bytes[1].set_bit(7, false);
    let sp = u32::from_be_bytes([0, bytes[1].0, bytes[2].0, bytes[3].0]);

    (sign, exp.0 as u32, sp)
}

impl Byte {
    pub fn new(byte: u8) -> Self { Self(byte) }
    pub fn flags(flags: Vec<usize>) -> Self {
        flags.iter().fold(Self(0), |acc, flag| acc + Self::bitmask(*flag))
    }
    pub fn bitmask(index: usize) -> Self {
        Self(1 << index)
    }
    pub fn get_bit(&self, index: usize) -> bool {
        (self.0 & Self::bitmask(index).0).count_ones() > 0
    }
    pub fn set_bit(&mut self, index: usize, val: bool) {
        match val {
            true => self.0 = self.0 | Self::bitmask(index).0,
            false => self.0 = self.0 & !Self::bitmask(index).0
        }
    }
    pub fn toggle_bit(&mut self, index: usize) {
        self.0 = self.0 ^ Self::bitmask(index).0
    }
    pub fn clear(&mut self) {
        self.0 = self.0 & 0;
    }
}

impl Byte {
    pub fn from_u16_ne(from: u16) -> [Self; 2] {
        let bytes = from.to_ne_bytes();
        [Byte(bytes[0]), Byte(bytes[1])]
    }
    pub fn from_u32_ne(from: u32) -> [Self; 4] {
        let bytes = from.to_ne_bytes();
        [Byte(bytes[0]), Byte(bytes[1]),
        Byte(bytes[2]), Byte(bytes[3])]
    }
    pub fn from_u64_ne(from: u64) -> [Self; 8] {
        let bytes = from.to_ne_bytes();
        [Byte(bytes[0]), Byte(bytes[1]),
        Byte(bytes[2]), Byte(bytes[3]),
        Byte(bytes[4]), Byte(bytes[5]),
        Byte(bytes[6]), Byte(bytes[7])]
    }
    pub fn from_u128_ne(from: u128) -> [Self; 16] {
        let bytes = from.to_ne_bytes();
        [Byte(bytes[0]), Byte(bytes[1]),
        Byte(bytes[2]), Byte(bytes[3]),
        Byte(bytes[4]), Byte(bytes[5]),
        Byte(bytes[6]), Byte(bytes[7]),
        Byte(bytes[8]), Byte(bytes[9]),
        Byte(bytes[10]), Byte(bytes[11]),
        Byte(bytes[12]), Byte(bytes[13]),
        Byte(bytes[14]), Byte(bytes[15])]
    }
    pub fn from_u16_le(from: u16) -> [Self; 2] {
        let bytes = from.to_le_bytes();
        [Byte(bytes[0]), Byte(bytes[1])]
    }
    pub fn from_u32_le(from: u32) -> [Self; 4] {
        let bytes = from.to_le_bytes();
        [Byte(bytes[0]), Byte(bytes[1]),
        Byte(bytes[2]), Byte(bytes[3])]
    }
    pub fn from_u64_le(from: u64) -> [Self; 8] {
        let bytes = from.to_le_bytes();
        [Byte(bytes[0]), Byte(bytes[1]),
        Byte(bytes[2]), Byte(bytes[3]),
        Byte(bytes[4]), Byte(bytes[5]),
        Byte(bytes[6]), Byte(bytes[7])]
    }
    pub fn from_u128_le(from: u128) -> [Self; 16] {
        let bytes = from.to_le_bytes();
        [Byte(bytes[0]), Byte(bytes[1]),
        Byte(bytes[2]), Byte(bytes[3]),
        Byte(bytes[4]), Byte(bytes[5]),
        Byte(bytes[6]), Byte(bytes[7]),
        Byte(bytes[8]), Byte(bytes[9]),
        Byte(bytes[10]), Byte(bytes[11]),
        Byte(bytes[12]), Byte(bytes[13]),
        Byte(bytes[14]), Byte(bytes[15])]
    }
    pub fn from_u16_be(from: u16) -> [Self; 2] {
        let bytes = from.to_be_bytes();
        [Byte(bytes[0]), Byte(bytes[1])]
    }
    pub fn from_u32_be(from: u32) -> [Self; 4] {
        let bytes = from.to_be_bytes();
        [Byte(bytes[0]), Byte(bytes[1]),
        Byte(bytes[2]), Byte(bytes[3])]
    }
    pub fn from_u64_be(from: u64) -> [Self; 8] {
        let bytes = from.to_be_bytes();
        [Byte(bytes[0]), Byte(bytes[1]),
        Byte(bytes[2]), Byte(bytes[3]),
        Byte(bytes[4]), Byte(bytes[5]),
        Byte(bytes[6]), Byte(bytes[7])]
    }
    pub fn from_u128_be(from: u128) -> [Self; 16] {
        let bytes = from.to_be_bytes();
        [Byte(bytes[0]), Byte(bytes[1]),
        Byte(bytes[2]), Byte(bytes[3]),
        Byte(bytes[4]), Byte(bytes[5]),
        Byte(bytes[6]), Byte(bytes[7]),
        Byte(bytes[8]), Byte(bytes[9]),
        Byte(bytes[10]), Byte(bytes[11]),
        Byte(bytes[12]), Byte(bytes[13]),
        Byte(bytes[14]), Byte(bytes[15])]
    }
}

impl std::ops::BitOr for Byte {
    type Output = Byte;
    fn bitor(self, other: Byte) -> Byte {
        Byte(self.0 | other.0)
    }
}
impl std::ops::BitAnd for Byte {
    type Output = Byte;
    fn bitand(self, other: Byte) -> Byte {
        Byte(self.0 & other.0)
    }
}
impl std::ops::Not for Byte {
    type Output = Byte;
    fn not(self) -> Byte {
        Byte(!self.0)
    }
}
impl std::ops::Add for Byte {
    type Output = Byte;
    fn add(self, other: Byte) -> Byte { self | other }
}
