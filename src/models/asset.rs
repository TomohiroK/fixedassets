use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Asset {
    pub id: String,
    pub name: String,
    pub category: Category,
    pub acquisition_date: String, // YYYY-MM-DD
    pub cost: Decimal,
    pub salvage_value: Decimal,
    pub useful_life: u32, // years
    pub depreciation_method: DepreciationMethod,
    pub location: String,
    pub description: String,
    pub status: AssetStatus,
    pub created_at: String,
    pub updated_at: String,
}

impl Asset {
    pub fn new(
        name: String,
        category: Category,
        acquisition_date: String,
        cost: Decimal,
        salvage_value: Decimal,
        useful_life: u32,
        depreciation_method: DepreciationMethod,
        location: String,
        description: String,
    ) -> Self {
        let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string();
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            category,
            acquisition_date,
            cost,
            salvage_value,
            useful_life,
            depreciation_method,
            location,
            description,
            status: AssetStatus::InUse,
            created_at: now.clone(),
            updated_at: now,
        }
    }

    pub fn acquisition_date_parsed(&self) -> Option<NaiveDate> {
        NaiveDate::parse_from_str(&self.acquisition_date, "%Y-%m-%d").ok()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Category {
    Building,
    Vehicle,
    Machinery,
    Furniture,
    Electronics,
    Software,
    Other,
}

impl Category {
    pub fn all() -> Vec<Category> {
        vec![
            Category::Building,
            Category::Vehicle,
            Category::Machinery,
            Category::Furniture,
            Category::Electronics,
            Category::Software,
            Category::Other,
        ]
    }

    pub fn i18n_key(&self) -> &str {
        match self {
            Category::Building => "category.building",
            Category::Vehicle => "category.vehicle",
            Category::Machinery => "category.machinery",
            Category::Furniture => "category.furniture",
            Category::Electronics => "category.electronics",
            Category::Software => "category.software",
            Category::Other => "category.other",
        }
    }

    pub fn from_index(i: usize) -> Self {
        match i {
            0 => Category::Building,
            1 => Category::Vehicle,
            2 => Category::Machinery,
            3 => Category::Furniture,
            4 => Category::Electronics,
            5 => Category::Software,
            _ => Category::Other,
        }
    }

    pub fn to_index(&self) -> usize {
        match self {
            Category::Building => 0,
            Category::Vehicle => 1,
            Category::Machinery => 2,
            Category::Furniture => 3,
            Category::Electronics => 4,
            Category::Software => 5,
            Category::Other => 6,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum AssetStatus {
    InUse,
    Disposed,
    Transferred,
    Maintenance,
}

impl AssetStatus {
    pub fn all() -> Vec<AssetStatus> {
        vec![
            AssetStatus::InUse,
            AssetStatus::Disposed,
            AssetStatus::Transferred,
            AssetStatus::Maintenance,
        ]
    }

    pub fn i18n_key(&self) -> &str {
        match self {
            AssetStatus::InUse => "status.in_use",
            AssetStatus::Disposed => "status.disposed",
            AssetStatus::Transferred => "status.transferred",
            AssetStatus::Maintenance => "status.maintenance",
        }
    }

    pub fn badge_class(&self) -> &str {
        match self {
            AssetStatus::InUse => "badge-green",
            AssetStatus::Disposed => "badge-red",
            AssetStatus::Transferred => "badge-blue",
            AssetStatus::Maintenance => "badge-yellow",
        }
    }

    pub fn from_index(i: usize) -> Self {
        match i {
            0 => AssetStatus::InUse,
            1 => AssetStatus::Disposed,
            2 => AssetStatus::Transferred,
            _ => AssetStatus::Maintenance,
        }
    }

    pub fn to_index(&self) -> usize {
        match self {
            AssetStatus::InUse => 0,
            AssetStatus::Disposed => 1,
            AssetStatus::Transferred => 2,
            AssetStatus::Maintenance => 3,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum DepreciationMethod {
    StraightLine,
    DecliningBalance,
}

impl DepreciationMethod {
    pub fn all() -> Vec<DepreciationMethod> {
        vec![
            DepreciationMethod::StraightLine,
            DepreciationMethod::DecliningBalance,
        ]
    }

    pub fn i18n_key(&self) -> &str {
        match self {
            DepreciationMethod::StraightLine => "depreciation.straight_line",
            DepreciationMethod::DecliningBalance => "depreciation.declining_balance",
        }
    }

    pub fn from_index(i: usize) -> Self {
        match i {
            0 => DepreciationMethod::StraightLine,
            _ => DepreciationMethod::DecliningBalance,
        }
    }

    pub fn to_index(&self) -> usize {
        match self {
            DepreciationMethod::StraightLine => 0,
            DepreciationMethod::DecliningBalance => 1,
        }
    }
}
