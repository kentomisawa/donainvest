extern crate core;

use std::borrow::{Borrow, BorrowMut};
use ic_cdk::{
    api::call::ManualReply,
    export::{
        candid::{CandidType, Deserialize},
        Principal,
    },
    storage,
};
use ic_cdk_macros::*;
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::str::FromStr;

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct TokenColumns {
    name: String,
    amount: f64,
    price: f64
}

type Users = BTreeSet<Principal>;
type TokenIds = BTreeSet<String>;
type TokenTable = BTreeMap<String, TokenColumns>;
type Balance = HashMap<String, f64>;
type UserBalance = BTreeMap<Principal, Balance>;

thread_local! {
    static USERS: RefCell<Users> = RefCell::default();
    static TOKEN_POOL: RefCell<TokenTable> = RefCell::default();
    static TOKEN_IDS: RefCell<TokenIds> = RefCell::default();
    static USER_BALANCE: RefCell<UserBalance> = RefCell::default();
}

#[init]
fn init() {
    USERS.with(|users| users.borrow_mut().insert(ic_cdk::api::caller()));
}

fn is_user() -> Result<(), String> {
    if USERS.with(|users| users.borrow().contains(&ic_cdk::api::caller())) {
        Ok(())
    } else {
        Err("Store can only be set by the owner of the asset canister.".to_string())
    }
}

fn get_mock_user() -> Principal {
    let address = "ytfqb-zgtuo-3lzup-tug4q-4uqya-on5dj-npzql-h67zr-nautt-rw3is-eqe";
    Principal::from_str(address).unwrap()
}

#[update]
fn seed() {
    let usp_id = "USP";
    let btc_id = "BTC";
    let eth_id = "ETH";
    let icp_id = "ICP";

    let usp = TokenColumns {
        name: usp_id.to_string(),
        amount: 1000.0,
        price: 1.0,
    };
    let btc = TokenColumns {
        name: btc_id.to_string(),
        amount: 1000.0,
        price: 400.0,
    };
    let eth = TokenColumns {
        name: eth_id.to_string(),
        amount: 1000.0,
        price: 50.0,
    };
    let icp = TokenColumns {
        name: icp_id.to_string(),
        amount: 1000.0,
        price: 10.0,
    };

    let principal = get_mock_user();

    USERS.with(|users| users.borrow_mut().insert(principal));

    TOKEN_IDS.with(|tokens| tokens.borrow_mut().insert(usp_id.to_string()));
    TOKEN_IDS.with(|tokens| tokens.borrow_mut().insert(btc_id.to_string()));
    TOKEN_IDS.with(|tokens| tokens.borrow_mut().insert(eth_id.to_string()));
    TOKEN_IDS.with(|tokens| tokens.borrow_mut().insert(icp_id.to_string()));

    TOKEN_POOL.with(|token_pool| token_pool.borrow_mut().insert(
        usp_id.to_string(), usp
    ));
    TOKEN_POOL.with(|token_pool| token_pool.borrow_mut().insert(
        btc_id.to_string(), btc
    ));
    TOKEN_POOL.with(|token_pool| token_pool.borrow_mut().insert(
        eth_id.to_string(), eth
    ));
    TOKEN_POOL.with(|token_pool| token_pool.borrow_mut().insert(
        icp_id.to_string(), icp
    ));

    let mut balance = HashMap::new();
    balance.insert(usp_id.to_string(), 1000.0);
    balance.insert(btc_id.to_string(), 1000.0);
    balance.insert(eth_id.to_string(), 1000.0);
    balance.insert(icp_id.to_string(), 1000.0);

    USER_BALANCE.with(|user_balance| user_balance.borrow_mut().insert(
        principal, balance
    ));
}

fn get_price(token_id: &str) -> f64 {
    TOKEN_POOL.with(|pool| match pool.borrow().get(token_id) {
        Some(token) => token.price,
        None => panic!("Invalid token_id")
    })
}

fn get_usp_value(balance: &Balance) -> f64 {
    match balance.get("USP") {
        Some(amount) => *amount,
        None => 0.0,
    }
}

fn deposit_in_pool(token_id: &String, amount: f64) {
    TOKEN_POOL.with(|pool| match pool.clone().borrow().get(token_id) {
        Some(token) => {
            pool.borrow_mut()
                .insert(token_id.clone(), TokenColumns {
                    name: token.name.clone(),
                    amount: token.amount + amount,
                    price: token.price,
                });
        },
        None => panic!("Invalid token_id")
    });
}

