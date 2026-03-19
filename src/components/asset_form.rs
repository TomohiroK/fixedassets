use leptos::prelude::*;
use rust_decimal::Decimal;
use std::str::FromStr;
use crate::i18n::use_i18n;
use crate::models::asset::*;

#[component]
pub fn AssetForm(
    #[prop(optional, into)] initial: Option<Asset>,
    on_submit: Callback<Asset>,
    #[prop(into)] submit_label: Signal<String>,
) -> impl IntoView {
    let i18n = use_i18n();
    let is_edit = initial.is_some();

    let name = RwSignal::new(initial.as_ref().map(|a| a.name.clone()).unwrap_or_default());
    let category = RwSignal::new(initial.as_ref().map(|a| a.category.to_index()).unwrap_or(0));
    let acquisition_date = RwSignal::new(
        initial.as_ref().map(|a| a.acquisition_date.clone())
            .unwrap_or_else(|| chrono::Utc::now().format("%Y-%m-%d").to_string()),
    );
    let cost = RwSignal::new(
        initial.as_ref().map(|a| a.cost.to_string()).unwrap_or_else(|| "0".to_string()),
    );
    let salvage_value = RwSignal::new(
        initial.as_ref().map(|a| a.salvage_value.to_string()).unwrap_or_else(|| "0".to_string()),
    );
    let useful_life = RwSignal::new(
        initial.as_ref().map(|a| a.useful_life.to_string()).unwrap_or_else(|| "5".to_string()),
    );
    let depreciation_method = RwSignal::new(
        initial.as_ref().map(|a| a.depreciation_method.to_index()).unwrap_or(0),
    );
    let location = RwSignal::new(initial.as_ref().map(|a| a.location.clone()).unwrap_or_default());
    let description = RwSignal::new(initial.as_ref().map(|a| a.description.clone()).unwrap_or_default());
    let status = RwSignal::new(initial.as_ref().map(|a| a.status.to_index()).unwrap_or(0));

    let initial_clone = initial.clone();

    view! {
        <form
            class="space-y-4"
            on:submit=move |ev| {
                ev.prevent_default();
                let cost_val = Decimal::from_str(&cost.get()).unwrap_or(Decimal::ZERO);
                let salvage_val = Decimal::from_str(&salvage_value.get()).unwrap_or(Decimal::ZERO);
                let life_val = useful_life.get().parse::<u32>().unwrap_or(5);

                let asset = if let Some(ref existing) = initial_clone {
                    let mut a = existing.clone();
                    a.name = name.get();
                    a.category = Category::from_index(category.get());
                    a.acquisition_date = acquisition_date.get();
                    a.cost = cost_val;
                    a.salvage_value = salvage_val;
                    a.useful_life = life_val;
                    a.depreciation_method = DepreciationMethod::from_index(depreciation_method.get());
                    a.location = location.get();
                    a.description = description.get();
                    a.status = AssetStatus::from_index(status.get());
                    a.updated_at = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string();
                    a
                } else {
                    Asset::new(
                        name.get(),
                        Category::from_index(category.get()),
                        acquisition_date.get(),
                        cost_val,
                        salvage_val,
                        life_val,
                        DepreciationMethod::from_index(depreciation_method.get()),
                        location.get(),
                        description.get(),
                    )
                };

                on_submit.run(asset);
            }
        >
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
                    prop:value=move || category.get().to_string()
                    on:change=move |ev| {
                        let val: usize = event_target_value(&ev).parse().unwrap_or(0);
                        category.set(val);
                    }
                >
                    {Category::all().into_iter().enumerate().map(|(i, cat)| {
                        let key = cat.i18n_key().to_string();
                        view! {
                            <option value=i.to_string()>{move || i18n.t(&key)}</option>
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
                    prop:value=move || depreciation_method.get().to_string()
                    on:change=move |ev| {
                        let val: usize = event_target_value(&ev).parse().unwrap_or(0);
                        depreciation_method.set(val);
                    }
                >
                    {DepreciationMethod::all().into_iter().enumerate().map(|(i, method)| {
                        let key = method.i18n_key().to_string();
                        view! {
                            <option value=i.to_string()>{move || i18n.t(&key)}</option>
                        }
                    }).collect::<Vec<_>>()}
                </select>
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

            // Status (only show in edit mode)
            {if is_edit {
                Some(view! {
                    <div>
                        <label class="label">{move || i18n.t("asset.status")}</label>
                        <select
                            class="input-field"
                            prop:value=move || status.get().to_string()
                            on:change=move |ev| {
                                let val: usize = event_target_value(&ev).parse().unwrap_or(0);
                                status.set(val);
                            }
                        >
                            {AssetStatus::all().into_iter().enumerate().map(|(i, s)| {
                                let key = s.i18n_key().to_string();
                                view! {
                                    <option value=i.to_string()>{move || i18n.t(&key)}</option>
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

            // Submit
            <div class="pt-2">
                <button type="submit" class="btn-primary">{submit_label}</button>
            </div>
        </form>
    }
}
