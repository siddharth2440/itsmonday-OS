// Interrupt Descriptor Table

use core::marker::PhantomData;

use crate::{segmentation::SegmentSelector, DescriptorTablePointer};

#[repr(C)]
pub struct InterruptDescriptorTable {
    pub divide_by_zero:                 Entry<HandlerFunc>,
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
                divide_by_zero: Entry::missing(), 
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
}

#[derive(Clone, Copy, Debug)]
pub struct EntryOptions{
    cs: SegmentSelector,
    bits: u16
}

impl EntryOptions {
    const fn minimal() -> Self {
        EntryOptions { 
            cs: SegmentSelector(0), 
            bits: 0b1110_0000_0000
        }
    }
}