fn withdraw_from_pool(token_id: &String, amount: f64) {
    TOKEN_POOL.with(|pool| match pool.clone().borrow().get(token_id) {
        Some(token) => {
            if token.amount > amount {
                pool.borrow_mut()
                    .insert(token_id.clone(), TokenColumns {
                        name: token.name.clone(),
                        amount: token.amount - amount,
                        price: token.price,
                    });
            } else {
                panic!("Price will be infinite")
            }
        },
        None => panic!("Invalid token_id")
    });
}

#[query(manual_reply = true)]
fn get_current_mock_user() -> ManualReply<Principal> {
    ManualReply::one(get_mock_user())
}

#[query(manual_reply = true)]
fn list_tokens() -> ManualReply<BTreeMap<String, TokenColumns>> {
    TOKEN_POOL.with(|pool| ManualReply::one(pool))
}

// #[update(guard = "is_user")]
#[query(manual_reply = true)]
fn get_balance() -> ManualReply<Balance> {
    let principal = get_mock_user();
    USER_BALANCE.with(|user_balance| match user_balance.borrow().get(&principal) {
        Some(balance) => ManualReply::one(balance),
        None => ManualReply::empty(),
    })
}

#[update]
fn deposit(user_id: String, token_id: String, amount: f64) {
    let principal = Principal::from_str(&user_id).unwrap();
    USER_BALANCE.with(|user_balance| match user_balance.clone().borrow().get(&principal) {
        Some(balance) => match balance.get(&token_id) {
            Some(token_balance) => {
                if *token_balance > amount {
                    let usp_value = get_usp_value(balance);
                    let price = get_price(&token_id);
                    deposit_in_pool(&token_id, amount);
                    let mut new_balance = balance.clone();
                    new_balance.insert(token_id, token_balance - amount);
                    new_balance.insert("USP".to_string(), amount * price + usp_value);
                    user_balance.borrow_mut().insert(principal, new_balance)
                } else {
                    panic!("Balance not enough")
                }
            },
            None => {
                let mut new_balance = balance.clone();
                new_balance.insert(token_id, amount).unwrap();
                user_balance.borrow_mut().insert(principal, new_balance)
            },
        },
        None => panic!("No user")
    });
}

#[update]
fn withdraw(user_id: String, token_id: String, amount: f64) {
    let principal = Principal::from_str(&user_id).unwrap();
    USER_BALANCE.with(|user_balance| match user_balance.clone().borrow().get(&principal) {
        Some(balance) => match balance.get(&token_id) {
            Some(token_balance) => {
                let usp_value = get_usp_value(balance);
                let price = get_price(&token_id);
                if usp_value > amount * price {
                    withdraw_from_pool(&token_id, amount);
                    let mut new_balance = balance.clone();
                    new_balance.insert(token_id, token_balance + amount);
                    new_balance.insert("USP".to_string(), usp_value - amount * price);
                    user_balance.borrow_mut().insert(principal, new_balance)
                } else {
                    panic!("USP not enough")
                }
            },
            None => panic!("No balance"),
        },
        None => panic!("No user")
    });
}

// #[update(guard = "is_user")]
// fn store(path: String, contents: Vec<u8>) {
//     STORE.with(|store| store.borrow_mut().insert(path, contents));
// }
//
// #[query(manual_reply = true)]
// fn retrieve(path: String) -> ManualReply<Vec<u8>> {
//     STORE.with(|store| match store.borrow().get(&path) {
//         Some(content) => ManualReply::one(content),
//         None => panic!("Path {} not found.", path),
//     })
// }
//
// #[update(guard = "is_user")]
// fn add_user(principal: Principal) {
//     USERS.with(|users| users.borrow_mut().insert(principal));
// }
//
// #[pre_upgrade]
// fn pre_upgrade() {
//     USERS.with(|users| storage::stable_save((users,)).unwrap());
// }

// #[post_upgrade]
// fn post_upgrade() {
//     let (old_users,): (BTreeSet<Principal>,) = storage::stable_restore().unwrap();
//     USERS.with(|users| *users.borrow_mut() = old_users);
// }
