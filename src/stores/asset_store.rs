use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{IdbDatabase, IdbObjectStoreParameters, IdbTransactionMode, IdbRequest, IdbOpenDbRequest};
use js_sys::Array;

use crate::models::asset::Asset;

const DB_NAME: &str = "fixedassets_db";
const DB_VERSION: u32 = 1;
const STORE_NAME: &str = "assets";

fn has_store(db: &IdbDatabase, name: &str) -> bool {
    let list = db.object_store_names();
    for i in 0..list.length() {
        if let Some(s) = list.get(i) {
            if s == name {
                return true;
            }
        }
    }
    false
}

async fn open_db() -> Result<IdbDatabase, String> {
    let window = web_sys::window().ok_or("No window")?;
    let idb_factory = window
        .indexed_db()
        .map_err(|_| "IndexedDB not available")?
        .ok_or("IndexedDB not available")?;

    let open_request: IdbOpenDbRequest = idb_factory
        .open_with_u32(DB_NAME, DB_VERSION)
        .map_err(|e| format!("Failed to open DB: {:?}", e))?;

    let (tx, rx) = futures_channel::oneshot::channel::<Result<IdbDatabase, String>>();
    let tx = Rc::new(RefCell::new(Some(tx)));

    let on_upgrade = Closure::wrap(Box::new(move |event: web_sys::Event| {
        let target = event.target().unwrap();
        let request: IdbOpenDbRequest = target.unchecked_into();
        let db: IdbDatabase = request.result().unwrap().unchecked_into();

        if !has_store(&db, STORE_NAME) {
            let params = IdbObjectStoreParameters::new();
            params.set_key_path(&JsValue::from_str("id"));
            let _store = db
                .create_object_store_with_optional_parameters(STORE_NAME, &params)
                .unwrap();
        }
    }) as Box<dyn FnMut(_)>);

    let tx_s = Rc::clone(&tx);
    let on_success = Closure::wrap(Box::new(move |event: web_sys::Event| {
        let target = event.target().unwrap();
        let request: IdbOpenDbRequest = target.unchecked_into();
        let db: IdbDatabase = request.result().unwrap().unchecked_into();
        if let Some(sender) = tx_s.borrow_mut().take() {
            let _ = sender.send(Ok(db));
        }
    }) as Box<dyn FnMut(_)>);

    let tx_e = Rc::clone(&tx);
    let on_error = Closure::wrap(Box::new(move |_event: web_sys::Event| {
        if let Some(sender) = tx_e.borrow_mut().take() {
            let _ = sender.send(Err("Failed to open database".to_string()));
        }
    }) as Box<dyn FnMut(_)>);

    open_request.set_onupgradeneeded(Some(on_upgrade.as_ref().unchecked_ref()));
    open_request.set_onsuccess(Some(on_success.as_ref().unchecked_ref()));
    open_request.set_onerror(Some(on_error.as_ref().unchecked_ref()));

    on_upgrade.forget();
    on_success.forget();
    on_error.forget();

    rx.await.map_err(|_| "Channel error".to_string())?
}

fn idb_request_to_future(request: &IdbRequest) -> futures_channel::oneshot::Receiver<Result<JsValue, String>> {
    let (tx, rx) = futures_channel::oneshot::channel::<Result<JsValue, String>>();
    let tx = Rc::new(RefCell::new(Some(tx)));

    let tx_s = Rc::clone(&tx);
    let on_success = Closure::wrap(Box::new(move |event: web_sys::Event| {
        let target = event.target().unwrap();
        let request: IdbRequest = target.unchecked_into();
        let result = request.result().unwrap_or(JsValue::UNDEFINED);
        if let Some(sender) = tx_s.borrow_mut().take() {
            let _ = sender.send(Ok(result));
        }
    }) as Box<dyn FnMut(_)>);

    let tx_e = Rc::clone(&tx);
    let on_error = Closure::wrap(Box::new(move |_event: web_sys::Event| {
        if let Some(sender) = tx_e.borrow_mut().take() {
            let _ = sender.send(Err("IDB request failed".to_string()));
        }
    }) as Box<dyn FnMut(_)>);

    request.set_onsuccess(Some(on_success.as_ref().unchecked_ref()));
    request.set_onerror(Some(on_error.as_ref().unchecked_ref()));

    on_success.forget();
    on_error.forget();

    rx
}

