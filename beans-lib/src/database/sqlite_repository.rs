//! SQLite implementation of the Repository trait.

use crate::database::{EntryFilter, Repository};
use crate::error::{BeansError, BeansResult};
use crate::models::{Currency, EntryType, LedgerEntry, LedgerEntryBuilder, Tag};
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Transaction};
use rust_decimal::Decimal;
use std::collections::HashSet;
use std::path::Path;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

/// SQLite implementation of the Repository trait.
#[derive(Debug)]
pub struct SQLiteRepository {
    /// Connection to the SQLite database.
    conn: Arc<Mutex<Connection>>,
}

impl SQLiteRepository {
    /// Creates a new SQLiteRepository with the given connection.
    pub fn new(conn: Connection) -> Self {
        Self {
            conn: Arc::new(Mutex::new(conn)),
        }
    }

    /// Opens a SQLite database at the given path.
    pub fn open<P: AsRef<Path>>(path: P) -> BeansResult<Self> {
        let conn = Connection::open(path)
            .map_err(|e| BeansError::database(format!("Failed to open database: {}", e)))?;
        
        // Enable foreign keys
        conn.execute("PRAGMA foreign_keys = ON", [])
            .map_err(|e| BeansError::database(format!("Failed to enable foreign keys: {}", e)))?;
        
        Ok(Self::new(conn))
    }

    /// Creates an in-memory SQLite database.
    pub fn in_memory() -> BeansResult<Self> {
        let conn = Connection::open_in_memory()
            .map_err(|e| BeansError::database(format!("Failed to open in-memory database: {}", e)))?;
        
        // Enable foreign keys
        conn.execute("PRAGMA foreign_keys = ON", [])
            .map_err(|e| BeansError::database(format!("Failed to enable foreign keys: {}", e)))?;
        
        Ok(Self::new(conn))
    }

