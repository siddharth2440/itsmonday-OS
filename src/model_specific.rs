#[cfg_attr(
    not(all(feature="instructions", target_arch = "x86_64")), 
    allow(dead_code))]
pub struct Msr(u32);


impl Msr {

    // New instance of Model Specific register
    pub const fn new(reg: u32) -> Msr {
        Msr(reg)
    }
}

// Extended Feature enable register 
#[derive(Debug)]
pub struct Efer;


// [FS].BASE model specific register
#[derive(Debug)]
pub struct FsBase;



#[cfg_attr(
    all(feature = "instructions", target_arch = "x86_64"), 
    doc = "[`GS-SWAP`] swaps this register with [`KernelGsBase`]."
)]
#[derive(Debug)]
pub struct GsBase;

#[cfg_attr(
    all(feature = "instructions", target_arch="x86_64"), 
    doc = "[`GS-Swap`] swaps this register with [`GsBase`]."
)]
#[derive(Debug)]
pub struct KernelGsBase;


#[derive(Debug)]
pub struct Star; // Syscall Register


#[derive(Debug)]
pub struct LStar; // Syscall Register


#[doc(alias = "FMask")]
#[derive(Debug)]
pub struct SFMask; // Syscall Register


#[derive(Debug)] 
pub struct Ucet;  // User Mode Enforcement Technology


#[derive(Debug)]
pub struct Scet;  // Supervised mode enforcement technology


#[derive(Debug)]
pub struct ApicBase; // Advanced Programmable interrupt controller


impl Efer {
    pub const MSR: Msr = Msr( 0xC000_0080 );
}

impl FsBase {
    pub const MSR: Msr = Msr( 0xC000_0100 );
}


impl GsBase {
    pub const MSR: Msr = Msr( 0xC000_0101 );
}

impl KernelGsBase {
    pub const MSR: Msr = Msr( 0xC000_0102 );
}


impl Star {
    pub const MSR: Msr = Msr( 0xC000_0081 );
}


impl LStar {
    pub const MSR: Msr = Msr( 0xC000_0082 );
}


impl SFMask {
    pub const MSR: Msr = Msr( 0xC000_0084 );
}


impl Ucet {
    pub const MSR: Msr = Msr( 0x6A0 );
}

impl Scet {
    pub const MSR: Msr = Msr( 0x6A2 );
}

impl ApicBase {
    pub const MSR: Msr = Msr( 0x1B );
}

// Continue from here......