use core::sync::atomic::{AtomicBool, AtomicU64};

pub static ENC_BIT_MASK: AtomicU64 = AtomicU64::new(0);
pub static ENC_BIT_REVERSED: AtomicBool = AtomicBool::new(false);