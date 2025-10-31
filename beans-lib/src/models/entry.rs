//! Ledger entry model for representing financial transactions.

use crate::error::{BeansError, BeansResult};
use crate::models::{Currency, Tag};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;

/// Type of ledger entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EntryType {
    /// Income entry (money coming in).
    Income,
    /// Expense entry (money going out).
    Expense,
}

/// Represents a financial transaction in the ledger.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerEntry {
    /// Unique identifier for the entry.
    id: Uuid,
    /// Date and time of the transaction.
    date: DateTime<Utc>,
    /// Name/title of the transaction.
    name: String,
    /// Currency of the transaction.
    currency: Currency,
    /// Amount of the transaction.
    amount: Decimal,
    /// Optional description of the transaction.
    description: Option<String>,
    /// Tags for categorizing the transaction.
    tags: HashSet<Tag>,
    /// Type of the transaction (income or expense).
    entry_type: EntryType,
    /// Date and time the entry was created.
    created_at: DateTime<Utc>,
    /// Date and time the entry was last updated.
    updated_at: DateTime<Utc>,
}

impl LedgerEntry {
    /// Returns the entry's unique identifier.
    pub fn id(&self) -> Uuid {
        self.id
    }

    /// Returns the date and time of the transaction.
    pub fn date(&self) -> DateTime<Utc> {
        self.date
    }

    /// Returns the name/title of the transaction.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the currency of the transaction.
    pub fn currency(&self) -> &Currency {
        &self.currency
    }

    /// Returns the amount of the transaction.
    pub fn amount(&self) -> Decimal {
        self.amount
    }

    /// Returns the description of the transaction, if any.
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Returns the tags for the transaction.
    pub fn tags(&self) -> &HashSet<Tag> {
        &self.tags
    }

    /// Returns the type of the transaction.
    pub fn entry_type(&self) -> EntryType {
        self.entry_type
    }

    /// Returns the date and time the entry was created.
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    /// Returns the date and time the entry was last updated.
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

/// Builder for creating ledger entries.
#[derive(Debug, Default)]
pub struct LedgerEntryBuilder {
    id: Option<Uuid>,
    date: Option<DateTime<Utc>>,
    name: Option<String>,
    currency: Option<Currency>,
    amount: Option<Decimal>,
    description: Option<String>,
    tags: HashSet<Tag>,
    entry_type: Option<EntryType>,
}

impl LedgerEntryBuilder {
    /// Creates a new builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the entry's unique identifier.
    pub fn id(mut self, id: Uuid) -> Self {
        self.id = Some(id);
        self
    }

    /// Sets the date and time of the transaction.
    pub fn date(mut self, date: DateTime<Utc>) -> Self {
        self.date = Some(date);
        self
    }

    /// Sets the name/title of the transaction.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Sets the currency of the transaction.
    pub fn currency(mut self, currency: Currency) -> Self {
        self.currency = Some(currency);
        self
    }

    /// Sets the amount of the transaction.
    pub fn amount(mut self, amount: Decimal) -> Self {
        self.amount = Some(amount);
        self
    }

    /// Sets the description of the transaction.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Adds a tag to the transaction.
    pub fn tag(mut self, tag: Tag) -> Self {
        self.tags.insert(tag);
        self
    }

    /// Sets the type of the transaction.
    pub fn entry_type(mut self, entry_type: EntryType) -> Self {
        self.entry_type = Some(entry_type);
        self
    }

    /// Builds the ledger entry.
    pub fn build(self) -> BeansResult<LedgerEntry> {
        let now = Utc::now();
        
        let name = self.name.ok_or_else(|| {
            BeansError::validation("Entry name is required")
        })?;
        
        if name.trim().is_empty() {
            return Err(BeansError::validation(
                "Entry name cannot be empty"
            ));
        }
        
        let currency = self.currency.ok_or_else(|| {
            BeansError::validation("Entry currency is required")
        })?;
        
        let amount = self.amount.ok_or_else(|| {
            BeansError::validation("Entry amount is required")
        })?;
        
        let entry_type = self.entry_type.ok_or_else(|| {
            BeansError::validation("Entry type is required")
        })?;
        
        Ok(LedgerEntry {
            id: self.id.unwrap_or_else(Uuid::new_v4),
            date: self.date.unwrap_or_else(Utc::now),
            name,
            currency,
            amount,
            description: self.description,
            tags: self.tags,
            entry_type,
            created_at: now,
            updated_at: now,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_entry_builder() {
        let entry = LedgerEntryBuilder::new()
            .name("Groceries")
            .currency(Currency::usd())
            .amount(dec!(42.50))
            .entry_type(EntryType::Expense)
            .build()
            .unwrap();

        assert_eq!(entry.name(), "Groceries");
        assert_eq!(entry.currency().code(), "USD");
        assert_eq!(entry.amount(), dec!(42.50));
        assert_eq!(entry.entry_type(), EntryType::Expense);
    }
}
