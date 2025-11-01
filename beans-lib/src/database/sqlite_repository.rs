//! SQLite implementation of the Repository trait.

use crate::database::{EntryFilter, Repository};
use crate::error::{BeansError, BeansResult};
use crate::models::{EntryType, LedgerEntry, LedgerEntryBuilder, Tag};
use chrono::{DateTime, Utc};
use rusqlite::{params, types::Type, Connection, Transaction};
use rust_decimal::Decimal;
use sql_query_builder as sql;
use std::path::Path;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

/// SQLite implementation of the Repository trait.
#[derive(Debug)]
pub struct SQLiteRepository {
    /// Connection to the SQLite database.
    pub conn: Arc<Mutex<Connection>>,
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
        let conn = Connection::open_in_memory().map_err(|e| {
            BeansError::database(format!("Failed to open in-memory database: {}", e))
        })?;

        // Enable foreign keys
        conn.execute("PRAGMA foreign_keys = ON", [])
            .map_err(|e| BeansError::database(format!("Failed to enable foreign keys: {}", e)))?;

        Ok(Self::new(conn))
    }

    /// Gets a reference to the connection.
    pub fn get_connection(&self) -> BeansResult<&Arc<Mutex<Connection>>> {
        Ok(&self.conn)
    }

    /// Gets a tag ID by name, creating it if it doesn't exist.
    fn get_or_create_tag_id(&self, tx: &Transaction, tag_name: &str) -> BeansResult<i64> {
        // Try to get the tag ID
        let select_query = sql::Select::new()
            .select("id")
            .from("tags")
            .where_clause("name = ?")
            .as_string();

        let mut stmt = tx
            .prepare(&select_query)
            .map_err(|e| BeansError::database(format!("Failed to prepare tag query: {}", e)))?;

        let tag_id: Result<i64, rusqlite::Error> =
            stmt.query_row(params![tag_name], |row| row.get(0));

        match tag_id {
            Ok(id) => Ok(id),
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                // Tag doesn't exist, create it
                let insert_query = sql::Insert::new()
                    .insert_into("tags (name)")
                    .values("(?)")
                    .as_string();

                tx.execute(&insert_query, params![tag_name])
                    .map_err(|e| BeansError::database(format!("Failed to insert tag: {}", e)))?;

                Ok(tx.last_insert_rowid())
            }
            Err(e) => Err(BeansError::database(format!("Failed to query tag: {}", e))),
        }
    }

    /// Saves the tags for an entry.
    fn save_tags(&self, tx: &Transaction, entry_id: &Uuid, tags: &[Tag]) -> BeansResult<()> {
        // Delete existing tags for this entry
        let delete_query = sql::Delete::new()
            .delete_from("entry_tags")
            .where_clause("entry_id = ?")
            .as_string();

        tx.execute(&delete_query, params![entry_id.to_string()])
            .map_err(|e| BeansError::database(format!("Failed to delete existing tags: {}", e)))?;

        // Insert new tags
        for tag in tags {
            let tag_id = self.get_or_create_tag_id(tx, tag.name())?;

            let insert_query = sql::Insert::new()
                .insert_into("entry_tags (entry_id, tag_id)")
                .values("(?, ?)")
                .as_string();

            tx.execute(&insert_query, params![entry_id.to_string(), tag_id])
                .map_err(|e| BeansError::database(format!("Failed to insert entry tag: {}", e)))?;
        }

        Ok(())
    }

    /// Loads the tags for an entry.
    fn load_tags(&self, tx: &Transaction, entry_id: &Uuid) -> BeansResult<Vec<Tag>> {
        let select_query = sql::Select::new()
            .select("t.name")
            .from("tags t")
            .inner_join("entry_tags et ON t.id = et.tag_id")
            .where_clause("et.entry_id = ?")
            .order_by("t.name")
            .as_string();

        let mut stmt = tx
            .prepare(&select_query)
            .map_err(|e| BeansError::database(format!("Failed to prepare tags query: {}", e)))?;

        let tag_iter = stmt
            .query_map(params![entry_id.to_string()], |row| {
                let name: String = row.get(0)?;
                Ok(name)
            })
            .map_err(|e| BeansError::database(format!("Failed to query tags: {}", e)))?;

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
    fn row_to_entry(&self, tx: &Transaction, row: &rusqlite::Row) -> rusqlite::Result<LedgerEntry> {
        let id_str: String = row.get(0)?;
        let id = Uuid::parse_str(&id_str).map_err(|_| {
            rusqlite::Error::InvalidColumnType(0, "Invalid UUID".to_string(), Type::Text)
        })?;

        let date_str: String = row.get(1)?;
        let date = DateTime::parse_from_rfc3339(&date_str)
            .map_err(|_| {
                rusqlite::Error::InvalidColumnType(1, "Invalid date".to_string(), Type::Text)
            })?
            .with_timezone(&Utc);

        let name: String = row.get(2)?;

        let currency_code: String = row.get(3)?;
        // Use the helper function to create a static currency
        let currency = rusty_money::iso::find(&currency_code).ok_or(
            rusqlite::Error::InvalidColumnType(3, "Invalid amount".to_string(), Type::Text),
        )?;
        let amount_str: String = row.get(4)?;
        let amount = Decimal::from_str_exact(&amount_str).map_err(|_| {
            rusqlite::Error::InvalidColumnType(4, "Invalid amount".to_string(), Type::Text)
        })?;

        let description: Option<String> = row.get(5)?;

        let entry_type_str: String = row.get(6)?;
        let entry_type = match entry_type_str.as_str() {
            "Income" => EntryType::Income,
            "Expense" => EntryType::Expense,
            _ => {
                return Err(rusqlite::Error::InvalidColumnType(
                    6,
                    "Invalid entry type".to_string(),
                    Type::Text,
                ))
            }
        };

        let created_at_str: String = row.get(7)?;
        let _created_at = DateTime::parse_from_rfc3339(&created_at_str)
            .map_err(|_| {
                rusqlite::Error::InvalidColumnType(7, "Invalid created_at".to_string(), Type::Text)
            })?
            .with_timezone(&Utc);

        let updated_at_str: String = row.get(8)?;
        let _updated_at = DateTime::parse_from_rfc3339(&updated_at_str)
            .map_err(|_| {
                rusqlite::Error::InvalidColumnType(8, "Invalid updated_at".to_string(), Type::Text)
            })?
            .with_timezone(&Utc);

        // Load tags
        let tags = match self.load_tags(tx, &id) {
            Ok(t) => t,
            Err(_) => Vec::new(), // Fallback to empty tags on error
        };

        // Build the entry
        let mut builder = LedgerEntryBuilder::new()
            .id(id)
            .date(date)
            .name(name)
            .currency_code(currency.iso_alpha_code.to_owned())
            .amount(amount) // Add the amount to the builder
            .entry_type(entry_type);

        if let Some(desc) = description {
            builder = builder.description(desc);
        }

        for tag in tags {
            builder = builder.tag(tag);
        }

        match builder.build() {
            Ok(entry) => Ok(entry),
            Err(e) => Err(rusqlite::Error::InvalidColumnType(
                0,
                format!("Failed to build entry: {}", e),
                Type::Text,
            )),
        }
    }

    /// Builds a SELECT query with filters applied.
    fn build_filtered_query(
        &self,
        filter: &EntryFilter,
    ) -> (sql::Select, Vec<Box<dyn rusqlite::ToSql>>) {
        let mut select = sql::Select::new()
            .select(
                "id, date, name, currency, amount, description, entry_type, created_at, updated_at",
            )
            .from("entries");

        let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(start_date) = filter.start_date {
            select = select.where_clause("date >= ?");
            params.push(Box::new(start_date.to_rfc3339()));
        }

        if let Some(end_date) = filter.end_date {
            select = select.where_clause("date <= ?");
            params.push(Box::new(end_date.to_rfc3339()));
        }

        if let Some(entry_type) = &filter.entry_type {
            select = select.where_clause("entry_type = ?");
            params.push(Box::new(format!("{:?}", entry_type)));
        }

        if let Some(currency) = &filter.currency {
            select = select.where_clause("currency = ?");
            params.push(Box::new(currency.clone()));
        }

        // Handle tags filter if there are any tags
        if !filter.tags.is_empty() {
            let placeholders = vec!["?"; filter.tags.len()].join(", ");
            let tag_subquery = format!(
                "id IN (
                    SELECT entry_id FROM entry_tags
                    JOIN tags ON entry_tags.tag_id = tags.id
                    WHERE tags.name IN ({})
                    GROUP BY entry_id
                    HAVING COUNT(DISTINCT tags.name) = ?
                )",
                placeholders
            );

            select = select.where_clause(&tag_subquery);

            for tag in &filter.tags {
                params.push(Box::new(tag.clone()));
            }

            // Add the count of tags to ensure all tags are matched
            params.push(Box::new(filter.tags.len() as i64));
        }

        (select, params)
    }
}

