use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Custom serde module for Option<Decimal> to ensure string serialization.
/// This prevents JavaScript float precision loss when stored in IndexedDB via JSON::parse.
mod option_decimal_str {
    use rust_decimal::Decimal;
    use serde::{self, Deserialize, Deserializer, Serializer};
    use std::str::FromStr;

    pub fn serialize<S>(value: &Option<Decimal>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(d) => serializer.serialize_str(&d.to_string()),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Accept both string and number formats
        let opt: Option<serde_json::Value> = Option::deserialize(deserializer)?;
        match opt {
            None => Ok(None),
            Some(serde_json::Value::String(s)) => {
                if s.is_empty() {
                    Ok(None)
                } else {
                    Decimal::from_str(&s).map(Some).map_err(serde::de::Error::custom)
                }
            }
            Some(serde_json::Value::Number(n)) => {
                Decimal::from_str(&n.to_string()).map(Some).map_err(serde::de::Error::custom)
            }
            Some(_) => Ok(None),
        }
    }
}

/// Record of a single impairment event
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ImpairmentRecord {
    pub date: String,       // YYYY-MM-DD
    pub amount: Decimal,    // Impairment loss amount
    pub reason: String,     // Reason for impairment
}

/// Record of a capital expenditure (資本的支出)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CapExRecord {
    pub date: String,           // YYYY-MM-DD
    pub amount: Decimal,        // CapEx amount
    pub description: String,    // What was added/improved
}

/// Record of a monthly depreciation posting (月次償却仕訳)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DepreciationPosting {
    pub year: u32,          // Calendar year (e.g. 2026)
    pub month: u32,         // Month 1-12
    pub amount: Decimal,    // Monthly depreciation amount posted
    pub posted_at: String,  // ISO timestamp when the posting was created
}

/// Record of an asset transfer between departments (配置転換)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TransferRecord {
    pub date: String,               // YYYY-MM-DD
    pub from_department_id: Option<String>,  // None = unassigned
    pub to_department_id: String,    // destination department
    pub reason: String,              // reason for transfer
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Asset {
    pub id: String,
    #[serde(default)]
    pub asset_number: String,
    pub name: String,
    pub category: Category,
    pub acquisition_date: String, // YYYY-MM-DD
    pub cost: Decimal,
    pub salvage_value: Decimal,
    pub useful_life: u32, // years
    pub depreciation_method: DepreciationMethod,
    #[serde(default)]
    pub prior_depreciation_years: u32,
    #[serde(default)]
    pub prior_depreciation_months: u32,
    pub location: String,
    pub description: String,
    pub status: AssetStatus,
    #[serde(default)]
    pub tags: Vec<String>,
    /// "disposal" or "sale" — distinguishes 除却 from 売却
    #[serde(default)]
    pub disposal_type: Option<String>,
    /// Sub-type for disposal: "normal", "casualty", "disaster", "theft"
    #[serde(default)]
    pub disposal_sub_type: Option<String>,
    /// Disposal/sale date (YYYY-MM-DD)
    #[serde(default)]
    pub disposal_date: Option<String>,
    /// Proceeds — sale price for 売却, scrap value for 除却
    #[serde(default, with = "option_decimal_str")]
    pub disposal_proceeds: Option<Decimal>,
    /// Reason/note for disposal, or buyer name for sale
    #[serde(default)]
    pub disposal_reason: Option<String>,
    /// Impairment loss records
    #[serde(default)]
    pub impairments: Vec<ImpairmentRecord>,
    /// Capital expenditure records (資本的支出)
    #[serde(default)]
    pub capex_records: Vec<CapExRecord>,
    /// Current department id
    #[serde(default)]
    pub department_id: Option<String>,
    /// Department transfer history (配置転換履歴)
    #[serde(default)]
    pub transfers: Vec<TransferRecord>,
    /// Depreciation posting history (月次償却仕訳履歴)
    #[serde(default)]
    pub postings: Vec<DepreciationPosting>,
    /// Company ID for multi-tenant isolation
    #[serde(default)]
    pub company_id: String,
    // ---- IFRS dual-book fields ----
    /// IFRS useful life (may differ from local/tax useful life)
    #[serde(default)]
    pub ifrs_useful_life: Option<u32>,
    /// IFRS salvage value (serialized as string to avoid JS float precision loss)
    #[serde(default, with = "option_decimal_str")]
    pub ifrs_salvage_value: Option<Decimal>,
    /// IFRS depreciation method (StraightLine, DecliningBalance, UnitsOfProduction)
    #[serde(default)]
    pub ifrs_method: Option<String>,
    /// IFRS depreciation postings (separate from local/tax postings)
    #[serde(default)]
    pub ifrs_postings: Vec<DepreciationPosting>,
    pub created_at: String,
    pub updated_at: String,
}

