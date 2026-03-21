use serde::{Deserialize, Serialize};

/// ASEAN countries supported
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum AseanCountry {
    Japan,
    Singapore,
    Malaysia,
    Thailand,
    Indonesia,
    Philippines,
    Vietnam,
    Myanmar,
    Cambodia,
    Laos,
    Brunei,
}

impl AseanCountry {
    pub fn all() -> Vec<AseanCountry> {
        vec![
            AseanCountry::Japan,
            AseanCountry::Singapore,
            AseanCountry::Malaysia,
            AseanCountry::Thailand,
            AseanCountry::Indonesia,
            AseanCountry::Philippines,
            AseanCountry::Vietnam,
            AseanCountry::Myanmar,
            AseanCountry::Cambodia,
            AseanCountry::Laos,
            AseanCountry::Brunei,
        ]
    }

    pub fn code(&self) -> &str {
        match self {
            AseanCountry::Japan => "JP",
            AseanCountry::Singapore => "SG",
            AseanCountry::Malaysia => "MY",
            AseanCountry::Thailand => "TH",
            AseanCountry::Indonesia => "ID",
            AseanCountry::Philippines => "PH",
            AseanCountry::Vietnam => "VN",
            AseanCountry::Myanmar => "MM",
            AseanCountry::Cambodia => "KH",
            AseanCountry::Laos => "LA",
            AseanCountry::Brunei => "BN",
        }
    }

    pub fn name_en(&self) -> &str {
        match self {
            AseanCountry::Japan => "Japan",
            AseanCountry::Singapore => "Singapore",
            AseanCountry::Malaysia => "Malaysia",
            AseanCountry::Thailand => "Thailand",
            AseanCountry::Indonesia => "Indonesia",
            AseanCountry::Philippines => "Philippines",
            AseanCountry::Vietnam => "Vietnam",
            AseanCountry::Myanmar => "Myanmar",
            AseanCountry::Cambodia => "Cambodia",
            AseanCountry::Laos => "Laos",
            AseanCountry::Brunei => "Brunei",
        }
    }

    pub fn name_ja(&self) -> &str {
        match self {
            AseanCountry::Japan => "日本",
            AseanCountry::Singapore => "シンガポール",
            AseanCountry::Malaysia => "マレーシア",
            AseanCountry::Thailand => "タイ",
            AseanCountry::Indonesia => "インドネシア",
            AseanCountry::Philippines => "フィリピン",
            AseanCountry::Vietnam => "ベトナム",
            AseanCountry::Myanmar => "ミャンマー",
            AseanCountry::Cambodia => "カンボジア",
            AseanCountry::Laos => "ラオス",
            AseanCountry::Brunei => "ブルネイ",
        }
    }

    pub fn flag(&self) -> &str {
        match self {
            AseanCountry::Japan => "\u{1F1EF}\u{1F1F5}",
            AseanCountry::Singapore => "\u{1F1F8}\u{1F1EC}",
            AseanCountry::Malaysia => "\u{1F1F2}\u{1F1FE}",
            AseanCountry::Thailand => "\u{1F1F9}\u{1F1ED}",
            AseanCountry::Indonesia => "\u{1F1EE}\u{1F1E9}",
            AseanCountry::Philippines => "\u{1F1F5}\u{1F1ED}",
            AseanCountry::Vietnam => "\u{1F1FB}\u{1F1F3}",
            AseanCountry::Myanmar => "\u{1F1F2}\u{1F1F2}",
            AseanCountry::Cambodia => "\u{1F1F0}\u{1F1ED}",
            AseanCountry::Laos => "\u{1F1F1}\u{1F1E6}",
            AseanCountry::Brunei => "\u{1F1E7}\u{1F1F3}",
        }
    }

    pub fn local_currency(&self) -> Currency {
        match self {
            AseanCountry::Japan => Currency::JPY,
            AseanCountry::Singapore => Currency::SGD,
            AseanCountry::Malaysia => Currency::MYR,
            AseanCountry::Thailand => Currency::THB,
            AseanCountry::Indonesia => Currency::IDR,
            AseanCountry::Philippines => Currency::PHP,
            AseanCountry::Vietnam => Currency::VND,
            AseanCountry::Myanmar => Currency::MMK,
            AseanCountry::Cambodia => Currency::KHR,
            AseanCountry::Laos => Currency::LAK,
            AseanCountry::Brunei => Currency::BND,
        }
    }

