use leptos::prelude::*;
use rust_decimal::Decimal;
use std::str::FromStr;
use crate::i18n::use_i18n;
use crate::models::asset::*;
use crate::models::department::Department;
use crate::models::depreciation;
use crate::components::photo_uploader::PhotoGallery;

/// Generate sequential asset numbers.
/// "FA-001" + offset 0 → "FA-001", offset 1 → "FA-002", offset 2 → "FA-003"
/// "DESK" + offset 0 → "DESK-1", offset 1 → "DESK-2"
/// "" + offset → ""
fn generate_sequential_number(base: &str, offset: u32) -> String {
    if base.is_empty() {
        return String::new();
    }

    // Find trailing digits
    let num_start = base.rfind(|c: char| !c.is_ascii_digit()).map(|i| i + 1).unwrap_or(0);

    if num_start < base.len() && num_start > 0 {
        // Has trailing digits: "FA-001" → prefix="FA-", num_str="001"
        let prefix = &base[..num_start];
        let num_str = &base[num_start..];
        let width = num_str.len();
        let num: u32 = num_str.parse().unwrap_or(0);
        format!("{}{:0>width$}", prefix, num + offset, width = width)
    } else if num_start == 0 && base.chars().all(|c| c.is_ascii_digit()) {
        // All digits: "001" → "002"
        let width = base.len();
        let num: u32 = base.parse().unwrap_or(0);
        format!("{:0>width$}", num + offset, width = width)
    } else {
        // No trailing digits: "DESK" → "DESK-1", "DESK-2"
        format!("{}-{}", base, offset + 1)
    }
}

