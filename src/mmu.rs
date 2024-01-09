/// The TLB is basically just a VIVT cache that stores the physical address, and access values
pub struct Tlb<'cache>(crate::cache::Cache<'cache, u64, TlbEntry>);

impl<'cache> Tlb<'cache> {
    pub fn new(size: usize) -> Self {
        Self(crate::cache::Cache::new(size))
    }

    pub fn fetch(&self, vaddr: u64) -> Option<TlbEntry> {
        let entry = self.0.get(vaddr as usize);
        if entry.tag() == vaddr {
            Some(**entry)
        } else {
            None
        }
    }

    /// Swaps out a TLB entry, returns the vaddr, paddr, and flags
    pub fn insert(&mut self, vaddr: u64, entry: TlbEntry) -> (u64, TlbEntry) {
        let old = self.0.insert(
            vaddr as usize, 
            crate::cache::CacheEntry::new(
                vaddr, 
                entry
            )
        );

        (
            old.tag(),
            *old
        )
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
    /// Marks a page as not cacheable
    nc, set_nc: 56;
}

impl TlbEntry {
    pub fn new(paddr: u64, read: bool, write: bool, exec: bool, nc: bool) -> Self {
        let mut new = Self(0);

        new.set_ppn(paddr >> 12);
        new.set_valid(true);
        new.set_read(read);
        new.set_write(write);
        new.set_exec(exec);
        new.set_nc(nc);

        new
    }
}