pub mod dram;
pub mod mmu;
pub mod cache;

enum CpuInstructionState {
    /// Fetch the instruction from memory
    Fetch,
    /// Decode the instruction
    Decode,
    /// Execute the instruction
    Execute,
    /// Perform needed memory accesses
    Memory,
    /// Perform needed register accesses
    RegisterWrite,
}

pub struct CpuState {
    /// State of instruction execution
    instr_state: CpuInstructionState,
    /// Instruction TLB cache, guaranteed to have at least 1 slot
    /// Must purge
    itlb: mmu::Tlb,
    /// Data TLB cache, guaranteed to have at least 1 slot
    /// Must purge
    dtlb: mmu::Tlb,
    /// Instruction TLB register, guaranteed to have at least 8 slots
    /// Must not purge
    itlbr: mmu::Tlb,
    /// Data TLB register, guaranteed to have at least 8 slots
    /// Must not purge
    dtlbr: mmu::Tlb,
}

impl CpuState {
    pub fn new(itlb_size: usize, dtlb_size: usize, itlbr_size: usize, dtlbr_size: usize) -> Self {
        assert!(itlb_size >= 1, "ITLB Must be at least 1 entry");
        assert!(dtlb_size >= 1, "DTLB Must be at least 1 entry");
        assert!(itlbr_size >= 8, "ITLBR Must be at least 8 entries");
        assert!(dtlbr_size >= 8, "DTLBR Must be at least 8 entries");

        Self {
            instr_state: CpuInstructionState::Fetch,
            itlb: mmu::Tlb::new(itlb_size, mmu::PurgeRule::MustPurge),
            dtlb: mmu::Tlb::new(dtlb_size, mmu::PurgeRule::MustPurge),
            itlbr: mmu::Tlb::new(itlbr_size, mmu::PurgeRule::MustNotPurge),
            dtlbr: mmu::Tlb::new(dtlbr_size, mmu::PurgeRule::MustNotPurge),
        }
    }

    /// Add Entry Register Instruction.
    /// Adds an entry to the specified ITLB register
    pub fn aer_i(&mut self, index: usize, vaddr: u64, entry: mmu::TlbEntry) -> Option<()> {
        self.itlb.inval(vaddr);
        self.itlbr.insert(index, vaddr, entry)
    }

    /// Add Entry Register Data.
    /// Adds an entry to the specified DTLB register
    pub fn aer_d(&mut self, index: usize, vaddr: u64, entry: mmu::TlbEntry) -> Option<()> {
        self.dtlb.inval(vaddr);
        self.dtlbr.insert(index, vaddr, entry)
    }
}