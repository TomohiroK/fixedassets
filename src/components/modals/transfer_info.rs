use leptos::prelude::*;
use crate::i18n::use_i18n;
use crate::models::asset::TransferRecord;
use crate::models::department::Department;

/// Section showing department transfer history
#[component]
pub fn TransferInfoSection(
    transfers: Vec<TransferRecord>,
    current_department_id: Option<String>,
) -> impl IntoView {
    let i18n = use_i18n();
    let count = transfers.len();

    let current_name = current_department_id
        .as_ref()
        .map(|id| Department::display_name(id))
        .unwrap_or_else(|| "—".to_string());

    view! {
        <div class="mt-4 border rounded-xl p-4 bg-indigo-50 border-indigo-200">
            <h3 class="text-sm font-semibold mb-3 flex items-center gap-2 text-indigo-800">
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7h12m0 0l-4-4m4 4l-4 4m0 6H4m0 0l4 4m-4-4l4-4"/>
                </svg>
                {move || i18n.t("asset.transfer_history")}
                <span class="text-xs font-normal text-indigo-500">
                    "(" {count} {if count == 1 { " record" } else { " records" }} ")"
                </span>
            </h3>
            <div class="space-y-2 text-sm">
                // Current department
                <div class="flex justify-between items-center">
                    <span class="text-gray-600 font-medium">{move || i18n.t("asset.dept_current")}</span>
                    <span class="text-xs font-medium px-2 py-0.5 rounded-full bg-indigo-100 text-indigo-700">
                        {current_name.clone()}
                    </span>
                </div>

                // Transfer history (newest first)
                <div class="border-t border-indigo-200 pt-2 mt-2 space-y-2">
                    {transfers.into_iter().rev().enumerate().map(|(i, record)| {
                        let from_name = record.from_department_id.as_ref()
                            .map(|id| Department::display_name(id))
                            .unwrap_or_else(|| "—".to_string());
                        let to_name = Department::display_name(&record.to_department_id);
                        let date = record.date.clone();
                        let reason = record.reason.clone();
                        let has_reason = !reason.is_empty();
                        let label_num = count - i;
                        view! {
                            <div class="bg-white/60 rounded-lg p-2.5">
                                <div class="flex items-center justify-between mb-1">
                                    <span class="text-xs text-indigo-600 font-medium">"#" {label_num} " - " {date}</span>
                                </div>
                                <div class="flex items-center gap-1.5 text-xs">
                                    <span class="text-gray-500 bg-gray-100 px-1.5 py-0.5 rounded">{from_name}</span>
                                    <svg class="w-3 h-3 text-indigo-400 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3"/>
                                    </svg>
                                    <span class="text-indigo-700 bg-indigo-100 px-1.5 py-0.5 rounded font-medium">{to_name}</span>
                                </div>
                                {if has_reason {
                                    Some(view! {
                                        <p class="text-xs text-gray-500 mt-1">{reason}</p>
                                    })
                                } else {
                                    None
                                }}
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}
