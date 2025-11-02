use leptos::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct EntryData {
    id: Option<String>,
    date: String,
    name: String,
    currency: String,
    amount: String,
    description: Option<String>,
    tags: Vec<String>,
    entry_type: String,
}

#[component]
pub fn EditEntry() -> impl IntoView {
    let (filter_date, set_filter_date) = create_signal(String::new());
    let (filter_tags, set_filter_tags) = create_signal(String::new());
    let (entries, set_entries) = create_signal::<Vec<EntryData>>(Vec::new());
    let (selected_entry, set_selected_entry) = create_signal::<Option<EntryData>>(None);
    let (loading, set_loading) = create_signal(false);
    let (submitting, set_submitting) = create_signal(false);
    
    // Form fields for editing
    let (edit_date, set_edit_date) = create_signal(String::new());
    let (edit_name, set_edit_name) = create_signal(String::new());
    let (edit_currency, set_edit_currency) = create_signal(String::new());
    let (edit_amount, set_edit_amount) = create_signal(String::new());
    let (edit_description, set_edit_description) = create_signal(String::new());
    let (edit_tags, set_edit_tags) = create_signal(String::new());
    let (edit_entry_type, set_edit_entry_type) = create_signal(String::new());
    
    let load_entries = move |_| {
        set_loading.set(true);
        
        spawn_local(async move {
            let tag_list: Vec<String> = if filter_tags.get().is_empty() {
                Vec::new()
            } else {
                filter_tags.get().split(',').map(|s| s.trim().to_string()).collect()
            };
            
            let filter = serde_json::json!({
                "start_date": if filter_date.get().is_empty() { None::<String> } else { Some(filter_date.get()) },
                "end_date": if filter_date.get().is_empty() { None::<String> } else { Some(filter_date.get()) },
                "tags": if tag_list.is_empty() { None::<Vec<String>> } else { Some(tag_list) },
                "currency": None::<String>,
                "entry_type": None::<String>,
            });
            
            let args = serde_wasm_bindgen::to_value(&serde_json::json!({
                "filter": filter
            })).unwrap();
            
            let result = invoke("get_entries_filtered", args).await;
            
            match serde_wasm_bindgen::from_value::<Result<Vec<EntryData>, String>>(result) {
                Ok(Ok(data)) => {
                    set_entries.set(data);
                },
                Ok(Err(e)) => {
                    web_sys::window().unwrap().alert_with_message(&format!("Error: {}", e)).ok();
                },
                Err(e) => {
                    web_sys::window().unwrap().alert_with_message(&format!("Error: {:?}", e)).ok();
                }
            }
            
            set_loading.set(false);
        });
    };
    
    let select_entry = move |entry: EntryData| {
        set_edit_date.set(entry.date.clone());
        set_edit_name.set(entry.name.clone());
        set_edit_currency.set(entry.currency.clone());
        set_edit_amount.set(entry.amount.clone());
        set_edit_description.set(entry.description.clone().unwrap_or_default());
        set_edit_tags.set(entry.tags.join(", "));
        set_edit_entry_type.set(entry.entry_type.clone());
        set_selected_entry.set(Some(entry));
    };
    
    let update_entry = move |_| {
        if edit_name.get().is_empty() || edit_amount.get().is_empty() {
            web_sys::window().unwrap().alert_with_message("Please fill in all required fields").ok();
            return;
        }
        
        let Some(current_entry) = selected_entry.get() else {
            web_sys::window().unwrap().alert_with_message("No entry selected").ok();
            return;
        };
        
        set_submitting.set(true);
        
        spawn_local(async move {
            let tag_list: Vec<String> = if edit_tags.get().is_empty() {
                Vec::new()
            } else {
                edit_tags.get().split(',').map(|s| s.trim().to_string()).collect()
            };
            
            let entry = serde_json::json!({
                "id": current_entry.id,
                "date": edit_date.get(),
                "name": edit_name.get(),
                "currency": edit_currency.get(),
                "amount": edit_amount.get(),
                "description": if edit_description.get().is_empty() { None::<String> } else { Some(edit_description.get()) },
                "tags": tag_list,
                "entry_type": edit_entry_type.get(),
            });
            
            let args = serde_wasm_bindgen::to_value(&serde_json::json!({
                "entry": entry
            })).unwrap();
            
            let result = invoke("update_entry", args).await;
            
            match serde_wasm_bindgen::from_value::<Result<String, String>>(result) {
                Ok(Ok(msg)) => {
                    web_sys::window().unwrap().alert_with_message(&msg).ok();
                    set_selected_entry.set(None);
                    // Reload entries
                    load_entries(());
                },
                Ok(Err(e)) => {
                    web_sys::window().unwrap().alert_with_message(&format!("Error: {}", e)).ok();
                },
                Err(e) => {
                    web_sys::window().unwrap().alert_with_message(&format!("Error: {:?}", e)).ok();
                }
            }
            
            set_submitting.set(false);
        });
    };
    
    let delete_entry = move |_| {
        let Some(current_entry) = selected_entry.get() else {
            return;
        };
        
        let confirmed = web_sys::window()
            .unwrap()
            .confirm_with_message("Are you sure you want to delete this entry?")
            .unwrap();
        
        if !confirmed {
            return;
        }
        
        let entry_id = current_entry.id.clone().unwrap();
        
        spawn_local(async move {
            let args = serde_wasm_bindgen::to_value(&serde_json::json!({
                "id": entry_id
            })).unwrap();
            
            let result = invoke("delete_entry", args).await;
            
            match serde_wasm_bindgen::from_value::<Result<String, String>>(result) {
                Ok(Ok(msg)) => {
                    web_sys::window().unwrap().alert_with_message(&msg).ok();
                    set_selected_entry.set(None);
                    // Reload entries
                    load_entries(());
                },
                Ok(Err(e)) => {
                    web_sys::window().unwrap().alert_with_message(&format!("Error: {}", e)).ok();
                },
                Err(e) => {
                    web_sys::window().unwrap().alert_with_message(&format!("Error: {:?}", e)).ok();
                }
            }
        });
    };
    
    view! {
        <div class="view-container">
            <h1>"Edit Entry"</h1>
            
            <div class="filter-section">
                <h2>"Find Entry"</h2>
                <div class="filter-inputs">
                    <div class="input-group">
                        <label>"Filter by Date:"</label>
                        <input 
                            type="date" 
                            prop:value=filter_date
                            on:input=move |ev| set_filter_date.set(event_target_value(&ev))
                        />
                    </div>
                    <div class="input-group">
                        <label>"Filter by Tags (comma-separated):"</label>
                        <input 
                            type="text" 
                            prop:value=filter_tags
                            on:input=move |ev| set_filter_tags.set(event_target_value(&ev))
                            placeholder="e.g., food, transport"
                        />
                    </div>
                    <button 
                        class="btn-primary" 
                        on:click=load_entries
                        disabled=move || loading.get()
                    >
                        {move || if loading.get() { "Loading..." } else { "Search Entries" }}
                    </button>
                </div>
            </div>
            
            <div class="entries-list">
                <h2>"Select Entry to Edit"</h2>
                {move || {
                    if entries.get().is_empty() {
                        view! {
                            <p class="no-entries">"No entries found. Try adjusting your filters or add some entries first."</p>
                        }.into_view()
                    } else {
                        view! {
                            <ul class="entry-items">
                                <For
                                    each=move || entries.get()
                                    key=|entry| entry.id.clone().unwrap_or_default()
                                    let:entry
                                >
                                    <li 
                                        class="entry-item"
                                        class:selected=move || {
                                            selected_entry.get().as_ref().and_then(|e| e.id.as_ref()) 
                                                == entry.id.as_ref()
                                        }
                                        on:click={
                                            let entry = entry.clone();
                                            move |_| select_entry(entry.clone())
                                        }
                                    >
                                        <div class="entry-header">
                                            <span class="entry-name">{&entry.name}</span>
                                            <span class={format!("entry-type {}", entry.entry_type)}>
                                                {&entry.entry_type}
                                            </span>
                                        </div>
                                        <div class="entry-details">
                                            <span>{&entry.date}</span>
                                            <span class="entry-amount">{&entry.amount} " " {&entry.currency}</span>
                                        </div>
                                        {(!entry.tags.is_empty()).then(|| view! {
                                            <div class="entry-tags">
                                                {entry.tags.iter().map(|tag| view! {
                                                    <span class="tag">{tag}</span>
                                                }).collect::<Vec<_>>()}
                                            </div>
                                        })}
                                    </li>
                                </For>
                            </ul>
                        }.into_view()
                    }
                }}
            </div>
            
            {move || selected_entry.get().map(|_| view! {
                <div class="edit-form-section">
                    <h2>"Edit Selected Entry"</h2>
                    
                    <form class="entry-form" on:submit=|e| e.prevent_default()>
                        <div class="form-row">
                            <div class="input-group">
                                <label>"Date:" <span class="required">"*"</span></label>
                                <input 
                                    type="date" 
                                    prop:value=edit_date
                                    on:input=move |ev| set_edit_date.set(event_target_value(&ev))
                                    required
                                />
                            </div>
                            
                            <div class="input-group">
                                <label>"Entry Type:" <span class="required">"*"</span></label>
                                <select 
                                    prop:value=edit_entry_type
                                    on:change=move |ev| set_edit_entry_type.set(event_target_value(&ev))
                                >
                                    <option value="income">"Income"</option>
                                    <option value="expense">"Expense"</option>
                                </select>
                            </div>
                        </div>
                        
                        <div class="form-row">
                            <div class="input-group flex-2">
                                <label>"Name:" <span class="required">"*"</span></label>
                                <input 
                                    type="text" 
                                    prop:value=edit_name
                                    on:input=move |ev| set_edit_name.set(event_target_value(&ev))
                                    required
                                />
                            </div>
                        </div>
                        
                        <div class="form-row">
                            <div class="input-group">
                                <label>"Currency:" <span class="required">"*"</span></label>
                                <select 
                                    prop:value=edit_currency
                                    on:change=move |ev| set_edit_currency.set(event_target_value(&ev))
                                >
                                    <option value="USD">"USD"</option>
                                    <option value="EUR">"EUR"</option>
                                    <option value="GBP">"GBP"</option>
                                    <option value="JPY">"JPY"</option>
                                    <option value="CAD">"CAD"</option>
                                    <option value="AUD">"AUD"</option>
                                </select>
                            </div>
                            
                            <div class="input-group">
                                <label>"Amount:" <span class="required">"*"</span></label>
                                <input 
                                    type="number" 
                                    step="0.01"
                                    prop:value=edit_amount
                                    on:input=move |ev| set_edit_amount.set(event_target_value(&ev))
                                    required
                                />
                            </div>
                        </div>
                        
                        <div class="input-group">
                            <label>"Description:"</label>
                            <textarea 
                                prop:value=edit_description
                                on:input=move |ev| set_edit_description.set(event_target_value(&ev))
                                rows="3"
                            />
                        </div>
                        
                        <div class="input-group">
                            <label>"Tags (comma-separated):"</label>
                            <input 
                                type="text" 
                                prop:value=edit_tags
                                on:input=move |ev| set_edit_tags.set(event_target_value(&ev))
                            />
                        </div>
                        
                        <div class="form-actions">
                            <button 
                                type="submit"
                                class="btn-primary" 
                                on:click=update_entry
                                disabled=move || submitting.get()
                            >
                                {move || if submitting.get() { "Updating..." } else { "Update Entry" }}
                            </button>
                            <button 
                                type="button"
                                class="btn-danger" 
                                on:click=delete_entry
                            >
                                "Delete Entry"
                            </button>
                        </div>
                    </form>
                </div>
            })}
        </div>
    }
}

