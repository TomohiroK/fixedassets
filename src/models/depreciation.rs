use chrono::NaiveDate;
use chrono::Datelike;
use rust_decimal::Decimal;
use std::str::FromStr;
use super::asset::{Asset, AssetStatus, Category, DepreciationMethod};
use super::company::{AseanCountry, CompanySetup};
use super::country_rules::CountryRules;

#[derive(Clone, Debug)]
pub struct DepreciationScheduleRow {
    pub year: u32,
    pub opening_value: Decimal,
    pub expense: Decimal,
    pub closing_value: Decimal,
    pub is_prior: bool,
    /// Label for this row (e.g., "IA+AA" for capital allowance year 1)
    pub label: Option<String>,
}

/// Get the current country rules from company setup
fn current_rules() -> CountryRules {
    CompanySetup::load()
        .and_then(|s| s.country())
        .map(|c| CountryRules::for_country(&c))
        .unwrap_or_else(|| CountryRules::for_country(&AseanCountry::Japan))
}

fn current_country() -> AseanCountry {
    CompanySetup::load()
        .and_then(|s| s.country())
        .unwrap_or(AseanCountry::Japan)
}

/// Returns true if the asset category is non-depreciable
pub fn is_non_depreciable(category: &Category) -> bool {
    current_rules().is_non_depreciable(category)
}

/// Returns true if the category is an intangible asset
pub fn is_intangible(category: &Category) -> bool {
    current_rules().is_intangible(category)
}

/// Returns true if the category can use declining balance method
pub fn can_use_declining_balance(category: &Category) -> bool {
    current_rules().can_use_declining_balance(category)
}

/// Get suggested useful life for the current country + category
pub fn suggested_useful_life(category: &Category) -> Option<u32> {
    current_rules().suggested_useful_life(category)
}

/// Get the effective salvage value for calculation
fn effective_salvage_value(asset: &Asset) -> Decimal {
    current_rules().effective_salvage(&asset.category, asset.salvage_value)
}

/// Get the effective total cost (original + CapEx)
fn effective_cost(asset: &Asset) -> Decimal {
    asset.total_cost()
}

/// Get the effective depreciation method
fn effective_method(asset: &Asset) -> DepreciationMethod {
    current_rules().effective_method(&asset.category, &asset.depreciation_method)
}

pub fn calculate_schedule(asset: &Asset) -> Vec<DepreciationScheduleRow> {
    if is_non_depreciable(&asset.category) {
        return vec![];
    }
    let cost = effective_cost(asset);
    if asset.useful_life == 0 || cost <= Decimal::ZERO {
        return vec![];
    }

    let rules = current_rules();
    let country = current_country();
    let salvage = effective_salvage_value(asset);

    if cost <= salvage {
        return vec![];
    }

    // Capital Allowance countries (SG, MY) use a different calculation
    if rules.is_capital_allowance {
        return capital_allowance_schedule(asset, &country, &rules);
    }

    // Indonesia: use group-based rates
    if matches!(country, AseanCountry::Indonesia) {
        return indonesia_schedule(asset, salvage);
    }

    match effective_method(asset) {
        DepreciationMethod::StraightLine => straight_line_schedule(asset, salvage),
        DepreciationMethod::DecliningBalance => {
            match country {
                AseanCountry::Japan => japan_declining_balance_schedule(asset, salvage),
                _ => standard_declining_balance_schedule(asset, salvage, &country),
            }
        }
    }
}

// ============================================================
// Straight-Line (universal)
// 年間償却費 = 取得原価 × 償却率 (償却率 = 1 / 耐用年数)
// ============================================================
fn straight_line_schedule(asset: &Asset, salvage: Decimal) -> Vec<DepreciationScheduleRow> {
    let cost = effective_cost(asset);
    let depreciation_rate = Decimal::ONE / Decimal::from(asset.useful_life);
    let annual_expense = (cost * depreciation_rate).round_dp(2);
    let prior_months = asset.prior_months_total();
    let depreciable_amount = cost - salvage;

    let mut rows = Vec::new();
    let mut opening = cost;
    let mut accumulated = Decimal::ZERO;

    for year in 1..=asset.useful_life {
        let remaining = depreciable_amount - accumulated;
        let expense = if remaining <= Decimal::ZERO {
            Decimal::ZERO
        } else if year == asset.useful_life || annual_expense >= remaining {
            remaining.round_dp(2)
        } else {
            annual_expense
        };

        let closing = (opening - expense).max(salvage).round_dp(2);

        let year_end_months = year * 12;
        let is_prior = year_end_months <= prior_months;

        rows.push(DepreciationScheduleRow {
            year,
            opening_value: opening.round_dp(2),
            expense: expense.round_dp(2),
            closing_value: closing,
            is_prior,
            label: None,
        });

        accumulated += expense;
        opening = closing;
    }

    rows
}

