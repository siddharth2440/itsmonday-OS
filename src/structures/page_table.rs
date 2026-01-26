use core::sync::atomic::AtomicU64;

use bitflags::bitflags;

use crate::addr::PhyAddr;

#[derive(Debug, Clone, Copy)]

// Error returened by the FrameEntry.
pub enum FrameError {
    FrameNotPresent,
    HugeFrame
}


pub(crate) static PHYSICAL_ADDRESS_MASK: AtomicU64 = AtomicU64::new(0x000f_ffff_ffff_f000_u64);

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct PageTableEntry {
    entry: u64
}


impl PageTableEntry {
    
    #[inline]
    pub fn new() -> Self {
        Self { entry: 0 }
    }

    #[inline]
    pub const fn is_unused(&self) -> bool {
        return self.entry == 0;
    }

    #[inline]
    #[const_fn(cfg(not( feature = "memory_encryption")))]
    pub fn set_unused(&mut self) {
        self.entry = 0;
    }

    #[inline]
    #[const_fn(cfg(not( feature = "memory_encryption")))]
    pub const fn flags(&self) -> PageTableFlags {
        PageTableFlags::from_bits_retain(self.entry & !Self::physical)
    }


    // returns PA mapped by this entry.
    #[inline]
    pub fn addr(&self) -> PhyAddr {
        PhyAddr::new(self.entry & )
    }


    pub fn 




}


bitflags! {
    pub struct PageTableFlags: u64 {
        const PRESENT =               1;
        const WRITABLE =              1<<1;
        const USERACCESSIBLE =        1<<2;
        const WRITETHROUGH =          1<<3;
        const NOCACHE =               1<<4;
        const ACCESSED =              1<<5;
        const DIRTY =                 1<<6;
        const HUGE_PAGE =             1<<7;
        const GLOBAL =                1<<8;
        const BIT9 =                  1<<9;
        const BIT10 =                 1<<10;
        const BIT11 =                 1<<11;
        const BIT52 =                 1<<52;
        const BIT53 =                 1<<53;
        const BIT54 =                 1<<54;
        const BIT55 =                 1<<55;
        const BIT56 =                 1<<56;
        const BIT57 =                 1<<57;
        const BIT58 =                 1<<58;
        const BIT59 =                 1<<59;
        const BIT60 =                 1<<60;
        const BIT61 =                 1<<61;
        const BIT62 =                 1<<62;
        const NO_EXECUTE =            1<<63;
    }
}


const ENTRY_COUNT: usize = 512;


#[repr(align(4096))]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PageTable {
    entries: [PageTableEntry; ENTRY_COUNT],
}


impl PageTable {

    #[inline]
    pub const fn new() -> Self {
        const EMPTY: PageTableEntry = PageTableEntry::new();
        Self { entries: [EMPTY; ENTRY_COUNT] }
    }
}