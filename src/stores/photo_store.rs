use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{IdbDatabase, IdbObjectStoreParameters, IdbTransactionMode, IdbRequest, IdbOpenDbRequest};
use js_sys::Array;

use crate::models::photo::AssetPhoto;

const DB_NAME: &str = "fixedassets_db";
const DB_VERSION: u32 = 2;
const PHOTO_STORE: &str = "photos";
const ASSET_STORE: &str = "assets";

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

        if !has_store(&db, ASSET_STORE) {
            let params = IdbObjectStoreParameters::new();
            params.set_key_path(&JsValue::from_str("id"));
            let _store = db
                .create_object_store_with_optional_parameters(ASSET_STORE, &params)
                .unwrap();
        }

        if !has_store(&db, PHOTO_STORE) {
            let params = IdbObjectStoreParameters::new();
            params.set_key_path(&JsValue::from_str("id"));
            let store = db
                .create_object_store_with_optional_parameters(PHOTO_STORE, &params)
                .unwrap();
            // Index by asset_id for efficient lookups
            let idx_params = web_sys::IdbIndexParameters::new();
            idx_params.set_unique(false);
            let _ = store.create_index_with_str_and_optional_parameters(
                "asset_id_idx", "asset_id", &idx_params,
            );
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

pub async fn save_photo(photo: &AssetPhoto) -> Result<(), String> {
    let db = open_db().await?;
    let transaction = db
        .transaction_with_str_and_mode(PHOTO_STORE, IdbTransactionMode::Readwrite)
        .map_err(|e| format!("Transaction error: {:?}", e))?;
    let store = transaction
        .object_store(PHOTO_STORE)
        .map_err(|e| format!("Store error: {:?}", e))?;

    let json = serde_json::to_string(photo).map_err(|e| format!("Serialize error: {}", e))?;
    let js_value = js_sys::JSON::parse(&json).map_err(|e| format!("JSON parse error: {:?}", e))?;

    let request = store
        .put(&js_value)
        .map_err(|e| format!("Put error: {:?}", e))?;

    let rx = idb_request_to_future(&request);
    rx.await.map_err(|_| "Channel error".to_string())??;
    Ok(())
}

pub async fn get_photos_for_asset(asset_id: &str) -> Result<Vec<AssetPhoto>, String> {
    let db = open_db().await?;
    let transaction = db
        .transaction_with_str_and_mode(PHOTO_STORE, IdbTransactionMode::Readonly)
        .map_err(|e| format!("Transaction error: {:?}", e))?;
    let store = transaction
        .object_store(PHOTO_STORE)
        .map_err(|e| format!("Store error: {:?}", e))?;

    // Use index to query by asset_id
    let index = store
        .index("asset_id_idx")
        .map_err(|e| format!("Index error: {:?}", e))?;

    let request = index
        .get_all_with_key(&JsValue::from_str(asset_id))
        .map_err(|e| format!("GetAll error: {:?}", e))?;

    let rx = idb_request_to_future(&request);
    let result = rx.await.map_err(|_| "Channel error".to_string())??;

    let array: Array = result.unchecked_into();
    let mut photos = Vec::new();

    for i in 0..array.length() {
        let item = array.get(i);
        let json = js_sys::JSON::stringify(&item)
            .map_err(|e| format!("Stringify error: {:?}", e))?;
        let json_str: String = json.into();
        match serde_json::from_str::<AssetPhoto>(&json_str) {
            Ok(photo) => photos.push(photo),
            Err(e) => log::warn!("Failed to deserialize photo: {}", e),
        }
    }

    photos.sort_by(|a, b| a.created_at.cmp(&b.created_at));
    Ok(photos)
}

pub async fn delete_photo(id: &str) -> Result<(), String> {
    let db = open_db().await?;
    let transaction = db
        .transaction_with_str_and_mode(PHOTO_STORE, IdbTransactionMode::Readwrite)
        .map_err(|e| format!("Transaction error: {:?}", e))?;
    let store = transaction
        .object_store(PHOTO_STORE)
        .map_err(|e| format!("Store error: {:?}", e))?;

    let request = store
        .delete(&JsValue::from_str(id))
        .map_err(|e| format!("Delete error: {:?}", e))?;

    let rx = idb_request_to_future(&request);
    rx.await.map_err(|_| "Channel error".to_string())??;
    Ok(())
}

pub async fn delete_photos_for_asset(asset_id: &str) -> Result<(), String> {
    let photos = get_photos_for_asset(asset_id).await?;
    for photo in &photos {
        delete_photo(&photo.id).await?;
    }
    Ok(())
}

pub async fn count_photos_for_asset(asset_id: &str) -> Result<usize, String> {
    let photos = get_photos_for_asset(asset_id).await?;
    Ok(photos.len())
}
