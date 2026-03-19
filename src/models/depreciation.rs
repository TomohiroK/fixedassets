use rust_decimal::Decimal;
use super::asset::{Asset, DepreciationMethod};

#[derive(Clone, Debug)]
pub struct DepreciationScheduleRow {
    pub year: u32,
    pub opening_value: Decimal,
    pub expense: Decimal,
    pub closing_value: Decimal,
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
    let annual_expense = depreciable_amount / Decimal::from(asset.useful_life);
    let mut rows = Vec::new();
    let mut opening = asset.cost;

    for year in 1..=asset.useful_life {
        let expense = if year == asset.useful_life {
            opening - asset.salvage_value
        } else {
            annual_expense.round_dp(2)
        };
        let closing = (opening - expense).max(asset.salvage_value);

        rows.push(DepreciationScheduleRow {
            year,
            opening_value: opening.round_dp(2),
            expense: expense.round_dp(2),
            closing_value: closing.round_dp(2),
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

        rows.push(DepreciationScheduleRow {
            year,
            opening_value: opening.round_dp(2),
            expense: expense.round_dp(2),
            closing_value: closing.round_dp(2),
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
