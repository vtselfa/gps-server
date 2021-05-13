use std::sync::RwLock;
use std::collections::HashMap;
use chrono::prelude::*;
use std::sync::atomic::AtomicUsize;
use std::str;
use std::io;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::currency;
use crate::money::Money;


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

    #[error("{0}")]
    CurrencyError(String),

    #[error(transparent)]
    Utf8Error(#[from] str::Utf8Error),

    #[error(transparent)]
    IoError(#[from] io::Error),

    #[error(transparent)]
    DecimalError(#[from] rust_decimal::Error),

    #[error("Amount not parseable")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("action code '{num:?}': {msg:?}")]
    ActionCode{num: i32, msg: String},
}


#[derive(Default, Serialize, Deserialize)]
pub struct State {
    // This is a flag that indicates that we are replacing the state with a new one.
    // It is taken for writing when replacing the state with a new one, or when deleting it.
    // It is takes for reading in any other case.
    #[serde(skip_serializing)]
    pub replacing_state:   RwLock<bool>,

    pub next_public_token: AtomicUsize,
    pub next_item_id:      AtomicUsize,
    pub wsids:             RwLock<HashMap<u64, String>>,
    pub public_tokens:     RwLock<HashMap<String, Card>>,
    pub transactions:      RwLock<Vec<Transaction>>,
}


#[derive(Clone, Serialize, Deserialize)]
pub struct Consumer {
    pub title: String,
    pub first_name: String,
    pub last_name: String,
}


#[derive(Clone, Serialize, Deserialize)]
pub struct Card {
    pub wsid: i64,
    pub public_token: String,
    pub external_ref: Option<String>,
    pub start_date: DateTime<Utc>,
    pub exp_date: DateTime<Utc>,
    pub balance: Money,
    pub is_live: bool,
    pub pan: String,
    pub cvv: String,
    pub transactions: Vec<Transaction>,
    pub status: CardStatus,
    pub owner: Consumer,
    // TODO: Groups
}


#[derive(Copy, Clone, Serialize, Deserialize)]
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


#[derive(Clone, Serialize, Deserialize)]
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
        format!("{:0>2}", self.status as i32)
    }

    pub fn get_emboss_name(&self) -> String {
        format!("{} {} {}", self.owner.title, self.owner.first_name, self.owner.last_name)
    }

    pub fn get_currency(&self) -> currency::Currency {
        self.balance.currency
    }

    pub fn get_currency_info(&self) -> currency::CurrencyInfo {
        currency::get_currency_info(self.get_currency())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::currency::Currency;
    use rust_decimal::prelude::*;

    #[test]
    fn test_get_status_code() {
        let card = Card {
            wsid: 0,
            public_token: format!("000000001"),
            external_ref: None,
            start_date: Utc::now(),
            exp_date: Utc::now(),
            balance: Money::new(Decimal::new(0, 2), Currency::EUR),
            is_live: false,
            pan: format!("1234123412341234"),
            cvv: format!("123"),
            transactions: vec![],
            status: CardStatus::AllGood,
            owner: Consumer {
                title: format!(""),
                first_name: format!(""),
                last_name: format!(""),
            },
        };
        assert_eq!(card.get_status_code(), "00");
    }
}
