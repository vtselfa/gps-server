use chrono::prelude::*;
use rust_decimal::prelude::*;

use crate::currency;
use crate::types;
use crate::types::GpsError;

// Given a Option<String> representing a public token and the global state, this macro declares two
// variables: the first one contains the card (mut ref), the second one the map of public tokens to cards.
#[macro_export]
macro_rules! get_mut_card {
    ($param:expr, $state:expr, $card:ident, $cards_map:ident) => {
        // Ensure the public_token passed as a parameter it's not None
        $param.as_ref().ok_or(GpsError::ActionCode {
            num: 999,
            msg: format!("Missing public_token"),
        })?;

        // Get the card, with a write mutex
        let mut $cards_map = $state.public_tokens.write().expect("Poisoned write lock");
        let $card = $cards_map
            .get_mut($param.as_ref().unwrap())
            .ok_or(GpsError::ActionCode {
                num: 999,
                msg: format!("Public token not found"),
            })?;
    };
}

// Given a Option<String> representing a public token and the global state, this macro declares two
// variables: the first one contains the card (ref), the second one the map of public tokens to cards.
#[macro_export]
macro_rules! get_card {
    ($param:expr, $state:expr, $card:ident, $cards_map:ident) => {
        // Ensure the public_token passed as a parameter it's not None
        $param.as_ref().ok_or(GpsError::ActionCode {
            num: 118,
            msg: format!("Missing public token"),
        })?;

        // Get the card, with a read mutex
        let $cards_map = $state.public_tokens.read().expect("Poisoned read lock");
        let $card = $cards_map
            .get($param.as_ref().unwrap())
            .ok_or(GpsError::ActionCode {
                num: 118,
                msg: format!("Public token not found"),
            })?;
    };
}

// We do not support a unload with a currency different than the one in the
// card. Not sure if GPS does.
pub fn check_currency(param: &Option<String>, card: &types::Card) -> Result<(), types::GpsError> {
    if let Some(v) = param.as_ref().map(String::as_str) {
        let currency = currency::find_by_alpha_code(v)?;
        if currency != card.balance.currency {
            return Err(GpsError::ActionCode {
                num: 999,
                msg: format!("Currency missmatch"),
            });
        }
    }
    Ok(())
}

pub fn get_strictly_positive_amount(amount: &str) -> Result<Decimal, types::GpsError> {
    // GPS expect the amount to be >= 0, so we too
    let amount = Decimal::from_str(amount)?;
    if amount.is_sign_negative() || amount.is_zero() {
        return Err(GpsError::ActionCode {
            num: 999,
            msg: format!("Unload amount has to be greater than zero"),
        });
    }
    Ok(amount)
}

pub fn error_to_action_code(error: &GpsError) -> String {
    match error {
        types::GpsError::ActionCode { num, msg: _ } => format!("{}", num),
        _ => format!("999"), // TODO: map better to action codes?
    }
}

// Format the local date as GPS does
pub fn sys_date() -> String {
    let utc: DateTime<Utc> = Utc::now();
    format!("{}", utc.format("%Y-%m-%d"))
}

// Format the local date as GPS does
pub fn loc_date() -> String {
    let utc: DateTime<Utc> = Utc::now();
    format!("{}", utc.format("%Y-%m-%d"))
}

// Format the local time as GPS does
pub fn loc_time() -> String {
    let utc: DateTime<Utc> = Utc::now();
    format!("{}", utc.format("%H%M%S"))
}
