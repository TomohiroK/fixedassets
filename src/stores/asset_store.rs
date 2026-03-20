use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{IdbDatabase, IdbObjectStoreParameters, IdbTransactionMode, IdbRequest, IdbOpenDbRequest};
use js_sys::Array;

use crate::models::asset::Asset;

const DB_NAME: &str = "fixedassets_db";
const DB_VERSION: u32 = 2;
const STORE_NAME: &str = "assets";
const PHOTO_STORE: &str = "photos";

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
        if !has_store(&db, PHOTO_STORE) {
            let params = IdbObjectStoreParameters::new();
            params.set_key_path(&JsValue::from_str("id"));
            let store = db
                .create_object_store_with_optional_parameters(PHOTO_STORE, &params)
                .unwrap();
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

/// Import assets from CSV text
pub async fn import_assets_csv(csv_text: &str) -> Result<usize, String> {
    use rust_decimal::Decimal;
    use std::str::FromStr;

    let mut lines = csv_text.lines();

    // Skip header
    let header = lines.next().ok_or("Empty CSV file")?;
    // Validate header has expected columns
    let cols: Vec<&str> = header.split(',').collect();
    if cols.len() < 10 {
        return Err(format!("CSV must have at least 10 columns. Found {} columns. Check the template format.", cols.len()));
    }

    let mut count = 0;
    for (line_num, line) in lines.enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let fields = parse_csv_line(line);
        if fields.len() < 10 {
            return Err(format!("Line {}: Expected at least 10 fields, found {}.", line_num + 2, fields.len()));
        }

        let asset_number = fields.get(0).unwrap_or(&String::new()).clone();
        let name = fields.get(1).unwrap_or(&String::new()).clone();
        if name.is_empty() {
            return Err(format!("Line {}: Asset name is required.", line_num + 2));
        }

        let category = parse_category(fields.get(2).map(|s| s.as_str()).unwrap_or("Other"));
        let acquisition_date = fields.get(3).unwrap_or(&String::new()).clone();
        if acquisition_date.is_empty() {
            return Err(format!("Line {}: Acquisition date is required (YYYY-MM-DD).", line_num + 2));
        }

        let cost = Decimal::from_str(fields.get(4).unwrap_or(&"0".to_string()))
            .map_err(|_| format!("Line {}: Invalid cost value.", line_num + 2))?;
        let salvage_value = Decimal::from_str(fields.get(5).unwrap_or(&"0".to_string()))
            .map_err(|_| format!("Line {}: Invalid salvage value.", line_num + 2))?;
        let useful_life: u32 = fields.get(6).unwrap_or(&"5".to_string()).parse()
            .map_err(|_| format!("Line {}: Invalid useful life.", line_num + 2))?;
        let depreciation_method = parse_method(fields.get(7).map(|s| s.as_str()).unwrap_or("SL"));
        let location = fields.get(8).unwrap_or(&String::new()).clone();
        let description = fields.get(9).unwrap_or(&String::new()).clone();

        let prior_years: u32 = fields.get(10).and_then(|s| s.parse().ok()).unwrap_or(0);
        let prior_months: u32 = fields.get(11).and_then(|s| s.parse().ok()).unwrap_or(0);
        let status = parse_status(fields.get(12).map(|s| s.as_str()).unwrap_or("InUse"));
        let tags: Vec<String> = fields.get(13)
            .map(|s| s.split(';').map(|t| t.trim().to_string()).filter(|t| !t.is_empty()).collect())
            .unwrap_or_default();

        let mut asset = Asset::new(
            asset_number,
            name,
            category,
            acquisition_date,
            cost,
            salvage_value,
            useful_life,
            depreciation_method,
            prior_years,
            prior_months,
            location,
            description,
            tags,
        );
        asset.status = status;

        save_asset(&asset).await?;
        count += 1;
    }

    Ok(count)
}

/// Parse a CSV line handling quoted fields with commas
fn parse_csv_line(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;

    for ch in line.chars() {
        if ch == '"' {
            in_quotes = !in_quotes;
        } else if ch == ',' && !in_quotes {
            fields.push(current.trim().to_string());
            current = String::new();
        } else {
            current.push(ch);
        }
    }
    fields.push(current.trim().to_string());
    fields
}

