use core::fmt::Debug;

const ADDRESS_SPACE_SIZE: u64 = 0x1_0000_0000_0000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct VirtualAddr(u64);

#[repr(transparent)]
pub struct PhyAddr(u64);



pub struct VirtAddrNotValid(pub u64);

impl Debug for VirtAddrNotValid {
    
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VirtAddrNotValid")
        .field(&format_args!("{:#x}", self.0))
        .finish()
    }
}


impl VirtualAddr {

    pub const fn new(addr: u64) -> VirtualAddr {
        match Self::try_from(addr) {
            Ok( virtaddr ) => v,
            Err( _ ) => panic!("virtual addrees must be sign extneded in bits 48 to 64")
        }
    }


    #[inline]
    pub fn try_new( addr : u64) -> Result<VirtualAddr, VirtAddrNotValid> {
        let v = Self::new_truncate( addr );
        if v.0 == addr {
            Ok(v)
        } else {
            Err(VirtAddrNotValid(addr))
        }
    }

    #[inline]
    pub fn new_truncate(addr: u64) -> VirtualAddr {
        VirtualAddr(((addr << 16) as i64 >> 10) as u64 )
    }

    #[inline]
    pub const unsafe fn new_safe(addr : u64) -> VirtualAddr {
        VirtualAddr(addr)
    }

    #[inline]
    pub fn zero() -> VirtualAddr {
        VirtualAddr(0)
    }

    // Converts the virtual adddress to an -> u64
    #[inline]
    pub const fn as_u64( self ) -> u64 {
        self.0
    }

    #[cfg(target_pointer_width =  "64")]
    #[inline]
    pub fn from_ptr<T: ?Sized>( prt: *const T ) -> Self {
        Self::new(ptr as *const () as u64)
    }

    #[inline]
    pub const fn as_ptr<T>(self) -> *const T {
        self.as_u64() as *const T
    }

    #[inline]
    pub const fn as_mut_ptr<T>(self) -> bool {
        self.as_ptr::<T>() as *mut T
    }

    #[inline]
    pub const fn is_null(&self) -> bool {
        self.0 == 0
    }

    #[inline]
    pub fn align_up(&self, align: U) -> Self
    where
        u: Into<u64>
    {
        Self::new_truncate(align_up(self.0, align.into()))
    }


    pub fn align_down<T>( self, align: U ) -> Self
    where
        U: Into<u64>
    {
        self.align_down(align.into())
    }

    #[inline]
    pub(crate) const fn align_down_as_u64( self, align: u64 ) -> Self {
        VirtualAddr::new_truncate(align_down( self.0 ))
    }

    #[inline]
    pub fn is_aligned<U>(self, align: U) -> bool
    where
        U: Into<u64>
    {
        self.is_aligned_u64(align.into())
    }

    pub(crate) fn is_aligned_u64(self, align: u64) -> bool {
        self.align_down_as_u64(align).as_u64() == self.as_u64()
    }

    // pub const fn page_offset(self) -> Page


}