// ============================================================
// Japan: 200% Declining Balance with Guarantee Amount
// 年間償却費 = 期首帳簿価額 × 償却率
// When expense < guarantee amount, switch to revised SL
// ============================================================
fn japan_declining_balance_schedule(asset: &Asset, salvage: Decimal) -> Vec<DepreciationScheduleRow> {
    let cost = effective_cost(asset);
    let rate = Decimal::from(2) / Decimal::from(asset.useful_life);
    let guarantee_rate = japan_guarantee_rate(asset.useful_life);
    let guarantee_amount = (cost * guarantee_rate).round_dp(2);
    let prior_months = asset.prior_months_total();

    let mut rows = Vec::new();
    let mut opening = cost;
    let mut switched_to_sl = false;
    let mut revised_annual: Decimal = Decimal::ZERO;

    for year in 1..=asset.useful_life {
        let expense;

        if switched_to_sl {
            let remaining = (opening - salvage).max(Decimal::ZERO);
            expense = if year == asset.useful_life {
                remaining.round_dp(2)
            } else {
                revised_annual.min(remaining).round_dp(2)
            };
        } else {
            let db_expense = (opening * rate).round_dp(2);

            if db_expense < guarantee_amount && year < asset.useful_life {
                switched_to_sl = true;
                let remaining_years = asset.useful_life - year + 1;
                let remaining_amount = (opening - salvage).max(Decimal::ZERO);
                revised_annual = (remaining_amount / Decimal::from(remaining_years)).round_dp(2);
                expense = revised_annual;
            } else if year == asset.useful_life {
                expense = (opening - salvage).max(Decimal::ZERO).round_dp(2);
            } else {
                let capped = if opening - db_expense < salvage {
                    (opening - salvage).max(Decimal::ZERO).round_dp(2)
                } else {
                    db_expense
                };
                expense = capped;
            }
        }

        let closing = (opening - expense).max(salvage).round_dp(2);
        let year_end_months = year * 12;
        let is_prior = year_end_months <= prior_months;

        rows.push(DepreciationScheduleRow {
            year,
            opening_value: opening.round_dp(2),
            expense: expense.round_dp(2),
            closing_value: closing,
            is_prior,
            label: if switched_to_sl && expense == revised_annual {
                Some("Revised SL".to_string())
            } else {
                None
            },
        });

        opening = closing;
    }

    rows
}

/// Japan guarantee rate (保証率) by useful life
fn japan_guarantee_rate(useful_life: u32) -> Decimal {
    let rate_str = match useful_life {
        2 => "0.00000",
        3 => "0.02789",
        4 => "0.05274",
        5 => "0.06249",
        6 => "0.05776",
        7 => "0.05496",
        8 => "0.05111",
        9 => "0.04731",
        10 => "0.04448",
        11 => "0.04123",
        12 => "0.03870",
        13 => "0.03633",
        14 => "0.03389",
        15 => "0.03217",
        16 => "0.03063",
        17 => "0.02905",
        18 => "0.02757",
        19 => "0.02616",
        20 => "0.02517",
        21 => "0.02408",
        22 => "0.02310",
        23 => "0.02216",
        24 => "0.02126",
        25 => "0.02069",
        26 => "0.01997",
        27 => "0.01927",
        28 => "0.01866",
        29 => "0.01803",
        30 => "0.01766",
        31 => "0.01688",
        32 => "0.01655",
        33 => "0.01585",
        34 => "0.01555",
        35 => "0.01532",
        36 => "0.01473",
        37 => "0.01440",
        38 => "0.01413",
        39 => "0.01370",
        40 => "0.01354",
        41 => "0.01305",
        42 => "0.01281",
        43 => "0.01248",
        44 => "0.01226",
        45 => "0.01210",
        46 => "0.01175",
        47 => "0.01153",
        48 => "0.01126",
        49 => "0.01109",
        50 => "0.01097",
        _ => {
            if useful_life <= 2 {
                return Decimal::ZERO;
            }
            // Interpolation for unlisted values
            let approx = 1.0 / (useful_life as f64).powf(1.5);
            return Decimal::from_str(&format!("{:.5}", approx))
                .unwrap_or(Decimal::ZERO);
        }
    };
    Decimal::from_str(rate_str).unwrap_or(Decimal::ZERO)
}

