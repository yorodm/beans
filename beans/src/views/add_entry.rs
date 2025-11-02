use leptos::*;
use wasm_bindgen::prelude::*;
use chrono::Local;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn AddEntry() -> impl IntoView {
    let (date, set_date) = create_signal(Local::now().format("%Y-%m-%d").to_string());
    let (name, set_name) = create_signal(String::new());
    let (currency, set_currency) = create_signal("USD".to_string());
    let (amount, set_amount) = create_signal(String::new());
    let (description, set_description) = create_signal(String::new());
    let (tags, set_tags) = create_signal(String::new());
    let (entry_type, set_entry_type) = create_signal("income".to_string());
    let (submitting, set_submitting) = create_signal(false);
    
    let submit_entry = move |_| {
        if name.get().is_empty() || amount.get().is_empty() {
            web_sys::window().unwrap().alert_with_message("Please fill in all required fields").ok();
            return;
        }
        
        set_submitting.set(true);
        
        spawn_local(async move {
            let tag_list: Vec<String> = if tags.get().is_empty() {
                Vec::new()
            } else {
                tags.get().split(',').map(|s| s.trim().to_string()).collect()
            };
            
            let entry = serde_json::json!({
                "id": None::<String>,
                "date": date.get(),
                "name": name.get(),
                "currency": currency.get(),
                "amount": amount.get(),
                "description": if description.get().is_empty() { None::<String> } else { Some(description.get()) },
                "tags": tag_list,
                "entry_type": entry_type.get(),
            });
            
            let args = serde_wasm_bindgen::to_value(&serde_json::json!({
                "entry": entry
            })).unwrap();
            
            let result = invoke("add_entry", args).await;
            
            match serde_wasm_bindgen::from_value::<Result<String, String>>(result) {
                Ok(Ok(msg)) => {
                    web_sys::window().unwrap().alert_with_message(&msg).ok();
                    // Clear form
                    set_name.set(String::new());
                    set_amount.set(String::new());
                    set_description.set(String::new());
                    set_tags.set(String::new());
                    set_date.set(Local::now().format("%Y-%m-%d").to_string());
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
    
    view! {
        <div class="view-container">
            <h1>"Add New Entry"</h1>
            
            <form class="entry-form" on:submit=|e| e.prevent_default()>
                <div class="form-row">
                    <div class="input-group">
                        <label>"Date:" <span class="required">"*"</span></label>
                        <input 
                            type="date" 
                            prop:value=date
                            on:input=move |ev| set_date.set(event_target_value(&ev))
                            required
                        />
                    </div>
                    
                    <div class="input-group">
                        <label>"Entry Type:" <span class="required">"*"</span></label>
                        <select 
                            prop:value=entry_type
                            on:change=move |ev| set_entry_type.set(event_target_value(&ev))
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
                            prop:value=name
                            on:input=move |ev| set_name.set(event_target_value(&ev))
                            placeholder="e.g., Salary, Groceries"
                            required
                        />
                    </div>
                </div>
                
                <div class="form-row">
                    <div class="input-group">
                        <label>"Currency:" <span class="required">"*"</span></label>
                        <select 
                            prop:value=currency
                            on:change=move |ev| set_currency.set(event_target_value(&ev))
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
                            prop:value=amount
                            on:input=move |ev| set_amount.set(event_target_value(&ev))
                            placeholder="0.00"
                            required
                        />
                    </div>
                </div>
                
                <div class="input-group">
                    <label>"Description:"</label>
                    <textarea 
                        prop:value=description
                        on:input=move |ev| set_description.set(event_target_value(&ev))
                        placeholder="Optional description"
                        rows="3"
                    />
                </div>
                
                <div class="input-group">
                    <label>"Tags (comma-separated):"</label>
                    <input 
                        type="text" 
                        prop:value=tags
                        on:input=move |ev| set_tags.set(event_target_value(&ev))
                        placeholder="e.g., food, transport, work"
                    />
                </div>
                
                <div class="form-actions">
                    <button 
                        type="submit"
                        class="btn-primary" 
                        on:click=submit_entry
                        disabled=move || submitting.get()
                    >
                        {move || if submitting.get() { "Saving..." } else { "Add Entry" }}
                    </button>
                </div>
            </form>
        </div>
    }
}

