use std::sync::RwLock;
use std::collections::HashMap;
use chrono::prelude::*;
use std::sync::atomic::AtomicUsize;
use std::str;
use std::io;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use strum_macros::Display;

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

    #[error(transparent)]
    DateError(#[from] chrono::ParseError),

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
    pub wsids:             RwLock<HashMap<u64, ActionResult>>,
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
    pub exp_date: NaiveDate,
    pub balance: Money,
    pub blocked_balance: Decimal, // Amount of the balance blocked by outstanding authorisations
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

#[derive(Display, Clone, Copy, Serialize, Deserialize)]
pub enum TransationTypeStatus {
    #[strum(serialize = "AA")]
    AuthApproved,
    #[strum(serialize = "AC")]
    AuthOffline,
    #[strum(serialize = "AI")]
    AuthDeclined,
    #[strum(serialize = "DA")]
    AuthReversal,
    #[strum(serialize = "PS")]
    Presentment,
    #[strum(serialize = "ES")]
    FinancialReversal,
    #[strum(serialize = "NS")]
    SecondPresentment,
    #[strum(serialize = "CS")]
    Chargeback,
    #[strum(serialize = "BS")]
    BalanceAdjustment,
    #[strum(serialize = "LS")]
    Load,
    #[strum(serialize = "US")]
    Unload,
}


#[derive(Clone, Serialize, Deserialize)]
pub struct ActionResult {
    pub timestamp: DateTime<Utc>,
    pub action_name: String,
    pub action_code: String,
    pub response_sent: String,
}


#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Fees {
    pub fee_fixed: Option<Decimal>,
    pub fee_rate: Option<Decimal>,
    pub dom_fee_fixed: Option<Decimal>,
    pub dom_fee_rate: Option<Decimal>,
    pub non_dom_fee_fixed: Option<Decimal>,
    pub non_dom_fee_rate: Option<Decimal>,
    pub fx_fee_fixed: Option<Decimal>,
    pub fx_fee_rate: Option<Decimal>,
    pub other_fee_fixed: Option<Decimal>,
}


#[derive(Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub item_id: u64, // GPS transaction ID
    pub wsid: Option<u64>, // If it was initiated by a webservice call

    pub txn_date: DateTime<Utc>,
    pub post_date: DateTime<Utc>,

    // The amount of the transaction billed (for financials) or blocked (for authorisations) to the
    // cardholder account, expressed in the account currency (after fx, fees excluded).
    pub amt_bill: Decimal,
    pub currency: currency::Currency, // Account currency

    pub amt_txn: Money, // The amount and currency of the original payment

    // Total fee = fee_fixed + fee_rate
    pub fee_fixed: Option<Decimal>,
    pub fee_rate: Option<Decimal>,
    pub dom_fee_fixed: Option<Decimal>,
    pub dom_fee_rate: Option<Decimal>,
    pub non_dom_fee_fixed: Option<Decimal>,
    pub non_dom_fee_rate: Option<Decimal>,
    pub fx_fee_fixed: Option<Decimal>,
    pub fx_fee_rate: Option<Decimal>,
    pub other_fee_fixed: Option<Decimal>,

    pub note: Option<String>,
    pub description: Option<String>,

    // Both are AFTER the transaction
    pub balance: Decimal, // Balance in the card
    pub blocked_balance: Decimal, // Amount of the balance blocked by outstanding authorisations

    pub mcc: Option<String>,
    pub proc_code: Option<String>,

    pub txn_type: TransationTypeStatus,
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

impl Transaction {
    // pub fn get_start_date(&self) -> String {
    //     format!("{}", self.start_date.format("%m/%Y"))
    // }
    //
    // pub fn get_exp_date(&self) -> String {
    //     format!("{}", self.exp_date.format("%Y-%m-%d"))
    // }
    //
    // // The end date is the exp date in a different format
    // pub fn get_end_date(&self) -> String {
    //     format!("{}", self.exp_date.format("%m/%Y"))
    // }
    //
    // pub fn get_status_code(&self) -> String {
    //     format!("{:0>2}", self.status as i32)
    // }
    //
    // pub fn get_emboss_name(&self) -> String {
    //     format!("{} {} {}", self.owner.title, self.owner.first_name, self.owner.last_name)
    // }

    pub fn get_tx_currency(&self) -> currency::Currency {
        self.amt_txn.currency
    }

    pub fn get_tx_currency_info(&self) -> currency::CurrencyInfo {
        currency::get_currency_info(self.get_tx_currency())
    }

    pub fn get_bill_currency_info(&self) -> currency::CurrencyInfo {
        currency::get_currency_info(self.currency)
    }

    pub fn get_type(&self) -> String {
        self.txn_type.to_string()[0..1].to_string()
    }

    pub fn get_status(&self) -> String {
        self.txn_type.to_string()[1..2].to_string()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::currency::Currency;

    #[test]
    fn test_get_status_code() {
        let card = Card {
            wsid: 0,
            public_token: format!("000000001"),
            external_ref: None,
            start_date: Utc::now(),
            exp_date: Utc::now().naive_utc().date(),
            balance: Money::new(Decimal::new(0, 2), Currency::EUR),
            blocked_balance: Money::new(Decimal::new(0, 2), Currency::EUR).amount,
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
