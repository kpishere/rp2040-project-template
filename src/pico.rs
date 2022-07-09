const BYTE_ALIGN: usize = 8usize;

extern crate alloc; // Heap allocator

use corosensei::stack::{Stack, StackPointer, MIN_STACK_SIZE};

use defmt::info; // Logging framework
                 //use defmt_rtt as _;

/// Default stack implementation which uses `mmap`.
pub struct DefaultStack {
    base: StackPointer,
    mmap_len: usize,
}

impl DefaultStack {
    /// Creates a new stack which has at least the given capacity.
    pub fn new(size: usize) -> Result<Self, alloc::alloc::AllocError> {
        // Apply minimum stack size.
        let size = size.max(MIN_STACK_SIZE);

        unsafe {
            // Reserve some address space for the stack.
            let mmap_len = alloc::alloc::Layout::from_size_align(size, BYTE_ALIGN).unwrap();
            let mmap_len_bytes = mmap_len.size();
            let mmap = alloc::alloc::alloc_zeroed(mmap_len);
            // Create the result here. If the mprotect call fails then this will
            // be dropped and the memory will be unmapped.
            let out = Self {
                base: StackPointer::new(mmap as usize + mmap_len_bytes).unwrap(),
                mmap_len: mmap_len_bytes,
            };

            Ok(out)
        }
    }
}

impl Default for DefaultStack {
    fn default() -> Self {
        Self::new(MIN_STACK_SIZE).expect("failed to allocate stack")
    }
}

impl Drop for DefaultStack {
    fn drop(&mut self) {
        unsafe {
            let mmap = self.base.get() - self.mmap_len;
            alloc::alloc::dealloc(
                mmap as _,
                alloc::alloc::Layout::from_size_align(self.mmap_len, BYTE_ALIGN).unwrap(),
            );
            info!("stack dealloc {} {}", mmap, self.mmap_len);
        }
    }
}

unsafe impl Stack for DefaultStack {
    #[inline]
    fn base(&self) -> StackPointer {
        self.base
    }

    #[inline]
    fn limit(&self) -> StackPointer {
        StackPointer::new(self.base.get() - self.mmap_len).unwrap()
    }
}
