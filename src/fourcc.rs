use crate::{error::RiffResult, RiffError};
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Clone)]
pub struct FourCC {
    data: [u8; 4],
}

/// Represents the ASCII identifier of a chunk.
///
/// # Example
///
/// ```rust
/// use riffu::FourCC;
/// let good = FourCC::new(b"1234");
/// // let bad = FourCC::new(b"12345"); // won't compile
/// ```
///
/// # NOTE
///
/// 1. AFAIK, the only valid identifier is in ASCII.
///    I am not entirely sure what all the possible value ASCII can take.
///    So we need to enforce correctness on that front too.
/// 2. Are there other conversions that we are missing out on?
impl FourCC {
    pub fn new(data: &[u8]) -> RiffResult<FourCC> {
        let data = data.try_into()?;
        Ok(FourCC { data })
    }

    /// View `&self` struct as a `&[u8]`.
    pub fn as_bytes(&self) -> &[u8; 4] {
        &self.data
    }

    /// Consume `self` and returns a `[u8; 4]`.
    pub fn into_bytes(self) -> [u8; 4] {
        self.data
    }
}

/// A `&[u8]` can be converted to a `FourCC`.
impl TryFrom<&[u8]> for FourCC {
    type Error = RiffError;

    /// Performs the conversion.
    /// ```
    /// use riffu::FourCC;
    /// use std::convert::TryInto;
    /// let buffer: &[u8] = &[80u8, 80u8, 80u8,80u8];
    /// let test: FourCC = buffer.try_into().unwrap();
    /// ```
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(FourCC {
            data: value.try_into()?,
        })
    }
}

/// A `&str` can be converted to a `FourCC`.
impl TryFrom<&str> for FourCC {
    type Error = RiffError;

    /// Performs the conversion.
    /// ```
    /// use riffu::FourCC;
    /// use std::convert::TryInto;
    /// let test : FourCC = "test".try_into().unwrap();
    /// ```
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.as_bytes().try_into()
    }
}
