use beans_lib::{
    ledger::LedgerManager,
    models::{entry::EntryBuilder, entry::EntryType, currency::Currency, tag::Tag},
    reporting::ReportGenerator,
};
use chrono::{NaiveDate, Local};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;

// Global state for the current ledger
pub struct AppState {
    pub ledger: Mutex<Option<LedgerManager>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntryData {
    pub id: Option<String>,
    pub date: String,
    pub name: String,
    pub currency: String,
    pub amount: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub entry_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterParams {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub tags: Option<Vec<String>>,
    pub currency: Option<String>,
    pub entry_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportData {
    pub total_income: String,
    pub total_expenses: String,
    pub net: String,
    pub currency: String,
}

#[tauri::command]
pub async fn open_ledger(path: String, state: State<'_, AppState>) -> Result<String, String> {
    let ledger = LedgerManager::open(&path)
        .map_err(|e| format!("Failed to open ledger: {}", e))?;
    
    *state.ledger.lock().unwrap() = Some(ledger);
    Ok("Ledger opened successfully".to_string())
}

#[tauri::command]
pub async fn create_ledger(path: String, state: State<'_, AppState>) -> Result<String, String> {
    let ledger = LedgerManager::create(&path)
        .map_err(|e| format!("Failed to create ledger: {}", e))?;
    
    *state.ledger.lock().unwrap() = Some(ledger);
    Ok("Ledger created successfully".to_string())
}

#[tauri::command]
pub async fn add_entry(entry: EntryData, state: State<'_, AppState>) -> Result<String, String> {
    let ledger_guard = state.ledger.lock().unwrap();
    let ledger = ledger_guard.as_ref()
        .ok_or_else(|| "No ledger opened".to_string())?;
    
    let date = NaiveDate::parse_from_str(&entry.date, "%Y-%m-%d")
        .map_err(|e| format!("Invalid date format: {}", e))?;
    
    let currency = Currency::from_code(&entry.currency)
        .map_err(|e| format!("Invalid currency: {}", e))?;
    
    let amount: f64 = entry.amount.parse()
        .map_err(|e| format!("Invalid amount: {}", e))?;
    
    let entry_type = match entry.entry_type.as_str() {
        "income" => EntryType::Income,
        "expense" => EntryType::Expense,
        _ => return Err("Invalid entry type".to_string()),
    };
    
    let tags: Result<Vec<Tag>, _> = entry.tags.iter()
        .map(|t| Tag::new(t))
        .collect();
    let tags = tags.map_err(|e| format!("Invalid tag: {}", e))?;
    
    let mut builder = EntryBuilder::new()
        .date(date)
        .name(entry.name)
        .currency(currency)
        .amount(amount)
        .entry_type(entry_type)
        .tags(tags);
    
    if let Some(desc) = entry.description {
        builder = builder.description(desc);
    }
    
    let ledger_entry = builder.build()
        .map_err(|e| format!("Failed to build entry: {}", e))?;
    
    ledger.add_entry(ledger_entry)
        .map_err(|e| format!("Failed to add entry: {}", e))?;
    
    Ok("Entry added successfully".to_string())
}

#[tauri::command]
pub async fn update_entry(entry: EntryData, state: State<'_, AppState>) -> Result<String, String> {
    let ledger_guard = state.ledger.lock().unwrap();
    let ledger = ledger_guard.as_ref()
        .ok_or_else(|| "No ledger opened".to_string())?;
    
    let id = entry.id.ok_or_else(|| "Entry ID is required for update".to_string())?;
    
    let date = NaiveDate::parse_from_str(&entry.date, "%Y-%m-%d")
        .map_err(|e| format!("Invalid date format: {}", e))?;
    
    let currency = Currency::from_code(&entry.currency)
        .map_err(|e| format!("Invalid currency: {}", e))?;
    
    let amount: f64 = entry.amount.parse()
        .map_err(|e| format!("Invalid amount: {}", e))?;
    
    let entry_type = match entry.entry_type.as_str() {
        "income" => EntryType::Income,
        "expense" => EntryType::Expense,
        _ => return Err("Invalid entry type".to_string()),
    };
    
    let tags: Result<Vec<Tag>, _> = entry.tags.iter()
        .map(|t| Tag::new(t))
        .collect();
    let tags = tags.map_err(|e| format!("Invalid tag: {}", e))?;
    
    let mut builder = EntryBuilder::new()
        .date(date)
        .name(entry.name)
        .currency(currency)
        .amount(amount)
        .entry_type(entry_type)
        .tags(tags);
    
    if let Some(desc) = entry.description {
        builder = builder.description(desc);
    }
    
    let mut ledger_entry = builder.build()
        .map_err(|e| format!("Failed to build entry: {}", e))?;
    
    // Set the ID from the existing entry
    ledger_entry.id = uuid::Uuid::parse_str(&id)
        .map_err(|e| format!("Invalid UUID: {}", e))?;
    
    ledger.update_entry(ledger_entry)
        .map_err(|e| format!("Failed to update entry: {}", e))?;
    
    Ok("Entry updated successfully".to_string())
}

#[tauri::command]
pub async fn delete_entry(id: String, state: State<'_, AppState>) -> Result<String, String> {
    let ledger_guard = state.ledger.lock().unwrap();
    let ledger = ledger_guard.as_ref()
        .ok_or_else(|| "No ledger opened".to_string())?;
    
    let uuid = uuid::Uuid::parse_str(&id)
        .map_err(|e| format!("Invalid UUID: {}", e))?;
    
    ledger.delete_entry(uuid)
        .map_err(|e| format!("Failed to delete entry: {}", e))?;
    
    Ok("Entry deleted successfully".to_string())
}

#[tauri::command]
pub async fn get_entries(state: State<'_, AppState>) -> Result<Vec<EntryData>, String> {
    let ledger_guard = state.ledger.lock().unwrap();
    let ledger = ledger_guard.as_ref()
        .ok_or_else(|| "No ledger opened".to_string())?;
    
    let entries = ledger.get_all_entries()
        .map_err(|e| format!("Failed to get entries: {}", e))?;
    
    let entry_data: Vec<EntryData> = entries.into_iter().map(|e| {
        EntryData {
            id: Some(e.id.to_string()),
            date: e.date.format("%Y-%m-%d").to_string(),
            name: e.name,
            currency: e.currency.code().to_string(),
            amount: e.amount.to_string(),
            description: Some(e.description),
            tags: e.tags.into_iter().map(|t| t.as_str().to_string()).collect(),
            entry_type: match e.entry_type {
                EntryType::Income => "income".to_string(),
                EntryType::Expense => "expense".to_string(),
            },
        }
    }).collect();
    
    Ok(entry_data)
}

#[tauri::command]
pub async fn get_entries_filtered(
    filter: FilterParams,
    state: State<'_, AppState>
) -> Result<Vec<EntryData>, String> {
    let ledger_guard = state.ledger.lock().unwrap();
    let ledger = ledger_guard.as_ref()
        .ok_or_else(|| "No ledger opened".to_string())?;
    
    let start_date = if let Some(date_str) = filter.start_date {
        Some(NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
            .map_err(|e| format!("Invalid start date: {}", e))?)
    } else {
        None
    };
    
    let end_date = if let Some(date_str) = filter.end_date {
        Some(NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
            .map_err(|e| format!("Invalid end date: {}", e))?)
    } else {
        None
    };
    
    let tags = if let Some(tag_strs) = filter.tags {
        let parsed_tags: Result<Vec<Tag>, _> = tag_strs.iter()
            .map(|t| Tag::new(t))
            .collect();
        Some(parsed_tags.map_err(|e| format!("Invalid tag: {}", e))?)
    } else {
        None
    };
    
    let currency = if let Some(curr_str) = filter.currency {
        Some(Currency::from_code(&curr_str)
            .map_err(|e| format!("Invalid currency: {}", e))?)
    } else {
        None
    };
    
    let entry_type = if let Some(type_str) = filter.entry_type {
        match type_str.as_str() {
            "income" => Some(EntryType::Income),
            "expense" => Some(EntryType::Expense),
            _ => return Err("Invalid entry type".to_string()),
        }
    } else {
        None
    };
    
    let entries = ledger.filter_entries(start_date, end_date, tags.as_deref(), currency, entry_type)
        .map_err(|e| format!("Failed to filter entries: {}", e))?;
    
    let entry_data: Vec<EntryData> = entries.into_iter().map(|e| {
        EntryData {
            id: Some(e.id.to_string()),
            date: e.date.format("%Y-%m-%d").to_string(),
            name: e.name,
            currency: e.currency.code().to_string(),
            amount: e.amount.to_string(),
            description: Some(e.description),
            tags: e.tags.into_iter().map(|t| t.as_str().to_string()).collect(),
            entry_type: match e.entry_type {
                EntryType::Income => "income".to_string(),
                EntryType::Expense => "expense".to_string(),
            },
        }
    }).collect();
    
    Ok(entry_data)
}

#[tauri::command]
pub async fn get_report_data(
    filter: FilterParams,
    state: State<'_, AppState>
) -> Result<ReportData, String> {
    let ledger_guard = state.ledger.lock().unwrap();
    let ledger = ledger_guard.as_ref()
        .ok_or_else(|| "No ledger opened".to_string())?;
    
    let start_date = if let Some(date_str) = filter.start_date {
        NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
            .map_err(|e| format!("Invalid start date: {}", e))?
    } else {
        NaiveDate::from_ymd_opt(Local::now().year(), 1, 1).unwrap()
    };
    
    let end_date = if let Some(date_str) = filter.end_date {
        NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
            .map_err(|e| format!("Invalid end date: {}", e))?
    } else {
        Local::now().naive_local().date()
    };
    
    let tags = if let Some(tag_strs) = filter.tags {
        let parsed_tags: Result<Vec<Tag>, _> = tag_strs.iter()
            .map(|t| Tag::new(t))
            .collect();
        Some(parsed_tags.map_err(|e| format!("Invalid tag: {}", e))?)
    } else {
        None
    };
    
    let target_currency = if let Some(curr_str) = filter.currency {
        Currency::from_code(&curr_str)
            .map_err(|e| format!("Invalid currency: {}", e))?
    } else {
        Currency::from_code("USD")
            .map_err(|e| format!("Invalid default currency: {}", e))?
    };
    
    let report_gen = ReportGenerator::new(ledger);
    let report = report_gen.generate_income_vs_expenses(start_date, end_date, tags.as_deref(), target_currency)
        .await
        .map_err(|e| format!("Failed to generate report: {}", e))?;
    
    Ok(ReportData {
        total_income: report.total_income.to_string(),
        total_expenses: report.total_expenses.to_string(),
        net: report.net.to_string(),
        currency: report.currency.code().to_string(),
    })
}

#[tauri::command]
pub async fn export_ledger(
    format: String,
    path: String,
    filter: Option<FilterParams>,
    state: State<'_, AppState>
) -> Result<String, String> {
    let ledger_guard = state.ledger.lock().unwrap();
    let ledger = ledger_guard.as_ref()
        .ok_or_else(|| "No ledger opened".to_string())?;
    
    // Get entries (filtered or all)
    let entries = if let Some(f) = filter {
        let start_date = if let Some(date_str) = f.start_date {
            Some(NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
                .map_err(|e| format!("Invalid start date: {}", e))?)
        } else {
            None
        };
        
        let end_date = if let Some(date_str) = f.end_date {
            Some(NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
                .map_err(|e| format!("Invalid end date: {}", e))?)
        } else {
            None
        };
        
        let tags = if let Some(tag_strs) = f.tags {
            let parsed_tags: Result<Vec<Tag>, _> = tag_strs.iter()
                .map(|t| Tag::new(t))
                .collect();
            Some(parsed_tags.map_err(|e| format!("Invalid tag: {}", e))?)
        } else {
            None
        };
        
        let currency = if let Some(curr_str) = f.currency {
            Some(Currency::from_code(&curr_str)
                .map_err(|e| format!("Invalid currency: {}", e))?)
        } else {
            None
        };
        
        let entry_type = if let Some(type_str) = f.entry_type {
            match type_str.as_str() {
                "income" => Some(EntryType::Income),
                "expense" => Some(EntryType::Expense),
                _ => return Err("Invalid entry type".to_string()),
            }
        } else {
            None
        };
        
        ledger.filter_entries(start_date, end_date, tags.as_deref(), currency, entry_type)
            .map_err(|e| format!("Failed to filter entries: {}", e))?
    } else {
        ledger.get_all_entries()
            .map_err(|e| format!("Failed to get entries: {}", e))?
    };
    
    // Export based on format
    match format.as_str() {
        "json" => {
            let json = serde_json::to_string_pretty(&entries)
                .map_err(|e| format!("Failed to serialize to JSON: {}", e))?;
            std::fs::write(&path, json)
                .map_err(|e| format!("Failed to write file: {}", e))?;
        },
        "csv" => {
            // Simple CSV export
            let mut csv = String::from("ID,Date,Name,Currency,Amount,Description,Tags,Type\n");
            for entry in entries {
                csv.push_str(&format!(
                    "{},{},{},{},{},{},{},{}\n",
                    entry.id,
                    entry.date.format("%Y-%m-%d"),
                    entry.name,
                    entry.currency.code(),
                    entry.amount,
                    entry.description,
                    entry.tags.iter().map(|t| t.as_str()).collect::<Vec<_>>().join(";"),
                    match entry.entry_type {
                        EntryType::Income => "income",
                        EntryType::Expense => "expense",
                    }
                ));
            }
            std::fs::write(&path, csv)
                .map_err(|e| format!("Failed to write file: {}", e))?;
        },
        _ => return Err(format!("Unsupported export format: {}", format)),
    }
    
    Ok(format!("Ledger exported successfully to {}", path))
}

