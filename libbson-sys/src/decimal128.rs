use core::ffi::{c_char, c_int};

#[cfg(target_endian = "big")]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct bson_decimal128_t {
    pub high: u64,
    pub low: u64,
}

#[cfg(target_endian = "little")]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct bson_decimal128_t {
    pub low: u64,
    pub high: u64,
}

extern "C" {
    pub fn bson_decimal128_from_string(s: *const c_char, dec: *mut bson_decimal128_t);
    pub fn bson_decimal128_from_string_w_len(
        s: *const c_char,
        len: c_int,
        dec: *mut bson_decimal128_t,
    );
    pub fn bson_decimal128_to_string(dec: *const bson_decimal128_t, s: *mut c_char);
}

#[cfg(test)]
mod tests {
    use alloc::ffi::CString;

    use super::*;

    macro_rules! assert_to_string_eq {
        (($high:literal, $low:literal) => $s:literal) => {
            let dec = bson_decimal128_t {
                high: $high,
                low: $low,
            };

            let ptr = CString::default().into_raw();
            unsafe {
                bson_decimal128_to_string(&dec, ptr);
                assert_eq!(CString::from_raw(ptr), CString::new($s).unwrap());
            }
        };
    }

    #[allow(non_snake_case)]
    #[test]
    fn to_string__nan() {
        assert_to_string_eq!((0x7c00000000000000, 0x0000000000000000) => "NaN");
        assert_to_string_eq!((0xfc00000000000000, 0x0000000000000000) => "NaN");
        assert_to_string_eq!((0x7e00000000000000, 0x0000000000000000) => "NaN");
        assert_to_string_eq!((0xfe00000000000000, 0x0000000000000000) => "NaN");
        assert_to_string_eq!((0x7e00000000000000, 0x000000000000000c) => "NaN");
    }

    #[allow(non_snake_case)]
    #[test]
    fn to_string__regular() {
        assert_to_string_eq!((0x3040000000000000, 0x0000000000000000) => "0");
        assert_to_string_eq!((0x3040000000000000, 0x0000000000000001) => "1");
        assert_to_string_eq!((0x3040000000000000, 0x0000000000000002) => "2");
        assert_to_string_eq!((0xb040000000000000, 0x0000000000000001) => "-1");
        assert_to_string_eq!((0xb040000000000000, 0x0000000000000000) => "-0");
        assert_to_string_eq!((0x303e000000000000, 0x0000000000000001) => "0.1");
        assert_to_string_eq!((0x3034000000000000, 0x00000000000004d2) => "0.001234");
        assert_to_string_eq!((0x3040000000000000, 0x0000001cbe991a14) => "123456789012");
        assert_to_string_eq!((0x302a000000000000, 0x00000000075aef40) => "0.00123400000");
        assert_to_string_eq!((0x2ffc3cde6fff9732, 0xde825cd07e96aff2) => "0.1234567890123456789012345678901234");
        assert_to_string_eq!((0x3040ffffffffffff, 0xffffffffffffffff) => "5192296858534827628530496329220095");
    }

    #[allow(non_snake_case)]
    #[test]
    fn to_string__scientific() {
        assert_to_string_eq!((0x5ffe314dc6448d93, 0x38c15b0a00000000) => "1.000000000000000000000000000000000E+6144");
        assert_to_string_eq!((0x0000000000000000, 0x0000000000000001) => "1E-6176");
        assert_to_string_eq!((0x8000000000000000, 0x0000000000000001) => "-1E-6176");
        assert_to_string_eq!((0x3108000000000000, 0x000009184db63eb1) => "9.999987654321E+112");
        assert_to_string_eq!((0x5fffed09bead87c0, 0x378d8e63ffffffff) => "9.999999999999999999999999999999999E+6144");
        assert_to_string_eq!((0x0001ed09bead87c0, 0x378d8e63ffffffff) => "9.999999999999999999999999999999999E-6143");
        assert_to_string_eq!((0x304c000000000000, 0x000000000000041a) => "1.050E+9");
        assert_to_string_eq!((0x3042000000000000, 0x000000000000041a) => "1.050E+4");
        assert_to_string_eq!((0x3040000000000000, 0x0000000000000069) => "105");
        assert_to_string_eq!((0x3042000000000000, 0x0000000000000069) => "1.05E+3");
        assert_to_string_eq!((0x3046000000000000, 0x0000000000000001) => "1E+3");
    }

