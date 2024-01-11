pub mod dram;
pub mod mmu;
pub mod cache;

pub struct CpuState {
    itlb: mmu::Tlb,
    dtlb: mmu::Tlb,
    /// Instruction TLB register, guaranteed to have at least 8 slots
    itlbr: mmu::Tlb,
    /// Data TLB register, guaranteed to have at least 16 slots
    dtlbr: mmu::Tlb,
}

impl CpuState {
    /// Add Entry Register Instruction.
    /// Adds an entry to the specified ITLB register
    pub fn aer_i(&mut self, index: usize, vaddr: u64, entry: mmu::TlbEntry) -> Option<()> {
        self.itlbr.insert(index, vaddr, entry)
    }
}