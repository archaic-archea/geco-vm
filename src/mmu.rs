/// Translation Lookaside Buffer. Simulates a TLB by using an array of virtually tagged TLB entries.
pub struct Tlb(&'static mut [Option<(u64, TlbEntry)>], PurgeRule);

/// Specifies how the TLB will handle overlapping translations
pub enum PurgeRule {
    MustPurge,
    MayPurge,
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
                    size * size_of::<Option<(u64, TlbEntry)>>(), 
                    align_of::<Option<(u64, TlbEntry)>>()
                ).unwrap()
            ) as *mut Option<(u64, TlbEntry)>;

            if ptr.is_null() {
                panic!("Cache allocation failed, aborting");
            }

            std::slice::from_raw_parts_mut(ptr, size)
        };

        Self(buffer, rule)
    }

    pub fn fetch(&self, vaddr: u64) -> Option<TlbEntry> {
        for entry in self.0.iter() {
            if let Some(entry) = entry {
                if entry.0 == vaddr {
                    return Some(entry.1);
                }
            }
        }
        
        None
    }

    /// Inserts a new TLB entry, destroying the old one  
    /// See PurgeRule enum for information on how it can purge entries inside itself
    pub fn insert(&mut self, idx: usize, vaddr: u64, entry: TlbEntry) -> Option<()> {
        match self.1 {
            PurgeRule::MustPurge => {
                self.inval(vaddr)
            }
            _ => {
                // Dont purge if not needed
                for entry in self.0.iter() {
                    if let Some(entry) = entry {
                        if entry.0 == 0 {
                            return None;
                        }
                    }
                }
            }
        }

        self.0[idx] = Some((vaddr, entry));

        Some(())
    }

    pub fn inval(&mut self, vaddr: u64) {
        for entry in self.0.iter_mut() {
            if let Some(entry) = entry {
                if entry.0 == vaddr {
                    entry.1.set_valid(false);
                }
            }
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
    /// Physical Page Number
    ppn, set_ppn: 55, 4;
    /// Specifies page attributes
    attrib, set_attrib: 56;
}

impl TlbEntry {
    pub fn new(paddr: u64, read: bool, write: bool, exec: bool, attrib: bool) -> Self {
        let mut new = Self(0);

        new.set_ppn(paddr >> 12);
        new.set_valid(true);
        new.set_read(read);
        new.set_write(write);
        new.set_exec(exec);
        new.set_attrib(attrib);

        new
    }
}