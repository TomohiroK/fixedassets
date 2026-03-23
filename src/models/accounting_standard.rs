use leptos::prelude::*;

/// Accounting standard: Local (tax-based) or IFRS
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AccountingStandard {
    Local,
    IFRS,
}

impl AccountingStandard {
    pub fn is_ifrs(&self) -> bool {
        matches!(self, AccountingStandard::IFRS)
    }

    pub fn is_local(&self) -> bool {
        matches!(self, AccountingStandard::Local)
    }

    pub fn label_key(&self) -> &str {
        match self {
            AccountingStandard::Local => "standard.local",
            AccountingStandard::IFRS => "standard.ifrs",
        }
    }
}

/// Per-company accounting standard signal — provided as context
#[derive(Clone, Copy)]
pub struct AccountingStandardSignal(pub RwSignal<AccountingStandard>);

fn storage_key() -> String {
    let cid = crate::auth::get_current_company_id();
    if cid.is_empty() {
        "fa_accounting_standard".to_string()
    } else {
        format!("fa_accounting_standard_{}", cid)
    }
}

impl AccountingStandardSignal {
    pub fn new() -> Self {
        // Load from localStorage (scoped by company_id)
        let key = storage_key();
        let initial = if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                match storage.get_item(&key).ok().flatten().as_deref() {
                    Some("IFRS") => AccountingStandard::IFRS,
                    _ => AccountingStandard::Local,
                }
            } else {
                AccountingStandard::Local
            }
        } else {
            AccountingStandard::Local
        };
        Self(RwSignal::new(initial))
    }

    pub fn get(&self) -> AccountingStandard {
        self.0.get()
    }

    pub fn toggle(&self) {
        self.0.update(|s| {
            *s = match s {
                AccountingStandard::Local => AccountingStandard::IFRS,
                AccountingStandard::IFRS => AccountingStandard::Local,
            };
            // Persist to localStorage (scoped by company_id)
            let key = storage_key();
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    let val = match s {
                        AccountingStandard::IFRS => "IFRS",
                        AccountingStandard::Local => "Local",
                    };
                    let _ = storage.set_item(&key, val);
                }
            }
        });
    }

    pub fn is_ifrs(&self) -> bool {
        self.0.get().is_ifrs()
    }
}

pub fn use_accounting_standard() -> AccountingStandardSignal {
    use_context::<AccountingStandardSignal>().expect("AccountingStandardSignal not provided")
}
