use rusty_money::{Money,Currency};
use rust_decimal::prelude::*;

use crate::types;
use crate::types::GpsError;


// Given a Option<String> representing a public token and the global state, this macro declares two
// variables: the first one contains the card (mut ref), the second one the map of public tokens to cards.
#[macro_export]
macro_rules! get_mut_card {
    ($param:expr, $state:expr, $card:ident, $cards_map:ident) => {
        // Ensure the public_token passed as a parameter it's not None
        $param.as_ref().ok_or(GpsError::ActionCode{num: 999, msg: format!("Missing public_token")})?;

        // Get the card, with a write mutex
        let mut $cards_map = $state.public_tokens.write().expect("Poisoned write lock");
        let $card = $cards_map.get_mut($param.as_ref().unwrap()).ok_or(
            GpsError::ActionCode{num: 999, msg: format!("Public token not found")})?;
    };
}

// We do not support a unload with a currency different than the one in the
// card. Not sure if GPS does.
pub fn check_currency(param: &Option<String>, card: &types::Card) -> Result<(), types::GpsError> {
    if let Some(v) = param.as_ref().map(String::as_str) {
        let currency = Currency::find(v)?;
        if currency != card.currency {
            return Err(GpsError::ActionCode{num: 999, msg: format!("Currency missmatch")});
        }
    }
    Ok(())
}

pub fn get_amount(amount: &str, currency: &'static Currency) -> Result<Money, types::GpsError> {
    // Get the amount as a string because that's what Money expects
    // GPS expect the amount to be >= 0, so we too
    let amount = Decimal::from_str(amount)?;
    let amount = Money::from_decimal(amount, currency);
    if amount.amount().is_sign_negative() || amount.amount().is_zero() {
        return Err(GpsError::ActionCode{num: 999, msg: format!("Unload amount has to be greater than zero")});
    }
    Ok(amount)
}
