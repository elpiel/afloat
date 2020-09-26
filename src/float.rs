use num::{bigint::Sign, BigInt};
use std::fmt;
use typenum::{NonZero, Unsigned};

use crate::{to_string::to_str_bytes, UFloat};
#[derive(Debug, Eq, PartialEq)]
pub struct Float<U: NonZero + Unsigned> {
    pub(crate) sign: Sign,
    pub(crate) float: UFloat<U>,
}

impl<U> Float<U>
where
    U: NonZero + Unsigned,
{
    pub fn new(value: BigInt) -> Self {
        let (sign, value) = value.into_parts();

        Self {
            sign,
            float: UFloat::<U>::new(value),
        }
    }
}

impl<U> fmt::Display for Float<U>
where
    U: NonZero + Unsigned,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let float_string =
            String::from_utf8(to_str_bytes(&self.float)).expect("Should never ever NEVER fails");

        let is_nonnegative = if let Sign::Plus = self.sign {
            true
        } else {
            false
        };

        f.pad_integral(is_nonnegative, "", &float_string)
    }
}