pub async fn save_asset(asset: &Asset) -> Result<(), String> {
    let db = open_db().await?;
    let transaction = db
        .transaction_with_str_and_mode(STORE_NAME, IdbTransactionMode::Readwrite)
        .map_err(|e| format!("Transaction error: {:?}", e))?;
    let store = transaction
        .object_store(STORE_NAME)
        .map_err(|e| format!("Store error: {:?}", e))?;

    let json = serde_json::to_string(asset).map_err(|e| format!("Serialize error: {}", e))?;
    let js_value = js_sys::JSON::parse(&json).map_err(|e| format!("JSON parse error: {:?}", e))?;

    let request = store
        .put(&js_value)
        .map_err(|e| format!("Put error: {:?}", e))?;

    let rx = idb_request_to_future(&request);
    rx.await.map_err(|_| "Channel error".to_string())??;
    Ok(())
}

pub async fn get_all_assets() -> Result<Vec<Asset>, String> {
    let db = open_db().await?;
    let transaction = db
        .transaction_with_str_and_mode(STORE_NAME, IdbTransactionMode::Readonly)
        .map_err(|e| format!("Transaction error: {:?}", e))?;
    let store = transaction
        .object_store(STORE_NAME)
        .map_err(|e| format!("Store error: {:?}", e))?;

    let request = store
        .get_all()
        .map_err(|e| format!("GetAll error: {:?}", e))?;

    let rx = idb_request_to_future(&request);
    let result = rx.await.map_err(|_| "Channel error".to_string())??;

    let array: Array = result.unchecked_into();
    let mut assets = Vec::new();

    for i in 0..array.length() {
        let item = array.get(i);
        let json = js_sys::JSON::stringify(&item)
            .map_err(|e| format!("Stringify error: {:?}", e))?;
        let json_str: String = json.into();
        match serde_json::from_str::<Asset>(&json_str) {
            Ok(asset) => assets.push(asset),
            Err(e) => log::warn!("Failed to deserialize asset: {}", e),
        }
    }

    assets.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    Ok(assets)
}

pub async fn get_asset(id: &str) -> Result<Option<Asset>, String> {
    let db = open_db().await?;
    let transaction = db
        .transaction_with_str_and_mode(STORE_NAME, IdbTransactionMode::Readonly)
        .map_err(|e| format!("Transaction error: {:?}", e))?;
    let store = transaction
        .object_store(STORE_NAME)
        .map_err(|e| format!("Store error: {:?}", e))?;

    let request = store
        .get(&JsValue::from_str(id))
        .map_err(|e| format!("Get error: {:?}", e))?;

    let rx = idb_request_to_future(&request);
    let result = rx.await.map_err(|_| "Channel error".to_string())??;

    if result.is_undefined() || result.is_null() {
        return Ok(None);
    }

    let json = js_sys::JSON::stringify(&result)
        .map_err(|e| format!("Stringify error: {:?}", e))?;
    let json_str: String = json.into();
    let asset: Asset = serde_json::from_str(&json_str)
        .map_err(|e| format!("Deserialize error: {}", e))?;
    Ok(Some(asset))
}

pub async fn delete_asset(id: &str) -> Result<(), String> {
    let db = open_db().await?;
    let transaction = db
        .transaction_with_str_and_mode(STORE_NAME, IdbTransactionMode::Readwrite)
        .map_err(|e| format!("Transaction error: {:?}", e))?;
    let store = transaction
        .object_store(STORE_NAME)
        .map_err(|e| format!("Store error: {:?}", e))?;

    let request = store
        .delete(&JsValue::from_str(id))
        .map_err(|e| format!("Delete error: {:?}", e))?;

    let rx = idb_request_to_future(&request);
    rx.await.map_err(|_| "Channel error".to_string())??;
    Ok(())
}

pub async fn clear_all_assets() -> Result<(), String> {
    let db = open_db().await?;
    let transaction = db
        .transaction_with_str_and_mode(STORE_NAME, IdbTransactionMode::Readwrite)
        .map_err(|e| format!("Transaction error: {:?}", e))?;
    let store = transaction
        .object_store(STORE_NAME)
        .map_err(|e| format!("Store error: {:?}", e))?;

    let request = store
        .clear()
        .map_err(|e| format!("Clear error: {:?}", e))?;

    let rx = idb_request_to_future(&request);
    rx.await.map_err(|_| "Channel error".to_string())??;
    Ok(())
}

pub async fn export_all_assets() -> Result<String, String> {
    let assets = get_all_assets().await?;
    serde_json::to_string_pretty(&assets).map_err(|e| format!("Serialize error: {}", e))
}

pub async fn import_assets(json: &str) -> Result<usize, String> {
    let assets: Vec<Asset> = serde_json::from_str(json)
        .map_err(|e| format!("Parse error: {}", e))?;
    let count = assets.len();
    for asset in &assets {
        save_asset(asset).await?;
    }
    Ok(count)
}
