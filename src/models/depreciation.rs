use rust_decimal::Decimal;
use super::asset::{Asset, DepreciationMethod};

#[derive(Clone, Debug)]
pub struct DepreciationScheduleRow {
    pub year: u32,
    pub opening_value: Decimal,
    pub expense: Decimal,
    pub closing_value: Decimal,
    pub is_prior: bool,
}

pub fn calculate_schedule(asset: &Asset) -> Vec<DepreciationScheduleRow> {
    match asset.depreciation_method {
        DepreciationMethod::StraightLine => straight_line_schedule(asset),
        DepreciationMethod::DecliningBalance => declining_balance_schedule(asset),
    }
}

fn straight_line_schedule(asset: &Asset) -> Vec<DepreciationScheduleRow> {
    if asset.useful_life == 0 {
        return vec![];
    }

    let depreciable_amount = asset.cost - asset.salvage_value;
    let total_months = asset.useful_life * 12;
    let monthly_expense = depreciable_amount / Decimal::from(total_months);
    let annual_expense = (monthly_expense * Decimal::from(12)).round_dp(2);
    let prior_months = asset.prior_months_total();

    let mut rows = Vec::new();
    let mut opening = asset.cost;

    for year in 1..=asset.useful_life {
        let expense = if year == asset.useful_life {
            opening - asset.salvage_value
        } else {
            annual_expense
        };
        let closing = (opening - expense).max(asset.salvage_value);

        let year_end_months = year * 12;
        let is_prior = year_end_months <= prior_months;

        rows.push(DepreciationScheduleRow {
            year,
            opening_value: opening.round_dp(2),
            expense: expense.round_dp(2),
            closing_value: closing.round_dp(2),
            is_prior,
        });

        opening = closing;
    }

    rows
}

fn declining_balance_schedule(asset: &Asset) -> Vec<DepreciationScheduleRow> {
    if asset.useful_life == 0 {
        return vec![];
    }

    let rate = Decimal::from(2) / Decimal::from(asset.useful_life);
    let prior_months = asset.prior_months_total();
    let mut rows = Vec::new();
    let mut opening = asset.cost;

    for year in 1..=asset.useful_life {
        let mut expense = (opening * rate).round_dp(2);

        if opening - expense < asset.salvage_value {
            expense = opening - asset.salvage_value;
        }

        if year == asset.useful_life {
            expense = opening - asset.salvage_value;
        }

        let closing = (opening - expense).max(asset.salvage_value);

        let year_end_months = year * 12;
        let is_prior = year_end_months <= prior_months;

        rows.push(DepreciationScheduleRow {
            year,
            opening_value: opening.round_dp(2),
            expense: expense.round_dp(2),
            closing_value: closing.round_dp(2),
            is_prior,
        });

        opening = closing;
    }

    rows
}

pub fn accumulated_depreciation(asset: &Asset, years_elapsed: u32) -> Decimal {
    let schedule = calculate_schedule(asset);
    schedule
        .iter()
        .take(years_elapsed as usize)
        .map(|row| row.expense)
        .sum()
}

pub fn current_book_value(asset: &Asset, years_elapsed: u32) -> Decimal {
    asset.cost - accumulated_depreciation(asset, years_elapsed)
}

/// Returns the depreciation expense for the next un-depreciated year
pub fn current_year_expense(asset: &Asset, years_elapsed: u32) -> Decimal {
    let schedule = calculate_schedule(asset);
    // Find the first year that hasn't been fully depreciated yet
    let next_year = years_elapsed + 1;
    schedule
        .iter()
        .find(|row| row.year == next_year)
        .map(|row| row.expense)
        .unwrap_or(Decimal::ZERO)
}
