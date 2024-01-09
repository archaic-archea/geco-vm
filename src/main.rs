use geco_vm::*;

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dram_test() {
        let dram = dram::Dram::new(16);
        let dram_lock = dram.phys_ram.lock().unwrap();
        assert!(dram_lock.len() == 2, "DRAM length incorrect");
    }

    #[test]
    fn cache_test() {
        let mut icache = cache::Cache::new(1);
        icache.insert(
            0, 
            cache::CacheEntry::new(0, 0)
        );

        assert!(*icache.get(0) == cache::CacheEntry::new(0, 0), "Invalid cache entry");
    }

    #[test]
    fn tlb_test() {
        let mut itlb = mmu::Tlb::new(1);
        itlb.insert(
            0, 
            mmu::TlbEntry::new(0, true, true, false, false)
        );

        assert!(itlb.fetch(0).unwrap() == mmu::TlbEntry::new(0, true, true, false, false), "Failed to fetch TLB entry");
    }
}