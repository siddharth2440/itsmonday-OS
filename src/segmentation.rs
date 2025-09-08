use core::fmt;


// Segement Selector := it specifies which element to load into a segment from the descriptor table( i.e., is a index to LDT or GDT with some flags)
#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SegmentSelector( pub u16 );


pub trait Segment {
    fn get_reg() -> SegmentSelector;
    unsafe fn set_reg(sel: SegmentSelector);
}

pub trait Segment64: Segment {
    // MSR:= A CPU register which is used for controlling hardware level instructions
    const BASE: Msr;  // contains our segment base. This MSR can be used to set the base(Model Specific Register)

    fn read_base() -> VirtAddr;  // READS the ssegment base address
    unsafe fn write_base(base: VirtAddress);

}

impl SegmentSelector {
    // rpl := Request Privilege Level
    // index := index in GDT or LDT
    #[inline]
    pub const fn new(index: u16, rpl: PrivilegeLevel) -> SegmentSelector {
        SegmentSelector( (index << 3) | (rpl as u16) )
    }

    pub const NULL: Self = Self::ne(0, PrivilegeLevel::Ring0);


    // returns GDT index
    #[inline]
    pub fn index(self) -> u16 {
        self.0 >> 3
    }

    // returns our requested privilege level
    #[inline]
    pub fn rpl(self) -> u16 {
        self.0 >> 3
    }

    #[inline]
    pub fn set_rpl(&mut self, rpl: PrivilegeLevel) {
        self.0.set_bits(0..2, rpl as u16)
    }

}

impl fmt::Debug for SegmentSelector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("SegmentSelector");
        s.field("index", &self.index());
        s.field("rpl", &self.rpl());
        s.finish()
    }
}

// most fileds in Code Segment Register are unused in 64-bit long mode, some of them must be set to a specific value
pub struct CS;

// Entirely unused in 64-bit long mode; setting the segment register does nothing
pub struct SS;

pub struct DS;

pub struct ES;


// Only base is used in 64-bit mode. This is often used in user-mode for TLS (Thread-Local Storage)
pub struct FS;

// Only base is used in 64-bit mode. In kernel-mode, the GS base often points to a per-cpu kernel structure
pub struct GS;