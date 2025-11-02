use leptos::*;
use leptos_router::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "dialog"])]
    async fn open(options: JsValue) -> JsValue;
    
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "dialog"])]
    async fn save(options: JsValue) -> JsValue;
    
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn Ribbon() -> impl IntoView {
    let navigate = use_navigate();
    
    let open_create_ledger = move |_| {
        spawn_local(async move {
            // Show dialog to choose between open and create
            let choice = web_sys::window()
                .unwrap()
                .confirm_with_message("Click OK to open existing ledger, Cancel to create new")
                .unwrap();
            
            if choice {
                // Open existing ledger
                let options = serde_wasm_bindgen::to_value(&serde_json::json!({
                    "filters": [{
                        "name": "Beans Ledger",
                        "extensions": ["bean"]
                    }]
                })).unwrap();
                
                let result = open(options).await;
                if let Ok(path) = serde_wasm_bindgen::from_value::<Option<String>>(result) {
                    if let Some(path) = path {
                        let args = serde_wasm_bindgen::to_value(&serde_json::json!({
                            "path": path
                        })).unwrap();
                        
                        let result = invoke("open_ledger", args).await;
                        match serde_wasm_bindgen::from_value::<Result<String, String>>(result) {
                            Ok(Ok(msg)) => {
                                web_sys::window().unwrap().alert_with_message(&msg).ok();
                                navigate("/overview", Default::default());
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
            } else {
                // Create new ledger
                let options = serde_wasm_bindgen::to_value(&serde_json::json!({
                    "filters": [{
                        "name": "Beans Ledger",
                        "extensions": ["bean"]
                    }],
                    "defaultPath": "ledger.bean"
                })).unwrap();
                
                let result = save(options).await;
                if let Ok(path) = serde_wasm_bindgen::from_value::<Option<String>>(result) {
                    if let Some(path) = path {
                        let args = serde_wasm_bindgen::to_value(&serde_json::json!({
                            "path": path
                        })).unwrap();
                        
                        let result = invoke("create_ledger", args).await;
                        match serde_wasm_bindgen::from_value::<Result<String, String>>(result) {
                            Ok(Ok(msg)) => {
                                web_sys::window().unwrap().alert_with_message(&msg).ok();
                                navigate("/overview", Default::default());
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
            }
        });
    };
    
    view! {
        <div class="ribbon">
            <button class="ribbon-btn" on:click=open_create_ledger>
                <span class="ribbon-icon">"📁"</span>
                <span class="ribbon-label">"Open/Create"</span>
            </button>
            <A href="/overview" class="ribbon-btn">
                <span class="ribbon-icon">"📊"</span>
                <span class="ribbon-label">"Overview"</span>
            </A>
            <A href="/add-entry" class="ribbon-btn">
                <span class="ribbon-icon">"➕"</span>
                <span class="ribbon-label">"Add Entry"</span>
            </A>
            <A href="/edit-entry" class="ribbon-btn">
                <span class="ribbon-icon">"✏️"</span>
                <span class="ribbon-label">"Edit Entry"</span>
            </A>
            <A href="/export" class="ribbon-btn">
                <span class="ribbon-icon">"💾"</span>
                <span class="ribbon-label">"Export"</span>
            </A>
        </div>
    }
}

