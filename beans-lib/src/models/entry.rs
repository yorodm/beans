//! Ledger entry model for representing financial transactions.

use crate::error::{BeansError, BeansResult};
use crate::models::{Currency, Tag};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
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
            _ => Err(BeansError::validation(
                format!("Invalid entry type: '{}'. Expected 'income' or 'expense'", s)
            )),
        }
    }
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
    
    /// Returns a summary string of this entry.
    ///
    /// Format: "[date] [name] ([currency] [amount]) [tags]"
    pub fn summary(&self) -> String {
        let tags_str = if self.tags.is_empty() {
            String::new()
        } else {
            let mut tags: Vec<_> = self.tags.iter().collect();
            tags.sort_by(|a, b| a.name().cmp(b.name()));
            
            format!(" [{}]", tags.iter()
                .map(|tag| tag.name())
                .collect::<Vec<_>>()
                .join(", "))
        };
        
        let formatted_amount = self.currency.with_amount(self.amount.to_f64().unwrap_or(0.0));
        format!(
            "{} {} ({} {}){}",
            self.date.format("%Y-%m-%d"),
            self.name,
            self.currency,
            formatted_amount,
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
    pub fn currency(mut self, currency: Currency) -> Self {
        self.currency = Some(currency);
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

    /// Builds the ledger entry.
    ///
    /// Returns an error if any required field is missing or invalid.
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
        
        // Validate amount is positive
        if amount <= Decimal::ZERO {
            return Err(BeansError::validation(
                "Entry amount must be positive"
            ));
        }
        
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
    
    /// Creates a builder pre-populated with values from an existing entry.
    ///
    /// This is useful for creating a modified copy of an existing entry.
    pub fn from_entry(entry: &LedgerEntry) -> Self {
        Self {
            id: Some(entry.id),
            date: Some(entry.date),
            name: Some(entry.name.clone()),
            currency: Some(entry.currency.clone()),
            amount: Some(entry.amount),
            description: entry.description.clone(),
            tags: entry.tags.clone(),
            entry_type: Some(entry.entry_type),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::prelude::dec;

    #[test]
    fn test_entry_type_from_str() {
        assert_eq!(EntryType::from_str("income").unwrap(), EntryType::Income);
        assert_eq!(EntryType::from_str("INCOME").unwrap(), EntryType::Income);
        assert_eq!(EntryType::from_str(" income ").unwrap(), EntryType::Income);
        
        assert_eq!(EntryType::from_str("expense").unwrap(), EntryType::Expense);
        assert_eq!(EntryType::from_str("EXPENSE").unwrap(), EntryType::Expense);
        
        assert!(EntryType::from_str("invalid").is_err());
        assert!(EntryType::from_str("").is_err());
    }
    
    #[test]
    fn test_entry_type_display() {
        assert_eq!(format!("{}", EntryType::Income), "income");
        assert_eq!(format!("{}", EntryType::Expense), "expense");
    }
    
    #[test]
    fn test_entry_type_all() {
        let all = EntryType::all();
        assert_eq!(all.len(), 2);
        assert!(all.contains(&EntryType::Income));
        assert!(all.contains(&EntryType::Expense));
    }

    #[test]
    fn test_entry_builder_basic() {
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
        assert!(entry.description().is_none());
        assert!(entry.tags().is_empty());
    }
    
    #[test]
    fn test_entry_builder_full() {
        let tag1 = Tag::new("groceries").unwrap();
        let tag2 = Tag::new("food").unwrap();
        
        let id = Uuid::new_v4();
        let date = Utc::now();
        
        let entry = LedgerEntryBuilder::new()
            .id(id)
            .date(date)
            .name("Weekly Groceries")
            .currency(Currency::usd())
            .amount(dec!(125.40))
            .description("Weekly grocery shopping")
            .tag(tag1.clone())
            .tag(tag2.clone())
            .entry_type(EntryType::Expense)
            .build()
            .unwrap();

        assert_eq!(entry.id(), id);
        assert_eq!(entry.date(), date);
        assert_eq!(entry.name(), "Weekly Groceries");
        assert_eq!(entry.currency().code(), "USD");
        assert_eq!(entry.amount(), dec!(125.40));
        assert_eq!(entry.description(), Some("Weekly grocery shopping"));
        assert_eq!(entry.tags().len(), 2);
        assert!(entry.tags().contains(&tag1));
        assert!(entry.tags().contains(&tag2));
        assert_eq!(entry.entry_type(), EntryType::Expense);
    }
    
    #[test]
    fn test_entry_builder_validation() {
        // Missing name
        let result = LedgerEntryBuilder::new()
            .currency(Currency::usd())
            .amount(dec!(42.50))
            .entry_type(EntryType::Expense)
            .build();
        assert!(result.is_err());
        
        // Empty name
        let result = LedgerEntryBuilder::new()
            .name("")
            .currency(Currency::usd())
            .amount(dec!(42.50))
            .entry_type(EntryType::Expense)
            .build();
        assert!(result.is_err());
        
        // Missing currency
        let result = LedgerEntryBuilder::new()
            .name("Groceries")
            .amount(dec!(42.50))
            .entry_type(EntryType::Expense)
            .build();
        assert!(result.is_err());
        
        // Missing amount
        let result = LedgerEntryBuilder::new()
            .name("Groceries")
            .currency(Currency::usd())
            .entry_type(EntryType::Expense)
            .build();
        assert!(result.is_err());
        
        // Zero amount
        let result = LedgerEntryBuilder::new()
            .name("Groceries")
            .currency(Currency::usd())
            .amount(dec!(0))
            .entry_type(EntryType::Expense)
            .build();
        assert!(result.is_err());
        
        // Negative amount
        let result = LedgerEntryBuilder::new()
            .name("Groceries")
            .currency(Currency::usd())
            .amount(dec!(-42.50))
            .entry_type(EntryType::Expense)
            .build();
        assert!(result.is_err());
        
        // Missing entry type
        let result = LedgerEntryBuilder::new()
            .name("Groceries")
            .currency(Currency::usd())
            .amount(dec!(42.50))
            .build();
        assert!(result.is_err());
    }
    
    #[test]
    fn test_entry_builder_from_entry() {
        let tag = Tag::new("groceries").unwrap();
        
        let original = LedgerEntryBuilder::new()
            .name("Groceries")
            .currency(Currency::usd())
            .amount(dec!(42.50))
            .tag(tag.clone())
            .entry_type(EntryType::Expense)
            .build()
            .unwrap();
            
        // Create a modified copy
        let modified = LedgerEntryBuilder::from_entry(&original)
            .name("Updated Groceries")
            .amount(dec!(50.00))
            .build()
            .unwrap();
            
        // Check that modified values changed
        assert_eq!(modified.name(), "Updated Groceries");
        assert_eq!(modified.amount(), dec!(50.00));
        
        // Check that unmodified values stayed the same
        assert_eq!(modified.id(), original.id());
        assert_eq!(modified.currency().code(), "USD");
        assert_eq!(modified.entry_type(), EntryType::Expense);
        assert!(modified.tags().contains(&tag));
    }
    
    #[test]
    fn test_entry_tags_methods() {
        let tag1 = Tag::new("groceries").unwrap();
        let tag2 = Tag::new("food").unwrap();
        
        let entry = LedgerEntryBuilder::new()
            .name("Groceries")
            .currency(Currency::usd())
            .amount(dec!(42.50))
            .tag(tag1)
            .tag(tag2)
            .entry_type(EntryType::Expense)
            .build()
            .unwrap();
            
        assert!(entry.has_tag("groceries"));
        assert!(entry.has_tag("GROCERIES"));  // Case insensitive
        assert!(entry.has_tag("food"));
        assert!(!entry.has_tag("household"));
        
        assert!(entry.has_all_tags(&["groceries", "food"]));
        assert!(!entry.has_all_tags(&["groceries", "household"]));
        
        assert!(entry.has_any_tag(&["household", "groceries"]));
        assert!(!entry.has_any_tag(&["household", "electronics"]));
    }
    
    #[test]
    fn test_entry_summary_and_display() {
        let tag1 = Tag::new("groceries").unwrap();
        let tag2 = Tag::new("food").unwrap();
        
        let date = DateTime::parse_from_rfc3339("2023-01-15T12:00:00Z")
            .unwrap()
            .with_timezone(&Utc);
            
        let entry = LedgerEntryBuilder::new()
            .date(date)
            .name("Weekly Groceries")
            .currency(Currency::usd())
            .amount(dec!(125.40))
            .tag(tag1)
            .tag(tag2)
            .entry_type(EntryType::Expense)
            .build()
            .unwrap();
            
        let expected = "2023-01-15 Weekly Groceries (USD 125.40) [food, groceries]";
        assert_eq!(entry.summary(), expected);
        assert_eq!(format!("{}", entry), expected);
        
        // Test without tags
        let entry_no_tags = LedgerEntryBuilder::new()
            .date(date)
            .name("Weekly Groceries")
            .currency(Currency::usd())
            .amount(dec!(125.40))
            .entry_type(EntryType::Expense)
            .build()
            .unwrap();
            
        let expected_no_tags = "2023-01-15 Weekly Groceries (USD 125.40)";
        assert_eq!(entry_no_tags.summary(), expected_no_tags);
    }
    
    #[test]
    fn test_entry_with_updated_at() {
        let entry = LedgerEntryBuilder::new()
            .name("Groceries")
            .currency(Currency::usd())
            .amount(dec!(42.50))
            .entry_type(EntryType::Expense)
            .build()
            .unwrap();
            
        let original_updated_at = entry.updated_at();
        
        // Wait a moment to ensure different timestamp
        std::thread::sleep(std::time::Duration::from_millis(10));
        
        let new_time = Utc::now();
        let updated = entry.with_updated_at(new_time);
        
        // Check that updated_at changed
        assert_ne!(updated.updated_at(), original_updated_at);
        assert_eq!(updated.updated_at(), new_time);
        
        // Check that other fields stayed the same
        assert_eq!(updated.id(), entry.id());
        assert_eq!(updated.name(), "Groceries");
        assert_eq!(updated.amount(), dec!(42.50));
    }
    
    #[test]
    fn test_entry_builder_tags_method() {
        let tag1 = Tag::new("groceries").unwrap();
        let tag2 = Tag::new("food").unwrap();
        let tag3 = Tag::new("household").unwrap();
        
        let tags = vec![tag1.clone(), tag2.clone(), tag3.clone()];
        
        let entry = LedgerEntryBuilder::new()
            .name("Groceries")
            .currency(Currency::usd())
            .amount(dec!(42.50))
            .tags(tags)
            .entry_type(EntryType::Expense)
            .build()
            .unwrap();
            
        assert_eq!(entry.tags().len(), 3);
        assert!(entry.has_tag("groceries"));
        assert!(entry.has_tag("food"));
        assert!(entry.has_tag("household"));
    }
}