impl Repository for SQLiteRepository {
    fn create<'a>(&self, entry: &LedgerEntry) -> BeansResult<()> {
        let mut conn = self.conn.lock().unwrap();
        let tx = conn
            .transaction()
            .map_err(|e| BeansError::database(format!("Failed to start transaction: {}", e)))?;

        // Insert the entry
        let insert_query = sql::Insert::new()
            .insert_into("entries (id, date, name, currency, amount, description, entry_type, created_at, updated_at)")
            .values("(?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .as_string();

        tx.execute(
            &insert_query,
            params![
                entry.id().to_string(),
                entry.date().to_rfc3339(),
                entry.name(),
                entry.currency_code(),
                entry.amount().to_string(),
                entry.description(),
                format!("{:?}", entry.entry_type()),
                entry.created_at().to_rfc3339(),
                entry.updated_at().to_rfc3339(),
            ],
        )
        .map_err(|e| BeansError::database(format!("Failed to insert entry: {}", e)))?;

        // Convert HashSet<Tag> to Vec<Tag> for save_tags
        let tags_vec: Vec<Tag> = entry.tags().iter().cloned().collect();

        // Save tags
        self.save_tags(&tx, &entry.id(), &tags_vec)?;

        // Commit the transaction
        tx.commit()
            .map_err(|e| BeansError::database(format!("Failed to commit transaction: {}", e)))?;

        Ok(())
    }

