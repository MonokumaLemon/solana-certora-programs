use bytemuck::{Pod, Zeroable};
use spl_pod::primitives::PodU32;

// Ensures the struct has a predictable memory layout for zero-copy.
#[repr(C)]
#[derive(Debug, Clone, Pod, Copy, Zeroable)]

pub struct SimpleCounter {
    pub ctr: PodU32,
}

impl SimpleCounter {
    /// Increments the counter by 1 
    pub fn increment(&mut self) {
        let ctr: u32 = self.ctr.into();
        self.ctr = ctr.checked_add(1).unwrap().into();
    }
}
