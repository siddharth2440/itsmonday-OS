// Interrupt Descriptor Table

use core::{fmt::{self, write}, marker::PhantomData, ops::{Bound, Deref, IndexMut, Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive}};

use bitflags::bitflags;
use volatile::Volatile;

use crate::{segmentation::{SegmentSelector, CS}, DescriptorTablePointer};

#[repr(C)]
pub struct InterruptDescriptorTable {
    pub divide_error:                 Entry<HandlerFunc>,
    pub debug:                          Entry<HandlerFunc>,
    pub non_maskable_interrupt:         Entry<HandlerFunc>,
    pub breakpoint:                     Entry<HandlerFunc>,
    pub overflow:                       Entry<HandlerFunc>,
    pub bound_range_exceeded:           Entry<HandlerFunc>,
    pub invalid_opcode:                 Entry<HandlerFunc>,
    pub device_not_available:           Entry<HandlerFunc>,
    pub double_fault:                   Entry<HandlerFuncWithErrCode>,
    coprocessor_segment_overrun:        Entry<HandlerFunc>,
    pub invalid_tss:                    Entry<HandlerFuncWithErrCode>,
    pub segment_not_present:            Entry<HandlerFuncWithErrCode>,
    pub stack_segment_fault:            Entry<HandlerFuncWithErrCode>,
    pub general_protection_fault:       Entry<HandlerFuncWithErrCode>,
    pub page_fault:                     Entry<PageFaultHandlerFunc>,
    reserved_1:                         Entry<HandlerFunc>,
    pub x87_floating_point_exception:   Entry<HandlerFunc>,
    pub alignment_chk:                  Entry<HandlerFuncWithErrCode>,
    pub machine_chk:                    Entry<HandlerFunc>,
    pub simd_floating_point:            Entry<HandlerFunc>,
    pub virtualization:                 Entry<HandlerFunc>,
    pub cp_protection_exception:        Entry<HandlerFuncWithErrCode>,
    reserved_2:                         [Entry<HandlerFunc>; 6],
    pub hv_injection_exception:         Entry<HandlerFunc>,
    pub vmm_communication_exception:    Entry<HandlerFuncWithErrCode>,
    pub security_exception:             Entry<HandlerFuncWithErrCode>,
    reserved_3:                         Entry<HandlerFuncWithErrCode>,
    interrupts:                         [Entry<HandlerFunc>; 256 - 32]
}

impl InterruptDescriptorTable {

        #[inline]
        pub fn new() -> InterruptDescriptorTable {
            InterruptDescriptorTable { 
                divide_error: Entry::missing(), 
                debug: Entry::missing(), 
                non_maskable_interrupt: Entry::missing(), 
                breakpoint: Entry::missing(), 
                overflow: Entry::missing(), 
                bound_range_exceeded: Entry::missing(), 
                invalid_opcode: Entry::missing(), 
                device_not_available: Entry::missing(), 
                double_fault: Entry::missing(), 
                coprocessor_segment_overrun: Entry::missing(), 
                invalid_tss: Entry::missing(), 
                segment_not_present: Entry::missing(), 
                stack_segment_fault: Entry::missing(), 
                general_protection_fault: Entry::missing(), 
                page_fault: Entry::missing(), 
                reserved_1: Entry::missing(), 
                x87_floating_point_exception: Entry::missing(), 
                alignment_chk: Entry::missing(), 
                machine_chk: Entry::missing(), 
                simd_floating_point: Entry::missing(),
                virtualization: Entry::missing(), 
                cp_protection_exception: Entry::missing(), 
                reserved_2: [ Entry::missing(); 6 ], 
                hv_injection_exception: Entry::missing(), 
                vmm_communication_exception: Entry::missing(), 
                security_exception: Entry::missing(), 
                reserved_3: Entry::missing(), 
                interrupts: [Entry::missing(); 256 - 32 ]
            }
        }

        // resset our idt
        pub fn reset( &mut self ) {
            *self = Self::new();
        }

        pub fn load(&'static self) {
            unsafe { self.load_unsafe(); }
        }

        pub fn load_unsafe(&self) {
            unsafe {
                lidt(&self.pointer());
            }
        }

