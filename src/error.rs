// SPDX-License-Idnetifier: Apache-2.0
/// Errors generated by the numeric type impls
#[derive(Clone, Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// an unsigned-varint error
    #[error(transparent)]
    UnsignedVarintDecode(#[from] unsigned_varint::decode::Error),
}
