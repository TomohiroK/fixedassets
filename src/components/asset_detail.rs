use leptos::prelude::*;
use crate::i18n::use_i18n;
use crate::models::asset::Asset;
use crate::models::depreciation;
use crate::components::common::format_currency;

#[component]
pub fn AssetDetailView(asset: Asset) -> impl IntoView {
    let i18n = use_i18n();
    let schedule = depreciation::calculate_schedule(&asset);
    let status_class = asset.status.badge_class().to_string();
    let status_key = asset.status.i18n_key().to_string();
    let category_key = asset.category.i18n_key().to_string();
    let method_key = asset.depreciation_method.i18n_key().to_string();

    let years_elapsed = if let Some(acq) = asset.acquisition_date_parsed() {
        let today = chrono::Utc::now().date_naive();
        let days = (today - acq).num_days();
        (days as f64 / 365.25) as u32
    } else {
        0
    };

    let acc_dep = depreciation::accumulated_depreciation(&asset, years_elapsed);
    let book_val = depreciation::current_book_value(&asset, years_elapsed);

    let cost_str = format_currency(&asset.cost);
    let book_val_str = format_currency(&book_val);
    let acc_dep_str = format_currency(&acc_dep);
    let salvage_str = format_currency(&asset.salvage_value);
    let acq_date = asset.acquisition_date.clone();
    let useful_life_str = format!("{} years", asset.useful_life);
    let method_val = i18n.t(&method_key);
    let location = asset.location.clone();
    let description = asset.description.clone();
    let has_location = !asset.location.is_empty();
    let has_description = !asset.description.is_empty();

    view! {
        <div class="space-y-4">
            <div class="card">
                <div class="flex items-start justify-between mb-3">
                    <h2 class="text-xl font-bold text-gray-900">{asset.name.clone()}</h2>
                    <span class=status_class>{move || i18n.t(&status_key)}</span>
                </div>
                <p class="text-sm text-gray-500">{move || i18n.t(&category_key)}</p>
            </div>

            <div class="grid grid-cols-2 gap-3">
                <div class="card text-center">
                    <p class="text-xs text-gray-500 mb-1">{move || i18n.t("asset.cost")}</p>
                    <p class="text-lg font-bold text-gray-900">{cost_str.clone()}</p>
                </div>
                <div class="card text-center">
                    <p class="text-xs text-gray-500 mb-1">{move || i18n.t("asset.book_value")}</p>
                    <p class="text-lg font-bold text-blue-600">{book_val_str.clone()}</p>
                </div>
                <div class="card text-center">
                    <p class="text-xs text-gray-500 mb-1">{move || i18n.t("asset.accumulated_depreciation")}</p>
                    <p class="text-lg font-bold text-orange-600">{acc_dep_str.clone()}</p>
                </div>
                <div class="card text-center">
                    <p class="text-xs text-gray-500 mb-1">{move || i18n.t("asset.salvage_value")}</p>
                    <p class="text-lg font-bold text-gray-600">{salvage_str.clone()}</p>
                </div>
            </div>

            <div class="card space-y-3">
                <DetailRow label=Signal::derive(move || i18n.t("asset.acquisition_date")) value=acq_date.clone() />
                <DetailRow label=Signal::derive(move || i18n.t("asset.useful_life")) value=useful_life_str.clone() />
                <DetailRow label=Signal::derive(move || i18n.t("asset.depreciation_method")) value=method_val.clone() />
                {if has_location {
                    Some(view! { <DetailRow label=Signal::derive(move || i18n.t("asset.location")) value=location.clone() /> })
                } else {
                    None
                }}
                {if has_description {
                    Some(view! { <DetailRow label=Signal::derive(move || i18n.t("asset.description")) value=description.clone() /> })
                } else {
                    None
                }}
            </div>

            {if !schedule.is_empty() {
                Some(view! {
                    <div class="card">
                        <h3 class="font-semibold text-gray-900 mb-3">{move || i18n.t("depreciation.schedule")}</h3>
                        <div class="overflow-x-auto -mx-4 px-4">
                            <table class="w-full text-sm">
                                <thead>
                                    <tr class="border-b border-gray-200">
                                        <th class="text-left py-2 text-gray-500 font-medium">{move || i18n.t("depreciation.year")}</th>
                                        <th class="text-right py-2 text-gray-500 font-medium">{move || i18n.t("depreciation.opening_value")}</th>
                                        <th class="text-right py-2 text-gray-500 font-medium">{move || i18n.t("depreciation.expense")}</th>
                                        <th class="text-right py-2 text-gray-500 font-medium">{move || i18n.t("depreciation.closing_value")}</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {schedule.into_iter().map(|row| {
                                        let row_class = if row.year == years_elapsed { "bg-blue-50" } else { "" };
                                        view! {
                                            <tr class=format!("border-b border-gray-100 {}", row_class)>
                                                <td class="py-2">{row.year}</td>
                                                <td class="text-right py-2">{format_currency(&row.opening_value)}</td>
                                                <td class="text-right py-2 text-orange-600">{format_currency(&row.expense)}</td>
                                                <td class="text-right py-2">{format_currency(&row.closing_value)}</td>
                                            </tr>
                                        }
                                    }).collect::<Vec<_>>()}
                                </tbody>
                            </table>
                        </div>
                    </div>
                })
            } else {
                None
            }}
        </div>
    }
}

#[component]
fn DetailRow(
    label: Signal<String>,
    #[prop(into)] value: String,
) -> impl IntoView {
    view! {
        <div class="flex justify-between items-center py-1">
            <span class="text-sm text-gray-500">{label}</span>
            <span class="text-sm font-medium text-gray-900">{value}</span>
        </div>
    }
}