fn parse_category(s: &str) -> Category {
    use crate::models::asset::Category;
    match s.trim() {
        "Land" | "土地" => Category::Land,
        "Building" | "建物" => Category::Building,
        "BuildingEquipment" | "建物付属設備" => Category::BuildingEquipment,
        "Structures" | "構築物" => Category::Structures,
        "Machinery" | "機械装置" => Category::Machinery,
        "ToolsFixtures" | "工具器具備品" => Category::ToolsFixtures,
        "Vehicles" | "車両運搬具" => Category::Vehicles,
        "LeasedAssets" | "リース資産" => Category::LeasedAssets,
        "ConstructionInProgress" | "建設仮勘定" => Category::ConstructionInProgress,
        "Patents" | "特許権" => Category::Patents,
        "Trademarks" | "商標権" => Category::Trademarks,
        "LeaseholdRights" | "借地権" => Category::LeaseholdRights,
        "Software" | "ソフトウエア" => Category::Software,
        "FacilityRights" | "施設利用権" => Category::FacilityRights,
        "Goodwill" | "のれん" => Category::Goodwill,
        _ => Category::Other,
    }
}

fn parse_method(s: &str) -> DepreciationMethod {
    use crate::models::asset::DepreciationMethod;
    match s.trim() {
        "DB" | "DecliningBalance" | "定率法" => DepreciationMethod::DecliningBalance,
        _ => DepreciationMethod::StraightLine,
    }
}

fn parse_status(s: &str) -> AssetStatus {
    use crate::models::asset::AssetStatus;
    match s.trim() {
        "Disposed" | "除却済み" => AssetStatus::Disposed,
        "Transferred" | "移管済み" => AssetStatus::Transferred,
        "Maintenance" | "メンテナンス中" => AssetStatus::Maintenance,
        _ => AssetStatus::InUse,
    }
}

/// Generate CSV template string
pub fn csv_template() -> String {
    "asset_number,name,category,acquisition_date,cost,salvage_value,useful_life,depreciation_method,location,description,prior_years,prior_months,status,tags\n\
     FA-001,Office Desk,ToolsFixtures,2024-04-01,50000,1,8,SL,Tokyo Office,Executive desk,0,0,InUse,office;furniture\n\
     FA-002,Delivery Van,Vehicles,2023-10-15,2500000,1,6,DB,Warehouse,Toyota HiAce,1,3,InUse,vehicle;delivery\n\
     FA-003,Accounting Software,Software,2024-01-01,300000,0,5,SL,,Cloud license,0,0,InUse,software;accounting\n".to_string()
}

/// Generate JSON template string
pub fn json_template() -> String {
    use crate::models::asset::{Category, DepreciationMethod};
    let samples = vec![
        Asset::new(
            "FA-001".to_string(),
            "Office Desk".to_string(),
            Category::ToolsFixtures,
            "2024-04-01".to_string(),
            Decimal::from_str("50000").unwrap(),
            Decimal::ONE,
            8,
            DepreciationMethod::StraightLine,
            0, 0,
            "Tokyo Office".to_string(),
            "Executive desk".to_string(),
            vec!["office".to_string(), "furniture".to_string()],
        ),
        Asset::new(
            "FA-002".to_string(),
            "Delivery Van".to_string(),
            Category::Vehicles,
            "2023-10-15".to_string(),
            Decimal::from_str("2500000").unwrap(),
            Decimal::ONE,
            6,
            DepreciationMethod::DecliningBalance,
            1, 3,
            "Warehouse".to_string(),
            "Toyota HiAce".to_string(),
            vec!["vehicle".to_string(), "delivery".to_string()],
        ),
    ];
    serde_json::to_string_pretty(&samples).unwrap_or_default()
}

use rust_decimal::Decimal;
use std::str::FromStr;
use crate::models::asset::{Category, DepreciationMethod, AssetStatus};

/// Export all assets as CSV
pub async fn export_all_assets_csv() -> Result<String, String> {
    let assets = get_all_assets().await?;
    let mut csv = String::from("asset_number,name,category,acquisition_date,cost,salvage_value,useful_life,depreciation_method,location,description,prior_years,prior_months,status,tags\n");

    for a in &assets {
        let category = format!("{:?}", a.category);
        let method = match a.depreciation_method {
            DepreciationMethod::StraightLine => "SL",
            DepreciationMethod::DecliningBalance => "DB",
        };
        let status = format!("{:?}", a.status);
        let tags = a.tags.join(";");

        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            csv_escape(&a.asset_number),
            csv_escape(&a.name),
            category,
            a.acquisition_date,
            a.cost,
            a.salvage_value,
            a.useful_life,
            method,
            csv_escape(&a.location),
            csv_escape(&a.description),
            a.prior_depreciation_years,
            a.prior_depreciation_months,
            status,
            csv_escape(&tags),
        ));
    }

    Ok(csv)
}

fn csv_escape(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}