        #[cfg(all( feature = "instructions", target_arch="x86_64" ))]
        pub fn pointer(&self) -> DescriptorTablePointer {
            use core::mem::size_of;
            DescriptorTablePointer { 
                limit: (size_of::<Self>() - 1) as u16,
                base: VirtAddr::new(self as *const _ as u64)
            }
        }


        // retusn a normalized and ranged chekc slice from a tangebounds trait object.
        // panics if the entry is an exception
        fn condition_slice_bounds(&self, bounds: impl RangeBounds<u8>) -> (usize, usize) {
            let lower_idx = match bounds.start_bound() {
                core::ops::Bound::Included(start) => usize::from(*start),
                core::ops::Bound::Excluded(start) => usize::from(*start) + 1,
                core::ops::Bound::Unbounded => 0,
            };

            let upper_idx = match bounds.end_bound() {
                core::ops::Bound::Included(end) => usize::from(*end),
                core::ops::Bound::Excluded(end) => usize::from(*end) + 1,
                core::ops::Bound::Unbounded => 256
            };

            if lower_idx  < 32 {
                panic!("cannot return from traps, faults, and exception handlers");
            }

            ( lower_idx, upper_idx )
        }
        

        // returns slice of IDT with the specified range
        #[inline]
        pub fn slice(&self, bounds: impl RangeBounds<u8>) -> &[Entry<HandlerFunc>] {
            let ( lower_idx, upper_idx ) = self.condition_slice_bounds(bounds);
            &mut self.interrupts[ (lower_idx - 32)..(upper_idx - 32) ] 
        }


        #[inline]
        pub fn slice_mut(&mut self, bounds: impl RangeBounds<u8>) -> &mut [Entry<HandlerFunc>] {
            let ( lower_idx, upper_idx ) = self.condition_slice_bounds(bounds);
            &mut self.interrupts[ (lower_idx - 32)..(upper_idx - 32) ] 
        }
}

impl default for InterruptDescriptorTable {
    #[inline]
    fn default() -> Self {
        Self::new();
    }
}

impl Index<u8> for InterruptDescriptorTable {
    type Output = Entry<HandlerFunc>;

    #[inline]
    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 =>    &self.divide_error,
            1 =>    &self.debug,
            2 =>    &self.non_maskable_interrupt,
            3 =>    &self.breakpoint,
            4 =>    &self.overflow,
            5 =>    &self.bound_range_exceeded,
            6 =>    &self.invalid_opcode,
            7 =>    &self.device_not_available,
            9 =>    &self.coprocessor_segment_overrun,
            16 =>   &self.x87_floating_point_exception,
            19 =>   &self.simd_floating_point,
            20 =>   &self.virtualization,
            28 =>   &self.hv_injection_exception,
            i @ 32..=255 =>         &self.interrupts[usize::from(i) - 32],
            i @ 15 | i @ 31 | i @ 22..27 =>     panic!("entry {} is reserved", i),
            i @ 8 | i @ 10..=14 | i @ 17 | i @ 21 | i @ 29 | i @ 30 => {
                panic!("entry {} is an exception with wrror code", i)
            }
            i @ 18 => panic!("entry {} us an diverging exception (must not return)", i),
        }
    }
}

impl IndexMut<u8> for InterruptDescriptorTable {
    #[inline]
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        match index {
            0 =>    &mut self.divide_error,
            1 =>    &mut self.debug,
            2 =>    &mut self.non_maskable_interrupt,
            3 =>    &mut self.breakpoint,
            4 =>    &mut self.overflow,
            5 =>    &mut self.bound_range_exceeded,
            6 =>    &mut self.invalid_opcode,
            7 =>    &mut self.device_not_available,
            9 =>    &mut self.coprocessor_segment_overrun,
            16 =>   &mut self.x87_floating_point_exception,
            19 =>   &mut self.simd_floating_point,
            20 =>   &mut self.virtualization,
            28 =>   &mut self.hv_injection_exception,
            i @ 32..=255 =>         &mut self.interrupts[usize::from(i) - 32],
            i @ 15 | i @ 31 | i @ 22..27 =>     panic!("entry {} is reserved", i),
            i @ 8 | i @ 10..=14 | i @ 17 | i @ 21 | i @ 29 | i @ 30 => {
                panic!("entry {} is an exception with wrror code", i)
            }
            i @ 18 => panic!("entry {} us an diverging exception (must not return)", i),
        }
    }
}


