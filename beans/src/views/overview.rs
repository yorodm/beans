use leptos::*;
use leptos_chart::*;
use wasm_bindgen::prelude::*;
use chrono::Local;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Debug, Clone, serde::Deserialize)]
struct ReportData {
    total_income: String,
    total_expenses: String,
    net: String,
    currency: String,
}

#[component]
pub fn Overview() -> impl IntoView {
    let (start_date, set_start_date) = create_signal(String::new());
    let (end_date, set_end_date) = create_signal(String::new());
    let (tags, set_tags) = create_signal(String::new());
    let (report_data, set_report_data) = create_signal::<Option<ReportData>>(None);
    let (loading, set_loading) = create_signal(false);
    
    let current_date = Local::now().format("%Y-%m-%d").to_string();
    
    let load_report = move |_| {
        set_loading.set(true);
        
        spawn_local(async move {
            let tag_list: Vec<String> = if tags.get().is_empty() {
                Vec::new()
            } else {
                tags.get().split(',').map(|s| s.trim().to_string()).collect()
            };
            
            let filter = serde_json::json!({
                "start_date": if start_date.get().is_empty() { None::<String> } else { Some(start_date.get()) },
                "end_date": if end_date.get().is_empty() { None::<String> } else { Some(end_date.get()) },
                "tags": if tag_list.is_empty() { None::<Vec<String>> } else { Some(tag_list) },
                "currency": Some("USD".to_string()),
                "entry_type": None::<String>,
            });
            
            let args = serde_wasm_bindgen::to_value(&serde_json::json!({
                "filter": filter
            })).unwrap();
            
            let result = invoke("get_report_data", args).await;
            
            match serde_wasm_bindgen::from_value::<Result<ReportData, String>>(result) {
                Ok(Ok(data)) => {
                    set_report_data.set(Some(data));
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
    
    view! {
        <div class="view-container">
            <h1>"Overview"</h1>
            <div class="current-date">
                <p>"Current Date: " {current_date}</p>
            </div>
            
            <div class="filter-section">
                <h2>"Filters"</h2>
                <div class="filter-inputs">
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
                    <div class="input-group">
                        <label>"Tags (comma-separated):"</label>
                        <input 
                            type="text" 
                            prop:value=tags
                            on:input=move |ev| set_tags.set(event_target_value(&ev))
                            placeholder="e.g., food, transport"
                        />
                    </div>
                    <button 
                        class="btn-primary" 
                        on:click=load_report
                        disabled=move || loading.get()
                    >
                        {move || if loading.get() { "Loading..." } else { "Apply Filter" }}
                    </button>
                </div>
            </div>
            
            {move || {
                report_data.get().map(|data| {
                    let income = data.total_income.parse::<f64>().unwrap_or(0.0);
                    let expenses = data.total_expenses.parse::<f64>().unwrap_or(0.0);
                    
                    view! {
                        <div class="report-section">
                            <h2>"Income vs Expenses"</h2>
                            <div class="chart-container">
                                <BarChart
                                    data=vec![
                                        BarChartData {
                                            label: "Income".to_string(),
                                            value: income,
                                            color: "#4ade80".to_string(),
                                        },
                                        BarChartData {
                                            label: "Expenses".to_string(),
                                            value: expenses,
                                            color: "#ef4444".to_string(),
                                        },
                                    ]
                                    width=600
                                    height=400
                                />
                            </div>
                            <div class="summary">
                                <div class="summary-item income">
                                    <span class="label">"Total Income:"</span>
                                    <span class="value">{&data.total_income} " " {&data.currency}</span>
                                </div>
                                <div class="summary-item expense">
                                    <span class="label">"Total Expenses:"</span>
                                    <span class="value">{&data.total_expenses} " " {&data.currency}</span>
                                </div>
                                <div class="summary-item net">
                                    <span class="label">"Net:"</span>
                                    <span class="value">{&data.net} " " {&data.currency}</span>
                                </div>
                            </div>
                        </div>
                    }
                })
            }}
        </div>
    }
}

#[derive(Clone)]
struct BarChartData {
    label: String,
    value: f64,
    color: String,
}

#[component]
fn BarChart(
    data: Vec<BarChartData>,
    width: u32,
    height: u32,
) -> impl IntoView {
    let max_value = data.iter().map(|d| d.value).fold(0.0f64, f64::max);
    let bar_width = (width as f64 / data.len() as f64) * 0.8;
    let spacing = (width as f64 / data.len() as f64) * 0.2;
    
    view! {
        <svg width=width height=height class="bar-chart">
            <g>
                {data.into_iter().enumerate().map(|(i, item)| {
                    let x = (i as f64 * (bar_width + spacing)) + (spacing / 2.0);
                    let bar_height = if max_value > 0.0 {
                        (item.value / max_value) * (height as f64 * 0.8)
                    } else {
                        0.0
                    };
                    let y = height as f64 - bar_height - 40.0;
                    
                    view! {
                        <g>
                            <rect 
                                x=x 
                                y=y 
                                width=bar_width 
                                height=bar_height 
                                fill=item.color.clone()
                                rx="4"
                            />
                            <text 
                                x=x + (bar_width / 2.0) 
                                y=height - 20 
                                text-anchor="middle" 
                                class="chart-label"
                            >
                                {item.label}
                            </text>
                            <text 
                                x=x + (bar_width / 2.0) 
                                y=y - 5.0 
                                text-anchor="middle" 
                                class="chart-value"
                            >
                                {format!("{:.2}", item.value)}
                            </text>
                        </g>
                    }
                }).collect::<Vec<_>>()}
            </g>
        </svg>
    }
}

