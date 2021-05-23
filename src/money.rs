use core::ops::{Add, Neg, Sub};
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};

use crate::currency::Currency;
use crate::currency;
use crate::types::GpsError;

// Extra decimals to use to represent the decimal part of a monetary quantity.
// E.g. EUR uses 2 decimal places. If extra decimals is 6, a quantity in EUR
// will be stored with a precision of 0.00000001 EUR (2 + 6).
const EXTRA_MONEY_DECIMALS: u32 = 6;


#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Money {
    pub amount: Decimal,
    pub currency: Currency,
}


impl Money {
    pub fn new(amount: Decimal, curr: Currency) -> Self {
        let curr_info = currency::get_currency_info(curr);
        // We round the decimal to the exact number of decimal places that we use for this specific
        // currency, which is the decimal places used by the currency plus a fixed amount.
        Money {
            amount: amount.round_dp(curr_info.decimals + EXTRA_MONEY_DECIMALS),
            currency: curr,
        }
    }

    pub fn zero(curr: Currency) -> Self {
        Self::new(Decimal::new(0, 1), curr)
    }

    pub fn from_string(amount: &str, curr: Currency) -> Result<Money, GpsError> {
        let amount = Decimal::from_str(amount)?;
        Ok(Money::new(amount, curr))
    }
}


impl Add for Money {
    type Output = Result<Money, GpsError>;
    fn add(self: Money, rhs: Money) -> Self::Output {
        if self.currency != rhs.currency {
            return Err(GpsError::CurrencyError(format!("Trying to add {:?} to {:?}",
                self.currency, rhs.currency))
            );
        }
        Ok(Money::new(self.amount + rhs.amount, self.currency))
    }
}

impl Sub for Money {
    type Output = Result<Money, GpsError>;
    fn sub(self, rhs: Money) -> Self::Output {
        if self.currency != rhs.currency {
            return Err(GpsError::CurrencyError(format!("Trying to substract {:?} and {:?}",
                self.currency, rhs.currency))
            );
        }
        Ok(Money::new(self.amount - rhs.amount, self.currency))
    }
}

impl Neg for Money {
    type Output = Money;
    fn neg(self) -> Self::Output {
        Money::new(-self.amount, self.currency)
    }
}