    #[allow(non_snake_case)]
    #[test]
    fn to_string__zeros() {
        assert_to_string_eq!((0x3040000000000000, 0x0000000000000000) => "0");
        assert_to_string_eq!((0x3298000000000000, 0x0000000000000000) => "0E+300");
        assert_to_string_eq!((0x2b90000000000000, 0x0000000000000000) => "0E-600");
    }

    macro_rules! assert_from_string_eq_nan {
        ($s:literal) => {
            let mut dec = bson_decimal128_t::default();
            let s = CString::new($s).unwrap();

            unsafe {
                bson_decimal128_from_string(s.as_ptr(), &mut dec);
            }

            assert_eq!(dec.high, 0x7c00000000000000);
        };
        ($s:literal, $len:literal) => {
            let mut dec = bson_decimal128_t::default();
            let s = CString::new($s).unwrap();

            unsafe {
                bson_decimal128_from_string_w_len(s.as_ptr(), $len, &mut dec);
            }

            assert_eq!(dec.high, 0x7c00000000000000);
        };
    }

    #[allow(non_snake_case)]
    #[test]
    fn from_string__invalid_inputs() {
        assert_from_string_eq_nan!(".");
        assert_from_string_eq_nan!(".e");
        assert_from_string_eq_nan!("");
        assert_from_string_eq_nan!("invalid");
        assert_from_string_eq_nan!("in");
        assert_from_string_eq_nan!("i");
        assert_from_string_eq_nan!("E02");
        assert_from_string_eq_nan!("..1");
        assert_from_string_eq_nan!("1abcede");
        assert_from_string_eq_nan!("1.24abc");
        assert_from_string_eq_nan!("1.24abcE+02");
        assert_from_string_eq_nan!("1.24E+02abc2d");
        assert_from_string_eq_nan!("E+02");
        assert_from_string_eq_nan!("e+02");

        assert_from_string_eq_nan!(".", 1);
        assert_from_string_eq_nan!(".e", 2);
        assert_from_string_eq_nan!("", 0);
        assert_from_string_eq_nan!("invalid", 7);
        assert_from_string_eq_nan!("in", 2);
        assert_from_string_eq_nan!("i", 1);
        assert_from_string_eq_nan!("E02", 3);
        assert_from_string_eq_nan!("..1", 3);
        assert_from_string_eq_nan!("1abcede", 7);
        assert_from_string_eq_nan!("1.24abc", 7);
        assert_from_string_eq_nan!("1.24abcE+02", 11);
        assert_from_string_eq_nan!("1.24E+02abc2d", 13);
        assert_from_string_eq_nan!("E+02", 4);
        assert_from_string_eq_nan!("e+02", 4);
    }

    #[allow(non_snake_case)]
    #[test]
    fn from_string__nan() {
        assert_from_string_eq_nan!("NaN");
        assert_from_string_eq_nan!("+NaN");
        assert_from_string_eq_nan!("-NaN");
        assert_from_string_eq_nan!("-nan");
        assert_from_string_eq_nan!("1e");
        assert_from_string_eq_nan!("+nan");
        assert_from_string_eq_nan!("nan");
        assert_from_string_eq_nan!("Nan");
        assert_from_string_eq_nan!("+Nan");
        assert_from_string_eq_nan!("-Nan");

        assert_from_string_eq_nan!("NaN", 3);
        assert_from_string_eq_nan!("+NaN", 4);
        assert_from_string_eq_nan!("-NaN", 4);
        assert_from_string_eq_nan!("-nan", 4);
        assert_from_string_eq_nan!("1e", 2);
        assert_from_string_eq_nan!("+nan", 4);
        assert_from_string_eq_nan!("nan", 3);
        assert_from_string_eq_nan!("Nan", 3);
        assert_from_string_eq_nan!("+Nan", 4);
        assert_from_string_eq_nan!("-Nan", 4);
    }

    macro_rules! assert_from_string_eq_infinity {
        ((+) $s:literal) => {
            let mut dec = bson_decimal128_t::default();
            let s = CString::new($s).unwrap();

            unsafe {
                bson_decimal128_from_string(s.as_ptr(), &mut dec);
            }

            assert_eq!(dec.high, 0x7800000000000000);
        };
        ((-) $s:literal) => {
            let mut dec = bson_decimal128_t::default();
            let s = CString::new($s).unwrap();

            unsafe {
                bson_decimal128_from_string(s.as_ptr(), &mut dec);
            }

            assert_eq!(dec.high, 0xf800000000000000);
        };
        ((+) $s:literal, $len:literal) => {
            let mut dec = bson_decimal128_t::default();
            let s = CString::new($s).unwrap();

            unsafe {
                bson_decimal128_from_string_w_len(s.as_ptr(), $len, &mut dec);
            }

            assert_eq!(dec.high, 0x7800000000000000);
        };
        ((-) $s:literal, $len:literal) => {
            let mut dec = bson_decimal128_t::default();
            let s = CString::new($s).unwrap();

            unsafe {
                bson_decimal128_from_string_w_len(s.as_ptr(), $len, &mut dec);
            }

            assert_eq!(dec.high, 0xf800000000000000);
        };
    }

