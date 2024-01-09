use geco_vm::*;

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
        let mut itlb = mmu::Tlb::new(1);
        itlb.insert(
            0, 
            mmu::TlbEntry::new(0, true, true, false, false)
        );

        assert!(itlb.fetch(0).unwrap() == mmu::TlbEntry::new(0, true, true, false, false), "Failed to fetch TLB entry");
    }
}