// ============================================================
// Standard Declining Balance (TH, PH, VN, etc.)
// No guarantee amount switching. Simple DB with floor at salvage.
// ============================================================
fn standard_declining_balance_schedule(asset: &Asset, salvage: Decimal, country: &AseanCountry) -> Vec<DepreciationScheduleRow> {
    let cost = effective_cost(asset);
    let rate = db_rate_for_country(asset, country);
    let prior_months = asset.prior_months_total();

    let mut rows = Vec::new();
    let mut opening = cost;

    for year in 1..=asset.useful_life {
        let db_expense = (opening * rate).round_dp(2);

        let expense = if year == asset.useful_life {
            // Last year: write down to salvage
            (opening - salvage).max(Decimal::ZERO).round_dp(2)
        } else if opening - db_expense < salvage {
            (opening - salvage).max(Decimal::ZERO).round_dp(2)
        } else {
            db_expense
        };

        let closing = (opening - expense).max(salvage).round_dp(2);
        let year_end_months = year * 12;
        let is_prior = year_end_months <= prior_months;

        rows.push(DepreciationScheduleRow {
            year,
            opening_value: opening.round_dp(2),
            expense: expense.round_dp(2),
            closing_value: closing,
            is_prior,
            label: None,
        });

        opening = closing;
    }

    rows
}

/// Get the DB rate for non-Japan countries
fn db_rate_for_country(asset: &Asset, country: &AseanCountry) -> Decimal {
    match country {
        // Vietnam: 150% or 200% DB depending on useful life
        AseanCountry::Vietnam => {
            if asset.useful_life <= 4 {
                Decimal::from_str("1.5").unwrap() / Decimal::from(asset.useful_life)
            } else {
                Decimal::from(2) / Decimal::from(asset.useful_life)
            }
        }
        // Thailand/Philippines/Laos: 200% DDB
        _ => Decimal::from(2) / Decimal::from(asset.useful_life),
    }
}

// ============================================================
// Indonesia: Group-based depreciation rates (PMK-72/2023)
// ============================================================
fn indonesia_schedule(asset: &Asset, salvage: Decimal) -> Vec<DepreciationScheduleRow> {
    let (sl_rate, db_rate) = indonesia_rates(&asset.category, asset.useful_life);

    match effective_method(asset) {
        DepreciationMethod::StraightLine => {
            indonesia_sl_schedule(asset, salvage, sl_rate)
        }
        DepreciationMethod::DecliningBalance => {
            if let Some(dbr) = db_rate {
                indonesia_db_schedule(asset, salvage, dbr)
            } else {
                // Buildings: SL only
                indonesia_sl_schedule(asset, salvage, sl_rate)
            }
        }
    }
}

/// Get Indonesia SL/DB rates based on category
fn indonesia_rates(category: &Category, useful_life: u32) -> (Decimal, Option<Decimal>) {
    // Buildings: SL only
    if matches!(category, Category::Building | Category::Structures) {
        let rate = if useful_life <= 10 {
            Decimal::from_str("0.10").unwrap() // Non-permanent: 10%
        } else {
            Decimal::from_str("0.05").unwrap() // Permanent: 5%
        };
        return (rate, None);
    }

    // Non-building assets: group-based
    match useful_life {
        1..=4 => (
            Decimal::from_str("0.25").unwrap(),   // Group 1: 25% SL
            Some(Decimal::from_str("0.50").unwrap()), // 50% DB
        ),
        5..=8 => (
            Decimal::from_str("0.125").unwrap(),  // Group 2: 12.5% SL
            Some(Decimal::from_str("0.25").unwrap()), // 25% DB
        ),
        9..=16 => (
            Decimal::from_str("0.0625").unwrap(), // Group 3: 6.25% SL
            Some(Decimal::from_str("0.125").unwrap()), // 12.5% DB
        ),
        _ => (
            Decimal::from_str("0.05").unwrap(),   // Group 4: 5% SL
            Some(Decimal::from_str("0.10").unwrap()), // 10% DB
        ),
    }
}

fn indonesia_sl_schedule(asset: &Asset, salvage: Decimal, rate: Decimal) -> Vec<DepreciationScheduleRow> {
    let cost = effective_cost(asset);
    let annual_expense = (cost * rate).round_dp(2);
    let depreciable_amount = cost - salvage;
    let prior_months = asset.prior_months_total();

    let mut rows = Vec::new();
    let mut opening = cost;
    let mut accumulated = Decimal::ZERO;

    for year in 1..=asset.useful_life {
        let remaining = depreciable_amount - accumulated;
        let expense = if remaining <= Decimal::ZERO {
            Decimal::ZERO
        } else if year == asset.useful_life || annual_expense >= remaining {
            remaining.round_dp(2)
        } else {
            annual_expense
        };

        let closing = (opening - expense).max(salvage).round_dp(2);
        let year_end_months = year * 12;
        let is_prior = year_end_months <= prior_months;

        rows.push(DepreciationScheduleRow {
            year,
            opening_value: opening.round_dp(2),
            expense: expense.round_dp(2),
            closing_value: closing,
            is_prior,
            label: None,
        });

        accumulated += expense;
        opening = closing;
    }

    rows
}