impl Asset {
    pub fn new(
        asset_number: String,
        name: String,
        category: Category,
        acquisition_date: String,
        cost: Decimal,
        salvage_value: Decimal,
        useful_life: u32,
        depreciation_method: DepreciationMethod,
        prior_depreciation_years: u32,
        prior_depreciation_months: u32,
        location: String,
        description: String,
        tags: Vec<String>,
    ) -> Self {
        let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string();
        Self {
            id: Uuid::new_v4().to_string(),
            asset_number,
            name,
            category,
            acquisition_date,
            cost,
            salvage_value,
            useful_life,
            depreciation_method,
            prior_depreciation_years,
            prior_depreciation_months,
            location,
            description,
            status: AssetStatus::InUse,
            tags,
            disposal_type: None,
            disposal_sub_type: None,
            disposal_date: None,
            disposal_proceeds: None,
            disposal_reason: None,
            impairments: Vec::new(),
            capex_records: Vec::new(),
            department_id: None,
            transfers: Vec::new(),
            postings: Vec::new(),
            company_id: String::new(),
            ifrs_useful_life: None,
            ifrs_salvage_value: None,
            ifrs_method: None,
            ifrs_postings: Vec::new(),
            created_at: now.clone(),
            updated_at: now,
        }
    }

    /// Total prior depreciation in months
    pub fn prior_months_total(&self) -> u32 {
        self.prior_depreciation_years * 12 + self.prior_depreciation_months
    }

    /// Total cumulative impairment loss
    pub fn total_impairment(&self) -> Decimal {
        self.impairments.iter().map(|r| r.amount).sum()
    }

    /// Total capital expenditure amount
    pub fn total_capex(&self) -> Decimal {
        self.capex_records.iter().map(|r| r.amount).sum()
    }

    /// Total cost including CapEx (used for depreciation calculation)
    pub fn total_cost(&self) -> Decimal {
        self.cost + self.total_capex()
    }

    /// Total posted (accumulated) depreciation from actual postings
    pub fn total_posted_depreciation(&self) -> Decimal {
        self.postings.iter().map(|p| p.amount).sum()
    }

    /// Check if a posting exists for the given period
    pub fn has_posting(&self, year: u32, month: u32) -> bool {
        self.postings.iter().any(|p| p.year == year && p.month == month)
    }

    /// Get the latest posting period (year, month)
    pub fn latest_posting(&self) -> Option<(u32, u32)> {
        self.postings.iter()
            .max_by_key(|p| p.year * 100 + p.month)
            .map(|p| (p.year, p.month))
    }

    pub fn acquisition_date_parsed(&self) -> Option<NaiveDate> {
        NaiveDate::parse_from_str(&self.acquisition_date, "%Y-%m-%d").ok()
    }

    // ---- IFRS helpers ----

    /// Check if this asset has any IFRS-specific settings configured
    pub fn has_ifrs_settings(&self) -> bool {
        self.ifrs_useful_life.is_some()
            || self.ifrs_salvage_value.is_some()
            || self.ifrs_method.is_some()
    }

    /// Get effective useful life for IFRS (falls back to local if not set or 0)
    pub fn ifrs_useful_life_effective(&self) -> u32 {
        match self.ifrs_useful_life {
            Some(v) if v > 0 => v,
            _ => self.useful_life,
        }
    }

    /// Get effective salvage value for IFRS (falls back to local if not set)
    pub fn ifrs_salvage_value_effective(&self) -> Decimal {
        self.ifrs_salvage_value.unwrap_or(self.salvage_value)
    }

    /// Get effective IFRS depreciation method (falls back to local)
    pub fn ifrs_method_effective(&self) -> DepreciationMethod {
        match self.ifrs_method.as_deref() {
            Some("SL") | Some("StraightLine") => DepreciationMethod::StraightLine,
            Some("DB") | Some("DecliningBalance") => DepreciationMethod::DecliningBalance,
            _ => self.depreciation_method.clone(),
        }
    }

    /// Total posted IFRS depreciation
    pub fn total_posted_ifrs_depreciation(&self) -> Decimal {
        self.ifrs_postings.iter().map(|p| p.amount).sum()
    }

