use std::convert::TryInto;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, BitAnd, Div, Mul, Shl, Shr, Sub};

#[derive(Copy, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct FixedInt10 {
    value: i32,
}

impl FixedInt10 {
    // To prepare for a potential macro / generic
    fn exponent() -> i32 {
        10
    }
    fn multiplier() -> i32 {
        2_i32.pow(FixedInt10::exponent().try_into().unwrap())
    }

    // zero out the non-integer part
    pub fn floor(self) -> FixedInt10 {
        FixedInt10 {
            value: self.value & ((!0_i32) ^ (FixedInt10::multiplier() - 1)),
        }
    }

    // zero out the bits of the integer part
    pub fn fract(self) -> FixedInt10 {
        FixedInt10 {
            value: self.value & (FixedInt10::multiplier() - 1),
        }
    }
}

impl Debug for FixedInt10 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} f10",
            (self.value as f32) / (Self::multiplier() as f32)
        )
    }
}

impl Clone for FixedInt10 {
    fn clone(&self) -> Self {
        Self { value: self.value }
    }
}

impl From<FixedInt10> for i32 {
    fn from(val: FixedInt10) -> i32 {
        val.value >> FixedInt10::exponent()
    }
}

impl From<FixedInt10> for usize {
    fn from(val: FixedInt10) -> usize {
        (val.value as u32 >> FixedInt10::exponent() as u32) as usize
    }
}

impl From<FixedInt10> for u8 {
    fn from(val: FixedInt10) -> u8 {
        (val.value >> FixedInt10::exponent()) as u8
    }
}


impl From<FixedInt10> for f32 {
    fn from(val: FixedInt10) -> f32 {
        val.value as f32 / FixedInt10::multiplier() as f32
    }
}

impl From<i32> for FixedInt10 {
    fn from(other: i32) -> Self {
        Self {
            value: other << FixedInt10::exponent(),
        }
    }
}

impl From<u8> for FixedInt10 {
    fn from(other: u8) -> Self {
        Self {
            value: (other as i32) << FixedInt10::exponent(),
        }
    }
}

impl From<f32> for FixedInt10 {
    fn from(other: f32) -> Self {
        let brought_to_power = other * FixedInt10::multiplier() as f32;
        Self {
            value: brought_to_power as i32,
        }
    }
}

impl Shl<i32> for FixedInt10 {
    type Output = FixedInt10;

    fn shl(self, rhs: i32) -> Self::Output {
        Self {
            value: self.value << rhs,
        }
    }
}

impl Shr<i32> for FixedInt10 {
    type Output = FixedInt10;

    fn shr(self, rhs: i32) -> Self::Output {
        Self {
            value: self.value >> rhs,
        }
    }
}

impl BitAnd<i32> for FixedInt10 {
    type Output = FixedInt10;

    fn bitand(self, rhs: i32) -> Self::Output {
        Self {
            value: self.value & (rhs << FixedInt10::exponent()),
        }
    }
}

impl Div<i32> for FixedInt10 {
    type Output = FixedInt10;

    fn div(self, rhs: i32) -> Self::Output {
        Self {
            value: self.value / rhs,
        }
    }
}

impl Add for FixedInt10 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value + rhs.value,
        }
    }
}

impl Add<i32> for FixedInt10 {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        Self {
            value: self.value + (rhs << FixedInt10::exponent()),
        }
    }
}

impl Sub for FixedInt10 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value - rhs.value,
        }
    }
}

impl Sub<i32> for FixedInt10 {
    type Output = Self;

    fn sub(self, rhs: i32) -> Self::Output {
        Self {
            value: self.value - (rhs << FixedInt10::exponent()),
        }
    }
}

impl Mul<FixedInt10> for FixedInt10 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            value: (self.value * rhs.value) >> FixedInt10::exponent(),
        }
    }
}

impl Mul<u8> for FixedInt10 {
    type Output = Self;

    fn mul(self, rhs: u8) -> Self::Output {
        Self {
            value: (self.value * rhs as i32),
        }
    }
}

impl Mul<i32> for FixedInt10 {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            value: self.value * rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::fixed_int::FixedInt10;

    #[test]
    fn same_type() {
        let i_origin = 3;
        let f_origin = 3.;

        assert_eq!(i_origin, FixedInt10::from(i_origin).into());
        assert_eq!(f_origin, FixedInt10::from(f_origin).into());
    }

    #[test]
    fn across_types() {
        let i_origin = 3;
        let f_origin = 3.;

        assert_eq!(f_origin, FixedInt10::from(i_origin).into());
        assert_eq!(i_origin, FixedInt10::from(f_origin).into());
    }

    #[test]
    fn operations() {
        let origin = FixedInt10::from(200);
        assert_eq!(20, (origin / 10_i32).into());
        assert_eq!(2000, (origin * 10_i32).into());
        assert_eq!(210, (origin + 10_i32).into());
        assert_eq!(190, (origin - 10).into());
        assert_eq!(origin, (origin - 10) + 10);
        assert_eq!(origin, (origin / 10) * 10);

        let simple = FixedInt10::from(1);
        assert_eq!(2, (simple << 1).into());
        assert_eq!(4, (simple << 2).into());
        assert_eq!(simple, (simple << 2) >> 2);

        let simple = FixedInt10::from(1_u8);
        assert_eq!(8, (simple * 8_u8).into())
    }

    #[test]
    fn fixedpoint_div() {
        let origin = FixedInt10::from(0.5);
        assert_eq!(0.25_f32, (origin / 2).into());
        assert_eq!(1, (origin * 2_u8).into());
    }

    #[test]
    fn equality() {
        let origin = FixedInt10::from(200);
        assert_eq!(FixedInt10::from(220), origin + 20);
        assert_eq!(FixedInt10::from(220), 220.into());
        assert_ne!(FixedInt10::from(220), 200.into());
        assert_ne!(FixedInt10::from(0.), 1.into());
        assert_eq!(FixedInt10::from(0.), FixedInt10::from(1.) - 1);
    }

    #[test]
    fn bits_manipulation() {
        let origin = FixedInt10::from(2.5f32);
        assert_eq!(2, origin.floor().into());
        assert_eq!(0.5f32, origin.fract().into());
        assert_eq!(FixedInt10::from(2), origin.floor());
        assert_eq!(FixedInt10::from(0.5), origin.fract());
        assert_eq!(FixedInt10::from(5), origin << 1);
        assert_eq!(FixedInt10::from(1.25), origin >> 1);
        assert_eq!(FixedInt10::from(2), origin & 2);
    }
}
