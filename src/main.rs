use geco::*;

fn main() {
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;

    #[test]
    fn dram_init_test() {
        let dram = dram::Dram::new(32);
        let dram_lock = dram.phys_ram.lock().unwrap();
        
        assert!(dram_lock.len() == 4, "DRAM length incorrect");
    }

    #[test]
    fn rand_dram_rw_test() {
        let mut dram = dram::Dram::new(32);

        let mut rng = rand::thread_rng();

        for i in 0..64 {
            match i & 0b11 {
                0 => {
                    let addr = rng.gen_range(0..32);
                    let val = rng.gen();
                    dram.write_u8(addr, val).unwrap();
                    assert!(dram.read_u8(addr).unwrap() == val, "Read/Write test on DRAM failed");
                },
                1 => {
                    let addr = rng.gen_range(0..16) << 1;
                    let val = rng.gen();
                    dram.write_u16(addr, val).unwrap();
                    assert!(dram.read_u16(addr).unwrap() == val, "Read/Write test on DRAM failed");
                },
                2 => {
                    let addr = rng.gen_range(0..8) << 2;
                    let val = rng.gen();
                    dram.write_u32(addr, val).unwrap();
                    assert!(dram.read_u32(addr).unwrap() == val, "Read/Write test on DRAM failed");
                },
                3 => {
                    let addr = rng.gen_range(0..4) << 3;
                    let val = rng.gen();
                    dram.write_u64(addr, val).unwrap();
                    assert!(dram.read_u64(addr).unwrap() == val, "Read/Write test on DRAM failed");
                },
                _ => unreachable!()
            }
        }
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
        let mut rng = rand::thread_rng();

        //   Test MustNotPurge rule

        let mut itlbr = mmu::Tlb::new(8, mmu::PurgeRule::MustNotPurge);

        for _ in 0..1024 {
            let vaddr_0 = rng.gen();
            let vaddr_1 = rng.gen();

            let idx_0 = rng.gen_range(0..8);
            let idx_1 = rng.gen_range(0..8);

            const ENTRY_0: mmu::TlbEntry = mmu::TlbEntry::new(0, true, true, false, false);
            const ENTRY_1: mmu::TlbEntry = mmu::TlbEntry::new(0, true, true, true, false);

            let insert_0 = itlbr.insert(
                idx_0, 
                vaddr_0, 
                ENTRY_0
            );
            let insert_1 = itlbr.insert(
                idx_1, 
                vaddr_1, 
                ENTRY_1
            );
            
            assert!(insert_0.is_some(), "First insertion failed");

            if idx_0 == idx_1 {
                assert!(insert_1.is_some(), "Insertion failed");
                
                assert!(itlbr.fetch(vaddr_0).is_none(), "Overwrite failed");
                assert!(itlbr.fetch(vaddr_1) == Some(ENTRY_1), "Overwrite failed");
            } else {
                if (vaddr_0 & !0xfff) == (vaddr_1 & !0xfff) {
                    assert!(itlbr.fetch(vaddr_0) == Some(ENTRY_0), "Overwrite occured");
                    assert!(itlbr.fetch(vaddr_1).is_none() , "Overwrite occured");

                    assert!(insert_1.is_none(), "Purge occured");
                } else {
                    assert!(itlbr.fetch(vaddr_0) == Some(ENTRY_0), "Overwrite occured");
                    assert!(itlbr.fetch(vaddr_1) == Some(ENTRY_1), "Overwrite occured");

                    assert!(insert_1.is_some(), "Overlap occured");
                }
            }
        }
        
        //   Test MustPurge rule

        let mut itlbc = mmu::Tlb::new(8, mmu::PurgeRule::MustPurge);

        for _ in 0..1024 {
            let vaddr_0 = rng.gen();
            let vaddr_1 = rng.gen();

            let idx_0 = rng.gen_range(0..8);
            let idx_1 = rng.gen_range(0..8);

            const ENTRY_0: mmu::TlbEntry = mmu::TlbEntry::new(0, true, true, false, false);
            const ENTRY_1: mmu::TlbEntry = mmu::TlbEntry::new(0, true, true, true, false);

            let insert_0 = itlbc.insert(
                idx_0, 
                vaddr_0, 
                ENTRY_0
            );
            let insert_1 = itlbc.insert(
                idx_1, 
                vaddr_1, 
                ENTRY_1
            );
            
            assert!(insert_0.is_some(), "First insertion failed");

            if idx_0 == idx_1 {
                assert!(insert_1.is_some(), "Insertion failed");
                
                assert!(itlbc.fetch(vaddr_0).is_none(), "Overwrite failed");
                assert!(itlbc.fetch(vaddr_1) == Some(ENTRY_1), "Overwrite failed");
            } else {
                if (vaddr_0 & !0xfff) == (vaddr_1 & !0xfff) {
                    assert!(itlbc.fetch(vaddr_0).is_none(), "Overwrite occured");
                    assert!(itlbc.fetch(vaddr_1) == Some(ENTRY_1), "Overwrite occured");

                    assert!(insert_1.is_some(), "Purge didnt occur");
                } else {
                    assert!(itlbc.fetch(vaddr_0) == Some(ENTRY_0), "Overwrite occured");
                    assert!(itlbc.fetch(vaddr_1) == Some(ENTRY_1), "Overwrite occured");

                    assert!(insert_1.is_some(), "Overlap occured");
                }
            }
        }
    }
}