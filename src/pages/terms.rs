use leptos::prelude::*;
use crate::i18n::use_i18n;
use crate::auth::use_auth;

#[component]
pub fn TermsPage() -> impl IntoView {
    let i18n = use_i18n();
    let auth = use_auth();

    // Back link: logged in → /settings, not logged in → /welcome
    let back_href = move || {
        if auth.is_logged_in() { "/settings" } else { "/welcome" }
    };

    view! {
        <div class="min-h-screen bg-gray-50">
            // Header
            <div class="bg-white/80 backdrop-blur-lg border-b border-gray-200/60 sticky top-0 z-10">
                <div class="max-w-2xl mx-auto px-4 py-3 flex items-center justify-between">
                    <a href=back_href class="text-gray-600 text-sm font-medium flex items-center gap-1">
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"/>
                        </svg>
                        {move || i18n.t("app.title")}
                    </a>
                    <button
                        class="text-xs text-gray-500 border border-gray-200 px-2.5 py-1 rounded-full active:bg-gray-100"
                        on:click=move |_| {
                            let current = i18n.current_locale();
                            let next = if current == "en" { "ja" } else { "en" };
                            i18n.set_locale(next);
                        }
                    >
                        {move || if i18n.current_locale() == "en" { "日本語" } else { "EN" }}
                    </button>
                </div>
            </div>

            // Content
            <div class="max-w-2xl mx-auto px-4 py-6">
                <article class="bg-white rounded-xl border border-gray-200 p-5 sm:p-8">

                    // Title
                    <h1 class="text-xl font-bold text-gray-900 mb-1">"TERMS OF SERVICE"</h1>
                    <p class="text-sm font-semibold text-blue-600 mb-6">"FA ASEAN ASSET"</p>

                    <p class="text-sm text-gray-600 leading-relaxed mb-6">
                        "These Terms of Service (the \"Terms\") govern your access to and use of FA ASEAN ASSET (the \"Service\"). By accessing or using the Service, you agree to be bound by these Terms."
                    </p>

                    // Section 1
                    <Section number="1" title="Acceptance of Terms">
                        <p class="terms-text">
                            "By using the Service, you represent that you have the legal authority to bind the entity you represent to these Terms. If you do not agree to these Terms, do not use the Service."
                        </p>
                    </Section>

                    // Section 2
                    <Section number="2" title="Service Description">
                        <p class="terms-text">
                            "FA ASEAN ASSET is a fixed asset management system designed for businesses operating within the ASEAN region. The Service provides tools for calculating depreciation (e.g., Straight-line method, Declining balance method) and managing asset records according to local standards."
                        </p>
                    </Section>

                    // Section 3
                    <Section number="3" title="Account Registration and Restrictions">
                        <p class="terms-text mb-3">
                            "To ensure the integrity of the Service and compliance with regional data management, the following restrictions apply:"
                        </p>
                        <ul class="terms-list">
                            <li>
                                <strong>"One Account Per Entity Per Country: "</strong>
                                "Each legal entity is permitted to create only one (1) account per specific ASEAN country."
                            </li>
                            <li>
                                <strong>"Multiple Entities: "</strong>
                                "If a user represents multiple distinct legal entities or operates in multiple countries, a separate account must be created for each entity/country combination."
                            </li>
                            <li>
                                <strong>"Account Security: "</strong>
                                "You are responsible for maintaining the confidentiality of your account credentials and for all activities that occur under your account."
                            </li>
                        </ul>
                    </Section>

                    // Section 4
                    <Section number="4" title="Privacy and Data Collection">
                        <ul class="terms-list">
                            <li>
                                <strong>"No Personal Data Collection: "</strong>
                                "The Service is designed to manage corporate asset data only. The Service does not collect, store, or process Personal Data (as defined under various regional data protection laws such as Singapore\u{2019}s PDPA, Indonesia\u{2019}s PDP Law, etc.) of any individual."
                            </li>
                            <li>
                                <strong>"Prohibited Content: "</strong>
                                "Users are strictly prohibited from entering any personal identification information (e.g., names of individuals, private contact details, or government IDs) into any free-text fields or asset description areas within the Service."
                            </li>
                        </ul>
                    </Section>

                    // Section 5
                    <Section number="5" title="Use of Service and Restrictions">
                        <p class="terms-text mb-3">"You agree not to:"</p>
                        <ul class="terms-list">
                            <li>"Use the Service for any illegal purposes or in violation of local ASEAN regulations."</li>
                            <li>"Attempt to reverse engineer, decompile, or extract the source code of the Service."</li>
                            <li>"Interfere with or disrupt the integrity or performance of the Service."</li>
                        </ul>
                    </Section>

                    // Section 6
                    <Section number="6" title="General Disclaimers and Limitation of Liability">
                        <div class="space-y-4">
                            <div>
                                <h4 class="text-sm font-semibold text-gray-800 mb-1">"6.1. Accuracy of Calculations and Compliance"</h4>
                                <p class="terms-text">
                                    "The Service provides automated depreciation calculations based on standardized formulas. However, accounting standards, tax laws, and depreciation rates vary significantly across ASEAN member states and are subject to frequent changes. We do not warrant that the calculations provided by the Service will be 100% accurate, complete, or compliant with the specific local tax requirements of any particular jurisdiction at any given time."
                                </p>
                            </div>
                            <div>
                                <h4 class="text-sm font-semibold text-gray-800 mb-1">"6.2. No Professional Advice"</h4>
                                <p class="terms-text">
                                    "The output and information provided by the Service are for informational and administrative purposes only and do not constitute professional accounting, financial, tax, or legal advice. Users are solely responsible for verifying the accuracy of all data and calculations with a qualified professional (e.g., a certified public accountant or tax consultant) before filing tax returns or financial statements."
                                </p>
                            </div>
                            <div>
                                <h4 class="text-sm font-semibold text-gray-800 mb-1">"6.3. User Responsibility for Data Entry"</h4>
                                <p class="terms-text">
                                    "The accuracy of the Service\u{2019}s output is dependent on the data entered by the User. We shall not be held liable for any errors, losses, or penalties resulting from incorrect, incomplete, or inappropriate data input by the User."
                                </p>
                            </div>
                            <div>
                                <h4 class="text-sm font-semibold text-gray-800 mb-1">"6.4. Limitation of Liability for Local Penalties"</h4>
                                <p class="terms-text">
                                    "Under no circumstances shall the Service provider be liable for any fines, penalties, interest, or additional tax liabilities imposed on the User by any governmental or regulatory authority in any ASEAN country arising from the use of the Service."
                                </p>
                            </div>
                        </div>
                    </Section>

                    // Section 7
                    <Section number="7" title="Country-Specific Disclaimers">
                        <p class="terms-text mb-3">
                            "Depending on the country where the Service is used, the following specific terms apply:"
                        </p>
                        <ul class="terms-list">
                            <li>
                                <strong>"\u{1F1EE}\u{1F1E9} Indonesia: "</strong>
                                "Use of the Service does not satisfy the statutory bookkeeping requirements (Pembukuan) under Indonesian Tax Law unless integrated with approved accounting software. Users are responsible for ensuring all asset records comply with PSAK (Pernyataan Standar Akuntansi Keuangan)."
                            </li>
                            <li>
                                <strong>"\u{1F1F8}\u{1F1EC} Singapore: "</strong>
                                "The Service does not guarantee eligibility for tax incentives, such as the Productivity and Innovation Credit (PIC) or similar schemes. Calculation of capital allowances must be verified against current IRAS guidelines."
                            </li>
                            <li>
                                <strong>"\u{1F1F9}\u{1F1ED} Thailand: "</strong>
                                "Asset depreciation for tax purposes must comply with the Revenue Code of Thailand. The Service is not a substitute for the official \"Asset Register\" required by the Revenue Department."
                            </li>
                            <li>
                                <strong>"\u{1F1FB}\u{1F1F3} Vietnam: "</strong>
                                "Users are responsible for ensuring that depreciation methods and useful life ranges comply with Circular 45/2013/TT-BTC (and its amendments) issued by the Ministry of Finance."
                            </li>
                        </ul>
                    </Section>

                    // Section 8
                    <Section number="8" title="Termination">
                        <p class="terms-text">
                            "We reserve the right to suspend or terminate your account if you violate these Terms, including the \"one account per entity per country\" rule."
                        </p>
                    </Section>

                    // Section 9
                    <Section number="9" title="Governing Law">
                        <p class="terms-text">
                            "These Terms shall be governed by and construed in accordance with the laws of the jurisdiction where the service provider is registered, without regard to its conflict of law provisions."
                        </p>
                    </Section>

                    // Section 10
                    <Section number="10" title="Changes to Terms">
                        <p class="terms-text">
                            "We may modify these Terms at any time. Your continued use of the Service following the posting of changes constitutes your acceptance of such changes."
                        </p>
                    </Section>

                </article>

                // Footer
                <div class="text-center text-xs text-gray-400 mt-6 mb-8">
                    <p>{move || i18n.t("landing.footer")}</p>
                </div>
            </div>
        </div>
    }
}

/// Reusable section component
#[component]
fn Section(
    number: &'static str,
    title: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="mb-6">
            <h3 class="text-sm font-bold text-gray-900 mb-2">
                {format!("{}. {}", number, title)}
            </h3>
            {children()}
        </div>
    }
}
