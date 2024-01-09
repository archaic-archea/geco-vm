use std::sync::Mutex;

pub struct Dram<'a> {
    pub phys_ram: Mutex<&'a mut [Ramcell]>
}

impl<'a> Dram<'a> {
    pub fn new(size: usize) -> Self {
        use std::alloc::{alloc, Layout};
        let ramcell = unsafe {
            alloc(Layout::from_size_align(size, 8)
                .unwrap_or_else(|_| {panic!("Unaligned DRAM size {:?}", size)})
            ) as *mut Ramcell
        };

        let ramcells = unsafe {core::slice::from_raw_parts_mut(ramcell, size / 8)};

        if ramcell.is_null() {
            panic!("DRAM allocation failed, aborting");
        } else {
            Self {
                phys_ram: Mutex::new(ramcells)
            }
        }
    }

    /// Reads a u8 from DRAM. Cannot actually return None as it cannot be unaligned
    pub fn read_u8(&self, addr: u64) -> Option<u8> {
        let ramcell: [u8; 8] = self.phys_ram.lock().unwrap()[(addr >> 3) as usize].0;
        Some(ramcell[(addr & 0b111) as usize])
    }

    /// Reads a u16 from DRAM. Returns None if the address is unaligned
    pub fn read_u16(&self, addr: u64) -> Option<u16> {
        if (addr & 0b1) != 0 {
            return None;
        }

        let ramcell: [u16; 4] = bytemuck::cast(self.phys_ram.lock().unwrap()[(addr >> 3) as usize].0);
        Some(ramcell[((addr >> 1) & 0b11) as usize])
    }

    /// Reads a u32 from DRAM. Returns None if the address is unaligned
    pub fn read_u32(&self, addr: u64) -> Option<u32> {
        if (addr & 0b11) != 0 {
            return None;
        }
        
        let ramcell: [u32; 2] = bytemuck::cast(self.phys_ram.lock().unwrap()[(addr >> 3) as usize].0);
        Some(ramcell[((addr >> 2) & 0b1) as usize])
    }

    /// Reads a u64 from DRAM. Returns None if the address is unaligned
    pub fn read_u64(&self, addr: u64) -> Option<u64> {
        if (addr & 0b111) != 0 {
            return None;
        }
        
        let ramcell: [u64; 1] = bytemuck::cast(self.phys_ram.lock().unwrap()[(addr >> 3) as usize].0);
        Some(ramcell[0])
    }

    /// Writes a u8 to DRAM. Cannot actually return None as it cannot be unaligned
    pub fn write_u8(&mut self, addr: u64, val: u8) -> Option<()> {
        let ramcell: &mut [u8; 8] = &mut self.phys_ram.lock().unwrap()[(addr >> 3) as usize].0;
        ramcell[(addr & 0b111) as usize] = val;
        Some(())
    }

    /// Writes a u16 to DRAM. Returns None if the address is unaligned
    pub fn write_u16(&mut self, addr: u64, val: u16) -> Option<()> {
        if (addr & 0b1) != 0 {
            return None;
        }

        let ramcell: &mut [u16; 4] = unsafe {std::mem::transmute(&mut self.phys_ram.lock().unwrap()[(addr >> 3) as usize].0)};
        ramcell[((addr >> 1) & 0b11) as usize] = val;
        Some(())
    }

    /// Writes a u32 to DRAM. Returns None if the address is unaligned
    pub fn write_u32(&mut self, addr: u64, val: u32) -> Option<()> {
        if (addr & 0b11) != 0 {
            return None;
        }

        let ramcell: &mut [u32; 2] = unsafe {std::mem::transmute(&mut self.phys_ram.lock().unwrap()[(addr >> 3) as usize].0)};
        ramcell[((addr >> 2) & 0b1) as usize] = val;
        Some(())
    }

    /// Writes a u64 to DRAM. Returns None if the address is unaligned
    pub fn write_u64(&mut self, addr: u64, val: u64) -> Option<()> {
        if (addr & 0b111) != 0 {
            return None;
        }

        let ramcell: &mut [u64; 1] = unsafe {std::mem::transmute(&mut self.phys_ram.lock().unwrap()[(addr >> 3) as usize].0)};
        ramcell[0] = val;
        Some(())
    }

    pub fn size(&self) -> usize {
        self.phys_ram.lock().unwrap().len() * 8
    }
}

impl<'a> Drop for Dram<'a> {
    fn drop(&mut self) {
        let mut dram = self.phys_ram.lock().unwrap();
        let dram_size = dram.len();
        let dram_ptr = dram.as_mut_ptr();

        unsafe {
            std::alloc::dealloc(
                dram_ptr as *mut u8, 
                std::alloc::Layout::from_size_align(dram_size, 8).unwrap()
            )
        }
    }
}

#[repr(align(8))]
#[derive(Debug)]
pub struct Ramcell([u8; 8]);