    pub fn from_code(code: &str) -> Option<Self> {
        match code {
            "JP" => Some(AseanCountry::Japan),
            "SG" => Some(AseanCountry::Singapore),
            "MY" => Some(AseanCountry::Malaysia),
            "TH" => Some(AseanCountry::Thailand),
            "ID" => Some(AseanCountry::Indonesia),
            "PH" => Some(AseanCountry::Philippines),
            "VN" => Some(AseanCountry::Vietnam),
            "MM" => Some(AseanCountry::Myanmar),
            "KH" => Some(AseanCountry::Cambodia),
            "LA" => Some(AseanCountry::Laos),
            "BN" => Some(AseanCountry::Brunei),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Currency {
    JPY,
    USD,
    CNY,
    SGD,
    MYR,
    THB,
    IDR,
    PHP,
    VND,
    MMK,
    KHR,
    LAK,
    BND,
}

impl Currency {
    pub fn code(&self) -> &str {
        match self {
            Currency::JPY => "JPY",
            Currency::USD => "USD",
            Currency::CNY => "CNY",
            Currency::SGD => "SGD",
            Currency::MYR => "MYR",
            Currency::THB => "THB",
            Currency::IDR => "IDR",
            Currency::PHP => "PHP",
            Currency::VND => "VND",
            Currency::MMK => "MMK",
            Currency::KHR => "KHR",
            Currency::LAK => "LAK",
            Currency::BND => "BND",
        }
    }

    pub fn symbol(&self) -> &str {
        match self {
            Currency::JPY => "¥",
            Currency::USD => "$",
            Currency::CNY => "CN¥",
            Currency::SGD => "S$",
            Currency::MYR => "RM",
            Currency::THB => "฿",
            Currency::IDR => "Rp",
            Currency::PHP => "₱",
            Currency::VND => "₫",
            Currency::MMK => "K",
            Currency::KHR => "៛",
            Currency::LAK => "₭",
            Currency::BND => "B$",
        }
    }

    pub fn name_en(&self) -> &str {
        match self {
            Currency::JPY => "Japanese Yen",
            Currency::USD => "US Dollar",
            Currency::CNY => "Chinese Yuan",
            Currency::SGD => "Singapore Dollar",
            Currency::MYR => "Malaysian Ringgit",
            Currency::THB => "Thai Baht",
            Currency::IDR => "Indonesian Rupiah",
            Currency::PHP => "Philippine Peso",
            Currency::VND => "Vietnamese Dong",
            Currency::MMK => "Myanmar Kyat",
            Currency::KHR => "Cambodian Riel",
            Currency::LAK => "Lao Kip",
            Currency::BND => "Brunei Dollar",
        }
    }

    pub fn from_code(code: &str) -> Option<Self> {
        match code {
            "JPY" => Some(Currency::JPY),
            "USD" => Some(Currency::USD),
            "CNY" => Some(Currency::CNY),
            "SGD" => Some(Currency::SGD),
            "MYR" => Some(Currency::MYR),
            "THB" => Some(Currency::THB),
            "IDR" => Some(Currency::IDR),
            "PHP" => Some(Currency::PHP),
            "VND" => Some(Currency::VND),
            "MMK" => Some(Currency::MMK),
            "KHR" => Some(Currency::KHR),
            "LAK" => Some(Currency::LAK),
            "BND" => Some(Currency::BND),
            _ => None,
        }
    }

    /// Currencies available for a given country: local + USD + CNY
    pub fn available_for(country: &AseanCountry) -> Vec<Currency> {
        let local = country.local_currency();
        let mut currencies = vec![local.clone()];
        if local != Currency::USD {
            currencies.push(Currency::USD);
        }
        if local != Currency::CNY {
            currencies.push(Currency::CNY);
        }
        currencies
    }
}

/// Company setup stored in localStorage
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompanySetup {
    pub company_name: String,
    pub country_code: String,
    pub currency_code: String,
}

const STORAGE_KEY_PREFIX: &str = "fa_company_setup";

impl CompanySetup {
    /// Get the storage key scoped by company_id
    fn storage_key() -> String {
        let cid = crate::auth::get_current_company_id();
        if cid.is_empty() {
            STORAGE_KEY_PREFIX.to_string()
        } else {
            format!("{}_{}", STORAGE_KEY_PREFIX, cid)
        }
    }

    pub fn load() -> Option<Self> {
        let key = Self::storage_key();
        let window = web_sys::window()?;
        let storage = window.local_storage().ok()??;
        let json = storage.get_item(&key).ok()??;
        serde_json::from_str(&json).ok()
    }

    pub fn save(&self) {
        let key = Self::storage_key();
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(json) = serde_json::to_string(self) {
                    let _ = storage.set_item(&key, &json);
                }
            }
        }
    }

    pub fn clear() {
        let key = Self::storage_key();
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.remove_item(&key);
            }
        }
    }

    pub fn country(&self) -> Option<AseanCountry> {
        AseanCountry::from_code(&self.country_code)
    }

    pub fn currency(&self) -> Option<Currency> {
        Currency::from_code(&self.currency_code)
    }
}
