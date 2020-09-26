use num::BigUint;
use std::{fmt, marker::PhantomData, ops::Mul};
use typenum::{IsLessOrEqual, NonZero, Unsigned};

use crate::to_str_bytes;

#[derive(Debug, Eq, PartialEq)]
pub struct UFloat<U: NonZero + Unsigned> {
    pub(crate) value: BigUint,
    pub(crate) precision: PhantomData<U>,
}

impl<U> UFloat<U>
where
    U: NonZero + Unsigned,
{
    pub fn new(value: BigUint) -> Self {
        Self {
            value,
            precision: PhantomData::<U>,
        }
    }
}

impl<U, LE> std::ops::Sub<UFloat<LE>> for UFloat<U>
where
    U: NonZero + Unsigned,
    LE: NonZero + Unsigned + IsLessOrEqual<U>,
{
    type Output = UFloat<U>;

    fn sub(self, rhs: UFloat<LE>) -> Self::Output {
        let value = self.value - rhs.value.mul(BigUint::from(10_u16).pow(U::to_u32() - LE::to_u32()));

        Self {
            value,
            precision: self.precision,
        }
    }
}

// impl<U> std::ops::Sub<UFloat<U>> for UFloat<U>
// where
//     U: NonZero + Unsigned,
// {
//     type Output = UFloat<U>;

//     fn sub(self, rhs: UFloat<U>) -> Self::Output {
//         Self {
//             value: self.value.sub(rhs.value),
//             precision: self.precision,
//         }
//     }
// }

impl<U> fmt::Display for UFloat<U>
where
    U: NonZero + Unsigned,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let float_string =
            String::from_utf8(to_str_bytes(self)).expect("Should never ever NEVER fails");

        f.pad_integral(true, "", &float_string)
    }
}
