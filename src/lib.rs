use num::BigUint;
use std::{fmt, marker::PhantomData};
use typenum::{NonZero, Unsigned};

#[derive(Debug, Eq, PartialEq)]
pub struct Float<U: NonZero + Unsigned> {
    value: BigUint,
    precision: PhantomData<U>
}

impl<U: NonZero + Unsigned> Float<U> {
    pub fn new(value: BigUint) -> Self {
        Self {
            value,
            precision: PhantomData::<U>
        }
    }
}

impl<U: NonZero + Unsigned> std::ops::Sub<Float<U>> for Float<U> {
    type Output = Float<U>;

    fn sub(self, rhs: Float<U>) -> Self::Output {
        Self {
            value: self.value.sub(rhs.value),
            precision: self.precision,
        }
    }
}

impl<U: NonZero + Unsigned> fmt::Display for Float<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let float_string =
            String::from_utf8(to_str_bytes(self)).expect("Should never ever NEVER fails");

        f.pad_integral(true, "", &float_string)
    }
}

pub(crate) fn to_str_bytes<U: NonZero + Unsigned>(float: &Float<U>) -> Vec<u8> {
    let res = float.value.to_radix_le(10);

    let map_to_ascii = |mut r: u8| {
        if r < 10 {
            r += b'0';
        } else {
            r += b'a' - 10;
        }

        r
    };
    let precision = U::to_usize();

    let mut float_ascii = if precision >= res.len() {
        let zeros_after_comma = precision - res.len();

        res.into_iter()
            .map(map_to_ascii)
            .chain(vec![map_to_ascii(0); zeros_after_comma].into_iter())
            .chain(".0".as_bytes().to_vec())
            .collect()
    } else {
        let float_ascii = res
            .into_iter()
            .enumerate()
            .fold(Vec::new(), |mut acc, (index, r)| {
                // are we at the point where we need the decimal point?
                if index == precision {
                    acc.extend(".".as_bytes());
                }

                let ascii_num = map_to_ascii(r);
                acc.push(ascii_num);

                acc
            });

        float_ascii
    };

    float_ascii.reverse();

    float_ascii
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_float() {
        use typenum::{U12, U2, U20};
        let float = Float::<U12>::new(321_999_999_999_999_u64.into());
        assert_eq!("321.999999999999", float.to_string());

        let float = Float::<U2>::new(20_u16.into());
        assert_eq!("0.20", &float.to_string());

        let float = Float::<U20>::new(20_u16.into());
        assert_eq!("0.00000000000000000020", &float.to_string());

        let float = Float::<U20>::new(321_999_999_999_999_u64.into());
        assert_eq!("0.00000321999999999999", float.to_string());
    }

    #[test]
    fn test_float_subtraction() {
        use typenum::U12;

        let lhs_float = Float::<U12>::new(321_999_999_999_999_u64.into());
        let rhs_float = Float::<U12>::new(900_000_000_000_u64.into());

        let actual_result =lhs_float - rhs_float;
        let expected = Float::<U12>::new(321_099_999_999_999_u64.into());

        assert_eq!(&expected, &actual_result);
        assert_eq!("321.099999999999", actual_result.to_string());
    }
}
