use std::convert::TryInto;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, Div, Mul, Shl, Sub};

#[derive(Copy, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct FixedInt10 {
    value: i32,
}

impl Debug for FixedInt10 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}f10", self.value)
    }
}

impl FixedInt10 {
    // To prepare for a potential macro / generic
    fn exponent() -> i32 {
        10
    }
    fn multiplier() -> i32 {
        2_i32.pow(FixedInt10::exponent().try_into().unwrap())
    }
}

impl Clone for FixedInt10 {
    fn clone(&self) -> Self {
        Self { value: self.value }
    }
}

impl Into<i32> for FixedInt10 {
    fn into(self) -> i32 {
        self.value >> FixedInt10::exponent()
    }
}

impl Into<f32> for FixedInt10 {
    fn into(self) -> f32 {
        (self.value >> FixedInt10::exponent()) as f32
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
        assert_eq!(20, (origin / 10).into());
        assert_eq!(2000, (origin * 10).into());
        assert_eq!(210, (origin + 10).into());
        assert_eq!(190, (origin - 10).into());
    }

    #[test]
    fn fixedpoint_div() {
        let origin = FixedInt10::from(0.5);
        assert_eq!(0.25_f32, (origin / 2).into());
        assert_eq!(1, (origin * 2).into());
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
}
