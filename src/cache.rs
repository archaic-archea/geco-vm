pub struct Cache<'cache, Tag: Sized + Copy, Data: Sized> {
    data: &'cache mut [CacheEntry<Tag, Data>],
    index_mask: usize
}

impl<'cache, Tag: Copy, Data> Cache<'cache, Tag, Data> {
    pub fn new(size: usize) -> Self {
        use std::alloc::{alloc_zeroed, Layout};
        use std::mem::{size_of, align_of};

        if !size.is_power_of_two() {
            panic!("Invalid cache size, cache size must be power of two")
        }

        let buffer = unsafe {
            let ptr = alloc_zeroed(
                Layout::from_size_align(
                    size * size_of::<CacheEntry<Tag, Data>>(), 
                    align_of::<CacheEntry<Tag, Data>>()
                ).unwrap()
            ) as *mut CacheEntry<Tag, Data>;

            if ptr.is_null() {
                panic!("Cache allocation failed, aborting");
            }

            std::slice::from_raw_parts_mut(ptr, size)
        };

        Self {
            data: buffer,
            index_mask: size - 1,
        }
    }

    pub fn insert(&mut self, index: usize, mut entry: CacheEntry<Tag, Data>) -> CacheEntry<Tag, Data> {
        std::mem::swap(&mut entry, &mut self.data[index & self.index_mask]);

        entry
    }

    pub fn get(&self, index: usize) -> &CacheEntry<Tag, Data> {
        &self.data[index & self.index_mask]
    }
}

impl<'cache, Tag: Copy, Data> Drop for Cache<'cache, Tag, Data> {
    fn drop(&mut self) {
        use std::alloc::{dealloc, Layout};
        use std::mem::{size_of, align_of};

        unsafe {
            dealloc(
                self.data.as_ptr() as *mut u8, 
                Layout::from_size_align(
                    self.data.len() * size_of::<CacheEntry<Tag, Data>>(), 
                    align_of::<CacheEntry<Tag, Data>>()
                ).unwrap()
            );
        }
    }
}

#[derive(PartialEq, Eq)]
pub struct CacheEntry<Tag: Sized + Copy, Data: Sized>(Tag, Data);

impl<Tag: Copy, Data> CacheEntry<Tag, Data> {
    pub fn new(tag: Tag, data: Data) -> Self {
        Self(tag, data)
    }

    pub fn tag(&self) -> Tag {
        self.0
    }

    pub fn set_tag(&mut self, tag: Tag) {
        self.0 = tag
    }
}

impl<Tag: Copy, Data> std::ops::Deref for CacheEntry<Tag, Data> {
    type Target = Data;

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl<Tag: Copy, Data> std::ops::DerefMut for CacheEntry<Tag, Data> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.1
    }
}

impl<Tag: std::fmt::Display + Copy, Data: std::fmt::Display> std::fmt::Display for CacheEntry<Tag, Data> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.0, self.1)
    }
}