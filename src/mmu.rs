/// Translation Lookaside Buffer. Simulates a TLB by using an array of virtually tagged TLB entries.
pub struct Tlb(&'static mut [(u64, TlbEntry)], PurgeRule);

/// Specifies how the TLB will handle overlapping translations
pub enum PurgeRule {
    /// Overlaps should always purge, and will never error
    MustPurge,
    /// Overlaps MUST NOT purge, an error will be returned
    MustNotPurge,
}

impl Tlb {
    pub fn new(size: usize, rule: PurgeRule) -> Self {
        use std::alloc::{alloc_zeroed, Layout};
        use std::mem::{size_of, align_of};

        if !size.is_power_of_two() {
            panic!("Invalid cache size, cache size must be power of two")
        }

        let buffer = unsafe {
            let ptr = alloc_zeroed(
                Layout::from_size_align(
                    size * size_of::<(u64, TlbEntry)>(), 
                    align_of::<(u64, TlbEntry)>()
                ).unwrap()
            ) as *mut (u64, TlbEntry);

            if ptr.is_null() {
                panic!("Cache allocation failed, aborting");
            }

            std::slice::from_raw_parts_mut(ptr, size)
        };

        Self(buffer, rule)
    }

    pub fn fetch(&self, vaddr: u64) -> Option<TlbEntry> {
        let vaddr = vaddr >> 12;
        for entry in self.0.iter() {
            if entry.1.valid() && entry.0 == vaddr {
                return Some(entry.1);
            }
        }
        
        None
    }

    /// Inserts a new TLB entry, destroying the old one
    ///  
    /// See PurgeRule enum for information on how it can purge entries inside itself
    pub fn insert(&mut self, idx: usize, vaddr: u64, entry: TlbEntry) -> Option<()> {
        let vaddr = vaddr >> 12;
        match self.1 {
            PurgeRule::MustPurge => {
                self.inval(vaddr)
            }
            _ => {
                // Dont purge if not needed
                for entry in self.0.iter() {
                    if entry.1.valid() && entry.0 == vaddr {
                        return None;
                    }
                }
            }
        }

        self.0[idx] = (vaddr, entry);

        Some(())
    }

    pub fn inval(&mut self, vaddr: u64) {
        for entry in self.0.iter_mut() {
            if entry.1.valid() && entry.0 == vaddr {
                entry.1.set_valid(false);
            }
        }
    }
}

impl Drop for Tlb {
    fn drop(&mut self) {
        use std::alloc::{dealloc, Layout};
        use std::mem::{size_of, align_of};

        unsafe {
            dealloc(
                self.0.as_ptr() as *mut u8, 
                Layout::from_size_align(
                    self.0.len() * size_of::<(u64, TlbEntry)>(), 
                    align_of::<(u64, TlbEntry)>()
                ).unwrap()
            );
        }
    }
}

bitfield::bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct TlbEntry(u64);
    impl Debug;

    /// Whether or not to ignore the entry
    valid, set_valid: 0;
    /// Whether or not an entry has readable data
    read, set_read: 1;
    /// Whether or not an entry has writeable data
    write, set_write: 2;
    /// Whether or not an entry has executable data
    exec, set_exec: 3;
    /// Specifies page attributes
    attrib, set_attrib: 4;
    /// Physical Page Number
    ppn, set_ppn: 63, 12;
}

impl TlbEntry {
    pub const fn new(paddr: u64, read: bool, write: bool, exec: bool, attrib: bool) -> Self {
        Self(
            // ppn
            (paddr & 0xffff_ffff_ffff_f000) |
            // read
            ((read as u64) << 1) |
            // write
            ((write as u64) << 2) |
            // exec
            ((exec as u64) << 3) |
            // attrib
            ((attrib as u64) << 4) |
            // valid
            (1)
        )
    }
}