macro_rules! impl_idx_for_idt {
    ($ty: ty) => {
        impl Index<$ty> for InterruptDescriptorTable {
            type Output = [Entry<HandlerFunc>];

            #[inline]
            fn index(&self, index: $ty) -> &Self::Output {
                self.slice(index)
            }

            impl IndexMut<$ty> for InterruptDescriptorTable {
                
                #[inline]
                fn index_mut(&mut self, index: $ty) -> &mut Self::Output {
                    self.slice_mut(index)
                }
            }
        }
    };
}


impl_idx_for_idt!(( Bound<&u8>, Bound<&u8> ));
impl_idx_for_idt!(( Bound<u8>, Bound<u8> ));
impl_idx_for_idt!(Range<&u8>);
impl_idx_for_idt!(Range<u8>);
impl_idx_for_idt!(RangeFrom<&u8>);
impl_idx_for_idt!(RangeFrom<u8>);
impl_idx_for_idt!(RangeInclusive<&u8>);
impl_idx_for_idt!(RangeInclusive<u8>);
impl_idx_for_idt!(RangeTo<&u8>);
impl_idx_for_idt!(RangeTo<u8>);
impl_idx_for_idt!(RangeToInclusive<&u8>);
impl_idx_for_idt!(RangeToInclusive<u8>);
impl_idx_for_idt!(RangeFull);


#[derive(Clone, Copy)]
#[repr(C)]
pub struct Entry<F> {
    pointer_low: u16,
    pointer_high: u16,
    options: EntryOptions,
    pointer_middle: u16,
    reserved: u32,
    phantom: PhantomData<F>
}


impl<T> fmt::Debug for Entry<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Entry")
        .field("handler_addr", &format_args!("{:x}", self.handler_addr()))
        .field("options", &self.options)
        .finish()
    }
}


impl<T> PartialEq for Entry<T> {
    fn eq(&self, other: &Self) -> bool {
        self.pointer_low == other.pointer_low
        && self.options == other.options
        && self.pointer_high == other.pointer_high
        && self.reserved == other.reserved
    }
}

// HandlerFunc

#[cfg(all(
    any(target_arch="x86", target_arch="x86_64", feature = "abi_x86_interrupt"),
    feature = "abi_x86_interrupt"
))]
pub type HandlerFunc = extern "x86-interrupt" fn(InterruptStackFrame);

#[cfg(not(all(
    any(target_arch="x86", target_arch="x86_64"),
    feature = "abi_x86_interrupt"
)))]
#[derive(Debug, Copy, Clone)]
pub struct HandlerFunc(());



// HandleFuncWithErrorCode
#[cfg(all(
    any(target_arch="x86", target_arch="x86_64"),
    feature = "abi_x86_interrupt"
))]
pub type HandlerFuncWithErrCode = extern "x86-interrupt" fn(InterruptStackFrame, error_code: u64);

#[cfg(not(all(
    any(target_arch="x86", target_arch="x86_64"),
    feature = "abi_x86_interrupt"
)))]

#[derive(Debug, Clone, Copy)]
pub struct HandlerFuncWithErrCode(());

// Page fault handler function
#[cfg(all(
    any(target_arch="x86", target_arch="x86_64"),
    feature = "abi_x86_interrupt"
))]
pub type PageFaultHandlerFunc = extern "abi_x86_interrupt" fn(InterruptStackFrame, error_code: PageFaultErrorCode);

#[cfg(not(all(
    any(target_abi="x86", target_abi="x86_64"),
    feature = "abi_x86_interrupt"
)))]
#[derive(Debug, Clone, Copy)]
pub struct PageFaultHandlerFunc(());

// Diverging function handler
#[cfg(all(
    any(target_arch="x86", target_arch="x86_64"),
    feature = "abi_x86_interrupt"
))]
pub type DivergingHandlerFunc = extern "x86-interrupt" fn(InterruptStackFrame) -> !;


