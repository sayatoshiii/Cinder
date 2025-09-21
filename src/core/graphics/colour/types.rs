#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Colour {
    pub fn bitwise<T: From<u32>>(&self) -> T {
        T::from(self.b as u32 | (self.g as u32) << 8 | (self.r as u32) << 16)
    }

    pub fn from_hex(hex: impl Into<String>) -> Self {
        let mut hex = hex.into();
        hex.retain(|c| c != '#');

        if hex.len() != 6 {
            panic!("Invalid Hex Colour");
        }

        let r = u8::from_str_radix(&hex[0..2], 16).expect("Invalid Red Hex");
        let g = u8::from_str_radix(&hex[2..4], 16).expect("Invalid Green Hex");
        let b = u8::from_str_radix(&hex[4..6], 16).expect("Invalid Blue Hex");

        Self { r, g, b }
    }
}
