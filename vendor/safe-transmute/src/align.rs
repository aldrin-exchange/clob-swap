//! Alignment checking primitives.


use core::mem::{align_of, size_of};
use self::super::error::UnalignedError;


fn validate_alignment<S, T>(data: &[S]) -> Result<(), usize> {
    // TODO this could probably become more efficient once `ptr::align_offset`
    // is stabilized (#44488)
    let ptr = data.as_ptr();
    let offset = ptr as usize % align_of::<T>();
    if offset > 0 {
        // reverse the offset (from "bytes to insert" to "bytes to remove")
        Err(size_of::<T>() - offset)
    } else {
        Ok(())
    }
}


/// Check whether the given data slice of `S`s is properly aligned for reading
/// and writing as a slice of `T`s.
///
/// # Errors
///
/// An `Error::Unaligned` error is returned with the number of bytes to discard
/// from the front in order to make the conversion safe from alignment concerns.
pub fn check_alignment<S, T>(data: &[S]) -> Result<(), UnalignedError<S, T>> {
    validate_alignment::<_, T>(data).map_err(move |off| UnalignedError::new(off, data))
}

/// Check whether the given mutable data slice of `S`s is properly aligned for
/// reading and writing as a slice of `T`s, returning the same slice back if
/// it is.
///
/// # Errors
///
/// An `Error::Unaligned` error is returned with the number of bytes to discard
/// from the front in order to make the conversion safe from alignment concerns.
pub fn check_alignment_mut<S, T>(data: &mut [S]) -> Result<&mut [S], UnalignedError<S, T>> {
    match validate_alignment::<_, T>(data) {
        Ok(()) => Ok(data),
        Err(off) => Err(UnalignedError::new(off, data)),
    }
}
