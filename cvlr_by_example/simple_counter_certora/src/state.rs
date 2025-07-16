use bytemuck::{Pod, Zeroable};
use spl_pod::primitives::*;

// Ensures the struct has a predictable memory layout for zero-copy.
#[repr(C)]
#[derive(Debug, Clone, Pod, Copy, Zeroable)]

pub struct SimpleCounter {
    pub ctr: PodU32,
    pub x: PodBool,
    pub list: [PodU32;10],
}

impl SimpleCounter {
    /// Increments the counter by 1 
    pub fn increment(&mut self, /* Nomapping */) {
        let ctr: u32 = self.ctr.into();
        self.ctr = ctr.checked_add(1).unwrap().into();
        self.x = PodBool::from_bool(true);
    }

    pub fn set_false(&mut self) {
        self.x = PodBool::from_bool(false);
    }

    pub fn set_true(&mut self) {
        self.x = PodBool::from_bool(true);
    }

    pub fn set(&mut self) {
        self.list[0] = (1u32).into();
    }
}
