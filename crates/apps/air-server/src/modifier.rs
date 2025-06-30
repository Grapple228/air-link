use std::ops::{BitAnd, BitOr, Not};

#[derive(Debug, Clone, Copy, Default)]
#[repr(u16)]
pub enum Modifier {
    #[default]
    None = 0,
    Shift = 1 << 0,     // 1
    AltGr = 1 << 1,     // 2
    Control = 1 << 2,   // 4
    Alt = 1 << 3,       // 8
    LShift = 1 << 4,    // 16
    RShift = 1 << 5,    // 32
    LControl = 1 << 6,  // 64
    RControl = 1 << 7,  // 128
    CapsShift = 1 << 8, // 256
}

impl Modifier {
    pub fn is_shift(&self) -> bool {
        self.contains(Modifier::Shift)
            || self.contains(Modifier::LShift)
            || self.contains(Modifier::RShift)
    }

    pub fn is_control(&self) -> bool {
        self.contains(Modifier::Control)
            || self.contains(Modifier::LControl)
            || self.contains(Modifier::RControl)
    }

    pub fn contains(self, other: Modifier) -> bool {
        (self as u16) & (other as u16) == (other as u16)
    }

    pub fn remove(&mut self, other: Modifier) {
        *self = Modifier::from_bits_truncate((self.clone() as u16) & !(other as u16));
    }

    pub fn set(&mut self, other: Modifier) {
        *self = Modifier::from_bits_truncate((self.clone() as u16) | (other as u16));
    }

    pub fn from_bits_truncate(bits: u16) -> Self {
        match bits {
            0 => Modifier::None,
            1 => Modifier::Shift,
            2 => Modifier::AltGr,
            4 => Modifier::Control,
            8 => Modifier::Alt,
            16 => Modifier::LShift,
            32 => Modifier::RShift,
            64 => Modifier::LControl,
            128 => Modifier::RControl,
            256 => Modifier::CapsShift,
            _ => Modifier::None, // или можно вернуть значение по умолчанию
        }
    }
}

impl BitOr for Modifier {
    type Output = u16;

    fn bitor(self, rhs: Self) -> Self::Output {
        (self as u16) | (rhs as u16)
    }
}

impl BitAnd for Modifier {
    type Output = u16;

    fn bitand(self, rhs: Self) -> Self::Output {
        (self as u16) & (rhs as u16)
    }
}

impl Not for Modifier {
    type Output = u16;

    fn not(self) -> Self::Output {
        !(self as u16)
    }
}
