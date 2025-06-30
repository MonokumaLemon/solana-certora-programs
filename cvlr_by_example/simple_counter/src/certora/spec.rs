use crate::{processor::*, state::SimpleCounter};
use cvlr::{mathint::NativeInt, prelude::*};
use cvlr_solana::cvlr_deserialize_nondet_accounts;
use solana_program::account_info::{next_account_info, AccountInfo};
use borsh::BorshDeserialize;

struct FvSimpleCounter {
    ctr: NativeInt,
}

impl From<&SimpleCounter> for FvSimpleCounter {
    fn from(counter: &SimpleCounter) -> FvSimpleCounter {
        let ctr: u64 = counter.ctr.into();
        FvSimpleCounter {
            ctr: ctr.into(),
        }
    }
}

impl<'a> From<&AccountInfo<'a>> for FvSimpleCounter {
    fn from(acc_info: &AccountInfo) -> FvSimpleCounter {
        let data = acc_info.data.borrow();
        let counter = SimpleCounter::try_from_slice(&data).unwrap();
        FvSimpleCounter::from(&counter)
    }
}

#[rule]
pub fn rule_correct_increment() {
    let account_infos = cvlr_deserialize_nondet_accounts();
    let account_info_iter = &mut account_infos.iter();
    let simple_counter: &AccountInfo = next_account_info(account_info_iter).unwrap();
    let fv_counter_pre: FvSimpleCounter = simple_counter.into();
    cvlr_assume!(fv_counter_pre.ctr < u32::MAX.into());
    let program_id = &crate::id();
    process_start(program_id, &account_infos).unwrap();
    let fv_counter_post: FvSimpleCounter = simple_counter.into();
    cvlr_assert!(fv_counter_pre.ctr < fv_counter_post.ctr);
}