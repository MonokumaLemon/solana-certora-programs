use bytemuck::{Pod, Zeroable};
use solana_program::pubkey::Pubkey;
use spl_pod::primitives::PodU64;

// Ensures the struct has a predictable memory layout for zero-copy.
#[repr(C)]
#[derive(Debug, Clone, Pod, Copy, Zeroable)]

pub struct Vault {
    pub owner: Pubkey,
    pub shares_total: PodU64,
    pub token_total: PodU64,
}

impl Vault {

    // Deposit tokens into the vault and mint user shares based on current token/share ratio.
    // If the vault is empty (equal total shares and tokens), mint 1:1. 
    // Otherwise, compute proportional shares.
    pub fn deposit(&mut self, tkn: u64) {
        let shares_total: u64 = self.shares_total.into();
        let token_total: u64 = self.token_total.into();
        let shares_for_user = if shares_total == token_total {
            tkn
        } else {
        // !! This calculation is redundant: token_total / token_total = 1
        // mul_div_floor(tkn, shares_total, token_total)
            mul_div_floor(tkn, token_total, token_total)
        };

        self.mint_shares(shares_for_user);
        self.add_token(tkn);
    }

    // Withdraw tokens from the vault and burn user shares based on current token/share ratio.
    // If the vault is balanced (total shares == total tokens), treat 1 share as 1 token.
    // Otherwise, calculate the proportional token amount for the given shares.
    pub fn withdraw(&mut self, shares: u64) {
        let shares_total: u64 = self.shares_total.into();
        let token_total: u64 = self.token_total.into();
        let tkn_for_user = if shares_total == token_total {
            shares
        } else {
            mul_div_floor(shares, token_total, shares_total)
        };

        self.burn_shares(shares);
        self.del_token(tkn_for_user);
    }

    pub fn reward(&mut self, tkn: u64) {
        self.add_token(tkn);
    }

    pub fn slash(&mut self, tkn: u64) {
        self.del_token(tkn);
    }

    fn mint_shares(&mut self, shares_for_user: u64) {
        assert!(shares_for_user > 0);
        let shares_total: u64 = self.shares_total.into();
        self.shares_total = shares_total.checked_add(shares_for_user).unwrap().into();
    }

    fn burn_shares(&mut self, shares: u64) {
        let shares_total: u64 = self.shares_total.into();
        self.shares_total = shares_total.checked_sub(shares).unwrap().into();
    }

    fn add_token(&mut self, tkn: u64) {
        assert!(tkn > 0);
        let token_total: u64 = self.token_total.into();
        self.token_total = token_total.checked_add(tkn).unwrap().into();
    }

    fn del_token(&mut self, tkn_for_user: u64) {
        let token_total: u64 = self.token_total.into();
        self.token_total = token_total.checked_sub(tkn_for_user).unwrap().into();
    }
}


// This function performs a × b using u128 to safely handle large intermediate results,
// then divides by c and returns the floored result as u64.
fn mul_div_floor(a: u64, b: u64, c: u64) -> u64 {
    (a as u128)
        .checked_mul(b as u128)
        .unwrap()
        .checked_div(c as u128)
        .unwrap()
        .try_into()
        .unwrap()
}
