use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::auth::get_current_company_id;

const STORAGE_KEY_PREFIX: &str = "fa_departments";

/// Department master record
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Department {
    pub id: String,
    pub code: String,       // e.g. "SALES", "IT", "HR"
    pub name: String,       // Display name
}

impl Department {
    pub fn new(code: String, name: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            code,
            name,
        }
    }

    /// Get the storage key scoped by company_id
    fn storage_key() -> String {
        let cid = get_current_company_id();
        if cid.is_empty() {
            STORAGE_KEY_PREFIX.to_string()
        } else {
            format!("{}_{}", STORAGE_KEY_PREFIX, cid)
        }
    }

    /// Load all departments from localStorage (scoped by company)
    pub fn load_all() -> Vec<Department> {
        let key = Self::storage_key();
        let window = match web_sys::window() {
            Some(w) => w,
            None => return vec![],
        };
        let storage = match window.local_storage() {
            Ok(Some(s)) => s,
            _ => return vec![],
        };
        let json = match storage.get_item(&key) {
            Ok(Some(j)) => j,
            _ => return vec![],
        };
        serde_json::from_str(&json).unwrap_or_default()
    }

    /// Save all departments to localStorage (scoped by company)
    pub fn save_all(departments: &[Department]) {
        let key = Self::storage_key();
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(json) = serde_json::to_string(departments) {
                    let _ = storage.set_item(&key, &json);
                }
            }
        }
    }

    /// Add a department
    pub fn add(dept: Department) {
        let mut all = Self::load_all();
        all.push(dept);
        Self::save_all(&all);
    }

    /// Remove a department by id
    pub fn remove(id: &str) {
        let mut all = Self::load_all();
        all.retain(|d| d.id != id);
        Self::save_all(&all);
    }

    /// Update a department
    pub fn update(dept: &Department) {
        let mut all = Self::load_all();
        if let Some(existing) = all.iter_mut().find(|d| d.id == dept.id) {
            existing.code = dept.code.clone();
            existing.name = dept.name.clone();
        }
        Self::save_all(&all);
    }

    /// Find by id
    pub fn find_by_id(id: &str) -> Option<Department> {
        Self::load_all().into_iter().find(|d| d.id == id)
    }

    /// Get display name for a department id (returns "—" if not found)
    pub fn display_name(id: &str) -> String {
        Self::find_by_id(id)
            .map(|d| {
                if d.code.is_empty() {
                    d.name
                } else {
                    format!("{} - {}", d.code, d.name)
                }
            })
            .unwrap_or_else(|| "—".to_string())
    }
}
