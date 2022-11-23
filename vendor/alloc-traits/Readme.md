# alloc-traits

Defines `no_std` and bare metal usable traits that can be implemented to
defined different kinds of allocators, similar to `alloc::GlobalAlloc`. But
unlike the standard library trait they do not presume global uniqueness and
static lifetime of the memory resource provider. In return, the allocators are
not required to implement the `Sync` bound and can easily be built without
operating system support to be usable.

There are additional independent crates with additional abstractions on-top:
* [`static-alloc`]: A simple allocator drawing from a memory region statically
  embedded within the compiled binary.
* [`without-alloc`]: A set of data structures (`Box`, `Vec`, `Rc`, ...) that can
  be allocated from the implementors of the traits defined here.

[`static-alloc`]: https://crates.io/crates/static-alloc
[`without-alloc`]: https://crates.io/crates/without-alloc
