#[cfg(feature = "alloc")]
use core::mem::{align_of, size_of, forget};
#[cfg(feature = "alloc")]
use alloc::vec::Vec;


/// Create a new vector that contains the given bytes and is sure to have a
/// memory alignment compatible with `T` at creation time.
///
/// # Examples
///
/// ```
/// let data: &[u8] = &[0xFF, 0xFF, 0x03, 0x00];
/// let vev = aligned_vec::<u32>(data);
/// // the vector's data is guaranteed to be aligned for access as a u32
/// assert_eq!((vec.as_ptr() as usize) % align_of::<u32>(), 0);
/// ```
///
/// # Safety
///
/// The resulting vector must then be deallocated with
/// `dealloc_aligned_vec()`, and the exact same `T` parameter.
///
/// Modifying or not moving into `dealloc_aligned_vec()` *will* yield UB.
#[cfg(feature = "alloc")]
unsafe fn aligned_vec<T>(bytes: &[u8]) -> Vec<u8> {
    let vec_len_offset = bytes.len() % size_of::<T>();
    let vec_len = bytes.len() / size_of::<T>();
    let capacity = if vec_len_offset > 0 {
        vec_len + 1
    } else {
        vec_len
    };

    // The following code allocates a `Vec<T>` and turns it into
    // a `Vec<u8>`. Assuming that this vector will not be dropped
    // in this state, reading bytes from it is safe.
    #[allow(unused_unsafe)]
    unsafe {
        let mut v: Vec<T> = Vec::with_capacity(capacity);
        let ptr = v.as_mut_ptr() as *mut u8;
        bytes.as_ptr().copy_to_nonoverlapping(ptr, bytes.len());

        forget(v);
        let vec = Vec::from_raw_parts(ptr, bytes.len(), capacity * size_of::<T>());

        assert_eq!((vec.as_ptr() as usize) % align_of::<T>(), 0);
        assert_eq!(vec.len(), bytes.len());
        assert!(vec.capacity() >= vec.len());

        vec
    }
}

/// Deallocate a vector created by `aligned_vec`.
///
/// # Safety
///
/// Shall not be called on a vector not created by `aligned_vec()`.
/// The `T` parameter must also match the one used to
/// create the vector.
#[cfg(feature = "alloc")]
unsafe fn dealloc_aligned_vec<T>(vec: Vec<u8>) {
    safe_transmute::base::transmute_vec::<_, T>(vec);
}
