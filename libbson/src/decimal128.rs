#[cfg(feature = "alloc")]
use alloc::ffi::CString;
use core::convert::Infallible;
use core::fmt::{self, Debug, Display};
use core::str::FromStr;

#[derive(Clone, Copy)]
pub struct Decimal128(libbson_sys::bson_decimal128_t);

impl Decimal128 {
    /// Constructs a BSON `Decimal128` from a little-endian byte representation.
    pub fn from_bytes(bytes: [u8; 16]) -> Self {
        let (low, high) = bytes.split_at(8);

        Self(libbson_sys::bson_decimal128_t {
            high: u64::from_le_bytes({
                let mut bytes = [0; 8];
                bytes.copy_from_slice(high);

                bytes
            }),
            low: u64::from_le_bytes({
                let mut bytes = [0; 8];
                bytes.copy_from_slice(low);

                bytes
            }),
        })
    }

    /// Converts a BSON `Decimal128` into its little-endian byte representation.
    pub fn to_bytes(&self) -> [u8; 16] {
        let mut bytes = [0; 16];

        let (low, high) = bytes.split_at_mut(8);
        high.copy_from_slice(&self.0.high.to_le_bytes());
        low.copy_from_slice(&self.0.low.to_le_bytes());

        bytes
    }
}

#[cfg(feature = "alloc")]
impl Debug for Decimal128 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Decimal128")
            .field(&self.to_string())
            .finish()
    }
}

#[cfg(feature = "alloc")]
impl Display for Decimal128 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ptr = CString::default().into_raw();
        let s = unsafe {
            libbson_sys::bson_decimal128_to_string(&self.0, ptr);
            CString::from_raw(ptr)
        };

        f.write_str(
            s.to_str()
                .expect("invariant: `bson_decimal128_to_string` must not generate invalid utf-8"),
        )
    }
}

impl FromStr for Decimal128 {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut dec = libbson_sys::bson_decimal128_t::default();
        let s = CString::new(s).expect("invariant: `str` must not contain any null bytes");

        unsafe {
            libbson_sys::bson_decimal128_from_string(s.as_ptr(), &mut dec);
        }

        Ok(Self(dec))
    }
}