#[cfg(not(all(
    any( target_arch="x86", target_arch="x86_64"),
    feature = "abi_x86_interrupt"
)))]
#[derive(Debug, Clone, Copy)]
pub struct DivergingHandlerFunc(());


// Diverging Handler function for ErrorCode
#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    feature = "abi_x86_interrupt"
))]
pub type DivergingHandlerFuncWithErrCode = extern "x86_64" fn(InterruptStackFrame, error_code: u64) -> !;

#[cfg(not(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    feature = "abi_x86_interrupt"
)))]
#[derive(Debug, Clone, Copy)]
pub struct DivergingHandlerFuncWithErrCode(());


// Entry point
pub type GeneralHandlerFunc = fn(InterruptStackFrame, index: u8, error_code: Option<u64>);

impl<T> GeneralHandlerFunc<T> {
    pub const fn missing() -> Self {
        Entry {
            pointer_low: 0,
            pointer_middle: 0,
            pointer_high: 0,
            options: EntryOptions::minimal(),
            reserved: 0,
            phantom: PhantomData
        }
    }

    // Sets the handle funtiofor each idt entry and sets the following default
    // 1. The code selector is the code segment currently active in the CPU 
    // 2. Present bit set
    // 3. Interrupts are disables on handler invocation 
    // 4. Pivilege level Ring0 
    // 4. No IST is configured (existing stack will be used).. 

    #[cfg(all(feature = "instructions", target_arch = "x86_64"))]
    #[inline]
    pub unsafe fn set_handler_func(&mut self, addr: VirtAddr) -> &mut EntryOptions {
        let addr = addr.as_u64();

        self.pointer_low = addr as u16;
        self.pointer_high = ( addr >> 32 ) as u32;
        self.pointer_middle = ( addr >> 16 ) as u16;

        self.options = EntryOptions::minimal();

        unsafe {
            self.options.set_code_selector(CS::get_reg())
        }
        self.options.set_present(true);
        &mut self.options
    }

    #[inline]
    pub fn handler_addr(&self) -> VirtAddr {
        let addr = self.pointer_low as u64 | ((self.pointer_high as u64) >> 32 ) | ((self.pointer_middle as u64) >> 16 );
        VirtAddr::new_truncate(addr)
    }

}


// common trait for all handler function
pub unsafe trait HandlerFuncType {
    fn to_virt_addr(self) -> VirtAddr;
}


impl <F: HandlerFuncType> Entry<F> {
    pub fn set_handler_func(&mut self, handler: F) -> &mut EntryOptions {
        unsafe {
            self.set_handler_addr(handler.to_virt_addr())
        }
    }
}


macro_rules! impl_handler_func_type {
    ($f: ty) => {
        #[cfg(all(
            any(target_arch="x86" or target_arch="x86_64"),
            feature = "abi_x64_interrupt"
        ))]
        unsafe impl HandlerFuncType for $f {
            #[inline]
            fn to_virt_addr(self) -> VirtAddr {
                #[cfg_attr(
                    any( target_pointer_width: "32", target_pointer_width = "64" ),
                    allow(clippy::fn_to_numeric_casr)
                )]
                VirtAddr::new(self as u64)
            }
        }
    };
}


impl_handler_func_type!(HandlerFunc);
impl_handler_func_type!(HandlerFuncWithErrCode);
impl_handler_func_type!(PageFaultHandlerFunc);
impl_handler_func_type!(DivergingHandlerFunc);
impl_handler_func_type!(DivergingHandlerFuncWithErrCode);


// represents the 4 non-offset bytes for an IDT entry
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EntryOptions{
    cs: SegmentSelector,
    bits: u16
}

impl fmt::Debug for  EntryOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EntryOptions")
        .field("code_selector", &self.cs)
        .field("stack_index", &self.stack_index())
        .field("tyoe", &format_args!("{:04b}", self.bits.get_bits(8..12)))
        .field("privilege_level", &self.privilege_level())
        .field("present", &self.present())
        .finish()
    }
}

impl EntryOptions {
    
