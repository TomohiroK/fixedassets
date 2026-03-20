use rust_decimal::Decimal;
use std::str::FromStr;
use super::asset::{Category, DepreciationMethod};
use super::company::AseanCountry;

/// Country-specific depreciation rules
#[derive(Clone, Debug)]
pub struct CountryRules {
    pub country: AseanCountry,
    /// Whether this country uses Capital Allowance system instead of depreciation
    pub is_capital_allowance: bool,
    /// Initial Allowance rate (for SG/MY capital allowance)
    pub initial_allowance_rate: Option<Decimal>,
    /// Default salvage value (e.g., 1 JPY for Japan)
    pub default_salvage: Decimal,
    /// Whether Japan's guarantee amount rule applies
    pub has_guarantee_amount: bool,
}

impl CountryRules {
    pub fn for_country(country: &AseanCountry) -> Self {
        match country {
            AseanCountry::Japan => Self {
                country: country.clone(),
                is_capital_allowance: false,
                initial_allowance_rate: None,
                default_salvage: Decimal::ONE, // 1 JPY memorandum value
                has_guarantee_amount: true,
            },
            AseanCountry::Singapore => Self {
                country: country.clone(),
                is_capital_allowance: true,
                initial_allowance_rate: Some(Decimal::from_str("0.20").unwrap()),
                default_salvage: Decimal::ZERO,
                has_guarantee_amount: false,
            },
            AseanCountry::Malaysia => Self {
                country: country.clone(),
                is_capital_allowance: true,
                initial_allowance_rate: Some(Decimal::from_str("0.20").unwrap()),
                default_salvage: Decimal::ZERO,
                has_guarantee_amount: false,
            },
            _ => Self {
                country: country.clone(),
                is_capital_allowance: false,
                initial_allowance_rate: None,
                default_salvage: Decimal::ZERO,
                has_guarantee_amount: false,
            },
        }
    }

    /// Check if a category is non-depreciable for this country
    pub fn is_non_depreciable(&self, category: &Category) -> bool {
        // Land and Construction in Progress are universal
        if matches!(category, Category::Land | Category::ConstructionInProgress) {
            return true;
        }
        match self.country {
            // Japan: leasehold rights generally non-depreciable
            AseanCountry::Japan => matches!(category, Category::LeaseholdRights),
            // Singapore: buildings not qualifying (except IBA)
            AseanCountry::Singapore => matches!(category, Category::Building | Category::BuildingEquipment | Category::Structures | Category::LeaseholdRights),
            _ => matches!(category, Category::LeaseholdRights),
        }
    }

    /// Check if a category is intangible (SL only, salvage=0)
    pub fn is_intangible(&self, category: &Category) -> bool {
        matches!(
            category,
            Category::Patents | Category::Trademarks | Category::Software
                | Category::FacilityRights
        )
    }

    /// Check if declining balance is allowed for this category in this country
    pub fn can_use_declining_balance(&self, category: &Category) -> bool {
        if self.is_non_depreciable(category) || self.is_intangible(category) {
            return false;
        }
        match self.country {
            AseanCountry::Japan => {
                // Post-2016: Buildings, Building Equipment, Structures are SL only
                matches!(category, Category::Machinery | Category::Vehicles
                    | Category::ToolsFixtures | Category::LeasedAssets | Category::Other)
            }
            AseanCountry::Singapore | AseanCountry::Malaysia => false, // Capital allowance, no DB
            AseanCountry::Myanmar => false, // SL only
            AseanCountry::Thailand | AseanCountry::Philippines | AseanCountry::Laos => {
                // Most tangible assets can use DB
                !self.is_intangible(category)
            }
            AseanCountry::Indonesia => {
                // Non-building assets can use DB
                !matches!(category, Category::Building | Category::Structures)
            }
            AseanCountry::Vietnam => {
                // DB available for tangible assets
                !self.is_intangible(category) && !matches!(category, Category::Building)
            }
            AseanCountry::Cambodia => {
                // Classes 2-4 use DB (pooled), Class 1 uses SL
                // Buildings (class 1) are SL only
                !matches!(category, Category::Building | Category::Structures)
            }
            _ => true,
        }
    }

    /// Get the effective depreciation method (force SL if DB not allowed)
    pub fn effective_method(&self, category: &Category, chosen: &DepreciationMethod) -> DepreciationMethod {
        if !self.can_use_declining_balance(category) {
            DepreciationMethod::StraightLine
        } else {
            chosen.clone()
        }
    }

    /// Get the effective salvage value
    pub fn effective_salvage(&self, category: &Category, user_salvage: Decimal) -> Decimal {
        if self.is_intangible(category) {
            // Intangible assets: salvage = 0 (all countries)
            Decimal::ZERO
        } else if self.is_capital_allowance {
            // Capital allowance countries: no salvage concept
            Decimal::ZERO
        } else {
            user_salvage
        }
    }

    /// Get default useful life suggestion for a category
    pub fn suggested_useful_life(&self, category: &Category) -> Option<u32> {
        match self.country {
            AseanCountry::Japan => match category {
                Category::Building => Some(22),
                Category::BuildingEquipment => Some(15),
                Category::Structures => Some(20),
                Category::Machinery => Some(10),
                Category::ToolsFixtures => Some(8),
                Category::Vehicles => Some(6),
                Category::LeasedAssets => Some(5),
                Category::Software => Some(5),
                Category::Patents => Some(8),
                Category::Trademarks => Some(10),
                Category::FacilityRights => Some(5),
                _ => None,
            },
            AseanCountry::Singapore => match category {
                Category::Vehicles => Some(6),
                Category::Machinery => Some(6),
                Category::ToolsFixtures => Some(6),
                _ => Some(6),
            },
            AseanCountry::Thailand => match category {
                Category::Building => Some(20),
                Category::Machinery => Some(5),
                Category::Vehicles => Some(5),
                Category::Software => Some(3),
                Category::Patents | Category::Trademarks => Some(10),
                _ => Some(5),
            },
            AseanCountry::Indonesia => match category {
                Category::Building | Category::Structures => Some(20),
                Category::ToolsFixtures => Some(4),
                Category::Vehicles => Some(8),
                Category::Machinery => Some(8),
                Category::Software => Some(4),
                _ => Some(8),
            },
            AseanCountry::Vietnam => match category {
                Category::Building | Category::Structures => Some(25),
                Category::Machinery => Some(7),
                Category::Vehicles => Some(8),
                Category::Software => Some(5),
                _ => Some(5),
            },
            AseanCountry::Philippines => match category {
                Category::Building => Some(20),
                Category::Vehicles => Some(5),
                Category::Machinery => Some(5),
                Category::Software => Some(5),
                _ => Some(5),
            },
            AseanCountry::Myanmar => match category {
                Category::Building => Some(40),
                Category::Machinery => Some(10),
                Category::Vehicles => Some(5),
                Category::ToolsFixtures => Some(10),
                _ => Some(20),
            },
            AseanCountry::Cambodia => match category {
                Category::Building | Category::Structures => Some(20),
                Category::Machinery => Some(5),
                Category::Vehicles => Some(4),
                Category::ToolsFixtures => Some(4),
                Category::Software => Some(10),
                _ => Some(5),
            },
            AseanCountry::Laos => match category {
                Category::Building => Some(20),
                Category::Machinery => Some(5),
                Category::Vehicles => Some(5),
                Category::Software => Some(5),
                _ => Some(5),
            },
            _ => Some(5),
        }
    }
}

