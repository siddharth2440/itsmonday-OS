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

mod x86_64 {
    use core::fmt;

    use crate::{addr::VirtualAddr, model_specific::{Efer, EferFlags, FsBase, GsBase, KernelGsBase, LStar, Msr, SFMask, Star, Ucet}, segmentation::SegmentSelector};

    impl Msr {
        // reads 64 bits msr register
        #[inline]
        pub unsafe fn read(&self) -> u64 {
            let ( high, low ): (u32, u32);
            unsafe {
                asm!(
                    "rdmsr" // read MSR(model specific register)
                    in("ecx") self.0
                    out("eax") low, out("edx") high,
                    options(nomem, nostack, preserves_flags),
                )
            }
            ((high as u64) << 32 ) | ((low as u64))
        }

        // write 64 bits to msr registers
        #[inline]
        pub unsafe fn write(&mut self, value: u64) {
            let low = value as u32;
            let high = (value >> 32) as u32;

            unsafe {
                asm!(
                    "wrmsr",
                    in("ecx") self.0,
                    out("eax") low, in("edx") high,
                    options(nostack, preserves_flags),
                )
            }
        }
    }

    impl Efer {
        #[inline]
        pub fn read() -> EferFlags {
            EferFlags::from_bits_truncate(Self::read_raw())
        }


        // Reads the current Efer flags
        #[inline]
        pub fn read_raw() {
            unsafe {
                Self::MSR.read()
            }
        }


        #[inline]
        pub unsafe fn write(flags: EferFlags){
            let old_value = Self::read_raw();
            let reserved = old_value & !(EferFlags::all().bits());
            let new_value = reserved | flags.bits();

            unsafe {
                Self::write_raw(new_value)
            }
        }

        #[inline]
        pub unsafe fn write_raw(flags: u64) {
            let mut msr = Self::MSR;
            unsafe {
                msr.write(flags);
            }
        }

        #[inline]
        pub unsafe fn update<F>(f: F) where
        F: FnOnce(&mut EferFlags) {
            let mut flags = Self::read();
            f(&mut flags);
            unsafe {
                Self::write(flags);
            }
        }
    }


    impl FsBase {

        // read
        #[inline]
        pub fn read() -> VirtAddress {
            VirtAddress::new( unsafe {
                Self::MSR.read()
            })
        }
        
        // write
        #[inline]
        pub fn write(address: VirtualAddr) {
            let mut msr = Self::MSR;
            unsafe {
                msr.write(address.as_u64());
            }
        }
    }


    impl GsBase {

        #[inline]
        pub fn read() -> VirtualAddr {
            VirtualAddr::new(unsafe{
                Self::MSR.read()
            })
        }

        #[inline]
        pub unsafe fn write(address: VirtualAddr) {
            let mut msr = Self::MSR;
            unsafe {
                msr.write(address.as_u64());
            }
        }
    }


    impl KernelGsBase {

        #[inline]
        pub fn read() -> VirtualAddr {
            VirtualAddr::new(unsafe{
                Self::MSR.read()
            })
        }

        #[inline]
        pub unsafe fn write(address: VirtualAddr) {
            let mut msr = Self::MSR;
            unsafe {
                msr.write(address.as_u64());
            }
        }
    }

    impl Star {
        #[inline]
        pub fn read_raw() -> (u16, u16) {
            let msr_value = unsafe {
                Self::MSR.read()
            };
            let sysret = msr_value.get_bits(48..64);
            let syscall = msr_value.get_bits(32..48);
            ( sysret.try_into().unwrap(), syscall.try_into().unwrap() )
        }

        #[inline]
        pub fn read() -> 
        (   SegmentSelector,  // CS Selector
            SegmentSelector,  // SS Selector
            SegmentSelector,  // CS Selector
            SegmentSelector   // SS Selector
        ) {
            let raw = Self::read_raw();
            (
                SegmentSelector(raw.0 + 16),
                SegmentSelector(raw.0 + 8),
                SegmentSelector(raw.1),
                SegmentSelector(raw.1 + 8)
            )
        }

        #[inline]
        pub fn write_raw(sysret: u16, syscall: u16) {
            let mut msr_value = 0u64;
            msr_value.set_bits(48..64, sysret.into());
            msr_value.set_bits(32..48, syscall.into());
            let mut msr = Self::MSR;
            unsafe {
                msr.write(msr_value);
            }
        }

