use core::{fmt, marker::PhantomData, ops::Add};

use crate::addr::VirtualAddr;

pub trait PageSize: Copy + Eq + PartialEq + PartialOrd {
    const SIZE: u64;
    const DEBUG_STR: &'static str;
}

// pub trait PageSize<S>
// where
//     S: Copy + Eq + PartialEq + PartialOrd
// {
//     const SIZE: u64;
//     const DEBUG_STR: &'static str;
// }
// We can do this but S(Generic) isn't used anywhere.

// for 4KB and 2MB Pages, not for 1GB Pages
pub trait NotGiantPageSize: PageSize {}


#[derive( Clone, PartialEq, PartialOrd, Ord, Eq, Hash )]
pub enum Size4Kib {}


#[derive( Clone, PartialEq, PartialOrd, Ord, Eq, Hash )]
pub enum Size2Mib {}


#[derive( Clone, PartialEq, PartialOrd, Ord, Eq, Hash )]
pub enum Size1GiB {}

impl PageSize for Size4Kib {
    const SIZE: u64 = 4096;
    const DEBUG_STR: &'static str = "4KiB";
}

impl PageSize for Size2Mib {
    const SIZE: u64 = Size4Kib::SIZE * 512;
    const DEBUG_STR: &'static str = "2MiB";
}

impl PageSize for Size1GiB {
    const SIZE: u64 = Size2Mib::SIZE * 512;
    const DEBUG_STR: &'static str = "1GiB";
}


impl NotGiantPageSize for Size4Kib{}
impl NotGiantPageSize for Size2Mib{}

pub struct Page<S: PageSize = Size4Kib> {
    start_address: VirtualAddr,
    size: PhantomData<S>
}


impl<S: PageSize> Page<S> {
    pub const SIZE: u64 = S::SIZE;


    pub fn from_start_address( address: VirtualAddr ) -> Result<Self, AddressNotAligned> {
        if !address.is_aligned_u64(S::SIZE) {
            return Err(AddressNotAligned);
        }
        Ok(Page::containing_address(address))
    }


    pub fn containing_address( addr: VirtualAddr ) -> Self {
        Page { 
            start_address: addr.align_down_as_u64(S::SIZE), 
            size: PhantomData
        }
    }
}


#[derive(Debug)]
pub struct AddressNotAligned;

impl fmt::Display for AddressNotAligned {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Address alignment is insufficient")
    }
}