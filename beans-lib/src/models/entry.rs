//! Ledger entry model for representing financial transactions.

use crate::error::{BeansError, BeansResult};
use crate::models::{Currency, Tag};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt;
use std::str::FromStr;
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

impl EntryType {
    /// Returns a string representation of the entry type.
    pub fn as_str(&self) -> &'static str {
        match self {
            EntryType::Income => "income",
            EntryType::Expense => "expense",
        }
    }

    /// Returns all possible entry types.
    pub fn all() -> [EntryType; 2] {
        [EntryType::Income, EntryType::Expense]
    }
}

impl fmt::Display for EntryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for EntryType {
    type Err = BeansError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "income" => Ok(EntryType::Income),
            "expense" => Ok(EntryType::Expense),
            _ => Err(BeansError::validation(format!(
                "Invalid entry type: '{}'. Expected 'income' or 'expense'",
                s
            ))),
        }
    }
}

/// Represents a financial transaction in the ledger.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LedgerEntry {
    /// Unique identifier for the entry.
    id: Uuid,
    /// Date and time of the transaction.
    date: DateTime<Utc>,
    /// Name/title of the transaction.
    name: String,
    /// Currency of the transaction (ISO Code).
    currency_code: String,
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

// We're using the currency_serde module from the currency module

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
    pub fn currency_code(&self) -> String {
        self.currency_code.clone()
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

    /// Creates an updated copy of this entry with the given update time.
    ///
    /// This is primarily used when updating entries in the database.
    pub fn with_updated_at(&self, updated_at: DateTime<Utc>) -> Self {
        let mut entry = self.clone();
        entry.updated_at = updated_at;
        entry
    }

    /// Returns true if this entry has the specified tag.
    pub fn has_tag(&self, tag_name: &str) -> bool {
        let normalized = tag_name.trim().to_lowercase();
        self.tags.iter().any(|tag| tag.name() == normalized)
    }

    /// Returns true if this entry has all the specified tags.
    pub fn has_all_tags<I, S>(&self, tags: I) -> bool
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        tags.into_iter().all(|tag| self.has_tag(tag.as_ref()))
    }

    /// Returns true if this entry has any of the specified tags.
    pub fn has_any_tag<I, S>(&self, tags: I) -> bool
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        tags.into_iter().any(|tag| self.has_tag(tag.as_ref()))
    }

    pub fn currency<'a>(&self) -> BeansResult<Currency<'a>> {
        // Generates a Currency given the value of code and amount on this entry
        let c = Currency::new(self.amount, &self.currency_code);
        return c;
    }
    /// Returns a summary string of this entry.
    ///
    /// Format: "[date] [name] ([currency] [amount]) [tags]"
    pub fn summary(&self) -> String {
        let tags_str = if self.tags.is_empty() {
            String::new()
        } else {
            let mut tags: Vec<_> = self.tags.iter().collect();
            tags.sort_by(|a, b| a.name().cmp(b.name()));

            format!(
                " [{}]",
                tags.iter()
                    .map(|tag| tag.name())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        };

        format!(
            "{} {} ({} {}){}",
            self.date.format("%Y-%m-%d"),
            self.name,
            self.currency_code,
            self.amount,
            tags_str
        )
    }
}

impl fmt::Display for LedgerEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.summary())
    }
}

/// Builder for creating ledger entries.
#[derive(Debug, Default)]
pub struct LedgerEntryBuilder {
    id: Option<Uuid>,
    date: Option<DateTime<Utc>>,
    name: Option<String>,
    currency_code: Option<String>,
    amount: Option<Decimal>,
    description: Option<String>,
    tags: HashSet<Tag>,
    entry_type: Option<EntryType>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}

impl LedgerEntryBuilder {
    /// Creates a new builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the entry's unique identifier.
    ///
    /// If not set, a random UUID will be generated.
    pub fn id(mut self, id: Uuid) -> Self {
        self.id = Some(id);
        self
    }

    /// Sets the date and time of the transaction.
    ///
    /// If not set, the current date and time will be used.
    pub fn date(mut self, date: DateTime<Utc>) -> Self {
        self.date = Some(date);
        self
    }

    /// Sets the name/title of the transaction.
    ///
    /// This field is required.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Sets the currency of the transaction.
    ///
    /// This field is required.
    pub fn currency_code(mut self, currency_code: String) -> Self {
        self.currency_code = Some(currency_code);
        self
    }

    /// Sets the amount of the transaction.
    ///
    /// This field is required and must be positive.
    pub fn amount(mut self, amount: Decimal) -> Self {
        self.amount = Some(amount);
        self
    }

    /// Sets the description of the transaction.
    ///
    /// This field is optional.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Adds a tag to the transaction.
    ///
    /// Multiple tags can be added by calling this method multiple times.
    pub fn tag(mut self, tag: Tag) -> Self {
        self.tags.insert(tag);
        self
    }

    /// Adds multiple tags to the transaction.
    pub fn tags<I>(mut self, tags: I) -> Self
    where
        I: IntoIterator<Item = Tag>,
    {
        for tag in tags {
            self.tags.insert(tag);
        }
        self
    }

    /// Sets the type of the transaction (income or expense).
    ///
    /// This field is required.
    pub fn entry_type(mut self, entry_type: EntryType) -> Self {
        self.entry_type = Some(entry_type);
        self
    }

    pub fn created_at(mut self, created_at: DateTime<Utc>) -> Self {
        self.created_at = Some(created_at);
        self
    }

    pub fn updated_at(mut self, uptaded_at: DateTime<Utc>) -> Self {
        self.updated_at = Some(uptaded_at);
        self
    }

    /// Builds the ledger entry.
    ///
    /// Returns an error if any required field is missing or invalid.
    pub fn build(self) -> BeansResult<LedgerEntry> {
        let now = Utc::now();

        let name = self
            .name
            .ok_or_else(|| BeansError::validation("Entry name is required"))?;

        if name.trim().is_empty() {
            return Err(BeansError::validation("Entry name cannot be empty"));
        }

        let currency_code = self
            .currency_code
            .ok_or_else(|| BeansError::validation("Entry currency is required"))?;

        let amount = self
            .amount
            .ok_or_else(|| BeansError::validation("Entry amount is required"))?;

        // Validate amount is positive
        if amount <= Decimal::ZERO {
            return Err(BeansError::validation("Entry amount must be positive"));
        }

        let entry_type = self
            .entry_type
            .ok_or_else(|| BeansError::validation("Entry type is required"))?;

        Ok(LedgerEntry {
            id: self.id.unwrap_or_else(Uuid::new_v4),
            date: self.date.unwrap_or_else(Utc::now),
            name,
            currency_code,
            amount,
            description: self.description,
            tags: self.tags,
            entry_type,
            created_at: self.created_at.unwrap_or(now),
            updated_at: self.updated_at.unwrap_or(now),
        })
    }

    /// Creates a builder pre-populated with values from an existing entry.
    ///
    /// This is useful for creating a modified copy of an existing entry.
    pub fn from_entry(entry: &LedgerEntry) -> Self {
        Self {
            id: Some(entry.id),
            date: Some(entry.date),
            name: Some(entry.name.clone()),
            currency_code: Some(entry.currency_code.clone()),
            amount: Some(entry.amount),
            description: entry.description.clone(),
            tags: entry.tags.clone(),
            entry_type: Some(entry.entry_type),
            created_at: Some(entry.created_at),
            updated_at: Some(entry.updated_at),
        }
    }
}