    #[allow(non_snake_case)]
    #[test]
    fn from_string__infinity() {
        assert_from_string_eq_infinity!((+) "Infinity");
        assert_from_string_eq_infinity!((+) "+Infinity");
        assert_from_string_eq_infinity!((+) "+Inf");
        assert_from_string_eq_infinity!((-) "-Inf");
        assert_from_string_eq_infinity!((-) "-Infinity");

        assert_from_string_eq_infinity!((+) "Infinity", 8);
        assert_from_string_eq_infinity!((+) "+Infinity", 9);
        assert_from_string_eq_infinity!((+) "+Inf", 4);
        assert_from_string_eq_infinity!((-) "-Inf", 4);
        assert_from_string_eq_infinity!((-) "-Infinity", 9);
    }

    macro_rules! assert_from_string_eq {
        ($s:literal => ($high:literal, $low:literal)) => {
            let mut dec = bson_decimal128_t::default();
            let s = CString::new($s).unwrap();

            unsafe {
                bson_decimal128_from_string(s.as_ptr(), &mut dec);
            }

            assert_eq!((dec.high, dec.low), ($high, $low));
        };
        ($s:literal, $len:literal => ($high:literal, $low:literal)) => {
            let mut dec = bson_decimal128_t::default();
            let s = CString::new($s).unwrap();

            unsafe {
                bson_decimal128_from_string_w_len(s.as_ptr(), $len, &mut dec);
            }

            assert_eq!((dec.high, dec.low), ($high, $low));
        };
    }

    #[allow(non_snake_case)]
    #[test]
    fn from_string__simple() {
        assert_from_string_eq!("1" => (0x3040000000000000, 0x0000000000000001));
        assert_from_string_eq!("-1" => (0xb040000000000000, 0x0000000000000001));
        assert_from_string_eq!("0" => (0x3040000000000000, 0x0000000000000000));
        assert_from_string_eq!("-0" => (0xb040000000000000, 0x0000000000000000));
        assert_from_string_eq!("12345678901234567" => (0x3040000000000000, 0x002bdc545d6b4b87));
        assert_from_string_eq!("989898983458" => (0x3040000000000000, 0x000000e67a93c822));
        assert_from_string_eq!("-12345678901234567" => (0xb040000000000000, 0x002bdc545d6b4b87));
        assert_from_string_eq!("0.12345" => (0x3036000000000000, 0x0000000000003039));
        assert_from_string_eq!("0.0012345" => (0x3032000000000000, 0x0000000000003039));
        assert_from_string_eq!("00012345678901234567" => (0x3040000000000000, 0x002bdc545d6b4b87));

        assert_from_string_eq!("1", 1 => (0x3040000000000000, 0x0000000000000001));
        assert_from_string_eq!("-1", 2 => (0xb040000000000000, 0x0000000000000001));
        assert_from_string_eq!("0", 1 => (0x3040000000000000, 0x0000000000000000));
        assert_from_string_eq!("-0", 2 => (0xb040000000000000, 0x0000000000000000));
        assert_from_string_eq!("12345678901234567", 17 => (0x3040000000000000, 0x002bdc545d6b4b87));
        assert_from_string_eq!("989898983458", 12 => (0x3040000000000000, 0x000000e67a93c822));
        assert_from_string_eq!("-12345678901234567", 18 => (0xb040000000000000, 0x002bdc545d6b4b87));
        assert_from_string_eq!("0.12345", 7 => (0x3036000000000000, 0x0000000000003039));
        assert_from_string_eq!("0.0012345", 9 => (0x3032000000000000, 0x0000000000003039));
        assert_from_string_eq!("00012345678901234567", 20 => (0x3040000000000000, 0x002bdc545d6b4b87));
    }

    #[allow(non_snake_case)]
    #[test]
    fn from_string__scientific() {}

    #[allow(non_snake_case)]
    #[test]
    fn from_string__large() {}

    #[allow(non_snake_case)]
    #[test]
    fn from_string__exponent_normalization() {}

    #[allow(non_snake_case)]
    #[test]
    fn from_string__zeros() {}

    #[allow(non_snake_case)]
    #[test]
    fn from_string_w_len__special() {}
}
