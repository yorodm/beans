//! Wrapper around rust_money types

use std::fmt::Display;

use rust_decimal::Decimal;
use rusty_money::{
    iso::{self, Currency as IsoCurrency},
    Money,
};

use crate::{BeansError, BeansResult};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Currency<'a>(Money<'a, IsoCurrency>);

impl<'a> Currency<'a> {
    pub fn new(amount: Decimal, currency_code: &str) -> BeansResult<Self> {
        let code =
            iso::find(currency_code).ok_or(BeansError::Currency(currency_code.to_owned()))?;
        let m = Money::from_decimal(amount, code);
        Ok(Self(m))
    }

    pub fn code(&self) -> &'a str {
        self.0.currency().iso_alpha_code
    }

    pub fn amount(&self) -> &Decimal {
        self.0.amount()
    }
}

impl<'a> Display for Currency<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}
