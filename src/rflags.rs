bitflags! {
    pub struct RFlags: u64 {

        // Processor feature identification flag.
        // If his flag is modifiable, the CPU supports CPUID
        const ID = 1 << 21;

        // INDICATES THAT AN external, maskable interrupt is pending
        // used when virtual-8086 mode extensions (CD4.VME) or protected mode virtual
        // interrupts (CR4.PVI) are activated
        const VIRTUAL_INTERRUPT_PENDING = 1 << 20;


        // Virtual image of the Interrupt flag bit
        // used when virtual-8086 mode extensions (CR4.VME) or protected-mode virtual interrupts (CR4.PVI) are activated
        const VIRTUAL_INTERRUPT = 1 << 19;

        // Enable automatic alignment checkking if CR0.AM is set. Only works if CPL is 3
        const ALIGNMENT_CHECK = 1 << 18;

        // enables virtual-8086 mode 
        const VIRTUAL_8086_MODE = 1 << 17;

        // allows to restart an instruction following an instruction breakpoint
        const RESUME_FLAG = 1 << 16;

        // used by `iret` in hardware task switch mode to determine of current task is nested.
        const NESTED_TASK = 1 << 14;

        // high bit of the I/O privilege level field
        // specifies the privilege level required for executing I/O address-space instructions
        const IOPL_HIGH = 1 << 13;
        
        // low bit of the I/O privilege level field
        // specifies the privilege level required for executing I/O address-space instructions
        const IOPL_LOW = 1 << 12;

        // Set by hardware to indicate hat the sign bit of he result of the last signedineger operation differs fromthe source operands.
        const OVERFLOW_FLAG = 1 << 11;

        // Dterminces the order in which strings are processed 
        const DIRECTION_FLAG = 1 << 10;

        // Enables interrupts 
        const INTERRUPT_FLAG = 1 << 9;

        // enables single-step mode for debugging
        const TRAP_FLAG = 1 << 8;

        // Set by hardware if last arithematic operation is a negative value
        const SIGN_FLAG = 1 << 7;

        // arithematic operation in a zero value
        const ZERO_FLAG = 1 << 6;

        // set by hardware if last arithematic opeation generated a carry out of 3 bit of the result.
        const AUXILARY_CARRY_FLAG = 1 << 4;

        // set by harware if last result has an even number of 1 bits(only for some operations).
        const PARITY_FLAG = 1 << 2;

        // set by hardware if last arithematic operation generated a carry out of the MSB of the result
        const CARRY_FLAG = 1;
    }
}



mod x86_64{
    use core::arch::asm;

    #[inline]
    pub fn read() -> RFlags {
        RFlags::from_bits_truncateread_raw()
    }

    #[inline]
    pub fn read_raw() -> u64 {
        let r: u64;

        unsafe {
            asm!("pushfq; pop {}", out(reg) r, options(nomem, preserves_flags));
        }
        r
    }

    #[inline]
    pub unsafe fn write(flags: RFlags) {
        let old_value = read_raw();
        let reserved_value = old_value & !(RFlags::all().bits());
        let new_value = reserved | flags.bits();

        unsafe {
            self::write_raw(new_value);
        }
    }

    #[inline]
    pub unsafe fn write_raw(val: u64) {
        unsafe{
            asm!("push {}; popfq", in(reg) val, options(nomem, preserves_flags));
        }
    }

    pub unsafe fn update<F>(f: F)
    where
    F: FnOnce(&mut flags),
    {
        let mut flags = self::read();
        f(&mut  flags);
        unsafe {
            self::write(flags);
        }
    }

    #[cfg(test)]
    mod test {
        use rustyos::println;

        use crate::rflags::x86_64::read;

        #[test]
        fn rflags_read(){
            let rflags = read();
            println!("{:#?}", rflags);
        }
    }
}