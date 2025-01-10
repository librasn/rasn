use num_traits::float::FloatCore;

/// Represents a real type in Rust that can be decoded or encoded into any
/// ASN.1 codec.
pub trait RealType: Sized + core::fmt::Debug + core::fmt::Display {
    /// The byte level width of the floating point type.
    const BYTE_WIDTH: usize;

    /// Returns the IEEE 754 encoded bytes of the real type, byte count defined in `usize`
    fn to_ieee754_bytes(&self) -> (impl AsRef<[u8]>, usize);

    /// Attempts to decode the IEEE 754 encoded bytes into `Self`
    fn try_from_ieee754_bytes(bytes: &[u8]) -> Result<Self, TryFromRealError>;

    /// Attempts to convert a generic floating point type into `Self`
    fn try_from_float(value: impl FloatCore) -> Option<Self>;

    /// Attempts to convert `Self` into a generic floating point type
    fn try_to_float(&self) -> Option<impl FloatCore>;
}

#[cfg(feature = "f64")]
impl RealType for f64 {
    const BYTE_WIDTH: usize = core::mem::size_of::<Self>();

    #[inline]
    fn to_ieee754_bytes(&self) -> (impl AsRef<[u8]>, usize) {
        let bytes = self.to_be_bytes();
        (bytes, bytes.len())
    }

    #[inline]
    fn try_from_ieee754_bytes(bytes: &[u8]) -> Result<Self, TryFromRealError> {
        let bytes = bytes
            .try_into()
            .map_err(|_| TryFromRealError::InvalidEncoding)?;

        Ok(f64::from_be_bytes(bytes))
    }

    fn try_from_float(value: impl FloatCore) -> Option<Self> {
        value.to_f64()
    }

    fn try_to_float(&self) -> Option<impl FloatCore> {
        Some(*self)
    }
}

#[cfg(feature = "f32")]
impl RealType for f32 {
    const BYTE_WIDTH: usize = core::mem::size_of::<Self>();

    #[inline]
    fn to_ieee754_bytes(&self) -> (impl AsRef<[u8]>, usize) {
        let bytes = self.to_be_bytes();
        (bytes, bytes.len())
    }

    #[inline]
    fn try_from_ieee754_bytes(bytes: &[u8]) -> Result<Self, TryFromRealError> {
        let bytes = bytes
            .try_into()
            .map_err(|_| TryFromRealError::InvalidEncoding)?;

        Ok(f32::from_be_bytes(bytes))
    }

    fn try_from_float(value: impl FloatCore) -> Option<Self> {
        value.to_f32()
    }

    fn try_to_float(&self) -> Option<impl FloatCore> {
        Some(*self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TryFromRealError {
    InvalidEncoding,
}
