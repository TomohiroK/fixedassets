use leptos::prelude::*;
use rust_decimal::Decimal;
use std::str::FromStr;
use crate::i18n::use_i18n;
use crate::models::asset::*;
use crate::models::department::Department;
use crate::models::depreciation;
use crate::components::common::currency_symbol;

#[derive(Clone, Copy, PartialEq, Eq)]
enum WizardStep {
    Name,
    Category,
    AcquisitionDate,
    Cost,
    Depreciation,
    Optional,
    Confirm,
}

impl WizardStep {
    fn index(&self) -> usize {
        match self {
            Self::Name => 0,
            Self::Category => 1,
            Self::AcquisitionDate => 2,
            Self::Cost => 3,
            Self::Depreciation => 4,
            Self::Optional => 5,
            Self::Confirm => 6,
        }
    }

    fn total() -> usize { 7 }

    fn next(&self) -> Option<Self> {
        match self {
            Self::Name => Some(Self::Category),
            Self::Category => Some(Self::AcquisitionDate),
            Self::AcquisitionDate => Some(Self::Cost),
            Self::Cost => Some(Self::Depreciation),
            Self::Depreciation => Some(Self::Optional),
            Self::Optional => Some(Self::Confirm),
            Self::Confirm => None,
        }
    }

    fn prev(&self) -> Option<Self> {
        match self {
            Self::Name => None,
            Self::Category => Some(Self::Name),
            Self::AcquisitionDate => Some(Self::Category),
            Self::Cost => Some(Self::AcquisitionDate),
            Self::Depreciation => Some(Self::Cost),
            Self::Optional => Some(Self::Depreciation),
            Self::Confirm => Some(Self::Optional),
        }
    }
}

