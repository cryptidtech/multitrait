use thiserror::Error;

/// Errors generated by the numeric type impls
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    /// an unsigned-varint error
    #[error(transparent)]
    UnsignedVarintDecode(#[from] unsigned_varint::decode::Error),
}