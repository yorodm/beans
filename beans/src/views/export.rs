use leptos::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "dialog"])]
    async fn save(options: JsValue) -> JsValue;
    
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn Export() -> impl IntoView {
    let (format, set_format) = create_signal("json".to_string());
    let (use_filters, set_use_filters) = create_signal(false);
    let (start_date, set_start_date) = create_signal(String::new());
    let (end_date, set_end_date) = create_signal(String::new());
    let (tags, set_tags) = create_signal(String::new());
    let (currency, set_currency) = create_signal(String::new());
    let (entry_type, set_entry_type) = create_signal("all".to_string());
    let (exporting, set_exporting) = create_signal(false);
    
    let export_ledger = move |_| {
        set_exporting.set(true);
        
        spawn_local(async move {
            let format_val = format.get();
            let extension = match format_val.as_str() {
                "json" => "json",
                "csv" => "csv",
                _ => "txt",
            };
            
            // Show save dialog
            let options = serde_wasm_bindgen::to_value(&serde_json::json!({
                "filters": [{
                    "name": format!("{} File", format_val.to_uppercase()),
                    "extensions": [extension]
                }],
                "defaultPath": format!("ledger_export.{}", extension)
            })).unwrap();
            
            let result = save(options).await;
            if let Ok(path) = serde_wasm_bindgen::from_value::<Option<String>>(result) {
                if let Some(path) = path {
                    // Prepare filter if enabled
                    let filter = if use_filters.get() {
                        let tag_list: Vec<String> = if tags.get().is_empty() {
                            Vec::new()
                        } else {
                            tags.get().split(',').map(|s| s.trim().to_string()).collect()
                        };
                        
                        Some(serde_json::json!({
                            "start_date": if start_date.get().is_empty() { None::<String> } else { Some(start_date.get()) },
                            "end_date": if end_date.get().is_empty() { None::<String> } else { Some(end_date.get()) },
                            "tags": if tag_list.is_empty() { None::<Vec<String>> } else { Some(tag_list) },
                            "currency": if currency.get().is_empty() { None::<String> } else { Some(currency.get()) },
                            "entry_type": if entry_type.get() == "all" { None::<String> } else { Some(entry_type.get()) },
                        }))
                    } else {
                        None
                    };
                    
                    let args = serde_wasm_bindgen::to_value(&serde_json::json!({
                        "format": format_val,
                        "path": path,
                        "filter": filter,
                    })).unwrap();
                    
                    let result = invoke("export_ledger", args).await;
                    
                    match serde_wasm_bindgen::from_value::<Result<String, String>>(result) {
                        Ok(Ok(msg)) => {
                            web_sys::window().unwrap().alert_with_message(&msg).ok();
                        },
                        Ok(Err(e)) => {
                            web_sys::window().unwrap().alert_with_message(&format!("Error: {}", e)).ok();
                        },
                        Err(e) => {
                            web_sys::window().unwrap().alert_with_message(&format!("Error: {:?}", e)).ok();
                        }
                    }
                }
            }
            
            set_exporting.set(false);
        });
    };
    
    view! {
        <div class="view-container">
            <h1>"Export Ledger"</h1>
            
            <div class="export-section">
                <h2>"Export Settings"</h2>
                
                <div class="input-group">
                    <label>"Export Format:" <span class="required">"*"</span></label>
                    <select 
                        prop:value=format
                        on:change=move |ev| set_format.set(event_target_value(&ev))
                    >
                        <option value="json">"JSON"</option>
                        <option value="csv">"CSV"</option>
                    </select>
                    <p class="help-text">
                        {move || match format.get().as_str() {
                            "json" => "JSON format preserves all data including nested structures",
                            "csv" => "CSV format is suitable for spreadsheet applications",
                            _ => "",
                        }}
                    </p>
                </div>
                
                <div class="input-group">
                    <label>
                        <input 
                            type="checkbox" 
                            prop:checked=use_filters
                            on:change=move |ev| set_use_filters.set(event_target_checked(&ev))
                        />
                        " Apply Filters"
                    </label>
                    <p class="help-text">"Check this to export only filtered entries"</p>
                </div>
                
                {move || use_filters.get().then(|| view! {
                    <div class="filter-section">
                        <h3>"Filter Options"</h3>
                        
                        <div class="form-row">
                            <div class="input-group">
                                <label>"Start Date:"</label>
                                <input 
                                    type="date" 
                                    prop:value=start_date
                                    on:input=move |ev| set_start_date.set(event_target_value(&ev))
                                />
                            </div>
                            
                            <div class="input-group">
                                <label>"End Date:"</label>
                                <input 
                                    type="date" 
                                    prop:value=end_date
                                    on:input=move |ev| set_end_date.set(event_target_value(&ev))
                                />
                            </div>
                        </div>
                        
                        <div class="input-group">
                            <label>"Tags (comma-separated):"</label>
                            <input 
                                type="text" 
                                prop:value=tags
                                on:input=move |ev| set_tags.set(event_target_value(&ev))
                                placeholder="e.g., food, transport"
                            />
                        </div>
                        
                        <div class="form-row">
                            <div class="input-group">
                                <label>"Currency:"</label>
                                <select 
                                    prop:value=currency
                                    on:change=move |ev| set_currency.set(event_target_value(&ev))
                                >
                                    <option value="">"All Currencies"</option>
                                    <option value="USD">"USD"</option>
                                    <option value="EUR">"EUR"</option>
                                    <option value="GBP">"GBP"</option>
                                    <option value="JPY">"JPY"</option>
                                    <option value="CAD">"CAD"</option>
                                    <option value="AUD">"AUD"</option>
                                </select>
                            </div>
                            
                            <div class="input-group">
                                <label>"Entry Type:"</label>
                                <select 
                                    prop:value=entry_type
                                    on:change=move |ev| set_entry_type.set(event_target_value(&ev))
                                >
                                    <option value="all">"All Types"</option>
                                    <option value="income">"Income Only"</option>
                                    <option value="expense">"Expense Only"</option>
                                </select>
                            </div>
                        </div>
                    </div>
                })}
                
                <div class="form-actions">
                    <button 
                        class="btn-primary" 
                        on:click=export_ledger
                        disabled=move || exporting.get()
                    >
                        {move || if exporting.get() { "Exporting..." } else { "Export Ledger" }}
                    </button>
                </div>
            </div>
            
            <div class="info-section">
                <h2>"Export Information"</h2>
                <ul>
                    <li><strong>"JSON Export:"</strong>" Exports all entry data in JSON format, preserving all fields and data types."</li>
                    <li><strong>"CSV Export:"</strong>" Exports entries in CSV format, suitable for importing into spreadsheet applications like Excel or Google Sheets."</li>
                    <li><strong>"Filters:"</strong>" When enabled, only entries matching the specified criteria will be exported."</li>
                    <li><strong>"File Location:"</strong>" You will be prompted to choose where to save the exported file."</li>
                </ul>
            </div>
        </div>
    }
}

