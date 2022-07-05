#![allow(clippy::too_many_arguments, clippy::ptr_arg)]
use near_contract_standards::fungible_token::core::{ext_ft_core, FungibleTokenCore};
use near_contract_standards::fungible_token::metadata::FungibleTokenMetadata;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::{
    env, ext_contract, log, near_bindgen, require, serde_json, AccountId, Balance, Gas,
    PanicOnDefault, Promise, PromiseOrValue, StorageUsage,
};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

use uint::construct_uint;
construct_uint! {
    /// 256-bit unsigned integer.
    pub struct u256(4);
}

#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize, Clone, Serialize, Deserialize)]
pub struct PoolTokenInfo {
    account_id: AccountId,
    name: String,
    ticker: Balance,
    decimals: u8,
}

#[ext_contract(ext_ft_token)]
pub trait FungibleToken {
    fn create_wallet();
    fn ft_metadata(&self) -> FungibleTokenMetadata;
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct AMMContract {
    gov: AccountId,
    token_a: PoolTokenInfo,
    token_b: PoolTokenInfo,
    ready: bool,
    k: String,
}

#[derive(Serialize, Deserialize)]
pub struct AMMMetaData {
    token_a: PoolTokenInfo,
    token_b: PoolTokenInfo,
    ratio: String,
}

#[ext_contract(ext_amm)]
trait AMM {
    fn cb_init(&mut self, contract_id: AccountId, #[callback] metadata: FungibleTokenMetadata);
    fn cb_log(#[callback] res: String);
}

const CHANGE_K: &'static str = "CHANGE_K";

#[near_bindgen]
impl AMMContract {
    #[init]
    pub fn new(gov: AccountId, token_a: AccountId, token_b: AccountId) -> Self {
        assert!(!env::state_exists(), "ERR_CONTRACT_IS_INITIALIZED");
        // get metadata from token
        ext_ft_token::ext(token_a.clone())
            .ft_metadata()
            .then(ext_amm::ext(env::current_account_id()).cb_init(token_a.clone()));
        ext_ft_token::ext(token_b.clone())
            .ft_metadata()
            .then(ext_amm::ext(env::current_account_id()).cb_init(token_b.clone()));
        let this = Self {
            gov: gov.into(),
            token_a: PoolTokenInfo {
                account_id: token_a.into(),
                name: "".to_string(),
                ticker: 0,
                decimals: 0,
            },
            token_b: PoolTokenInfo {
                account_id: token_b.into(),
                name: "".to_string(),
                ticker: 0,
                decimals: 0,
            },
            k: "0".to_string(),
            ready: false,
        };
        this.create_wallet();
        this
    }

    pub fn amm_metadata(&self) -> String {
        let metadata = AMMMetaData {
            token_a: self.token_a.clone(),
            token_b: self.token_b.clone(),
            ratio: self.k.clone(),
        };
        serde_json::to_string(&metadata).unwrap()
    }

    pub fn create_wallet(&self) {
        require!(
            env::predecessor_account_id() == self.gov,
            "ERR_ONLY_GOV_CAN_CREATE_WALLET"
        );
        ext_ft_token::ext(self.token_a.account_id.clone()).create_wallet();
        ext_ft_token::ext(self.token_b.account_id.clone()).create_wallet();
    }

    pub fn cb_init(&mut self, contract_id: AccountId, #[callback] metadata: FungibleTokenMetadata) {
        require!(
            env::predecessor_account_id() == env::current_account_id(),
            "only in init callback"
        );

        require!(
            contract_id == self.token_a.account_id || contract_id == self.token_b.account_id,
            "ERR_INVALID_CONTRACT_ID"
        );

        if contract_id == self.token_a.account_id {
            self.token_a.name = metadata.name;
            self.token_a.decimals = metadata.decimals;
        } else {
            self.token_b.name = metadata.name;
            self.token_b.decimals = metadata.decimals;
        }
        if self.token_a.name.len() > 0 && self.token_b.name.len() > 0 {
            self.ready = true;
        }
    }

    fn update_ratio(&mut self) {
        let a_num = u256::from(self.token_a.ticker);
        let b_num = u256::from(self.token_b.ticker);
        self.k = (a_num * b_num).to_string();
        log!("update_ratio: {}", self.k);
    }

    #[payable]
    pub fn ft_on_transfer(&mut self, sender_id: AccountId, amount: U128, msg: String) -> String {
        let pre_contract = env::predecessor_account_id();
        require!(
            pre_contract == self.token_a.account_id || pre_contract == self.token_b.account_id,
            "ERR_INVALID_CONTRACT_ID"
        );
        require!(
            sender_id != env::current_account_id(),
            "ERR_SENDER_IS_CURRENT_CONTRACT"
        );

        let k = u256::from_dec_str(&self.k).expect("ERR_K_IS_INVILID");
        if msg.as_str() == CHANGE_K {
            require!(sender_id == self.gov, "ERR_ONLY_GOV_CAN_CHANGE_K");
            if pre_contract == self.token_a.account_id {
                self.token_a.ticker += u128::from(amount);
            } else {
                self.token_b.ticker += u128::from(amount);
            }
            self.update_ratio();
        } else {
            require!(k > u256::from(0), "ERR_K_IS_ZERO");
            if pre_contract == self.token_a.account_id {
                let a_after = self.token_a.ticker + u128::from(amount);
                let b_after = k / a_after;
                log!(
                    "k:{}, a_after:{}, b_after:{}, a_ticker:{}, b_ticker:{}",
                    k,
                    a_after,
                    b_after,
                    self.token_a.ticker,
                    self.token_b.ticker
                );
                let b_pay = self.token_b.ticker - b_after.as_u128();
                self.token_b.ticker -= b_pay;
                self.token_a.ticker = a_after;

                ext_ft_core::ext(self.token_b.account_id.clone())
                    .with_attached_deposit(1)
                    .ft_transfer(sender_id.clone(), U128::from(b_pay), None);
                log!(
                    "user {} has exchanged {} tokens a for {} tokens b",
                    sender_id,
                    u128::from(amount),
                    b_pay
                );
            } else {
                let b_after = self.token_b.ticker + u128::from(amount);
                let a_after = k / b_after;
                log!(
                    "k:{}, a_after:{}, b_after:{}, a_ticker:{}, b_ticker:{}",
                    k,
                    a_after,
                    b_after,
                    self.token_a.ticker,
                    self.token_b.ticker
                );
                let a_pay = self.token_a.ticker - a_after.as_u128();
                self.token_a.ticker -= a_pay;
                self.token_b.ticker = b_after;
                ext_ft_core::ext(self.token_a.account_id.clone())
                    .with_attached_deposit(1)
                    .ft_transfer(sender_id.clone(), U128::from(a_pay), None);
                log!(
                    "user {} has exchanged {} tokens b for {} tokens a",
                    sender_id,
                    u128::from(amount),
                    a_pay
                );
            }
        }
        "0".to_string()
    }

    pub fn cb_log(#[callback] res: String) {
        log!("unused token: {}", res)
    }
}