#[component]
pub fn WizardForm(
    on_submit: Callback<Vec<Asset>>,
    #[prop(into)] submit_label: Signal<String>,
) -> impl IntoView {
    let i18n = use_i18n();

    let step = RwSignal::new(WizardStep::Name);

    // All field signals (same defaults as AssetForm)
    let name = RwSignal::new(String::new());
    let category = RwSignal::new(0usize);
    let acquisition_date = RwSignal::new(
        chrono::Utc::now().format("%Y-%m-%d").to_string(),
    );
    let cost = RwSignal::new("0".to_string());
    let salvage_value = RwSignal::new("1".to_string());
    let useful_life = RwSignal::new("5".to_string());
    let depreciation_method = RwSignal::new(0usize);
    let prior_years = RwSignal::new("0".to_string());
    let prior_months = RwSignal::new("0".to_string());
    let asset_number = RwSignal::new(String::new());
    let location = RwSignal::new(String::new());
    let description = RwSignal::new(String::new());
    let department_id = RwSignal::new(String::new());
    let tags = RwSignal::new(Vec::<String>::new());
    let tag_input = RwSignal::new(String::new());

    let departments = Department::load_all();
    let has_departments = !departments.is_empty();

    // Validation per step
    let can_proceed = move || {
        match step.get() {
            WizardStep::Name => !name.get().trim().is_empty(),
            WizardStep::Category => true, // always valid (default selection)
            WizardStep::AcquisitionDate => !acquisition_date.get().is_empty(),
            WizardStep::Cost => {
                let c: f64 = cost.get().parse().unwrap_or(0.0);
                c > 0.0
            }
            WizardStep::Depreciation => {
                let cat = Category::from_index(category.get());
                if depreciation::is_non_depreciable(&cat) {
                    true
                } else {
                    let l: u32 = useful_life.get().parse().unwrap_or(0);
                    l > 0
                }
            }
            WizardStep::Optional => true,
            WizardStep::Confirm => true,
        }
    };

    let go_next = move |_| {
        if let Some(next) = step.get().next() {
            // Auto-suggest useful life when moving from Category step
            if step.get() == WizardStep::Category {
                let cat = Category::from_index(category.get());
                if let Some(life) = depreciation::suggested_useful_life(&cat) {
                    useful_life.set(life.to_string());
                }
            }
            step.set(next);
        }
    };

    let go_prev = move |_| {
        if let Some(prev) = step.get().prev() {
            step.set(prev);
        }
    };

    let do_submit = move |_| {
        let cost_val = Decimal::from_str(&cost.get()).unwrap_or(Decimal::ZERO);
        let salvage_val = Decimal::from_str(&salvage_value.get()).unwrap_or(Decimal::ZERO);
        let life_val = useful_life.get().parse::<u32>().unwrap_or(5);
        let prior_y = prior_years.get().parse::<u32>().unwrap_or(0);
        let prior_m = prior_months.get().parse::<u32>().unwrap_or(0);
        let cat = Category::from_index(category.get());
        let method = DepreciationMethod::from_index(depreciation_method.get());
        let dept = department_id.get();

        let mut a = Asset::new(
            asset_number.get(),
            name.get(),
            cat,
            acquisition_date.get(),
            cost_val,
            salvage_val,
            life_val,
            method,
            prior_y,
            prior_m,
            location.get(),
            description.get(),
            tags.get(),
        );
        a.department_id = if dept.is_empty() { None } else { Some(dept) };

        on_submit.run(vec![a]);
    };

    // Progress bar
    let progress_view = move || {
        let current = step.get().index();
        let total = WizardStep::total();
        let pct = ((current as f64 / (total - 1) as f64) * 100.0) as u32;
        view! {
            <div class="mb-6">
                <div class="flex justify-between text-xs text-gray-400 mb-1.5">
                    <span>{move || i18n.t("wizard.step")} " " {current + 1} "/" {total}</span>
                    <span>{pct} "%"</span>
                </div>
                <div class="w-full bg-gray-200 rounded-full h-1.5">
                    <div
                        class="bg-blue-600 h-1.5 rounded-full transition-all duration-300"
                        style=format!("width: {}%", pct)
                    ></div>
                </div>
            </div>
        }
    };

    // Step: Name
    let step_name = move || {
        view! {
            <div class="space-y-3">
                <div class="text-center mb-4">
                    <div class="w-12 h-12 bg-blue-100 rounded-full mx-auto flex items-center justify-center mb-3">
                        <svg class="w-6 h-6 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 7h.01M7 3h5c.512 0 1.024.195 1.414.586l7 7a2 2 0 010 2.828l-7 7a2 2 0 01-2.828 0l-7-7A1.994 1.994 0 013 12V7a4 4 0 014-4z"/>
                        </svg>
                    </div>
                    <h3 class="text-lg font-bold text-gray-900">{move || i18n.t("wizard.what_name")}</h3>
                    <p class="text-sm text-gray-500">{move || i18n.t("wizard.name_hint")}</p>
                </div>
                <input
                    type="text"
                    class="input-field text-center text-lg"
                    autofocus=true
                    placeholder=move || i18n.t("wizard.name_placeholder")
                    prop:value=move || name.get()
                    on:input=move |ev| name.set(event_target_value(&ev))
                    on:keydown=move |ev: web_sys::KeyboardEvent| {
                        if ev.key() == "Enter" && !name.get().trim().is_empty() {
                            ev.prevent_default();
                            if let Some(next) = step.get().next() {
                                let cat = Category::from_index(category.get());
                                if let Some(life) = depreciation::suggested_useful_life(&cat) {
                                    useful_life.set(life.to_string());
                                }
                                step.set(next);
                            }
                        }
                    }
                />
            </div>
        }
    };

    // Step: Category
    let step_category = move || {
        view! {
            <div class="space-y-3">
                <div class="text-center mb-4">
                    <div class="w-12 h-12 bg-green-100 rounded-full mx-auto flex items-center justify-center mb-3">
                        <svg class="w-6 h-6 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"/>
                        </svg>
                    </div>
                    <h3 class="text-lg font-bold text-gray-900">{move || i18n.t("wizard.what_category")}</h3>
                    <p class="text-sm text-gray-500">{move || i18n.t("wizard.category_hint")}</p>
                </div>
                <div class="p-2.5 bg-blue-50 border border-blue-100 rounded-lg flex items-start gap-2">
                    <svg class="w-4 h-4 text-blue-400 shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                    </svg>
                    <p class="text-xs text-blue-600">{move || i18n.t("wizard.guide_category")}</p>
                </div>
                <div class="grid grid-cols-2 gap-2">
                    {Category::all().into_iter().enumerate().map(|(i, cat)| {
                        let key = cat.i18n_key().to_string();
                        view! {
                            <button
                                type="button"
                                class=move || {
                                    if category.get() == i {
                                        "p-3 rounded-xl border-2 border-blue-500 bg-blue-50 text-blue-700 font-medium text-sm text-left transition-all"
                                    } else {
                                        "p-3 rounded-xl border-2 border-gray-200 bg-white text-gray-700 text-sm text-left active:bg-gray-50 transition-all"
                                    }
                                }
                                on:click=move |_| category.set(i)
                            >
                                {move || i18n.t(&key)}
                            </button>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        }
    };

    // Step: Acquisition Date
    let step_date = move || {
        view! {
            <div class="space-y-3">
                <div class="text-center mb-4">
                    <div class="w-12 h-12 bg-purple-100 rounded-full mx-auto flex items-center justify-center mb-3">
                        <svg class="w-6 h-6 text-purple-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"/>
                        </svg>
                    </div>
                    <h3 class="text-lg font-bold text-gray-900">{move || i18n.t("wizard.when_acquired")}</h3>
                    <p class="text-sm text-gray-500">{move || i18n.t("wizard.date_hint")}</p>
                </div>
                <input
                    type="date"
                    class="input-field text-center text-lg"
                    prop:value=move || acquisition_date.get()
                    on:input=move |ev| acquisition_date.set(event_target_value(&ev))
                />
                <div class="p-2.5 bg-blue-50 border border-blue-100 rounded-lg flex items-start gap-2 mt-2">
                    <svg class="w-4 h-4 text-blue-400 shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                    </svg>
                    <p class="text-xs text-blue-600">{move || i18n.t("wizard.guide_date")}</p>
                </div>
            </div>
        }
    };

    // Step: Cost
    let step_cost = move || {
        view! {
            <div class="space-y-3">
                <div class="text-center mb-4">
                    <div class="w-12 h-12 bg-amber-100 rounded-full mx-auto flex items-center justify-center mb-3">
                        <svg class="w-6 h-6 text-amber-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                        </svg>
                    </div>
                    <h3 class="text-lg font-bold text-gray-900">{move || i18n.t("wizard.how_much")}</h3>
                    <p class="text-sm text-gray-500">{move || i18n.t("wizard.cost_hint")}</p>
                </div>
                <div class="relative">
                    <span class="absolute left-4 top-1/2 -translate-y-1/2 text-lg text-gray-400 font-medium">{currency_symbol()}</span>
                    <input
                        type="number"
                        step="0.01"
                        min="0"
                        class="input-field text-center text-lg pl-10"
                        prop:value=move || cost.get()
                        on:input=move |ev| cost.set(event_target_value(&ev))
                    />
                </div>
            </div>
        }
    };

    // Step: Depreciation
    let step_depreciation = move || {
        let is_non_dep = move || {
            let cat = Category::from_index(category.get());
            depreciation::is_non_depreciable(&cat)
        };

        view! {
            <div class="space-y-3">
                <div class="text-center mb-4">
                    <div class="w-12 h-12 bg-red-100 rounded-full mx-auto flex items-center justify-center mb-3">
                        <svg class="w-6 h-6 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 7h6m0 10v-3m-3 3h.01M9 17h.01M9 14h.01M12 14h.01M15 11h.01M12 11h.01M9 11h.01M7 21h10a2 2 0 002-2V5a2 2 0 00-2-2H7a2 2 0 00-2 2v14a2 2 0 002 2z"/>
                        </svg>
                    </div>
                    <h3 class="text-lg font-bold text-gray-900">{move || i18n.t("wizard.depreciation_settings")}</h3>
                    <p class="text-sm text-gray-500">{move || i18n.t("wizard.depreciation_hint")}</p>
                </div>
                <div class="p-2.5 bg-blue-50 border border-blue-100 rounded-lg flex items-start gap-2">
                    <svg class="w-4 h-4 text-blue-400 shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                    </svg>
                    <p class="text-xs text-blue-600">{move || i18n.t("wizard.guide_depreciation")}</p>
                </div>

                {move || if is_non_dep() {
                    view! {
                        <div class="p-4 bg-gray-50 border border-gray-200 rounded-xl text-center">
                            <p class="text-sm text-gray-500">{move || i18n.t("asset.non_depreciable_notice")}</p>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="space-y-4">
                            // Salvage Value
                            <div>
                                <label class="label">{move || i18n.t("asset.salvage_value")}</label>
                                <div class="relative">
                                    <span class="absolute left-3 top-1/2 -translate-y-1/2 text-sm text-gray-400 font-medium">{currency_symbol()}</span>
                                    <input
                                        type="number"
                                        step="0.01"
                                        min="0"
                                        class="input-field pl-8"
                                        prop:value=move || salvage_value.get()
                                        on:input=move |ev| salvage_value.set(event_target_value(&ev))
                                    />
                                </div>
                                <p class="text-xs text-gray-400 mt-1">{move || i18n.t("asset.salvage_hint")}</p>
                            </div>
                            // Useful Life
                            <div>
                                <label class="label">{move || i18n.t("asset.useful_life")}</label>
                                <input
                                    type="number"
                                    min="1"
                                    max="100"
                                    class="input-field"
                                    prop:value=move || useful_life.get()
                                    on:input=move |ev| useful_life.set(event_target_value(&ev))
                                />
                            </div>
                            // Method
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
                                        let key = method.i18n_key().to_string();
                                        view! {
                                            <option value=i.to_string() selected=move || depreciation_method.get() == i>{move || i18n.t(&key)}</option>
                                        }
                                    }).collect::<Vec<_>>()}
                                </select>
                            </div>
                            // Prior depreciation
                            <div>
                                <label class="label">{move || i18n.t("asset.prior_depreciation")}</label>
                                <div class="grid grid-cols-2 gap-2">
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
                                <p class="text-xs text-gray-400 mt-1">{move || i18n.t("asset.prior_depreciation_hint")}</p>
                            </div>
                        </div>
                    }.into_any()
                }}
            </div>
        }
    };

    // Step: Optional
    let departments_clone = departments.clone();
    let step_optional = move || {
        let depts = departments_clone.clone();
        view! {
            <div class="space-y-3">
                <div class="text-center mb-4">
                    <div class="w-12 h-12 bg-teal-100 rounded-full mx-auto flex items-center justify-center mb-3">
                        <svg class="w-6 h-6 text-teal-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4"/>
                        </svg>
                    </div>
                    <h3 class="text-lg font-bold text-gray-900">{move || i18n.t("wizard.optional_info")}</h3>
                    <p class="text-sm text-gray-500">{move || i18n.t("wizard.optional_hint")}</p>
                </div>
                <div class="p-2.5 bg-green-50 border border-green-100 rounded-lg flex items-start gap-2">
                    <svg class="w-4 h-4 text-green-500 shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6"/>
                    </svg>
                    <p class="text-xs text-green-700">{move || i18n.t("wizard.guide_optional")}</p>
                </div>
                <div class="space-y-4">
                    // Asset Number
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
                        let dept_list = depts.clone();
                        Some(view! {
                            <div>
                                <label class="label">{move || i18n.t("asset.department")}</label>
                                <select
                                    class="input-field"
                                    on:change=move |ev| department_id.set(event_target_value(&ev))
                                >
                                    <option value="">{move || i18n.t("asset.dept_unassigned")}</option>
                                    {dept_list.into_iter().map(|dept| {
                                        let dept_id_val = dept.id.clone();
                                        let label = if dept.code.is_empty() {
                                            dept.name.clone()
                                        } else {
                                            format!("{} - {}", dept.code, dept.name)
                                        };
                                        view! {
                                            <option value=dept_id_val>{label}</option>
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
                            rows="2"
                            prop:value=move || description.get()
                            on:input=move |ev| description.set(event_target_value(&ev))
                        ></textarea>
                    </div>
                    // Tags
                    <div>
                        <label class="label">{move || i18n.t("asset.tags")}</label>
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
                                                if !v.contains(&val) { v.push(val); }
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
                                            if !v.contains(&val) { v.push(val); }
                                        });
                                        tag_input.set(String::new());
                                    }
                                }
                            >"+"</button>
                        </div>
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
                    </div>
                </div>
            </div>
        }
    };

    // Step: Confirm
    let step_confirm = move || {
        let cat = Category::from_index(category.get());
        let cat_key = cat.i18n_key().to_string();
        let method = DepreciationMethod::from_index(depreciation_method.get());
        let method_key = method.i18n_key().to_string();
        let is_non_dep = depreciation::is_non_depreciable(&cat);

        view! {
            <div class="space-y-3">
                <div class="text-center mb-4">
                    <div class="w-12 h-12 bg-emerald-100 rounded-full mx-auto flex items-center justify-center mb-3">
                        <svg class="w-6 h-6 text-emerald-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>
                        </svg>
                    </div>
                    <h3 class="text-lg font-bold text-gray-900">{move || i18n.t("wizard.confirm_title")}</h3>
                    <p class="text-sm text-gray-500">{move || i18n.t("wizard.confirm_hint")}</p>
                </div>

                <div class="bg-gray-50 rounded-xl p-4 space-y-2.5">
                    // Name
                    <div class="flex justify-between">
                        <span class="text-xs text-gray-500">{move || i18n.t("asset.name")}</span>
                        <span class="text-sm font-medium text-gray-900">{move || name.get()}</span>
                    </div>
                    // Category
                    <div class="flex justify-between">
                        <span class="text-xs text-gray-500">{move || i18n.t("asset.category")}</span>
                        <span class="text-sm font-medium text-gray-900">{move || i18n.t(&cat_key)}</span>
                    </div>
                    // Date
                    <div class="flex justify-between">
                        <span class="text-xs text-gray-500">{move || i18n.t("asset.acquisition_date")}</span>
                        <span class="text-sm font-medium text-gray-900">{move || acquisition_date.get()}</span>
                    </div>
                    // Cost
                    <div class="flex justify-between">
                        <span class="text-xs text-gray-500">{move || i18n.t("asset.cost")}</span>
                        <span class="text-sm font-bold text-gray-900">{currency_symbol()} {move || cost.get()}</span>
                    </div>

                    {if !is_non_dep {
                        Some(view! {
                            <div class="border-t border-gray-200 pt-2 space-y-2.5">
                                <div class="flex justify-between">
                                    <span class="text-xs text-gray-500">{move || i18n.t("asset.salvage_value")}</span>
                                    <span class="text-sm text-gray-900">{currency_symbol()} {move || salvage_value.get()}</span>
                                </div>
                                <div class="flex justify-between">
                                    <span class="text-xs text-gray-500">{move || i18n.t("asset.useful_life")}</span>
                                    <span class="text-sm text-gray-900">{move || useful_life.get()} {move || i18n.t("asset.years")}</span>
                                </div>
                                <div class="flex justify-between">
                                    <span class="text-xs text-gray-500">{move || i18n.t("asset.depreciation_method")}</span>
                                    <span class="text-sm text-gray-900">{move || i18n.t(&method_key)}</span>
                                </div>
                            </div>
                        })
                    } else {
                        None
                    }}

                    // Optional fields (only show if filled)
                    {move || {
                        let an = asset_number.get();
                        let loc = location.get();
                        let desc = description.get();
                        let t = tags.get();
                        let has_optional = !an.is_empty() || !loc.is_empty() || !desc.is_empty() || !t.is_empty();
                        if has_optional {
                            Some(view! {
                                <div class="border-t border-gray-200 pt-2 space-y-2.5">
                                    {if !an.is_empty() {
                                        Some(view! {
                                            <div class="flex justify-between">
                                                <span class="text-xs text-gray-500">{move || i18n.t("asset.asset_number")}</span>
                                                <span class="text-sm text-gray-900">{an.clone()}</span>
                                            </div>
                                        })
                                    } else { None }}
                                    {if !loc.is_empty() {
                                        Some(view! {
                                            <div class="flex justify-between">
                                                <span class="text-xs text-gray-500">{move || i18n.t("asset.location")}</span>
                                                <span class="text-sm text-gray-900">{loc.clone()}</span>
                                            </div>
                                        })
                                    } else { None }}
                                    {if !desc.is_empty() {
                                        Some(view! {
                                            <div class="flex justify-between">
                                                <span class="text-xs text-gray-500">{move || i18n.t("asset.description")}</span>
                                                <span class="text-sm text-gray-900 max-w-[200px] truncate">{desc.clone()}</span>
                                            </div>
                                        })
                                    } else { None }}
                                    {if !t.is_empty() {
                                        let tags_str = t.join(", ");
                                        Some(view! {
                                            <div class="flex justify-between">
                                                <span class="text-xs text-gray-500">{move || i18n.t("asset.tags")}</span>
                                                <span class="text-sm text-gray-900">{tags_str}</span>
                                            </div>
                                        })
                                    } else { None }}
                                </div>
                            })
                        } else {
                            None
                        }
                    }}
                </div>
            </div>
        }
    };

    view! {
        <div class="space-y-4">
            {progress_view}

            // Step content
            {move || match step.get() {
                WizardStep::Name => step_name().into_any(),
                WizardStep::Category => step_category().into_any(),
                WizardStep::AcquisitionDate => step_date().into_any(),
                WizardStep::Cost => step_cost().into_any(),
                WizardStep::Depreciation => step_depreciation().into_any(),
                WizardStep::Optional => step_optional().into_any(),
                WizardStep::Confirm => step_confirm().into_any(),
            }}

            // Navigation buttons
            <div class="flex gap-3 pt-4">
                {move || if step.get() != WizardStep::Name {
                    Some(view! {
                        <button
                            type="button"
                            class="flex-1 py-3 bg-gray-100 text-gray-700 rounded-xl font-semibold active:bg-gray-200 transition-colors"
                            on:click=go_prev
                        >
                            {move || i18n.t("wizard.back")}
                        </button>
                    })
                } else {
                    None
                }}

                {move || if step.get() == WizardStep::Confirm {
                    view! {
                        <button
                            type="button"
                            class="flex-1 py-3 bg-blue-600 text-white rounded-xl font-bold active:bg-blue-700 transition-colors disabled:opacity-40"
                            on:click=do_submit
                        >
                            {move || submit_label.get()}
                        </button>
                    }.into_any()
                } else {
                    view! {
                        <button
                            type="button"
                            class="flex-1 py-3 bg-blue-600 text-white rounded-xl font-bold active:bg-blue-700 transition-colors disabled:opacity-40"
                            disabled=move || !can_proceed()
                            on:click=go_next
                        >
                            {move || i18n.t("common.next")}
                        </button>
                    }.into_any()
                }}
            </div>
        </div>
    }
}
