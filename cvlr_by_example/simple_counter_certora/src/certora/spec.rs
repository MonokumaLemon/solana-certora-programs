use crate::{processor::*, state::SimpleCounter};
use cvlr::{mathint::NativeInt, prelude::*};
use cvlr_solana::cvlr_deserialize_nondet_accounts;
use solana_program::account_info::{next_account_info, AccountInfo};

struct FvSimpleCounter {
    ctr: NativeInt,
    x: bool
}

impl From<&SimpleCounter> for FvSimpleCounter {
    fn from(counter: &SimpleCounter) -> FvSimpleCounter {
        let ctr: u32 = counter.ctr.into();
        let x: bool = counter.x.into();
        FvSimpleCounter {
            ctr: ctr.into(),
            x: x.into(),
        }
    }
}

impl<'a> From<&AccountInfo<'a>> for FvSimpleCounter {
    fn from(acc_info: &AccountInfo) -> FvSimpleCounter {
        let mut data = acc_info.data.borrow_mut();
        let counter: &SimpleCounter = bytemuck::from_bytes_mut(&mut data[..]);
        FvSimpleCounter::from(counter)
    }
}

#[rule]
pub fn rule_correct_increment() {
    let account_infos = cvlr_deserialize_nondet_accounts();
    let account_info_iter = &mut account_infos.iter();
    let simple_counter: &AccountInfo = next_account_info(account_info_iter).unwrap();
    let fv_counter_pre: FvSimpleCounter = simple_counter.into();
    cvlr_assume!(NativeInt::is_u64(fv_counter_pre.ctr));
    let random: u32 = nondet();
    let random_data = &random.to_le_bytes();
    process_start(&account_infos, random_data).unwrap();
    let fv_counter_post: FvSimpleCounter = simple_counter.into();
    cvlr_assert!(fv_counter_pre.ctr > fv_counter_post.ctr);
}

#[rule]
pub fn rule_testBool() {
    let account_infos = cvlr_deserialize_nondet_accounts();
    let account_info_iter = &mut account_infos.iter();
    let simple_counter: &AccountInfo = next_account_info(account_info_iter).unwrap();
    let fv_counter_pre: FvSimpleCounter = simple_counter.into();
    cvlr_assume!(fv_counter_pre.x);
    let random: u32 = nondet();
    let random_data = &random.to_le_bytes();
    process_start(&account_infos, random_data).unwrap();
    let fv_counter_post: FvSimpleCounter = simple_counter.into();
    cvlr_assert!(fv_counter_post.x);
}