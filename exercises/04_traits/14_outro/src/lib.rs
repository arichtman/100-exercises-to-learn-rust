// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

use std::ops::Add;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SaturatingU16 {
    value: u16,
}

impl SaturatingU16 {
    fn new(value: u16) -> Self {
        SaturatingU16 { value }
    }
}

impl Add<SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, addor: Self) -> Self::Output {
        SaturatingU16 {
            value: (self.value.saturating_add(addor.value)),
        }
    }
}
impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, addor: &Self) -> Self::Output {
        SaturatingU16 {
            value: (self.value.saturating_add(addor.value)),
        }
    }
}
impl Add<u16> for SaturatingU16 {
    type Output = u16;
    fn add(self, addor: u16) -> Self::Output {
        self.value.saturating_add(addor.into())
    }
}

impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        Self::new(value.into())
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> Self {
        Self::new(*value as u16)
    }
}

impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        Self::new(value)
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> Self {
        Self::new(*value)
    }
}

// hoo boy am I not ready for this advaced shennanigans
// impl<T: Into<u16> + for<'a> Into<&'a u16>> From<T> for SaturatingU16 {
//     fn from(value: T) -> Self {
//         Self::new(value.into())
//     }
// }
