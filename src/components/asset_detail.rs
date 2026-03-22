use leptos::prelude::*;
use crate::i18n::use_i18n;
use crate::models::asset::Asset;
use crate::models::depreciation;
use crate::models::department::Department;
use crate::models::accounting_standard::use_accounting_standard;
use crate::components::common::{format_currency, StandardToggle};
use crate::components::photo_uploader::PhotoGallery;

#[component]
pub fn AssetDetailView(asset: Asset) -> impl IntoView {
    let i18n = use_i18n();
    let standard = use_accounting_standard();
    let asset_clone = asset.clone();
    let schedule = depreciation::calculate_schedule(&asset);
    let ifrs_schedule = depreciation::calculate_ifrs_schedule(&asset);
    let status_class = asset.status.badge_class().to_string();
    let status_key = asset.status.i18n_key().to_string();
    let category_key = asset.category.i18n_key().to_string();
    let method_key = asset.depreciation_method.i18n_key().to_string();
    let ifrs_method_key = asset.ifrs_method_effective().i18n_key().to_string();

    let prior_months = asset.prior_months_total();
    let prior_years_full = prior_months / 12;

    let own_years = if let Some(acq) = asset.acquisition_date_parsed() {
        let today = chrono::Utc::now().date_naive();
        let days = (today - acq).num_days();
        (days as f64 / 365.25) as u32
    } else {
        0
    };
    let total_years_elapsed = prior_years_full + own_years;

    // Local (tax) calculations
    let posted_dep = asset.total_posted_depreciation();
    let schedule_dep = depreciation::accumulated_depreciation(&asset, total_years_elapsed);
    let acc_dep_local = if posted_dep > rust_decimal::Decimal::ZERO { posted_dep } else { schedule_dep };
    let book_val_local = asset.total_cost() - acc_dep_local - asset.total_impairment();
    let annual_expense_local = depreciation::current_year_expense(&asset, total_years_elapsed);

    // IFRS calculations
    let posted_ifrs = asset.total_posted_ifrs_depreciation();
    let schedule_ifrs = depreciation::ifrs_accumulated_depreciation(&asset, total_years_elapsed);
    let acc_dep_ifrs = if posted_ifrs > rust_decimal::Decimal::ZERO { posted_ifrs } else { schedule_ifrs };
    let book_val_ifrs = asset.total_cost() - acc_dep_ifrs - asset.total_impairment();
    let annual_expense_ifrs = depreciation::ifrs_current_year_expense(&asset, total_years_elapsed);

    let total_impairment = asset.total_impairment();
    let has_impairment = total_impairment > rust_decimal::Decimal::ZERO;

    let total_capex = asset.total_capex();
    let has_capex = total_capex > rust_decimal::Decimal::ZERO;
    let total_cost = asset.total_cost();

    let cost_str = format_currency(&asset.cost);
    let capex_str = format_currency(&total_capex);
    let total_cost_str = format_currency(&total_cost);
    // Local strings
    let book_val_local_str = format_currency(&book_val_local);
    let acc_dep_local_str = format_currency(&acc_dep_local);
    let salvage_local_str = format_currency(&asset.salvage_value);
    let annual_expense_local_str = format_currency(&annual_expense_local);
    let useful_life_local_str = format!("{}{}", asset.useful_life, i18n.t("asset.years"));
    // IFRS strings
    let book_val_ifrs_str = format_currency(&book_val_ifrs);
    let acc_dep_ifrs_str = format_currency(&acc_dep_ifrs);
    let salvage_ifrs_str = format_currency(&asset.ifrs_salvage_value_effective());
    let annual_expense_ifrs_str = format_currency(&annual_expense_ifrs);
    let useful_life_ifrs_str = format!("{}{}", asset.ifrs_useful_life_effective(), i18n.t("asset.years"));

    let impairment_str = format_currency(&total_impairment);
    let depreciation_done_local = annual_expense_local == rust_decimal::Decimal::ZERO;
    let depreciation_done_ifrs = annual_expense_ifrs == rust_decimal::Decimal::ZERO;
    let acq_date = asset.acquisition_date.clone();
    let method_val = i18n.t(&method_key);
    let ifrs_method_val = i18n.t(&ifrs_method_key);
    let location = asset.location.clone();
    let description = asset.description.clone();
    let has_location = !asset.location.is_empty();
    let has_description = !asset.description.is_empty();
    let has_department = asset.department_id.is_some();
    let department_name = asset.department_id.as_ref()
        .map(|id| Department::display_name(id))
        .unwrap_or_default();
    let has_prior = prior_months > 0;
    let tags = asset.tags.clone();
    let has_tags = !tags.is_empty();
    let prior_str = if asset.prior_depreciation_months > 0 {
        format!("{}{} {}{}", asset.prior_depreciation_years, i18n.t("asset.years"), asset.prior_depreciation_months, i18n.t("asset.months"))
    } else {
        format!("{}{}", asset.prior_depreciation_years, i18n.t("asset.years"))
    };

    let asset_id_for_photos = asset.id.clone();

    // Collapsible section states
    let show_photos = RwSignal::new(false);
    let show_financials = RwSignal::new(false);
    let show_details = RwSignal::new(false);
    let show_schedule = RwSignal::new(false);

    view! {
        <div class="space-y-3">
            // Header: name + status + category + tags in one compact card
            <div class="card py-3">
                <div class="flex items-center justify-between">
                    <div class="flex-1 min-w-0 mr-2">
                        {if !asset.asset_number.is_empty() {
                            let num = asset.asset_number.clone();
                            Some(view! { <p class="text-[10px] text-gray-400 font-mono">{num}</p> })
                        } else {
                            None
                        }}
                        <h2 class="text-lg font-bold text-gray-900 truncate">{asset.name.clone()}</h2>
                        <p class="text-xs text-gray-500">{move || i18n.t(&category_key)}</p>
                    </div>
                    <span class=format!("{} shrink-0", status_class)>{move || i18n.t(&status_key)}</span>
                </div>
                {if has_tags {
                    let tags = tags.clone();
                    Some(view! {
                        <div class="flex flex-wrap gap-1 mt-2">
                            {tags.into_iter().map(|tag| {
                                view! { <span class="text-[10px] bg-blue-50 text-blue-600 px-2 py-0.5 rounded-full">{tag}</span> }
                            }).collect::<Vec<_>>()}
                        </div>
                    })
                } else {
                    None
                }}
            </div>

            // Standard toggle + Primary info
            <div class="card py-3">
                <div class="flex justify-end mb-2">
                    <StandardToggle />
                </div>
                <div class="grid grid-cols-2 gap-4">
                    <div class="text-center">
                        <p class="text-[10px] text-gray-400 uppercase tracking-wide">{move || i18n.t("asset.book_value")}</p>
                        <p class="text-xl font-bold text-blue-600 mt-0.5">
                            {let local = book_val_local_str.clone(); let ifrs = book_val_ifrs_str.clone(); move || if standard.is_ifrs() { ifrs.clone() } else { local.clone() }}
                        </p>
                    </div>
                    <div class="text-center">
                        <p class="text-[10px] text-gray-400 uppercase tracking-wide">{move || i18n.t("depreciation.annual_expense")}</p>
                        {let local_str = annual_expense_local_str.clone(); let ifrs_str = annual_expense_ifrs_str.clone();
                        move || {
                            let done = if standard.is_ifrs() { depreciation_done_ifrs } else { depreciation_done_local };
                            if done {
                                view! { <p class="text-sm font-bold text-green-600 mt-1">{i18n.t("depreciation.fully_depreciated")}</p> }.into_any()
                            } else {
                                let val = if standard.is_ifrs() { ifrs_str.clone() } else { local_str.clone() };
                                view! { <p class="text-xl font-bold text-red-600 mt-0.5">{val}</p> }.into_any()
                            }
                        }}
                    </div>
                </div>
            </div>

            // Collapsible: Photos
            <div class="card py-0 overflow-hidden">
                <button
                    class="w-full flex items-center justify-between py-3 active:bg-gray-50"
                    on:click=move |_| show_photos.update(|v| *v = !*v)
                >
                    <div class="flex items-center gap-2">
                        <svg class="w-4 h-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"/>
                        </svg>
                        <span class="text-sm font-semibold text-gray-900">{move || i18n.t("photo.title")}</span>
                    </div>
                    <svg
                        class=move || format!("w-4 h-4 text-gray-400 transition-transform {}", if show_photos.get() { "rotate-180" } else { "" })
                        fill="none" stroke="currentColor" viewBox="0 0 24 24"
                    >
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/>
                    </svg>
                </button>
                {move || if show_photos.get() {
                    let aid = asset_id_for_photos.clone();
                    view! {
                        <div class="pb-3 border-t border-gray-100 pt-2">
                            <PhotoGallery asset_id=aid editable=true />
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }}
            </div>

            // Collapsible: Financial breakdown
            <div class="card py-0 overflow-hidden">
                <button
                    class="w-full flex items-center justify-between py-3 active:bg-gray-50"
                    on:click=move |_| show_financials.update(|v| *v = !*v)
                >
                    <span class="text-sm font-semibold text-gray-900">{move || i18n.t("asset.financial_summary")}</span>
                    <svg
                        class=move || format!("w-4 h-4 text-gray-400 transition-transform {}", if show_financials.get() { "rotate-180" } else { "" })
                        fill="none" stroke="currentColor" viewBox="0 0 24 24"
                    >
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/>
                    </svg>
                </button>
                {move || if show_financials.get() {
                    let cost_str = cost_str.clone();
                    let capex_str = capex_str.clone();
                    let total_cost_str = total_cost_str.clone();
                    let acc_dep_l = acc_dep_local_str.clone();
                    let acc_dep_i = acc_dep_ifrs_str.clone();
                    let bv_l = book_val_local_str.clone();
                    let bv_i = book_val_ifrs_str.clone();
                    let sv_l = salvage_local_str.clone();
                    let sv_i = salvage_ifrs_str.clone();
                    let imp_str = impairment_str.clone();
                    view! {
                        <div class="pb-3 space-y-2 border-t border-gray-100 pt-2">
                            <CompactRow label=Signal::derive(move || i18n.t("asset.cost")) value=cost_str.clone() />
                            {if has_capex {
                                let cx_str = capex_str.clone();
                                let tc_str = total_cost_str.clone();
                                Some(view! {
                                    <CompactRow label=Signal::derive(move || i18n.t("asset.capex_total")) value=format!("+{}", cx_str) />
                                    <div class="flex justify-between items-center px-0 border-t border-gray-100 pt-1">
                                        <span class="text-xs text-gray-500 font-medium">{move || i18n.t("asset.capex_new_total")}</span>
                                        <span class="text-xs font-bold text-gray-900">{tc_str.clone()}</span>
                                    </div>
                                })
                            } else {
                                None
                            }}
                            <CompactRow
                                label=Signal::derive(move || i18n.t("asset.accumulated_depreciation"))
                                value={let l = acc_dep_l.clone(); let i = acc_dep_i.clone(); if standard.is_ifrs() { i } else { l }}
                            />
                            {if has_impairment {
                                let imp = imp_str.clone();
                                Some(view! { <CompactRow label=Signal::derive(move || i18n.t("asset.impairment_total")) value=format!("-{}", imp) /> })
                            } else {
                                None
                            }}
                            <CompactRow
                                label=Signal::derive(move || i18n.t("asset.book_value"))
                                value={let l = bv_l.clone(); let i = bv_i.clone(); if standard.is_ifrs() { i } else { l }}
                            />
                            <CompactRow
                                label=Signal::derive(move || i18n.t("asset.salvage_value"))
                                value={let l = sv_l.clone(); let i = sv_i.clone(); if standard.is_ifrs() { i } else { l }}
                            />
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }}
            </div>

            // Collapsible: Asset details
            <div class="card py-0 overflow-hidden">
                <button
                    class="w-full flex items-center justify-between py-3 active:bg-gray-50"
                    on:click=move |_| show_details.update(|v| *v = !*v)
                >
                    <span class="text-sm font-semibold text-gray-900">{move || i18n.t("asset.info")}</span>
                    <svg
                        class=move || format!("w-4 h-4 text-gray-400 transition-transform {}", if show_details.get() { "rotate-180" } else { "" })
                        fill="none" stroke="currentColor" viewBox="0 0 24 24"
                    >
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/>
                    </svg>
                </button>
                {move || if show_details.get() {
                    let acq_date = acq_date.clone();
                    let ul_local = useful_life_local_str.clone();
                    let ul_ifrs = useful_life_ifrs_str.clone();
                    let m_local = method_val.clone();
                    let m_ifrs = ifrs_method_val.clone();
                    let prior_str = prior_str.clone();
                    let location = location.clone();
                    let description = description.clone();
                    view! {
                        <div class="pb-3 space-y-2 border-t border-gray-100 pt-2">
                            <CompactRow label=Signal::derive(move || i18n.t("asset.acquisition_date")) value=acq_date.clone() />
                            <CompactRow
                                label=Signal::derive(move || i18n.t("asset.useful_life"))
                                value={let l = ul_local.clone(); let i = ul_ifrs.clone(); if standard.is_ifrs() { i } else { l }}
                            />
                            <CompactRow
                                label=Signal::derive(move || i18n.t("asset.depreciation_method"))
                                value={let l = m_local.clone(); let i = m_ifrs.clone(); if standard.is_ifrs() { i } else { l }}
                            />
                            {if has_prior {
                                let prior_str = prior_str.clone();
                                Some(view! { <CompactRow label=Signal::derive(move || i18n.t("asset.prior_depreciation")) value=prior_str.clone() /> })
                            } else {
                                None
                            }}
                            {if has_department {
                                let dept = department_name.clone();
                                Some(view! { <CompactRow label=Signal::derive(move || i18n.t("asset.department")) value=dept.clone() /> })
                            } else {
                                None
                            }}
                            {if has_location {
                                let location = location.clone();
                                Some(view! { <CompactRow label=Signal::derive(move || i18n.t("asset.location")) value=location.clone() /> })
                            } else {
                                None
                            }}
                            {if has_description {
                                let description = description.clone();
                                Some(view! { <CompactRow label=Signal::derive(move || i18n.t("asset.description")) value=description.clone() /> })
                            } else {
                                None
                            }}
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }}
            </div>

            // Collapsible: Depreciation schedule
            {if !schedule.is_empty() {
                Some(view! {
                    <div class="card py-0 overflow-hidden">
                        <button
                            class="w-full flex items-center justify-between py-3 active:bg-gray-50"
                            on:click=move |_| show_schedule.update(|v| *v = !*v)
                        >
                            <span class="text-sm font-semibold text-gray-900">{move || i18n.t("depreciation.schedule")}</span>
                            <svg
                                class=move || format!("w-4 h-4 text-gray-400 transition-transform {}", if show_schedule.get() { "rotate-180" } else { "" })
                                fill="none" stroke="currentColor" viewBox="0 0 24 24"
                            >
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/>
                            </svg>
                        </button>
                        {move || if show_schedule.get() {
                            let schedule = if standard.is_ifrs() {
                                depreciation::calculate_ifrs_schedule(&asset_clone)
                            } else {
                                depreciation::calculate_schedule(&asset_clone)
                            };
                            view! {
                                <div class="pb-3 border-t border-gray-100 pt-2">
                                    <div class="overflow-x-auto -mx-4 px-4">
                                        <table class="w-full text-xs">
                                            <thead>
                                                <tr class="border-b border-gray-200">
                                                    <th class="text-left py-1.5 text-gray-400 font-medium">{move || i18n.t("depreciation.year")}</th>
                                                    <th class="text-right py-1.5 text-gray-400 font-medium">{move || i18n.t("depreciation.opening_value")}</th>
                                                    <th class="text-right py-1.5 text-gray-400 font-medium">{move || i18n.t("depreciation.expense")}</th>
                                                    <th class="text-right py-1.5 text-gray-400 font-medium">{move || i18n.t("depreciation.closing_value")}</th>
                                                </tr>
                                            </thead>
                                            <tbody>
                                                {schedule.into_iter().map(|row| {
                                                    let row_class = if row.is_prior {
                                                        "bg-gray-50 text-gray-400"
                                                    } else if row.year == total_years_elapsed {
                                                        "bg-blue-50"
                                                    } else {
                                                        ""
                                                    };
                                                    let label_text = row.label.clone().unwrap_or_default();
                                                    let has_label = !label_text.is_empty();
                                                    view! {
                                                        <tr class=format!("border-b border-gray-100 {}", row_class)>
                                                            <td class="py-1.5">
                                                                {row.year} {if row.is_prior { " *" } else { "" }}
                                                                {if has_label {
                                                                    Some(view! { <span class="ml-1 text-[9px] text-blue-500 font-medium">{label_text}</span> })
                                                                } else {
                                                                    None
                                                                }}
                                                            </td>
                                                            <td class="text-right py-1.5">{format_currency(&row.opening_value)}</td>
                                                            <td class="text-right py-1.5 text-orange-600">{format_currency(&row.expense)}</td>
                                                            <td class="text-right py-1.5">{format_currency(&row.closing_value)}</td>
                                                        </tr>
                                                    }
                                                }).collect::<Vec<_>>()}
                                            </tbody>
                                        </table>
                                    </div>
                                </div>
                            }.into_any()
                        } else {
                            view! { <div></div> }.into_any()
                        }}
                    </div>
                })
            } else {
                None
            }}
        </div>
    }
}

#[component]
fn CompactRow(
    label: Signal<String>,
    #[prop(into)] value: String,
) -> impl IntoView {
    view! {
        <div class="flex justify-between items-center px-0">
            <span class="text-xs text-gray-500">{label}</span>
            <span class="text-xs font-medium text-gray-900">{value}</span>
        </div>
    }
}
