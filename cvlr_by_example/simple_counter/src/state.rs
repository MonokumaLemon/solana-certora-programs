use borsh::{BorshDeserialize, BorshSerialize};

// Ensures the struct has a predictable memory layout for zero-copy.
#[repr(C)]

#[derive(BorshSerialize, BorshDeserialize, Debug)]

pub struct GreetingAccount {
    pub counter: u32,
}

impl GreetingAccount {
    /// Increments the counter by 1 
    pub fn increment(&mut self) {
        self.counter += 1;
    }
}