    const fn minimal() -> Self {
        EntryOptions {
            cs: SegmentSelector(0), 
            bits: 0b1110_0000_0000
        }
    }

    // function will be unsafe bcz the caller must ensure that the passed segment 
    // selector points to a valid, long-mode code segment.
    #[inline]
    pub unsafe fn set_code_selector( &mut self, cs: SegmentSelector ) -> &mut Self {
        self.cs = cs;
        self
    }


    // set or reset the present bit
    #[inline]
    pub fn set_present(&mut self, present: bool) -> &mut Self {
        self.bits.set_bit(15, present);
        self
    }


    #[inline]
    pub fn present(&self) -> bool {
        self.bits.get_bit(15)
    }

    #[inline]
    pub fn disable_interrupts(&mut self, disable: bool) -> &mut Self {
        self.bits.set_bit(8, !disable);
        self
    }

    #[inline]
    pub fn set_privilege_level(&mut self, dpl: PrivilegeLevel) -> &mut Self {
        self.bits.set_bits(13..15, dpl as u16);
        self
    }

    #[inline]
    pub fn privilege_level( &self ) -> PrivilegeLevel {
        PrivilegeLevel::from_u16(self.bits.get_bits(13..15))
    }

    #[inline]
    pub unsafe fn set_stack_index(&mut self, index: u16) -> &mut Self {
        self.bits.set_bits(0..3, index+1);
        self
    }

    fn stack_index(&self) -> Option<u16> {
        self.bits.get_bits(0..3).checked_sub(1)
    }


}


// represents the interrupt stack frame pushed by the CPU on the interrupt or exception entry. 
#[derive(Clone, Copy)]
#[repr(C)]
pub struct InterruptStackFrameValue {
    pub instruction_pointer: VirtAddr,
    pub code_segment: SegmentSelector,
    _reserved1: [u8; 8],
    pub cpu_flags: RFlags,
    pub stack_pointer: VirtAddr,
    pub stack_segment: SegmentSelector,
    _reserved2: [u8; 8]
}


impl InterruptStackFrameValue {

    pub fn new(
        instruction_pointer: VirtAddr,
        code_segment: SegmentSelector,
        cpu_flags: RFlags,
        stack_pointer: VirtAddr,
        stack_segment: SegmentSelector,
    ) -> Self {
        Self { 
            instruction_pointer, 
            code_segment, 
            _reserved1: Default::default(), 
            cpu_flags, 
            stack_pointer, stack_segment, _reserved2: Default::default() 
        }
    }


    pub unsafe fn iretq(&self) -> ! {
        unsafe {
            core::arch::asm!(
                "push {stack_segment:r}",
                "push {new_stack_pointer}",
                "push {rflags}",
                "push {code_segment:r}",
                "push {new_instruction_pointer}",
                "iretq",
                rflags = in(reg) self.cpu_flags.bits(),
                new_instruction_pointer = in(reg) self.instruction_pointer.as_u64(),
                new_stack_pointer = in(reg) self.stack_pointer.as_u64(),
                code_segment = in(reg) self.code_segment.0,
                stack_segment = in(reg) self.stack_segment.0,
                options(noreturn)
            )
        }
    }
}


impl fmt::Debug for InterruptStackFrameValue {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("InterruptStackFrameValue");
        s.field("instruction_pointer", &self.instruction_pointer);
        s.field("code_segment", &self.code_segment);
        s.field("cpu_flags", &self.cpu_flags);
        s.field("stack_pointer", &self.stack_pointer);
        s.field("stack_segment", &self.stack_segment);
        s.finish();
    }

}


#[repr(transparent)]
pub struct InterruptStackFrame(InterruptStackFrameValue);

impl InterruptStackFrame {

    // creates a new interrupt stackframe with given value
    
    #[inline]
    pub fn new(
        instruction_pointer: VirtAddr,
        code_segment:  SegmentSelector,
        cpu_flags: RFlags,
        stack_pointer: VirtAddr,
        stack_segment: SegmentSelector
    ) -> Self {

        Self(InterruptStackFrameValue::new(
            instruction_pointer, 
            code_segment, 
            cpu_flags, 
            stack_pointer, 
            stack_segment
        ))
    }