    /// Gets a tag ID by name, creating it if it doesn't exist.
    fn get_or_create_tag_id(&self, tx: &Transaction, tag_name: &str) -> BeansResult<i64> {
        // Try to get the tag ID
        let mut stmt = tx.prepare("SELECT id FROM tags WHERE name = ?")
            .map_err(|e| BeansError::database(format!("Failed to prepare tag query: {}", e)))?;
        
        let tag_id: Result<i64, rusqlite::Error> = stmt.query_row(params![tag_name], |row| row.get(0));
        
        match tag_id {
            Ok(id) => Ok(id),
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                // Tag doesn't exist, create it
                tx.execute("INSERT INTO tags (name) VALUES (?)", params![tag_name])
                    .map_err(|e| BeansError::database(format!("Failed to insert tag: {}", e)))?;
                
                Ok(tx.last_insert_rowid())
            },
            Err(e) => Err(BeansError::database(format!("Failed to query tag: {}", e))),
        }
    }

    /// Saves the tags for an entry.
    fn save_tags(&self, tx: &Transaction, entry_id: &Uuid, tags: &[Tag]) -> BeansResult<()> {
        // Delete existing tags for this entry
        tx.execute(
            "DELETE FROM entry_tags WHERE entry_id = ?",
            params![entry_id.to_string()],
        ).map_err(|e| BeansError::database(format!("Failed to delete existing tags: {}", e)))?;
        
        // Insert new tags
        for tag in tags {
            let tag_id = self.get_or_create_tag_id(tx, tag.name())?;
            
            tx.execute(
                "INSERT INTO entry_tags (entry_id, tag_id) VALUES (?, ?)",
                params![entry_id.to_string(), tag_id],
            ).map_err(|e| BeansError::database(format!("Failed to insert entry tag: {}", e)))?;
        }
        
        Ok(())
    }

    /// Loads the tags for an entry.
    fn load_tags(&self, tx: &Transaction, entry_id: &Uuid) -> BeansResult<Vec<Tag>> {
        let mut stmt = tx.prepare(
            "SELECT t.name FROM tags t
             JOIN entry_tags et ON t.id = et.tag_id
             WHERE et.entry_id = ?
             ORDER BY t.name"
        ).map_err(|e| BeansError::database(format!("Failed to prepare tags query: {}", e)))?;
        
        let tag_iter = stmt.query_map(params![entry_id.to_string()], |row| {
            let name: String = row.get(0)?;
            Ok(name)
        }).map_err(|e| BeansError::database(format!("Failed to query tags: {}", e)))?;
        
        let mut tags = Vec::new();
        for tag_result in tag_iter {
            let tag_name = tag_result
                .map_err(|e| BeansError::database(format!("Failed to read tag: {}", e)))?;
            
            let tag = Tag::new(&tag_name)
                .map_err(|e| BeansError::database(format!("Invalid tag in database: {}", e)))?;
            
            tags.push(tag);
        }
        
        Ok(tags)
    }

    /// Converts a database row to a LedgerEntry.
    fn row_to_entry(&self, tx: &Transaction, row: &rusqlite::Row) -> BeansResult<LedgerEntry<'static>> {
        let id: String = row.get(0)?;
        let id = Uuid::parse_str(&id)
            .map_err(|e| BeansError::database(format!("Invalid UUID in database: {}", e)))?;
        
        let date: String = row.get(1)?;
        let date = DateTime::parse_from_rfc3339(&date)
            .map_err(|e| BeansError::database(format!("Invalid date in database: {}", e)))?
            .with_timezone(&Utc);
        
        let name: String = row.get(2)?;
        
        let currency_code: String = row.get(3)?;
        let amount_str: String = row.get(4)?;
        let amount = Decimal::from_str_exact(&amount_str)
            .map_err(|e| BeansError::database(format!("Invalid amount in database: {}", e)))?;
        
        let description: Option<String> = row.get(5)?;
        
        let entry_type_str: String = row.get(6)?;
        let entry_type = match entry_type_str.as_str() {
            "Income" => EntryType::Income,
            "Expense" => EntryType::Expense,
            _ => return Err(BeansError::database(format!("Invalid entry type in database: {}", entry_type_str))),
        };
        
        let created_at: String = row.get(7)?;
        let created_at = DateTime::parse_from_rfc3339(&created_at)
            .map_err(|e| BeansError::database(format!("Invalid created_at in database: {}", e)))?
            .with_timezone(&Utc);
        
        let updated_at: String = row.get(8)?;
        let updated_at = DateTime::parse_from_rfc3339(&updated_at)
            .map_err(|e| BeansError::database(format!("Invalid updated_at in database: {}", e)))?
            .with_timezone(&Utc);
        
        // Load tags
        let tags = self.load_tags(tx, &id)?;
        
        // Create currency
        let currency = Currency::new(amount, &currency_code)
            .map_err(|e| BeansError::database(format!("Invalid currency in database: {}", e)))?;
        
        // Build the entry
        let mut builder = LedgerEntryBuilder::new()
            .id(id)
            .date(date)
            .name(name)
            .currency(currency)
            .entry_type(entry_type)
            .created_at(created_at)
            .updated_at(updated_at);
        
        if let Some(desc) = description {
            builder = builder.description(desc);
        }
        
        for tag in tags {
            builder = builder.tag(tag);
        }
        
        builder.build()
    }

    /// Builds a WHERE clause and parameters for the given filter.
    fn build_filter_clause(&self, filter: &EntryFilter) -> (String, Vec<Box<dyn rusqlite::ToSql>>) {
        let mut conditions = Vec::new();
        let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
        
        if let Some(start_date) = filter.start_date {
            conditions.push("date >= ?");
            params.push(Box::new(start_date.to_rfc3339()));
        }
        
        if let Some(end_date) = filter.end_date {
            conditions.push("date <= ?");
            params.push(Box::new(end_date.to_rfc3339()));
        }
        
        if let Some(entry_type) = &filter.entry_type {
            conditions.push("entry_type = ?");
            params.push(Box::new(format!("{:?}", entry_type)));
        }
        
        if let Some(currency) = &filter.currency {
            conditions.push("currency = ?");
            params.push(Box::new(currency.clone()));
        }
        
        // Handle tags filter if there are any tags
        if !filter.tags.is_empty() {
            let placeholders = vec!["?"; filter.tags.len()].join(", ");
            let tag_condition = format!(
                "id IN (
                    SELECT entry_id FROM entry_tags
                    JOIN tags ON entry_tags.tag_id = tags.id
                    WHERE tags.name IN ({})
                    GROUP BY entry_id
                    HAVING COUNT(DISTINCT tags.name) = ?
                )",
                placeholders
            );
            
            conditions.push(&tag_condition);
            
            for tag in &filter.tags {
                params.push(Box::new(tag.clone()));
            }
            
            // Add the count of tags to ensure all tags are matched
            params.push(Box::new(filter.tags.len() as i64));
        }
        
        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };
        
        (where_clause, params)
    }
}

