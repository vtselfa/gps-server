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

    #[error(transparent)]
    DecimalError(#[from] rust_decimal::Error),

    #[error("action code '{num:?}': {msg:?}")]
    ActionCode{num: i32, msg: String},
}


#[derive(Default)]
pub struct State {
    pub next_public_token: AtomicUsize,
    pub next_item_id: AtomicUsize,
    pub wsids: RwLock<HashMap<u64, String>>,
    pub public_tokens:  RwLock<HashMap<String, Card>>,
    pub transactions: RwLock<Vec<Transaction>>,
}

pub struct Consumer {
    title: String,
    first_name: String,
    last_name: String,
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
    pub transactions: Vec<Transaction>,
    pub status: CardStatus,
    pub owner: Consumer,
    // TODO: Groups
}

pub enum CardStatus {
    AllGood = 0,
    ReferToCardIssuer = 1, // Retired, do not use
    NotYetActivated = 2, // GPS does not seem to use it. A card created without activating it is in AllGood. Classic GPS.
    DoNotHonor = 5,
    Lost = 41,
    Stolen = 43,
    Expired = 54,
    Restricted = 62,
    SecurityViolation = 63,
    CardholderToContactIssuer = 70,
    PinRetriesExceeded = 75,
    Destroyed = 83,
    Refunded = 98,
    Voided = 99,
}

pub struct Transaction {
    pub item_id: u64, // GPS transaction ID
    pub txn_date: DateTime<Utc>,
    pub post_date: DateTime<Utc>,
    pub amt_bill: Money,
    pub amt_txn: Money,
    pub fixed_fee: Option<Money>,
    pub rate_fee: Option<Money>,
    pub note: Option<String>,
}

impl Card {
    pub fn get_start_date(&self) -> String {
        format!("{}", self.start_date.format("%m/%Y"))
    }

    pub fn get_exp_date(&self) -> String {
        format!("{}", self.exp_date.format("%Y-%m-%d"))
    }

    // The end date is the exp date in a different format
    pub fn get_end_date(&self) -> String {
        format!("{}", self.exp_date.format("%m/%Y"))
    }

    pub fn get_status_code(&self) -> String {
        format!("{:01}", self.status as i32)
    }

    pub fn get_emboss_name(&self) -> String {
        format!("{} {} {}", self.owner.title, self.owner.first_name, self.owner.last_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_status_code() {
        let card: Card;
        assert_eq!(card.get_status_code(), "00");
    }
}