fn indonesia_db_schedule(asset: &Asset, salvage: Decimal, rate: Decimal) -> Vec<DepreciationScheduleRow> {
    let cost = effective_cost(asset);
    let prior_months = asset.prior_months_total();

    let mut rows = Vec::new();
    let mut opening = cost;

    for year in 1..=asset.useful_life {
        let db_expense = (opening * rate).round_dp(2);

        let expense = if year == asset.useful_life {
            // Indonesia: remaining book value is expensed in final year (salvage = 0)
            (opening - salvage).max(Decimal::ZERO).round_dp(2)
        } else if opening - db_expense < salvage {
            (opening - salvage).max(Decimal::ZERO).round_dp(2)
        } else {
            db_expense
        };

        let closing = (opening - expense).max(salvage).round_dp(2);
        let year_end_months = year * 12;
        let is_prior = year_end_months <= prior_months;

        rows.push(DepreciationScheduleRow {
            year,
            opening_value: opening.round_dp(2),
            expense: expense.round_dp(2),
            closing_value: closing,
            is_prior,
            label: None,
        });

        opening = closing;
    }

    rows
}

// ============================================================
// Capital Allowance (Singapore, Malaysia)
// Year 1: IA + AA, subsequent years: AA only
// ============================================================
fn capital_allowance_schedule(asset: &Asset, country: &AseanCountry, rules: &CountryRules) -> Vec<DepreciationScheduleRow> {
    let cost = effective_cost(asset);
    let ia_rate = rules.initial_allowance_rate.unwrap_or(Decimal::from_str("0.20").unwrap());
    let aa_rate = capital_allowance_aa_rate(asset, country);
    let prior_months = asset.prior_months_total();

    let ia = (cost * ia_rate).round_dp(2);
    let aa = match country {
        AseanCountry::Singapore => {
            // SG: AA = (Cost - IA) / working life
            ((cost - ia) / Decimal::from(asset.useful_life)).round_dp(2)
        }
        AseanCountry::Malaysia => {
            // MY: AA = Cost × AA rate
            (cost * aa_rate).round_dp(2)
        }
        _ => Decimal::ZERO,
    };

    let mut rows = Vec::new();
    let mut opening = cost;
    let mut accumulated = Decimal::ZERO;

    for year in 1..=asset.useful_life {
        let expense = if year == 1 {
            // Year 1: IA + AA
            let first_year = ia + aa;
            first_year.min(cost - accumulated).round_dp(2)
        } else if year == asset.useful_life {
            // Last year: write off remaining
            (opening).max(Decimal::ZERO).round_dp(2)
        } else {
            let remaining = cost - accumulated;
            aa.min(remaining).round_dp(2)
        };

        let closing = (opening - expense).max(Decimal::ZERO).round_dp(2);
        let year_end_months = year * 12;
        let is_prior = year_end_months <= prior_months;

        rows.push(DepreciationScheduleRow {
            year,
            opening_value: opening.round_dp(2),
            expense: expense.round_dp(2),
            closing_value: closing,
            is_prior,
            label: if year == 1 {
                Some("IA+AA".to_string())
            } else {
                Some("AA".to_string())
            },
        });

        accumulated += expense;
        opening = closing;
    }

    rows
}

/// Get annual allowance rate for Malaysia (varies by category)
fn capital_allowance_aa_rate(asset: &Asset, country: &AseanCountry) -> Decimal {
    match country {
        AseanCountry::Malaysia => {
            match asset.category {
                Category::Machinery => Decimal::from_str("0.20").unwrap(),   // Heavy machinery: 20%
                Category::Vehicles => Decimal::from_str("0.20").unwrap(),    // Motor vehicles: 20%
                Category::Software => Decimal::from_str("0.40").unwrap(),    // Computers/ICT: 40%
                Category::ToolsFixtures => Decimal::from_str("0.10").unwrap(), // Furniture: 10%
                Category::Building => Decimal::from_str("0.03").unwrap(),    // IBA: 3%
                _ => Decimal::from_str("0.14").unwrap(),                     // General plant: 14%
            }
        }
        AseanCountry::Singapore => {
            // SG uses (Cost - IA) / life, not a fixed AA rate
            // This is a fallback; actual calc is done in capital_allowance_schedule
            Decimal::ONE / Decimal::from(asset.useful_life)
        }
        _ => Decimal::from_str("0.14").unwrap(),
    }
}

