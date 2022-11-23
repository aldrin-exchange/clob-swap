use core::fmt;
use core::marker::PhantomData;
use core::ptr::NonNull;

use crate::NonZeroLayout;
use crate::util::defaults;

/// A marker struct denoting a lifetime that is not simply coercible to another.
#[derive(Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AllocTime<'lt> {
    marker: PhantomData<&'lt fn(&'lt ())>,
}

/// A marker struct denoting a lifetime that is not simply coercible to another.
#[deprecated = "Use the new name 'AllocTime' instead"]
pub type Invariant<'lt> = AllocTime<'lt>;

/// An allocation valid for a particular lifetime.
///
/// It is advisable to try and deallocate it before the end of the lifetime instead of leaking the
/// allocation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Allocation<'alloc> {
    /// A pointer to the allocated and potentially uninitialized bytes.
    pub ptr: NonNull<u8>,
    /// The allocated layout.
    pub layout: NonZeroLayout,
    /// The lifetime of the allocation.
    pub lifetime: AllocTime<'alloc>,
}

/// An allocator providing memory regions valid for a particular lifetime.
///
/// It is useful to compare this trait to `std::alloc::GlobalAlloc`. Similar to the trait it is
/// required that the implementors adhere to the contract of the methods.
pub unsafe trait LocalAlloc<'alloc> {
    /// Allocate one block of memory.
    ///
    /// The callee guarantees that a successful return contains a pointer that is valid for **at
    /// least** the layout requested by the caller.
    fn alloc(&'alloc self, layout: NonZeroLayout) -> Option<Allocation<'alloc>>;

    /// Deallocate a block previously allocated.
    /// # Safety
    /// The caller must ensure that:
    /// * `alloc` has been previously returned from a call to `alloc`.
    /// * There are no more pointer to the allocation.
    unsafe fn dealloc(&'alloc self, alloc: Allocation<'alloc>);

    /// Allocate a block of memory initialized with zeros.
    ///
    /// The callee guarantees that a successful return contains a pointer that is valid for **at
    /// least** the layout requested by the caller and the contiguous region of bytes, starting at
    /// the pointer and with the size of the returned layout, is initialized and zeroed.
    fn alloc_zeroed(&'alloc self, layout: NonZeroLayout)
        -> Option<Allocation<'alloc>> 
    {
        defaults::local::alloc_zeroed(self, layout)
    }

    /// Change the layout of a block previously allocated.
    ///
    /// The callee guarantees that a successful return contains a pointer that is valid for **at
    /// least** the layout requested by the caller and the contiguous region of bytes, starting at
    /// the pointer and with the size of the returned layout, is initialized with the prefix of the
    /// previous allocation that is still valid.
    ///
    /// Note that it is *NOT* safe to elide the methods call for changing the alignment of the
    /// layout to a less strict one, or to an incidentally fulfilled stricter version. The
    /// allocator might make use of the alignment during deallocation.
    unsafe fn realloc(&'alloc self, alloc: Allocation<'alloc>, layout: NonZeroLayout)
        -> Option<Allocation<'alloc>>
    {
        defaults::local::realloc(self, alloc, layout)
    }
}

impl fmt::Debug for AllocTime<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad("AllocTime")
    }
}
