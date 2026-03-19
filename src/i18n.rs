use leptos::prelude::*;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
pub struct I18n {
    pub locale: RwSignal<String>,
    translations: RwSignal<HashMap<String, Value>>,
}

impl I18n {
    pub fn new() -> Self {
        let stored_locale = get_stored_locale().unwrap_or_else(|| "en".to_string());
        let locale = RwSignal::new(stored_locale.clone());
        let translations = RwSignal::new(HashMap::new());

        let en_json: Value = serde_json::from_str(include_str!("../locales/en.json"))
            .expect("Failed to parse en.json");
        let ja_json: Value = serde_json::from_str(include_str!("../locales/ja.json"))
            .expect("Failed to parse ja.json");

        let mut map = HashMap::new();
        map.insert("en".to_string(), en_json);
        map.insert("ja".to_string(), ja_json);
        translations.set(map);

        I18n {
            locale,
            translations,
        }
    }

    pub fn t(&self, key: &str) -> String {
        let locale = self.locale.get();
        let translations = self.translations.get();

        if let Some(lang) = translations.get(&locale) {
            let parts: Vec<&str> = key.split('.').collect();
            let mut current = lang;
            for part in &parts {
                match current.get(*part) {
                    Some(v) => current = v,
                    None => return key.to_string(),
                }
            }
            match current.as_str() {
                Some(s) => s.to_string(),
                None => key.to_string(),
            }
        } else {
            key.to_string()
        }
    }

    pub fn set_locale(&self, locale: &str) {
        self.locale.set(locale.to_string());
        store_locale(locale);
    }

    pub fn current_locale(&self) -> String {
        self.locale.get()
    }
}

fn get_stored_locale() -> Option<String> {
    let window = web_sys::window()?;
    let storage = window.local_storage().ok()??;
    storage.get_item("fa_locale").ok()?
}

fn store_locale(locale: &str) {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.set_item("fa_locale", locale);
        }
    }
}

pub fn use_i18n() -> I18n {
    expect_context::<I18n>()
}