        #[inline]
        pub fn write(
            cs_sysret: SegmentSelector,
            ss_sysret: SegmentSelector,
            cs_syscall: SegmentSelector,
            ss_syscall: SegmentSelector
        ) -> Result<(), InvalidStarSegmentSelectors> {
            let cs_sysret_cmp = i32::from(cs_sysret.0) - 16;
            let ss_sysret_cmp = i32::from(ss_sysret.0) - 8;
            let cs_syscall_cmp = i32::from(cs_syscall.0);
            let ss_syscall_cmp = i32::from(ss_syscall.0) - 8;

            if cs_sysret_cmp != ss_sysret_cmp {
                return Err(InvalidStarSegmentSelectors::SysretOffset);
            }

            if cs_syscall_cmp != ss_syscall_cmp {
                return Err(InvalidStarSegmentSelectors::SyscallOffset);
            }

            if ss_sysret.rpl() != PrivilegeLevel::Ring3 {
                return Err(InvalidStarSegmentSelectors::SyscallPrivilegeLevel);
            }

            if ss_syscall.rpl() != PrivilegeLevel::Ring0 {
                return Err(InvalidStarSegmentSelectors::SyscallPrivilegeLevel);
            }

            unsafe {
                Self::write_raw(ss_sysret.0 - 8, cs_syscall.0);
            }

        }

    }


    #[derive(Debug)]
    pub enum InvalidStarSegmentSelectors{
        SysretOffset,
        SyscallOffset,
        SysretPrivilegeLevel,
        SyscallPrivilegeLevel
    }

    impl fmt::Display for InvalidStarSegmentSelectors {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::SysretOffset => write!(f, "Sysret CS and SS are not offset by 8."),
                Self::SyscallOffset => write!(f, "Syscall CS and SS are not offset by 8."),
                Self::SysretPrivilegeLevel => write!(f, "Sysret segment must be a Ring3 segment."),
                Self::SyscallPrivilegeLevel => write!(f, "Syscall's segment must be Ring0 segment."),
            }
        }
    }

    impl LStar {
        #[inline]
        pub fn read() -> VirtualAddr {
            VirtualAddr::new(unsafe{Self::MSR.read()})
        }

        pub fn write(address: VirtualAddr) {
            let mut msr = Self::MSR;
            unsafe { msr.write(address.as_u64()); }
        }
    }

    impl SFMask {
        #[inline]
        pub fn read() -> RFlags {
            RFlags::from_bits(unsafe {
                Self::MSR.read()
            }).unwrap()
        }

        #[inline]
        pub fn write(value: RFlags) {
            let mut msr = Self::MSR;
            unsafe { msr.write(value.bits()) };
        }

        #[inline]
        pub fn update<F>(f: F)
        where
            F: FnOnce(&mut RFlags)
        {
            let mut flags = Self::read();
            f(&mut flags);
            Self::write(flags);
        }
    }

    impl Ucet {

        #[inline]
        pub fn read_raw() -> u64 {
            unsafe { Self::MSR.read() }
        }

        #[inline]
        pub fn write_raw(value: u64) {
            let mut msr = Self::MSR;
            unsafe {
                msr.write(value);
            }
        }

        #[inline]
        pub fn read() -> (CetFlags, Page) {
            let value = Self::read_raw();
            let cet_flags = CetFlags::from_bits_truncate(value);
            let legacy_bitmap = Page::from_start_address(VirtualAddr::new( value & !(Page::<Sinze4KiB>SIZE -1) )).unwrap();

            (cet_flags, legacy_bitmap)
        }

        #[inline]
        pub fn write(flas: CetFlags, legacy_bitmap: Page) {
            Self::write_raw(flas.bits() | legacy_bitmap.start_address().as_64());
        }

        #[inline]
        pub fn update<F>(f: F)
        where
            F: FnOnce(&mut CetFlags, &mut Page)
        {
            let (mut flags, mut legacy_bitmap) = Self::read();
            f(&mut flags, &mut legacy_bitmap);
            Self::write(flags, legacy_bitmap);
        }
    }

}