    fn get(&self, id: Uuid) -> BeansResult<LedgerEntry> {
        let mut conn = self.conn.lock().unwrap();
        let tx = conn
            .transaction()
            .map_err(|e| BeansError::database(format!("Failed to start transaction: {}", e)))?;

        let select_query = sql::Select::new()
            .select(
                "id, date, name, currency, amount, description, entry_type, created_at, updated_at",
            )
            .from("entries")
            .where_clause("id = ?")
            .as_string();

        let mut stmt = tx
            .prepare(&select_query)
            .map_err(|e| BeansError::database(format!("Failed to prepare query: {}", e)))?;

        let entry = stmt
            .query_row(params![id.to_string()], |row| self.row_to_entry(&tx, row))
            .map_err(|e| match e {
                rusqlite::Error::QueryReturnedNoRows => {
                    BeansError::not_found(format!("Entry with ID {} not found", id))
                }
                _ => BeansError::database(format!("Failed to query entry: {}", e)),
            })?;

        Ok(entry)
    }

    fn update<'a>(&self, entry: &LedgerEntry) -> BeansResult<()> {
        let mut conn = self.conn.lock().unwrap();
        let tx = conn
            .transaction()
            .map_err(|e| BeansError::database(format!("Failed to start transaction: {}", e)))?;

        // Check if the entry exists
        let check_query = sql::Select::new()
            .select("1")
            .from("entries")
            .where_clause("id = ?")
            .as_string();

        let exists: bool = tx
            .query_row(&check_query, params![entry.id().to_string()], |_| Ok(true))
            .unwrap_or(false);

        if !exists {
            return Err(BeansError::not_found(format!(
                "Entry with ID {} not found",
                entry.id()
            )));
        }

        // Update the entry
        let update_query = sql::Update::new()
            .update("entries")
            .set("date = ?, name = ?, currency = ?, amount = ?, description = ?, entry_type = ?, updated_at = ?")
            .where_clause("id = ?")
            .as_string();

        tx.execute(
            &update_query,
            params![
                entry.date().to_rfc3339(),
                entry.name(),
                entry.currency_code(),
                entry.amount().to_string(),
                entry.description(),
                format!("{:?}", entry.entry_type()),
                entry.updated_at().to_rfc3339(),
                entry.id().to_string(),
            ],
        )
        .map_err(|e| BeansError::database(format!("Failed to update entry: {}", e)))?;

        // Convert HashSet<Tag> to Vec<Tag> for save_tags
        let tags_vec: Vec<Tag> = entry.tags().iter().cloned().collect();

        // Save tags
        self.save_tags(&tx, &entry.id(), &tags_vec)?;

        // Commit the transaction
        tx.commit()
            .map_err(|e| BeansError::database(format!("Failed to commit transaction: {}", e)))?;

