mod float;
mod ufloat;

pub use float::Float;
pub use ufloat::UFloat;

pub(crate) use to_string::to_str_bytes;

mod to_string {
    use crate::UFloat;
    use typenum::{NonZero, Unsigned};

    pub(crate) fn to_str_bytes<U: NonZero + Unsigned>(ufloat: &UFloat<U>) -> Vec<u8> {
        let res = ufloat.value.to_radix_le(10);

        let map_to_ascii = |mut r: u8| {
            if r < 10 {
                r += b'0';
            } else {
                r += b'a' - 10;
            }

            r
        };
        let precision = U::to_usize();

        // when the whole value is contained after the decimal point
        let mut float_ascii = if precision >= res.len() {
            let zeros_after_comma = precision - res.len();

            res.into_iter()
                .map(map_to_ascii)
                .chain(vec![map_to_ascii(0); zeros_after_comma].into_iter())
                .chain(".0".as_bytes().to_vec())
                .collect()
        } else {
            let float_ascii =
                res.into_iter()
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
}
#[cfg(test)]
mod test {
    use num::BigInt;

    use super::*;

    #[test]
    fn test_ufloat() {
        use typenum::{U12, U2, U20};
        let float = UFloat::<U12>::new(321_999_999_999_999_u64.into());
        assert_eq!("321.999999999999", float.to_string());

        let float = UFloat::<U2>::new(20_u16.into());
        assert_eq!("0.20", &float.to_string());

        let float = UFloat::<U20>::new(20_u16.into());
        assert_eq!("0.00000000000000000020", &float.to_string());

        let float = UFloat::<U20>::new(321_999_999_999_999_u64.into());
        assert_eq!("0.00000321999999999999", float.to_string());
    }

    #[test]
    fn test_float() {
        use typenum::{U12, U20};
        let float = Float::<U12>::new(BigInt::from(-321_999_999_999_999_i64));
        assert_eq!("-321.999999999999", float.to_string());

        let float = Float::<U20>::new(BigInt::from(-20_i16));
        assert_eq!("-0.00000000000000000020", &float.to_string());
    }

    #[test]
    fn test_ufloat_subtraction() {
        use typenum::U12;

        let lhs_float = UFloat::<U12>::new(321_999_999_999_999_u64.into());
        let rhs_float = UFloat::<U12>::new(999_000_000_000_u64.into());

        let actual_result = lhs_float - rhs_float;
        let expected = UFloat::<U12>::new(321_099_999_999_999_u64.into());

        assert_eq!(&expected, &actual_result);
        assert_eq!("321.000999999999", actual_result.to_string());
    }

    #[test]
    fn test_ufloat_subtraction_with_less_precision() {
        use typenum::{U12, U6};

        let lhs_float = UFloat::<U12>::new(321_999_999_999_999_u64.into());
        let rhs_float = UFloat::<U6>::new(21_000_000_u64.into());

        let actual_result = lhs_float - rhs_float;
        let expected = UFloat::<U12>::new(300_999_999_999_999_u64.into());

        assert_eq!(&expected, &actual_result);
        assert_eq!("300.999999999999", actual_result.to_string());
    }
}