#[component]
pub fn AssetForm(
    #[prop(optional, into)] initial: Option<Asset>,
    on_submit: Callback<Vec<Asset>>,
    #[prop(into)] submit_label: Signal<String>,
) -> impl IntoView {
    let i18n = use_i18n();
    let is_edit = initial.is_some();

    let asset_number = RwSignal::new(initial.as_ref().map(|a| a.asset_number.clone()).unwrap_or_default());
    let name = RwSignal::new(initial.as_ref().map(|a| a.name.clone()).unwrap_or_default());
    let initial_category_index = initial.as_ref().map(|a| a.category.to_index()).unwrap_or(0);
    let category = RwSignal::new(initial_category_index);
    let category_ref = NodeRef::<leptos::html::Select>::new();
    let acquisition_date = RwSignal::new(
        initial.as_ref().map(|a| a.acquisition_date.clone())
            .unwrap_or_else(|| chrono::Utc::now().format("%Y-%m-%d").to_string()),
    );
    let cost = RwSignal::new(
        initial.as_ref().map(|a| a.cost.to_string()).unwrap_or_else(|| "0".to_string()),
    );
    let salvage_value = RwSignal::new(
        initial.as_ref().map(|a| a.salvage_value.to_string()).unwrap_or_else(|| "1".to_string()),
    );
    let useful_life = RwSignal::new(
        initial.as_ref().map(|a| a.useful_life.to_string()).unwrap_or_else(|| "5".to_string()),
    );
    let initial_method_index = initial.as_ref().map(|a| a.depreciation_method.to_index()).unwrap_or(0);
    let depreciation_method = RwSignal::new(initial_method_index);
    let prior_years = RwSignal::new(
        initial.as_ref().map(|a| a.prior_depreciation_years.to_string()).unwrap_or_else(|| "0".to_string()),
    );
    let prior_months = RwSignal::new(
        initial.as_ref().map(|a| a.prior_depreciation_months.to_string()).unwrap_or_else(|| "0".to_string()),
    );
    let location = RwSignal::new(initial.as_ref().map(|a| a.location.clone()).unwrap_or_default());
    let description = RwSignal::new(initial.as_ref().map(|a| a.description.clone()).unwrap_or_default());
    let initial_status_index = initial.as_ref().map(|a| a.status.to_index()).unwrap_or(0);
    let status = RwSignal::new(initial_status_index);
    let tags = RwSignal::new(initial.as_ref().map(|a| a.tags.clone()).unwrap_or_default());
    let tag_input = RwSignal::new(String::new());

    let quantity = RwSignal::new(1u32);

    let departments = Department::load_all();
    let has_departments = !departments.is_empty();
    let department_id = RwSignal::new(
        initial.as_ref().and_then(|a| a.department_id.clone()).unwrap_or_default()
    );

    // IFRS fields
    let ifrs_useful_life = RwSignal::new(
        initial.as_ref().and_then(|a| a.ifrs_useful_life).map(|v| v.to_string()).unwrap_or_default()
    );
    let ifrs_salvage_value = RwSignal::new(
        initial.as_ref().and_then(|a| a.ifrs_salvage_value).map(|v| v.normalize().to_string()).unwrap_or_default()
    );
    let ifrs_method = RwSignal::new(
        initial.as_ref().and_then(|a| a.ifrs_method.clone()).unwrap_or_default()
    );
    let show_ifrs = RwSignal::new(false);

    let initial_clone = initial.clone();
    let edit_asset_id = initial.as_ref().map(|a| a.id.clone());

    view! {
        <form
            class="space-y-4"
            on:submit=move |ev: web_sys::SubmitEvent| {
                ev.prevent_default();
                let cost_val = Decimal::from_str(&cost.get()).unwrap_or(Decimal::ZERO);
                let salvage_val = Decimal::from_str(&salvage_value.get()).unwrap_or(Decimal::ZERO);
                let life_val = useful_life.get().parse::<u32>().unwrap_or(5);
                let prior_y = prior_years.get().parse::<u32>().unwrap_or(0);
                let prior_m = prior_months.get().parse::<u32>().unwrap_or(0);

                let assets = if let Some(ref existing) = initial_clone {
                    // Edit mode: single asset
                    let mut a = existing.clone();
                    a.asset_number = asset_number.get();
                    a.name = name.get();
                    a.category = Category::from_index(category.get());
                    a.acquisition_date = acquisition_date.get();
                    a.cost = cost_val;
                    a.salvage_value = salvage_val;
                    a.useful_life = life_val;
                    a.depreciation_method = DepreciationMethod::from_index(depreciation_method.get());
                    a.prior_depreciation_years = prior_y;
                    a.prior_depreciation_months = prior_m;
                    a.location = location.get();
                    a.description = description.get();
                    a.status = AssetStatus::from_index(status.get());
                    a.tags = tags.get();
                    let dept = department_id.get();
                    a.department_id = if dept.is_empty() { None } else { Some(dept) };
                    // IFRS fields
                    let ifrs_ul = ifrs_useful_life.get();
                    a.ifrs_useful_life = if ifrs_ul.is_empty() { None } else { ifrs_ul.parse().ok() };
                    let ifrs_sv = ifrs_salvage_value.get();
                    a.ifrs_salvage_value = if ifrs_sv.is_empty() { None } else { Decimal::from_str(&ifrs_sv).ok() };
                    let ifrs_m = ifrs_method.get();
                    a.ifrs_method = if ifrs_m.is_empty() { None } else { Some(ifrs_m) };
                    a.updated_at = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string();
                    vec![a]
                } else {
                    // New mode: create N assets
                    let qty = quantity.get().max(1);
                    let base_number = asset_number.get();
                    let name_val = name.get();
                    let cat = Category::from_index(category.get());
                    let acq_date = acquisition_date.get();
                    let method = DepreciationMethod::from_index(depreciation_method.get());
                    let loc = location.get();
                    let desc = description.get();
                    let tags_val = tags.get();
                    let dept = department_id.get();
                    let dept_id = if dept.is_empty() { None } else { Some(dept) };

                    (0..qty).map(|i| {
                        let num = if qty == 1 {
                            base_number.clone()
                        } else {
                            generate_sequential_number(&base_number, i)
                        };
                        let mut a = Asset::new(
                            num,
                            name_val.clone(),
                            cat.clone(),
                            acq_date.clone(),
                            cost_val,
                            salvage_val,
                            life_val,
                            method.clone(),
                            prior_y,
                            prior_m,
                            loc.clone(),
                            desc.clone(),
                            tags_val.clone(),
                        );
                    a.department_id = dept_id.clone();
                        // IFRS fields
                        let ifrs_ul = ifrs_useful_life.get();
                        a.ifrs_useful_life = if ifrs_ul.is_empty() { None } else { ifrs_ul.parse().ok() };
                        let ifrs_sv = ifrs_salvage_value.get();
                        a.ifrs_salvage_value = if ifrs_sv.is_empty() { None } else { Decimal::from_str(&ifrs_sv).ok() };
                        let ifrs_m = ifrs_method.get();
                        a.ifrs_method = if ifrs_m.is_empty() { None } else { Some(ifrs_m.clone()) };
                        a
                    }).collect()
                };

                on_submit.run(assets);
            }
        >
            // Asset Number (optional)
            <div>
                <label class="label">{move || i18n.t("asset.asset_number")}</label>
                <input
                    type="text"
                    class="input-field"
                    placeholder=move || i18n.t("asset.asset_number_placeholder")
                    prop:value=move || asset_number.get()
                    on:input=move |ev| asset_number.set(event_target_value(&ev))
                />
            </div>

            // Quantity (only for new registration)
            {if !is_edit {
                let on_dec = move |_: web_sys::MouseEvent| {
                    quantity.update(|q| {
                        if *q > 1 { *q -= 1; }
                    });
                };
                let on_inc = move |_: web_sys::MouseEvent| {
                    quantity.update(|q| {
                        if *q < 100 { *q += 1; }
                    });
                };
                let on_qty_input = move |ev: web_sys::Event| {
                    let v: u32 = event_target_value(&ev).parse().unwrap_or(1);
                    quantity.set(v.max(1).min(100));
                };
                let is_min = move || quantity.get() <= 1;
                let is_max = move || quantity.get() == 100;
                Some(view! {
                    <div>
                        <label class="label">{move || i18n.t("asset.quantity")}</label>
                        <div class="flex items-center gap-3">
                            <button
                                type="button"
                                class="w-10 h-10 rounded-lg border border-gray-300 text-gray-600 font-bold text-lg flex items-center justify-center active:bg-gray-100 disabled:opacity-30"
                                disabled=is_min
                                on:click=on_dec
                            >{"\u{2212}"}</button>
                            <input
                                type="number"
                                min="1"
                                max="100"
                                class="input-field w-20 text-center font-bold"
                                prop:value=move || quantity.get().to_string()
                                on:input=on_qty_input
                            />
                            <button
                                type="button"
                                class="w-10 h-10 rounded-lg border border-gray-300 text-gray-600 font-bold text-lg flex items-center justify-center active:bg-gray-100 disabled:opacity-30"
                                disabled=is_max
                                on:click=on_inc
                            >{"\u{FF0B}"}</button>
                        </div>
                        {move || {
                            let qty = quantity.get();
                            let base = asset_number.get();
                            if qty > 1 && !base.is_empty() {
                                let first = generate_sequential_number(&base, 0);
                                let last = generate_sequential_number(&base, qty - 1);
                                view! {
                                    <p class="text-xs text-blue-500 mt-1">
                                        {format!("{} → {}", first, last)}
                                    </p>
                                }.into_any()
                            } else if qty > 1 {
                                view! {
                                    <p class="text-xs text-gray-400 mt-1">
                                        {move || i18n.t("asset.quantity_hint")}
                                    </p>
                                }.into_any()
                            } else {
                                view! { <span></span> }.into_any()
                            }
                        }}
                    </div>
                })
            } else {
                None
            }}

            // Asset Name
            <div>
                <label class="label">{move || i18n.t("asset.name")}</label>
                <input
                    type="text"
                    class="input-field"
                    required=true
                    prop:value=move || name.get()
                    on:input=move |ev| name.set(event_target_value(&ev))
                />
            </div>

            // Category
            <div>
                <label class="label">{move || i18n.t("asset.category")}</label>
                <select
                    class="input-field"
                    node_ref=category_ref
                    on:change=move |ev| {
                        let val: usize = event_target_value(&ev).parse().unwrap_or(0);
                        category.set(val);
                        // Auto-suggest useful life based on country rules
                        if !is_edit {
                            let cat = Category::from_index(val);
                            if let Some(life) = depreciation::suggested_useful_life(&cat) {
                                useful_life.set(life.to_string());
                            }
                        }
                    }
                >
                    {Category::all().into_iter().enumerate().map(|(i, cat)| {
                        let initial_cat = initial_category_index;
                        let key = cat.i18n_key().to_string();
                        view! {
                            <option value=i.to_string() selected=move || i == initial_cat>{move || i18n.t(&key)}</option>
                        }
                    }).collect::<Vec<_>>()}
                </select>
            </div>

            // Acquisition Date
            <div>
                <label class="label">{move || i18n.t("asset.acquisition_date")}</label>
                <input
                    type="date"
                    class="input-field"
                    required=true
                    prop:value=move || acquisition_date.get()
                    on:input=move |ev| acquisition_date.set(event_target_value(&ev))
                />
            </div>

            // Cost
            <div>
                <label class="label">{move || i18n.t("asset.cost")}</label>
                <input
                    type="number"
                    step="0.01"
                    min="0"
                    class="input-field"
                    required=true
                    prop:value=move || cost.get()
                    on:input=move |ev| cost.set(event_target_value(&ev))
                />
            </div>

            // Non-depreciable notice (shown only for land/leasehold/construction)
            <div class=move || {
                let cat = Category::from_index(category.get());
                if depreciation::is_non_depreciable(&cat) { "" } else { "hidden" }
            }>
                <div class="p-3 bg-gray-50 border border-gray-200 rounded-lg">
                    <p class="text-sm text-gray-500 flex items-center gap-2">
                        <svg class="w-4 h-4 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                        </svg>
                        {move || i18n.t("asset.non_depreciable_notice")}
                    </p>
                </div>
            </div>

            // Depreciation fields (hidden for non-depreciable assets)
            <div class=move || {
                let cat = Category::from_index(category.get());
                if depreciation::is_non_depreciable(&cat) { "hidden" } else { "space-y-4" }
            }>
                // Salvage Value
                <div>
                    <label class="label">{move || i18n.t("asset.salvage_value")}</label>
                    <input
                        type="number"
                        step="0.01"
                        min="0"
                        class="input-field"
                        prop:value=move || salvage_value.get()
                        on:input=move |ev| salvage_value.set(event_target_value(&ev))
                    />
                    // Intangible notice
                    <p class=move || {
                        let cat = Category::from_index(category.get());
                        if depreciation::is_intangible(&cat) { "text-xs text-blue-500 mt-1" } else { "hidden" }
                    }>{move || i18n.t("asset.intangible_salvage_zero")}</p>
                    // Normal hint
                    <p class=move || {
                        let cat = Category::from_index(category.get());
                        if depreciation::is_intangible(&cat) { "hidden" } else { "text-xs text-gray-400 mt-1" }
                    }>{move || i18n.t("asset.salvage_hint")}</p>
                    // Warning when salvage >= cost
                    {move || {
                        let c: f64 = cost.get().parse().unwrap_or(0.0);
                        let s: f64 = salvage_value.get().parse().unwrap_or(0.0);
                        if s > 0.0 && s >= c {
                            Some(view! {
                                <p class="text-xs text-red-500 mt-1 flex items-center gap-1">
                                    <svg class="w-3.5 h-3.5 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                                    </svg>
                                    {move || i18n.t("asset.salvage_warning")}
                                </p>
                            })
                        } else {
                            None
                        }
                    }}
                </div>

                // Useful Life
                <div>
                    <label class="label">{move || i18n.t("asset.useful_life")}</label>
                    <input
                        type="number"
                        min="1"
                        max="100"
                        class="input-field"
                        required=true
                        prop:value=move || useful_life.get()
                        on:input=move |ev| useful_life.set(event_target_value(&ev))
                    />
                </div>

                // Depreciation Method
                <div>
                    <label class="label">{move || i18n.t("asset.depreciation_method")}</label>
                    <select
                        class="input-field"
                        on:change=move |ev| {
                            let val: usize = event_target_value(&ev).parse().unwrap_or(0);
                            depreciation_method.set(val);
                        }
                    >
                        {DepreciationMethod::all().into_iter().enumerate().map(|(i, method)| {
                            let init_m = initial_method_index;
                            let key = method.i18n_key().to_string();
                            view! {
                                <option value=i.to_string() selected=move || i == init_m>{move || i18n.t(&key)}</option>
                            }
                        }).collect::<Vec<_>>()}
                    </select>
                    // Intangible notice
                    <p class=move || {
                        let cat = Category::from_index(category.get());
                        if depreciation::is_intangible(&cat) || !depreciation::can_use_declining_balance(&cat) {
                            "text-xs text-blue-500 mt-1"
                        } else {
                            "hidden"
                        }
                    }>{move || i18n.t("asset.intangible_method_notice")}</p>
                </div>

                // Prior Depreciation (for used assets)
                <div>
                    <label class="label">{move || i18n.t("asset.prior_depreciation")}</label>
                    <div class="grid grid-cols-2 gap-2">
                        <div>
                            <div class="flex items-center gap-1">
                                <input
                                    type="number"
                                    min="0"
                                    max="99"
                                    class="input-field"
                                    prop:value=move || prior_years.get()
                                    on:input=move |ev| prior_years.set(event_target_value(&ev))
                                />
                                <span class="text-sm text-gray-500 shrink-0">{move || i18n.t("asset.years")}</span>
                            </div>
                        </div>
                        <div>
                            <div class="flex items-center gap-1">
                                <input
                                    type="number"
                                    min="0"
                                    max="11"
                                    class="input-field"
                                    prop:value=move || prior_months.get()
                                    on:input=move |ev| prior_months.set(event_target_value(&ev))
                                />
                                <span class="text-sm text-gray-500 shrink-0">{move || i18n.t("asset.months")}</span>
                            </div>
                        </div>
                    </div>
                    <p class="text-xs text-gray-400 mt-1">{move || i18n.t("asset.prior_depreciation_hint")}</p>
                </div>
            </div>

            // IFRS Settings (collapsible)
            <div class="border border-blue-200 rounded-lg overflow-hidden">
                <button
                    type="button"
                    class="w-full flex items-center justify-between px-3 py-2.5 bg-blue-50 active:bg-blue-100"
                    on:click=move |_| show_ifrs.update(|v| *v = !*v)
                >
                    <div class="flex items-center gap-2">
                        <span class="text-[10px] font-bold text-blue-700 bg-blue-200 px-1.5 py-0.5 rounded">"IFRS"</span>
                        <span class="text-xs font-semibold text-blue-800">{move || i18n.t("standard.ifrs_settings")}</span>
                    </div>
                    <svg
                        class=move || format!("w-4 h-4 text-blue-400 transition-transform {}", if show_ifrs.get() { "rotate-180" } else { "" })
                        fill="none" stroke="currentColor" viewBox="0 0 24 24"
                    >
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/>
                    </svg>
                </button>
                {move || if show_ifrs.get() {
                    view! {
                        <div class="px-3 py-3 space-y-3 bg-white">
                            <p class="text-[10px] text-gray-400">{move || i18n.t("standard.ifrs_hint")}</p>
                            // IFRS Useful Life
                            <div>
                                <label class="label">{move || format!("{} (IFRS)", i18n.t("asset.useful_life"))}</label>
                                <input
                                    type="number"
                                    class="input-field"
                                    placeholder=move || i18n.t("standard.same_as_local")
                                    prop:value=move || ifrs_useful_life.get()
                                    on:input=move |ev| ifrs_useful_life.set(event_target_value(&ev))
                                />
                            </div>
                            // IFRS Salvage Value
                            <div>
                                <label class="label">{move || format!("{} (IFRS)", i18n.t("asset.salvage_value"))}</label>
                                <input
                                    type="number"
                                    step="0.01"
                                    class="input-field"
                                    placeholder=move || i18n.t("standard.same_as_local")
                                    prop:value=move || ifrs_salvage_value.get()
                                    on:input=move |ev| ifrs_salvage_value.set(event_target_value(&ev))
                                />
                            </div>
                            // IFRS Method
                            <div>
                                <label class="label">{move || format!("{} (IFRS)", i18n.t("asset.depreciation_method"))}</label>
                                <select
                                    class="input-field"
                                    on:change=move |ev| ifrs_method.set(event_target_value(&ev))
                                >
                                    <option value="" selected=move || ifrs_method.get().is_empty()>{move || i18n.t("standard.same_as_local")}</option>
                                    <option value="SL" selected=move || ifrs_method.get() == "SL">{move || i18n.t("depreciation.straight_line")}</option>
                                    <option value="DB" selected=move || ifrs_method.get() == "DB">{move || i18n.t("depreciation.declining_balance")}</option>
                                </select>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }}
            </div>

            // Location
            <div>
                <label class="label">{move || i18n.t("asset.location")}</label>
                <input
                    type="text"
                    class="input-field"
                    prop:value=move || location.get()
                    on:input=move |ev| location.set(event_target_value(&ev))
                />
            </div>

            // Department
            {if has_departments {
                let initial_dept = initial.as_ref().and_then(|a| a.department_id.clone()).unwrap_or_default();
                let initial_dept2 = initial_dept.clone();
                Some(view! {
                    <div>
                        <label class="label">{move || i18n.t("asset.department")}</label>
                        <select
                            class="input-field"
                            on:change=move |ev| {
                                department_id.set(event_target_value(&ev));
                            }
                        >
                            <option value="" selected=move || initial_dept.is_empty()>{move || i18n.t("asset.dept_unassigned")}</option>
                            {departments.clone().into_iter().map(|dept| {
                                let dept_id = dept.id.clone();
                                let is_selected = initial_dept2 == dept_id;
                                let label = if dept.code.is_empty() {
                                    dept.name.clone()
                                } else {
                                    format!("{} - {}", dept.code, dept.name)
                                };
                                view! {
                                    <option value=dept_id selected=is_selected>{label}</option>
                                }
                            }).collect::<Vec<_>>()}
                        </select>
                    </div>
                })
            } else {
                None
            }}

            // Status (only show in edit mode)
            {if is_edit {
                Some(view! {
                    <div>
                        <label class="label">{move || i18n.t("asset.status")}</label>
                        <select
                            class="input-field"
                            on:change=move |ev| {
                                let val: usize = event_target_value(&ev).parse().unwrap_or(0);
                                status.set(val);
                            }
                        >
                            {AssetStatus::all().into_iter().enumerate().map(|(i, s)| {
                                let init_s = initial_status_index;
                                let key = s.i18n_key().to_string();
                                view! {
                                    <option value=i.to_string() selected=move || i == init_s>{move || i18n.t(&key)}</option>
                                }
                            }).collect::<Vec<_>>()}
                        </select>
                    </div>
                })
            } else {
                None
            }}

            // Description
            <div>
                <label class="label">{move || i18n.t("asset.description")}</label>
                <textarea
                    class="input-field"
                    rows="3"
                    prop:value=move || description.get()
                    on:input=move |ev| description.set(event_target_value(&ev))
                ></textarea>
            </div>

            // Tags
            <div>
                <label class="label">{move || i18n.t("asset.tags")}</label>
                // Input row
                <div class="flex gap-2 mb-2">
                    <input
                        type="text"
                        class="input-field flex-1"
                        placeholder=move || i18n.t("asset.tag_placeholder")
                        prop:value=move || tag_input.get()
                        on:input=move |ev| tag_input.set(event_target_value(&ev))
                        on:keydown=move |ev: web_sys::KeyboardEvent| {
                            if ev.key() == "Enter" {
                                ev.prevent_default();
                                let val = tag_input.get().trim().to_string();
                                if !val.is_empty() {
                                    tags.update(|v| {
                                        if !v.contains(&val) {
                                            v.push(val);
                                        }
                                    });
                                    tag_input.set(String::new());
                                }
                            }
                        }
                    />
                    <button
                        type="button"
                        class="bg-blue-600 text-white text-sm font-bold px-4 rounded-lg shrink-0 active:bg-blue-700"
                        on:click=move |_| {
                            let val = tag_input.get().trim().to_string();
                            if !val.is_empty() {
                                tags.update(|v| {
                                    if !v.contains(&val) {
                                        v.push(val);
                                    }
                                });
                                tag_input.set(String::new());
                            }
                        }
                    >"+"</button>
                </div>
                // Tag pills
                {move || {
                    let current_tags = tags.get();
                    if current_tags.is_empty() {
                        None
                    } else {
                        Some(view! {
                            <div class="flex flex-wrap gap-1.5">
                                {current_tags.into_iter().map(|tag| {
                                    let tag_clone = tag.clone();
                                    let tag_display = tag.clone();
                                    view! {
                                        <span class="inline-flex items-center gap-1 bg-blue-600 text-white text-xs font-medium px-2.5 py-1 rounded-full">
                                            {tag_display}
                                            <button
                                                type="button"
                                                class="text-blue-200 hover:text-white ml-0.5"
                                                on:click=move |_| {
                                                    let t = tag_clone.clone();
                                                    tags.update(|v| v.retain(|x| x != &t));
                                                }
                                            >"×"</button>
                                        </span>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        })
                    }
                }}
                <p class="text-xs text-gray-400 mt-1">{move || i18n.t("asset.tag_hint")}</p>
            </div>

            // Photos
            <div>
                <label class="label">
                    <div class="flex items-center gap-2">
                        <svg class="w-4 h-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"/>
                        </svg>
                        {move || i18n.t("photo.title")}
                    </div>
                </label>
                {if let Some(aid) = edit_asset_id {
                    view! {
                        <div>
                            <PhotoGallery asset_id=aid editable=true />
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="text-xs text-gray-400 py-2 px-3 bg-gray-50 rounded-lg">
                            {move || i18n.t("photo.save_first")}
                        </div>
                    }.into_any()
                }}
            </div>

            // Submit
            <div class="pt-2">
                <button type="submit" class="btn-primary">
                    {move || {
                        let qty = quantity.get();
                        let label = submit_label.get();
                        if qty > 1 && !is_edit {
                            format!("{} (×{})", label, qty)
                        } else {
                            label
                        }
                    }}
                </button>
            </div>
        </form>
    }
}