impl Repository for SQLiteRepository {
    fn create<'a>(&self, entry: &LedgerEntry<'a>) -> BeansResult<()> {
        let conn = self.conn.lock().unwrap();
        let tx = conn.transaction()
            .map_err(|e| BeansError::database(format!("Failed to start transaction: {}", e)))?;
        
        // Insert the entry
        tx.execute(
            "INSERT INTO entries (id, date, name, currency, amount, description, entry_type, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                entry.id().to_string(),
                entry.date().to_rfc3339(),
                entry.name(),
                entry.currency().code(),
                entry.amount().to_string(),
                entry.description(),
                format!("{:?}", entry.entry_type()),
                entry.created_at().to_rfc3339(),
                entry.updated_at().to_rfc3339(),
            ],
        ).map_err(|e| BeansError::database(format!("Failed to insert entry: {}", e)))?;
        
        // Save tags
        self.save_tags(&tx, entry.id(), entry.tags())?;
        
        // Commit the transaction
        tx.commit()
            .map_err(|e| BeansError::database(format!("Failed to commit transaction: {}", e)))?;
        
        Ok(())
    }
    
    fn get(&self, id: Uuid) -> BeansResult<LedgerEntry<'_>> {
        let conn = self.conn.lock().unwrap();
        let tx = conn.transaction()
            .map_err(|e| BeansError::database(format!("Failed to start transaction: {}", e)))?;
        
        let mut stmt = tx.prepare(
            "SELECT id, date, name, currency, amount, description, entry_type, created_at, updated_at
             FROM entries
             WHERE id = ?"
        ).map_err(|e| BeansError::database(format!("Failed to prepare query: {}", e)))?;
        
        let entry = stmt.query_row(params![id.to_string()], |row| {
            self.row_to_entry(&tx, row)
        }).map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => BeansError::NotFound(format!("Entry with ID {} not found", id)),
            _ => BeansError::database(format!("Failed to query entry: {}", e)),
        })?;
        
        Ok(entry)
    }
    
    fn update<'a>(&self, entry: &LedgerEntry<'a>) -> BeansResult<()> {
        let conn = self.conn.lock().unwrap();
        let tx = conn.transaction()
            .map_err(|e| BeansError::database(format!("Failed to start transaction: {}", e)))?;
        
        // Check if the entry exists
        let exists: bool = tx.query_row(
            "SELECT 1 FROM entries WHERE id = ?",
            params![entry.id().to_string()],
            |_| Ok(true),
        ).unwrap_or(false);
        
        if !exists {
            return Err(BeansError::NotFound(format!("Entry with ID {} not found", entry.id())));
        }
        
        // Update the entry
        tx.execute(
            "UPDATE entries
             SET date = ?, name = ?, currency = ?, amount = ?, description = ?, entry_type = ?, updated_at = ?
             WHERE id = ?",
            params![
                entry.date().to_rfc3339(),
                entry.name(),
                entry.currency().code(),
                entry.amount().to_string(),
                entry.description(),
                format!("{:?}", entry.entry_type()),
                entry.updated_at().to_rfc3339(),
                entry.id().to_string(),
            ],
        ).map_err(|e| BeansError::database(format!("Failed to update entry: {}", e)))?;
        
        // Save tags
        self.save_tags(&tx, entry.id(), entry.tags())?;
        
        // Commit the transaction
        tx.commit()
            .map_err(|e| BeansError::database(format!("Failed to commit transaction: {}", e)))?;
        
        Ok(())
    }
    
    fn delete(&self, id: Uuid) -> BeansResult<()> {
        let conn = self.conn.lock().unwrap();
        
        // Check if the entry exists
        let exists: bool = conn.query_row(
            "SELECT 1 FROM entries WHERE id = ?",
            params![id.to_string()],
            |_| Ok(true),
        ).unwrap_or(false);
        
        if !exists {
            return Err(BeansError::NotFound(format!("Entry with ID {} not found", id)));
        }
        
        // Delete the entry (cascade will delete entry_tags)
        conn.execute(
            "DELETE FROM entries WHERE id = ?",
            params![id.to_string()],
        ).map_err(|e| BeansError::database(format!("Failed to delete entry: {}", e)))?;
        
        Ok(())
    }
    
    fn list(&self, filter: &EntryFilter) -> BeansResult<Vec<LedgerEntry<'_>>> {
        let conn = self.conn.lock().unwrap();
        let tx = conn.transaction()
            .map_err(|e| BeansError::database(format!("Failed to start transaction: {}", e)))?;
        
        // Build the filter clause
        let (where_clause, params) = self.build_filter_clause(filter);
        
        // Build the query
        let mut query = format!(
            "SELECT id, date, name, currency, amount, description, entry_type, created_at, updated_at
             FROM entries
             {}
             ORDER BY date DESC",
            where_clause
        );
        
        // Add limit and offset if specified
        if let Some(limit) = filter.limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }
        
        if let Some(offset) = filter.offset {
            query.push_str(&format!(" OFFSET {}", offset));
        }
        
        // Prepare and execute the query
        let mut stmt = tx.prepare(&query)
            .map_err(|e| BeansError::database(format!("Failed to prepare query: {}", e)))?;
        
        let mut param_refs: Vec<&dyn rusqlite::ToSql> = params.iter()
            .map(|p| p.as_ref() as &dyn rusqlite::ToSql)
            .collect();
        
        let rows = stmt.query(rusqlite::params_from_iter(param_refs.iter()))
            .map_err(|e| BeansError::database(format!("Failed to execute query: {}", e)))?;
        
        let mut entries = Vec::new();
        for row_result in rows.mapped(|row| self.row_to_entry(&tx, row)) {
            let entry = row_result
                .map_err(|e| BeansError::database(format!("Failed to read entry: {}", e)))?;
            
            entries.push(entry);
        }
        
        Ok(entries)
    }
    
    fn count(&self, filter: &EntryFilter) -> BeansResult<usize> {
        let conn = self.conn.lock().unwrap();
        
        // Build the filter clause
        let (where_clause, params) = self.build_filter_clause(filter);
        
        // Build the query
        let query = format!(
            "SELECT COUNT(*) FROM entries {}",
            where_clause
        );
        
        // Prepare and execute the query
        let mut stmt = conn.prepare(&query)
            .map_err(|e| BeansError::database(format!("Failed to prepare query: {}", e)))?;
        
        let mut param_refs: Vec<&dyn rusqlite::ToSql> = params.iter()
            .map(|p| p.as_ref() as &dyn rusqlite::ToSql)
            .collect();
        
        let count: i64 = stmt.query_row(
            rusqlite::params_from_iter(param_refs.iter()),
            |row| row.get(0),
        ).map_err(|e| BeansError::database(format!("Failed to count entries: {}", e)))?;
        
        Ok(count as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::currency::usd_with_amount;
    use rust_decimal_macros::dec;
    
    fn create_test_entry() -> LedgerEntry<'static> {
        LedgerEntryBuilder::new()
            .name("Test Entry")
            .amount(dec!(100.00))
            .currency(usd_with_amount(dec!(100.00)))
            .entry_type(EntryType::Income)
            .description("Test description")
            .tag(Tag::new("test").unwrap())
            .tag(Tag::new("example").unwrap())
            .build()
            .unwrap()
    }
    
    #[test]
    fn test_create_and_get_entry() -> BeansResult<()> {
        let repo = SQLiteRepository::in_memory()?;
        
        // Initialize schema
        let conn = repo.conn.lock().unwrap();
        crate::database::schema::initialize_schema(&conn)?;
        drop(conn);
        
        // Create a test entry
        let entry = create_test_entry();
        repo.create(&entry)?;
        
        // Get the entry
        let retrieved = repo.get(*entry.id())?;
        
        // Verify the entry
        assert_eq!(retrieved.id(), entry.id());
        assert_eq!(retrieved.name(), entry.name());
        assert_eq!(retrieved.amount(), entry.amount());
        assert_eq!(retrieved.currency().code(), entry.currency().code());
        assert_eq!(retrieved.entry_type(), entry.entry_type());
        assert_eq!(retrieved.description(), entry.description());
        
        // Verify tags
        let retrieved_tags: HashSet<String> = retrieved.tags().iter()
            .map(|t| t.name().to_string())
            .collect();
        
        let expected_tags: HashSet<String> = entry.tags().iter()
            .map(|t| t.name().to_string())
            .collect();
        
        assert_eq!(retrieved_tags, expected_tags);
        
        Ok(())
    }
    
    #[test]
    fn test_update_entry() -> BeansResult<()> {
        let repo = SQLiteRepository::in_memory()?;
        
        // Initialize schema
        let conn = repo.conn.lock().unwrap();
        crate::database::schema::initialize_schema(&conn)?;
        drop(conn);
        
        // Create a test entry
        let entry = create_test_entry();
        repo.create(&entry)?;
        
        // Update the entry
        let updated = LedgerEntryBuilder::from(&entry)
            .name("Updated Entry")
            .amount(dec!(200.00))
            .description("Updated description")
            .tag(Tag::new("updated").unwrap())
            .build()?;
        
        repo.update(&updated)?;
        
        // Get the updated entry
        let retrieved = repo.get(*entry.id())?;
        
        // Verify the entry
        assert_eq!(retrieved.name(), "Updated Entry");
        assert_eq!(retrieved.amount(), dec!(200.00));
        assert_eq!(retrieved.description(), Some("Updated description".to_string()));
        
        // Verify tags
        let retrieved_tags: HashSet<String> = retrieved.tags().iter()
            .map(|t| t.name().to_string())
            .collect();
        
        let expected_tags: HashSet<String> = vec!["updated".to_string()].into_iter().collect();
        
        assert_eq!(retrieved_tags, expected_tags);
        
        Ok(())
    }
    
    #[test]
    fn test_delete_entry() -> BeansResult<()> {
        let repo = SQLiteRepository::in_memory()?;
        
        // Initialize schema
        let conn = repo.conn.lock().unwrap();
        crate::database::schema::initialize_schema(&conn)?;
        drop(conn);
        
        // Create a test entry
        let entry = create_test_entry();
        repo.create(&entry)?;
        
        // Delete the entry
        repo.delete(*entry.id())?;
        
        // Try to get the entry
        let result = repo.get(*entry.id());
        assert!(matches!(result, Err(BeansError::NotFound(_))));
        
        Ok(())
    }
    
    #[test]
    fn test_list_entries() -> BeansResult<()> {
        let repo = SQLiteRepository::in_memory()?;
        
        // Initialize schema
        let conn = repo.conn.lock().unwrap();
        crate::database::schema::initialize_schema(&conn)?;
        drop(conn);
        
        // Create test entries
        let entry1 = LedgerEntryBuilder::new()
            .name("Income Entry")
            .amount(dec!(100.00))
            .currency(usd_with_amount(dec!(100.00)))
            .entry_type(EntryType::Income)
            .tag(Tag::new("salary").unwrap())
            .build()?;
        
        let entry2 = LedgerEntryBuilder::new()
            .name("Expense Entry")
            .amount(dec!(50.00))
            .currency(usd_with_amount(dec!(50.00)))
            .entry_type(EntryType::Expense)
            .tag(Tag::new("food").unwrap())
            .build()?;
        
        repo.create(&entry1)?;
        repo.create(&entry2)?;
        
        // List all entries
        let all_entries = repo.list(&EntryFilter::default())?;
        assert_eq!(all_entries.len(), 2);
        
        // Filter by entry type
        let income_filter = EntryFilter {
            entry_type: Some(EntryType::Income),
            ..Default::default()
        };
        
        let income_entries = repo.list(&income_filter)?;
        assert_eq!(income_entries.len(), 1);
        assert_eq!(income_entries[0].name(), "Income Entry");
        
        // Filter by tag
        let tag_filter = EntryFilter {
            tags: vec!["food".to_string()],
            ..Default::default()
        };
        
        let tagged_entries = repo.list(&tag_filter)?;
        assert_eq!(tagged_entries.len(), 1);
        assert_eq!(tagged_entries[0].name(), "Expense Entry");
        
        Ok(())
    }
    
    #[test]
    fn test_count_entries() -> BeansResult<()> {
        let repo = SQLiteRepository::in_memory()?;
        
        // Initialize schema
        let conn = repo.conn.lock().unwrap();
        crate::database::schema::initialize_schema(&conn)?;
        drop(conn);
        
        // Create test entries
        let entry1 = LedgerEntryBuilder::new()
            .name("Income Entry")
            .amount(dec!(100.00))
            .currency(usd_with_amount(dec!(100.00)))
            .entry_type(EntryType::Income)
            .build()?;
        
        let entry2 = LedgerEntryBuilder::new()
            .name("Expense Entry")
            .amount(dec!(50.00))
            .currency(usd_with_amount(dec!(50.00)))
            .entry_type(EntryType::Expense)
            .build()?;
        
        repo.create(&entry1)?;
        repo.create(&entry2)?;
        
        // Count all entries
        let all_count = repo.count(&EntryFilter::default())?;
        assert_eq!(all_count, 2);
        
        // Count by entry type
        let income_filter = EntryFilter {
            entry_type: Some(EntryType::Income),
            ..Default::default()
        };
        
        let income_count = repo.count(&income_filter)?;
        assert_eq!(income_count, 1);
        
        Ok(())
    }
}

