use core::{fmt, marker::PhantomData};
use crate::{addr::PhyAddr, structures::page::{AddressNotAligned, PageSize, Size4Kib}};


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct PhysFrame<S: PageSize = Size4Kib> {
    pub(crate) start_addr: PhyAddr,
    size: PhantomData<S>
}


impl<S: PageSize> PhysFrame<S> {    
    // Returns the starting frame at the given Virtual Address.
    #[inline]
    pub fn frame_from_start_addr( addr: PhyAddr ) -> Result<Self, AddressNotAligned> {
        if !addr.is_aligned(S::SIZE) {
            return Err(AddressNotAligned); 
        }
        
        Ok(unsafe {
            PhysFrame::from_start_address_unchecked(addr)
        })
    }

    // Returns the PhysFrame.
    // We need to have assurance for the correctly alignemnt of address. 
    #[inline]
    unsafe fn from_start_address_unchecked(start_addr: PhyAddr) -> Self {
        PhysFrame { start_addr, size: PhantomData }
    }

    // Is the frame contained the Given Physical address.
    #[inline]
    fn frame_containing_addr(addr: PhyAddr) -> Self {
        PhysFrame { 
            start_addr: addr.align_down_u64(S::SIZE), 
            size: PhantomData
        }
    }

    // returns the Start Address of the PhysFrame.
    #[inline]
    fn start_addr_of_physframe(&self) -> PhyAddr {
         self.start_addr
    }

    // retuns the size of the PhysFrame.
    #[inline]
    fn size_of_physframe(self) -> u64 {
        S::SIZE
    }

    // range of the frames.
    #[inline]
    fn range_of_physframe( start: PhysFrame<S>, end: PhysFrame<S> ) -> PhysFrameRange<S> {
        PhysFrameRange {
            start, end
        }
    }

    // is the range is_inclusive?
    #[inline]
    fn is_range_of_physframe_inclusive() -> PhysFrameRangeInclusive<S> {
        PhysFrameRangeInclusive { start, end }
    }

}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct PhysFrameRange<S: PageSize = Size4Kib> {
    pub start: PhysFrame<S>,
    pub end: PhysFrame<S>,
}

impl<S: PageSize> PhysFrameRange<S> {

    #[inline]
    fn is_empty(&self) -> bool {
        self.start >= self.end
    }

    #[inline]
    fn total_frames_within_range(&self) -> u64 {
        if !self.is_empty() {
            self.end - self.start
        } else { 
            0 
        }
    }

    // Returns the size of all frames withtin the range(in Bytes).
    #[inline]
    fn size(&self) -> u64 {
        S::SIZE * self.total_frames_within_range()   
    }

}


// Implement `Iterator` for `PhysFrameRange<S>` S: PageSize = Size4KiB.
// The `Iterator` trait only requires a method to be defined for the `next` element,
// and an `associated type` to declare the return type of the iterator.
impl Iterator<S: PageSize> for PhysFrameRange<S> {
    // We can refer to this type using Self::Item
    type Item = PhysFrame<S>;

    // Here, we define the sequence using `.curr` and `.next`.
    // The return type is `Option<T>`:
    //     * When the `Iterator` is finished, `None` is returned.
    //     * Otherwise, the next value is wrapped in `Some` and returned.
    // We use Self::Item in the return type, so we can change
    // the type without having to update the function signatures.
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            let frame = self.start;
            self.start += 1;
            Some(frame)
        } else { None }
    }
}


impl<S: PageSize> fmt::Debug for PhysFrameRange<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PhysFrameRange")
        .field("start", self.start.into())
        .field("end", self.end.into())
        .finish()
    }
}

#[repr(C)]
pub struct PhysFrameRangeInclusive<S: PageSize = Size4Kib> {
    pub start: PhysFrame<S>,
    pub end: PhysFrame<S>,
}

impl<S: PageSize> PhysFrameRangeInclusive<S> {
    
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.start > self.end
    }

    #[inline]
    pub fn len(&self) -> usize {
        if !self.is_empty() {
            self.end - self.start
        } else {
            0
        }
    }

    #[inline]
    pub fn size(&self) -> u64 {
        S::SIZE * self.len()
    }
}


impl Iterator<S: PageSize> for PhysFrameRangeInclusive<S> {
    type Item = PhysFrame<S>;


    fn next(&mut self) -> Option<Self::Item> {
        if self.start <= self.end {
            let frame = self.start;
            self.start += 1;
            Some(frame)
        } else {
            None
        }
    }
}


impl fmt::Debug<S: PageSize> for PhysFrameRangeInclusive<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PhysicalFrameInclusive")
            .field("start", self.start.into())
            .field("end", self.end.into())
            .finish()
    }
}