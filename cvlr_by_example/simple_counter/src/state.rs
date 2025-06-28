use borsh::{BorshDeserialize, BorshSerialize};

// Ensures the struct has a predictable memory layout for zero-copy.
#[repr(C)]

#[derive(BorshSerialize, BorshDeserialize, Debug)]

pub struct SimpleCounter {
    pub ctr: u32,
}

impl SimpleCounter {
    /// Increments the counter by 1 
    pub fn increment(&mut self) {
        self.ctr += 1;
    }
}
