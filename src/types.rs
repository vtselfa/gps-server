use std::sync::RwLock;
use std::collections::HashMap;
use chrono::prelude::*;
use rusty_money::{Money, Currency};
use std::sync::atomic::AtomicUsize;
use std::str;
use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum GpsError {
    #[error("{0}")]
    ContentLength(String),

    #[error("{0}")]
    RequestData(String),

    #[error("{0}")]
    Action(String),

    #[error("{0}")]
    Serialization(String),

    #[error(transparent)]
    Utf8Error(#[from] str::Utf8Error),

    #[error(transparent)]
    IoError(#[from] io::Error),

    #[error(transparent)]
    MoneyError(#[from] rusty_money::MoneyError),
}


#[derive(Default)]
pub struct State {
    pub next_public_token: AtomicUsize,
    pub wsids: RwLock<HashMap<u64, String>>,
    pub public_tokens:  RwLock<HashMap<String, Card>>,
    pub transactions: RwLock<Vec<transaction::Transaction>>,
}

pub struct Card {
    pub wsid: i64,
    pub public_token: String,
    pub external_ref: Option<String>,
    pub start_date: DateTime<Utc>, 
    pub exp_date: DateTime<Utc>,
    pub balance: Money,
    pub currency: &'static Currency,
    pub is_live: bool,
    pub pan: String,
    pub cvv: String,
    pub stat_code: String, 
    pub transactions: Vec<transaction::Transaction>,
    // TODO: Groups
}

pub mod transaction {
    use chrono::prelude::*;
    use rusty_money::Money;

    pub enum Direction {Debit, Credit}

    pub struct Transaction {
        pub item_id: u64, // GPS transaction ID
        pub txn_date: DateTime<Utc>,
        pub post_date: DateTime<Utc>,
        pub amt_bill: Money,
        pub amt_txn: Money,
        pub direction: Direction,
        pub fixed_fee: Money,
        pub rate_fee: Money,
        pub note: String,
    }
}
