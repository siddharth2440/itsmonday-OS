use bitflags::bitflags;

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
bitflags! {
    #[repr(transparent)]
    #[derive(Debug)]
    pub struct EferFlags: u64 {
        // SCE = Enables use of `syscall` and `sysret` instructions.
        const SYSTEM_CALL_EXTENSIONS            = 1;

        // LME = Enables 64 bit mode when paging is enable
        const LONG_MODE_ENABLE                  = 1 << 8;

        // LMA = Indicates that processor is currently in long mode
        const LONG_MODE_ACTIVE                  = 1 << 10;

        // NXE = Enables the NX bit in Page tables for memory protection
        const NO_EXECUTE_ENABLE                 = 1 << 11;

        // SVME = enables AMD sVM virtualization feature
        const SECURE_VIRTUAL_MACHINE_ENABLE     = 1 << 12;
        
        // LMSE = Enables segment limit checks in long mode
        const LONG_MODE_SEGMENT_ENABLE          = 1 << 13;

        // Optimizes saving/restoring x87/SSE state
        const FAST_FXSAVE_FXRSTOR               = 1 << 14;
        const TRANSLATION_CACHE_EXTENSION       = 1 << 15;
    }
}

bitflags! {

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct Cetflags: u64 {
        // SS(shadow stack) = a protected stack that mirrors return addresses to detect tampering
        // IBT(Indirect Branch Tracking) = ensures that indirect branches(like function pointers or virtual calls) land only on vvalid targets


        // enables shadow stack enforcement
        const SS_ENABLE                     = 1 << 0;

        // Enables WRSS instruction to write to the ss        
        const SS_WRITE_ENABLE               = 1 << 1;

        // Activates indirect branch tracking
        const IBT_ENABLE                    = 1 << 2;

        // Allows legacy code to opt into IBT tracking
        const IBT_LEGACY_ENABLE             = 1 << 3;
        
        // Disable tracking for certain indirect branches
        const IBT_NO_TRACK_ENABLE           = 1 << 4;

        // Supresses IBT enforcement for legacy code paths
        const IBT_LEGACY_SUPPRESS_ENABLE    = 1 << 5;

        // Possibly a super override for legacy supression name suggests full bypass
        const IBT_LEGACY_SUPREME_ENABLE     = 1 << 10;

        // Marks code regions as IBT-tracked- likely used for runtime validation
        const IBT_TRACKED                   = 1 << 11;
    }
}


bitflags! {

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct ApicBaseFlags: u64 {
        // bits 0 - 7 are reserved

        // Indicates whether the processor is bootstrap processor or not
        const BSP = 1 << 8;

        // bit 9 is also reserved

        // places our local APIC in the x2APIC mode.
        const X2APIC_ENABLE = 1 << 10;

        // Enables or Disables the local Apic
        const LAPIC_ENABLE = 1 << 11; 
    }
}