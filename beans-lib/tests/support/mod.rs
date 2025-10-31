// Helper functions for returning currency codes
pub fn usd<'a>() -> &'a str {
    rusty_money::iso::USD.iso_alpha_code
}

pub fn bgn<'a>() -> &'a str {
    rusty_money::iso::BGN.iso_alpha_code
}

pub fn eur<'a>() -> &'a str {
    rusty_money::iso::EUR.iso_alpha_code
}