    // gives us the mutable access to the contents of the interrupt stack frame
    pub unsafe fn as_mut(&mut self) -> Volatile<&mut InterruptStackFrameValue> {
        Volatile::new(&mut self.0)
    }

}




impl Deref for InterruptStackFrame {

    type Target = InterruptStackFrameValue;

    fn deref(&self) -> &Self::Target {
        &self.0
    }

}

impl fmt::Debug for InterruptStackFrame {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }

}

// Page fault error codes
bitflags! {

    pub struct PageFaultErrorCode: u64 {

        const PROTECTION_VIOLATION = 1;
        const CAUSED_BY_WRITE = 1 << 1;

        const USER_MODE = 1 << 2;
        const MALFORMED_TABLE = 1 << 3;
        const INSTRUCTION_FETCH = 1 << 4;
        const PROTECTION_KEY = 1 << 5; // fault is caused by protection key.
        const SHADOW_KEY = 1 << 6;  // Page fault is caused by Shadow key.

        // for AMD machines( Reverse map paging )
        // a structure that Resides in our DRAM and maps sPA's(system physical addresses) 
        // to gPA'a( guest physical addresses) Acc. to AMD spec.


        // In the context of AMD Secure Encrypted Virtualization-Secure Nested Paging (SEV-SNP) technology, 
        // the RMP stands for Reverse Map Table. 
        // The RMP is a critical hardware-managed data structure designed to enforce memory integrity and protect confidential virtual machines (CVMs) from an untrusted hypervisor. 
        const RMP = 1 << 31;

        // for Intel machines
        const HLAT = 1 << 7;

        // Intel SGX (Software Guard Extensions) is a set of CPU instruction codes that creates isolated, 
        // encrypted memory regions called enclaves to protect sensitive data and code during execution
        const SGX = 1 << 15; // softwareguard extensions

    }

}

#[repr(transparent)]
pub struct SelectorErrorCode {
    flags: u64
}


impl SelectorErrorCode {

    pub const fn new( value: u64 ) -> Option<Self> {
        if value > u16::MAX as u64 {
            None
        } else {
            Some( Self { flags: value } )
        }

    }

    #[inline]
    pub const fn new_truncate( value: u64 ) -> Self {
        Self { 
            flags: (value as u16) as u64
        }
    }


    // true -> indicates that the exception occured due to any external event. ie., interrupt or any other exception
    pub fn external(&self) -> bool {
        self.flags.get_bit(0)
    }

    // get the descriptor table on the basis of flag bits
    pub fn descriptor_table( &self ) -> DescriptorTable {
        match self.flags.get_bits(1..3) {
            0b00 => DescriptorTable::GDT,
            0b01 => DescriptorTable::IDT,
            0b10 => DescriptorTable::LDT,
            0b11 => DescriptorTable::IDT,
            _ => unreachable!(),
        }
    }

}

#[derive( Debug, Clone, Copy )]
pub enum DescriptorTable {
    GDT, IDT, LDT
}

#[repr(u8)]
pub enum ExceptionVector {

    Division = 0x00,
    Debug = 0x01,
    NonMaskableInterrupt = 0x02,
    Breakpoint = 0x03,
    Overflow = 0x04,
    BoundRange = 0x05,
    InvalidOpcode = 0x05,
    DeviceNotAvailable = 0x07,
    Double = 0x08,
    InvalidTss = 0x0A,
    SementNotpresent = 0x0B,
    Stack = 0x0c,
    GeneralProtection = 0x0D,
    Page = 0x0e,
    X87FloatingPoint = 0x10,
    AlignmentChack = 0x11,
    MachineCheck = 0x12,
    SimdFloatingPoint = 0x13,
    Virtualization = 0x14,
    ControlProtection = 0x15,
    HypervisorInjection = 0x1c,
    VmmCommunication = 0x1d,
    Security = 0x1e
}


#[derive(Debug)]
pub struct InvalidExceptionVectorNumber(u8);

impl fmt::Display for InvalidExceptionVectorNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} is not a valid exception vector", self.0)
    }
}