    /// Check if an IFRS posting exists for the given period
    pub fn has_ifrs_posting(&self, year: u32, month: u32) -> bool {
        self.ifrs_postings.iter().any(|p| p.year == year && p.month == month)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Category {
    Land,
    Building,
    BuildingEquipment,
    Structures,
    Machinery,
    ToolsFixtures,
    Vehicles,
    LeasedAssets,
    ConstructionInProgress,
    Patents,
    Trademarks,
    LeaseholdRights,
    Software,
    FacilityRights,
    Other,
}

impl Category {
    pub fn all() -> Vec<Category> {
        vec![
            Category::Land,
            Category::Building,
            Category::BuildingEquipment,
            Category::Structures,
            Category::Machinery,
            Category::ToolsFixtures,
            Category::Vehicles,
            Category::LeasedAssets,
            Category::ConstructionInProgress,
            Category::Patents,
            Category::Trademarks,
            Category::LeaseholdRights,
            Category::Software,
            Category::FacilityRights,
            Category::Other,
        ]
    }

    pub fn i18n_key(&self) -> &str {
        match self {
            Category::Land => "category.land",
            Category::Building => "category.building",
            Category::BuildingEquipment => "category.building_equipment",
            Category::Structures => "category.structures",
            Category::Machinery => "category.machinery",
            Category::ToolsFixtures => "category.tools_fixtures",
            Category::Vehicles => "category.vehicles",
            Category::LeasedAssets => "category.leased_assets",
            Category::ConstructionInProgress => "category.construction_in_progress",
            Category::Patents => "category.patents",
            Category::Trademarks => "category.trademarks",
            Category::LeaseholdRights => "category.leasehold_rights",
            Category::Software => "category.software",
            Category::FacilityRights => "category.facility_rights",
            Category::Other => "category.other",
        }
    }

    pub fn from_index(i: usize) -> Self {
        match i {
            0 => Category::Land,
            1 => Category::Building,
            2 => Category::BuildingEquipment,
            3 => Category::Structures,
            4 => Category::Machinery,
            5 => Category::ToolsFixtures,
            6 => Category::Vehicles,
            7 => Category::LeasedAssets,
            8 => Category::ConstructionInProgress,
            9 => Category::Patents,
            10 => Category::Trademarks,
            11 => Category::LeaseholdRights,
            12 => Category::Software,
            13 => Category::FacilityRights,
            _ => Category::Other,
        }
    }

    pub fn to_index(&self) -> usize {
        match self {
            Category::Land => 0,
            Category::Building => 1,
            Category::BuildingEquipment => 2,
            Category::Structures => 3,
            Category::Machinery => 4,
            Category::ToolsFixtures => 5,
            Category::Vehicles => 6,
            Category::LeasedAssets => 7,
            Category::ConstructionInProgress => 8,
            Category::Patents => 9,
            Category::Trademarks => 10,
            Category::LeaseholdRights => 11,
            Category::Software => 12,
            Category::FacilityRights => 13,
            Category::Other => 14,
        }
    }

    /// Emoji icon for category display
    pub fn emoji(&self) -> &str {
        match self {
            Category::Land => "🏞️",
            Category::Building => "🏢",
            Category::BuildingEquipment => "🔧",
            Category::Structures => "🏗️",
            Category::Machinery => "⚙️",
            Category::ToolsFixtures => "🔨",
            Category::Vehicles => "🚗",
            Category::LeasedAssets => "📋",
            Category::ConstructionInProgress => "🚧",
            Category::Patents => "📜",
            Category::Trademarks => "™️",
            Category::LeaseholdRights => "🔑",
            Category::Software => "💻",
            Category::FacilityRights => "🏭",
            Category::Other => "📦",
        }
    }

    /// Photo image path for category card background
    pub fn image_path(&self) -> &str {
        match self {
            Category::Land => "/images/categories/land.webp",
            Category::Building => "/images/categories/building.webp",
            Category::BuildingEquipment => "/images/categories/building_equipment.webp",
            Category::Structures => "/images/categories/structures.webp",
            Category::Machinery => "/images/categories/machinery.webp",
            Category::ToolsFixtures => "/images/categories/tools_fixtures.webp",
            Category::Vehicles => "/images/categories/vehicles.webp",
            Category::LeasedAssets => "/images/categories/leased_assets.webp",
            Category::ConstructionInProgress => "/images/categories/construction.webp",
            Category::Patents => "/images/categories/patents.webp",
            Category::Trademarks => "/images/categories/trademarks.webp",
            Category::LeaseholdRights => "/images/categories/leasehold_rights.webp",
            Category::Software => "/images/categories/software.webp",
            Category::FacilityRights => "/images/categories/facility_rights.webp",
            Category::Other => "/images/categories/other.webp",
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
