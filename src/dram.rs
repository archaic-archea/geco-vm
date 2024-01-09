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