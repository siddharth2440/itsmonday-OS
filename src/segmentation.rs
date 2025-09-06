#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SegmentSelector( pub u16 );


pub trait Segment {
    fn get_reg() -> SegmentSelector;
    unsafe fn set_reg(sel: SegmentSelector);
}