        Ok(())
    }

    fn delete(&self, id: Uuid) -> BeansResult<()> {
        let conn = self.conn.lock().unwrap();

        // Check if the entry exists
        let check_query = sql::Select::new()
            .select("1")
            .from("entries")
            .where_clause("id = ?")
            .as_string();

        let exists: bool = conn
            .query_row(&check_query, params![id.to_string()], |_| Ok(true))
            .unwrap_or(false);

        if !exists {
            return Err(BeansError::not_found(format!(
                "Entry with ID {} not found",
                id
            )));
        }

        // Delete the entry (cascade will delete entry_tags)
        let delete_query = sql::Delete::new()
            .delete_from("entries")
            .where_clause("id = ?")
            .as_string();

        conn.execute(&delete_query, params![id.to_string()])
            .map_err(|e| BeansError::database(format!("Failed to delete entry: {}", e)))?;

        Ok(())
    }

    fn list(&self, filter: &EntryFilter) -> BeansResult<Vec<LedgerEntry>> {
        let mut conn = self.conn.lock().unwrap();
        let tx = conn
            .transaction()
            .map_err(|e| BeansError::database(format!("Failed to start transaction: {}", e)))?;

        // Build the filtered query
        let (mut select, params) = self.build_filtered_query(filter);

        // Add ORDER BY
        select = select.order_by("date DESC");

        // Add limit and offset if specified
        // SQLite requires LIMIT when using OFFSET
        if let Some(limit) = filter.limit {
            select = select.limit(&limit.to_string());
        } else if filter.offset.is_some() {
            // If offset is specified but limit is not, use a large limit
            select = select.limit("18446744073709551615"); // SQLite max LIMIT value (2^64-1)
        }

        if let Some(offset) = filter.offset {
            select = select.offset(&offset.to_string());
        }

        let query = select.as_string();

        // Prepare and execute the query
        let mut stmt = tx
            .prepare(&query)
            .map_err(|e| BeansError::database(format!("Failed to prepare query: {}", e)))?;

        let param_refs: Vec<&dyn rusqlite::ToSql> = params
            .iter()
            .map(|p| p.as_ref() as &dyn rusqlite::ToSql)
            .collect();

        let rows = stmt
            .query(rusqlite::params_from_iter(param_refs.iter()))
            .map_err(|e| BeansError::database(format!("Failed to execute query: {}", e)))?;

        let mut entries = Vec::new();
        for row_result in rows.mapped(|row| self.row_to_entry(&tx, row)) {
            match row_result {
                Ok(entry) => entries.push(entry),
                Err(e) => return Err(BeansError::database(format!("Failed to read entry: {}", e))),
            }
        }

        Ok(entries)
    }

    fn count(&self, filter: &EntryFilter) -> BeansResult<usize> {
        let conn = self.conn.lock().unwrap();

        // Build the filtered query but change SELECT to COUNT(*)
        let (_ , params) = self.build_filtered_query(filter);


        // We need to replace the SELECT clause with COUNT(*)
        // Since sql_query_builder doesn't have a direct way to do this,
        // we'll build a new query using the filter conditions
        let mut count_select = sql::Select::new().select("COUNT(*)").from("entries");

        // Re-apply the same filters
        if let Some(_) = filter.start_date {
            count_select = count_select.where_clause("date >= ?");
        }

        if let Some(_) = filter.end_date {
            count_select = count_select.where_clause("date <= ?");
        }

        if let Some(_) = &filter.entry_type {
            count_select = count_select.where_clause("entry_type = ?");
        }

        if let Some(_) = &filter.currency {
            count_select = count_select.where_clause("currency = ?");
        }

        // Handle tags filter if there are any tags
        if !filter.tags.is_empty() {
            let placeholders = vec!["?"; filter.tags.len()].join(", ");
            let tag_subquery = format!(
                "id IN (
                    SELECT entry_id FROM entry_tags
                    JOIN tags ON entry_tags.tag_id = tags.id
                    WHERE tags.name IN ({})
                    GROUP BY entry_id
                    HAVING COUNT(DISTINCT tags.name) = ?
                )",
                placeholders
            );

            count_select = count_select.where_clause(&tag_subquery);
        }

        let query = count_select.as_string();

        // Prepare and execute the query
        let mut stmt = conn
            .prepare(&query)
            .map_err(|e| BeansError::database(format!("Failed to prepare query: {}", e)))?;

        let param_refs: Vec<&dyn rusqlite::ToSql> = params
            .iter()
            .map(|p| p.as_ref() as &dyn rusqlite::ToSql)
            .collect();

        let count: i64 = stmt
            .query_row(rusqlite::params_from_iter(param_refs.iter()), |row| {
                row.get(0)
            })
            .map_err(|e| BeansError::database(format!("Failed to count entries: {}", e)))?;

        Ok(count as usize)
    }
}
