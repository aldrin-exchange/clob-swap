//! Functions for safe transmutation to `bool`.
//!
//! Transmuting to `bool` is not undefined behavior if the transmuted value is
//! either 0 or 1. These functions will return an error if the integer value
//! behind the `bool` value is neither one.
//!
//! # Note
//!
//! Currently, these functions only work on systems in which the size of `bool`
//! is exactly 1 (which are all platforms supported by Rust at the time of
//! writing). In the event that you find a platform with an unexpected `bool`
//! size, please report at the project's
//! [issue tracker](https://github.com/nabijaczleweli/safe-transmute-rs/issues/new).


use self::super::guard::{PermissiveGuard, PedanticGuard, Guard};
use self::super::base::transmute_many;
#[cfg(feature = "alloc")]
use self::super::base::transmute_vec;
use core::mem::transmute;
use self::super::Error;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;


/// Makes sure that the bytes represent a sequence of valid boolean values.
///
/// # Examples
///
/// ```
/// # use safe_transmute::bool::bytes_are_bool;
/// assert!(bytes_are_bool(&[false as u8, true as u8]));
///
/// assert!(!bytes_are_bool(&[(false as u8 + true as u8) * 2]));
/// ```
#[inline]
pub fn bytes_are_bool(v: &[u8]) -> bool {
    let _bool_must_be_1_byte_pls_report = transmute::<bool, u8>;

    v.iter().cloned().all(byte_is_bool)
}

#[inline]
fn byte_is_bool(b: u8) -> bool {
    unsafe { b == transmute::<_, u8>(false) || b == transmute::<_, u8>(true) }
}

fn transmute_bool<G: Guard>(bytes: &[u8]) -> Result<&[bool], Error<u8, bool>> {
    check_bool(bytes)?;
    unsafe { transmute_many::<_, G>(bytes) }
}

/// Helper function for returning an error if any of the bytes does not make a
/// valid `bool`.
fn check_bool<'a, T>(bytes: &[u8]) -> Result<(), Error<'a, u8, T>> {
    if bytes_are_bool(bytes) {
        Ok(())
    } else {
        Err(Error::InvalidValue)
    }
}


/// View a byte slice as a slice of boolean values.
///
/// The resulting slice will have as many instances of `bool` as will fit, can be empty.
///
/// # Examples
///
/// ```
/// # use safe_transmute::{Error, transmute_bool_permissive};
/// # fn run() -> Result<(), Error<'static, u8, bool>> {
/// assert_eq!(transmute_bool_permissive(&[0x00, 0x01, 0x00, 0x01])?,
///            &[false, true, false, true]);
/// assert_eq!(transmute_bool_permissive(&[])?, &[]);
/// # Ok(())
/// # }
/// # run().unwrap()
/// ```
pub fn transmute_bool_permissive(bytes: &[u8]) -> Result<&[bool], Error<u8, bool>> {
    transmute_bool::<PermissiveGuard>(bytes)
}

/// View a byte slice as a slice of boolean values.
///
/// The byte slice must have at least enough bytes to fill a single `bool`.
///
/// # Examples
///
/// ```
/// # use safe_transmute::{Error, transmute_bool_pedantic};
/// # fn run() -> Result<(), Error<'static, u8, bool>> {
/// assert_eq!(transmute_bool_pedantic(&[0x01, 0x01, 0x01, 0x01])?,
///            &[true, true, true, true]);
/// assert!(transmute_bool_pedantic(&[]).is_err());
/// # Ok(())
/// # }
/// # run().unwrap()
/// ```
pub fn transmute_bool_pedantic(bytes: &[u8]) -> Result<&[bool], Error<u8, bool>> {
    transmute_bool::<PedanticGuard>(bytes)
}

/// Trasform a byte vector into a vector of bool.
///
/// The vector's allocated byte buffer will be reused when possible.
///
/// # Examples
///
/// ```
/// # use safe_transmute::{Error, transmute_bool_vec_permissive};
/// # fn run() -> Result<(), Error<'static, u8, bool>> {
/// assert_eq!(transmute_bool_vec_permissive(vec![0x00, 0x01, 0x00, 0x01])?,
///            vec![false, true, false, true]);
/// assert_eq!(transmute_bool_vec_permissive(vec![0x01, 0x00, 0x00, 0x00, 0x01])?,
///            vec![true, false, false, false, true]);
/// assert_eq!(transmute_bool_vec_permissive(vec![]), Ok(vec![]));
/// # Ok(())
/// # }
/// # run().unwrap()
/// ```
#[cfg(feature = "alloc")]
pub fn transmute_bool_vec_permissive(bytes: Vec<u8>) -> Result<Vec<bool>, Error<'static, u8, bool>> {
    check_bool(&bytes)?;
    PermissiveGuard::check::<u8>(&bytes)?;
    // Alignment guarantees are ensured, and all values have been checked,
    // so the conversion is safe.
    unsafe { Ok(transmute_vec::<u8, bool>(bytes)) }
}

/// Transform a byte vector into a vector of bool.
///
/// The vector's allocated byte buffer will be reused when possible, and
/// should not be empty.
///
/// # Examples
///
/// ```
/// # use safe_transmute::{Error, transmute_bool_vec_pedantic};
/// # fn run() -> Result<(), Error<'static, u8, bool>> {
/// assert_eq!(transmute_bool_vec_pedantic(vec![0x00, 0x01, 0x00, 0x01])?,
///            vec![false, true, false, true]);
///
/// assert!(transmute_bool_vec_pedantic(vec![]).is_err());
///
/// assert!(transmute_bool_vec_pedantic(vec![0x04, 0x00, 0xED]).is_err());
/// # Ok(())
/// # }
/// # run().unwrap()
/// ```
#[cfg(feature = "alloc")]
pub fn transmute_bool_vec_pedantic(bytes: Vec<u8>) -> Result<Vec<bool>, Error<'static, u8, bool>> {
    check_bool(&bytes)?;
    PedanticGuard::check::<u8>(&bytes)?;

    // alignment guarantees are ensured, and all values have been checked,
    // so the conversion is safe.
    unsafe { Ok(transmute_vec::<u8, bool>(bytes)) }
}