// ============================================================
// Helper functions
// ============================================================

pub fn accumulated_depreciation(asset: &Asset, years_elapsed: u32) -> Decimal {
    let schedule = calculate_schedule(asset);
    schedule
        .iter()
        .take(years_elapsed as usize)
        .map(|row| row.expense)
        .sum()
}

pub fn current_book_value(asset: &Asset, years_elapsed: u32) -> Decimal {
    (asset.total_cost() - accumulated_depreciation(asset, years_elapsed) - asset.total_impairment()).max(Decimal::ZERO)
}

/// Returns the depreciation expense for the current/next un-depreciated year
pub fn current_year_expense(asset: &Asset, years_elapsed: u32) -> Decimal {
    if is_non_depreciable(&asset.category) {
        return Decimal::ZERO;
    }
    let schedule = calculate_schedule(asset);
    if schedule.is_empty() {
        return Decimal::ZERO;
    }
    let target_year = if years_elapsed == 0 { 1 } else { years_elapsed };
    schedule
        .iter()
        .find(|row| row.year >= target_year && row.expense > Decimal::ZERO)
        .map(|row| row.expense)
        .unwrap_or(Decimal::ZERO)
}

// ============================================================
// Monthly depreciation posting (月次償却処理)
// ============================================================

/// Calculate the monthly depreciation amount for a specific (year, month).
/// Returns ZERO if asset is non-depreciable, disposed, or the period is invalid.
pub fn monthly_depreciation(asset: &Asset, year: u32, month: u32) -> Decimal {
    use rust_decimal::prelude::*;

    if is_non_depreciable(&asset.category) {
        return Decimal::ZERO;
    }
    if asset.status == AssetStatus::Disposed {
        // Check if disposed before target month
        if let Some(ref d) = asset.disposal_date {
            if let Ok(disp) = NaiveDate::parse_from_str(d, "%Y-%m-%d") {
                let disp_ym = disp.year() as u32 * 12 + disp.month();
                let target_ym = year * 12 + month;
                if target_ym > disp_ym {
                    return Decimal::ZERO;
                }
            }
        }
    }

    let schedule = calculate_schedule(asset);
    if schedule.is_empty() {
        return Decimal::ZERO;
    }

    // Parse acquisition date
    let acq_date = match NaiveDate::parse_from_str(&asset.acquisition_date, "%Y-%m-%d") {
        Ok(d) => d,
        Err(_) => return Decimal::ZERO,
    };

    // Effective depreciation start: acquisition month (Japanese convention: 取得月から)
    let acq_ym = acq_date.year() as u32 * 12 + acq_date.month();
    // Adjust for prior depreciation (already depreciated before acquisition by this system)
    let prior_months = asset.prior_months_total();

    let target_ym = year * 12 + month;

    // Target month must be on or after acquisition month
    if target_ym < acq_ym {
        return Decimal::ZERO;
    }

    // Months elapsed since acquisition (0-based: acquisition month = 0)
    let months_since_acq = target_ym - acq_ym;

    // Total months into depreciation (including prior)
    let total_dep_month = prior_months + months_since_acq;

    // Which schedule year does this month fall into? (0-based year index)
    let year_index = total_dep_month / 12;

    // Find the matching schedule row
    let row = match schedule.iter().find(|r| r.year == year_index + 1) {
        Some(r) => r,
        None => return Decimal::ZERO, // Beyond schedule (fully depreciated)
    };

    if row.expense == Decimal::ZERO {
        return Decimal::ZERO;
    }

    // Month position within this schedule year (0-11)
    let month_in_year = total_dep_month % 12;

    // Monthly amount: divide annual by 12, last month gets remainder
    let twelve = Decimal::from(12u32);
    let monthly = (row.expense / twelve).floor();

    if month_in_year == 11 {
        // Last month of the year: assign remainder to avoid rounding drift
        row.expense - monthly * Decimal::from(11u32)
    } else {
        monthly
    }
}

/// Check if an asset is eligible for depreciation posting
pub fn is_postable(asset: &Asset) -> bool {
    if is_non_depreciable(&asset.category) {
        return false;
    }
    if asset.status == AssetStatus::Disposed {
        return false;
    }
    true
}
