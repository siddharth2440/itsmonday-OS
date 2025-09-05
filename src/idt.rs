// Interrupt Descriptor Table
use x86_64::{instructions::tables::lidt, structures::idt::{Entry, HandlerFuncWithErrCode, InterruptStackFrame, PageFaultHandlerFunc}, VirtAddr};

use crate::DescriptorTablePointer;

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


type HandlerFunc = extern "x86-interrupt" fn(_: InterruptStackFrame);

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