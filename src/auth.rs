use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};

// ─── Constants ───────────────────────────────────────────────────────
pub const FREE_ASSET_LIMIT: usize = 5;
const SESSION_TIMEOUT_MS: f64 = 30.0 * 60.0 * 1000.0; // 30 minutes
const MAX_LOGIN_ATTEMPTS: u32 = 5;
const LOCKOUT_DURATION_MS: f64 = 15.0 * 60.0 * 1000.0; // 15 minutes

// ─── Public User (no password) ──────────────────────────────────────
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub email: String,
    pub name: String,
    #[serde(default)]
    pub paid: bool,
    #[serde(default)]
    pub company_id: String,
}

// ─── Stored User (hashed password) ──────────────────────────────────
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StoredUser {
    pub email: String,
    pub name: String,
    /// SHA-256(salt + password) hex string
    pub password_hash: String,
    /// Random hex salt
    pub salt: String,
    #[serde(default)]
    pub paid: bool,
    #[serde(default)]
    pub company_id: String,
    // Legacy field — consumed on first login, then removed
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// Last login timestamp (ms since epoch) — used for inactive account cleanup
    #[serde(default)]
    pub last_login: f64,
}

// ─── Session with timestamp ─────────────────────────────────────────
#[derive(Clone, Debug, Serialize, Deserialize)]
struct StoredSession {
    user: User,
    last_active: f64, // js timestamp (ms since epoch)
}

// ─── Rate limiting ──────────────────────────────────────────────────
#[derive(Clone, Debug, Serialize, Deserialize)]
struct LoginAttempts {
    count: u32,
    first_attempt_at: f64,
    locked_until: Option<f64>,
}

// ─── Hashing helpers ────────────────────────────────────────────────
fn generate_salt() -> String {
    let mut buf = [0u8; 16];
    getrandom::getrandom(&mut buf).unwrap_or_default();
    hex::encode(buf)
}

fn hash_password(password: &str, salt: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(salt.as_bytes());
    hasher.update(password.as_bytes());
    hex::encode(hasher.finalize())
}

fn verify_password(password: &str, salt: &str, hash: &str) -> bool {
    hash_password(password, salt) == hash
}

// ─── Password strength validation ───────────────────────────────────
pub fn validate_password_strength(password: &str) -> Result<(), String> {
    if password.len() < 8 {
        return Err("password_too_short".to_string());
    }
    let has_upper = password.chars().any(|c| c.is_ascii_uppercase());
    let has_lower = password.chars().any(|c| c.is_ascii_lowercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    if !has_upper || !has_lower || !has_digit {
        return Err("password_weak".to_string());
    }
    Ok(())
}

// ─── Timestamp helper ───────────────────────────────────────────────
fn now_ms() -> f64 {
    js_sys::Date::now()
}

// ─── AuthState ──────────────────────────────────────────────────────
#[derive(Clone, Copy, Debug)]
pub struct AuthState {
    pub user: RwSignal<Option<User>>,
}

impl AuthState {
    pub fn new() -> Self {
        seed_demo_accounts();
        // Migrate legacy plaintext passwords
        migrate_legacy_passwords();
        // Load session with timeout check
        let stored_user = load_session();
        AuthState {
            user: RwSignal::new(stored_user),
        }
    }

    pub fn is_logged_in(&self) -> bool {
        self.user.get().is_some()
    }

    pub fn login(&self, email: String, name: String, paid: bool, company_id: String) {
        let user = User { email, name, paid, company_id };
        save_session(&user);
        self.user.set(Some(user));
    }

    pub fn current_company_id(&self) -> String {
        self.user.get().map(|u| u.company_id.clone()).unwrap_or_default()
    }

    pub fn is_paid(&self) -> bool {
        self.user.get().map(|u| u.paid).unwrap_or(false)
    }

    pub fn signup(&self, email: String, name: String, password: String) -> Result<(), String> {
        // Validate password strength
        validate_password_strength(&password)?;

        let users_json = get_stored_string("fa_users").unwrap_or_else(|| "[]".to_string());
        let mut users: Vec<StoredUser> = serde_json::from_str(&users_json).unwrap_or_default();

        if users.iter().any(|u| u.email == email) {
            return Err("Email already registered".to_string());
        }

        let salt = generate_salt();
        let password_hash = hash_password(&password, &salt);
        let company_id = uuid::Uuid::new_v4().to_string();

        users.push(StoredUser {
            email: email.clone(),
            name: name.clone(),
            password_hash,
            salt,
            paid: false,
            company_id: company_id.clone(),
            password: None,
            last_login: now_ms(),
        });

        let json = serde_json::to_string(&users).unwrap_or_default();
        store_string("fa_users", &json);

        self.login(email, name, false, company_id);
        Ok(())
    }

    pub fn authenticate(&self, email: String, password: String) -> Result<(), String> {
        // Rate limiting check (scoped per email)
        if let Err(msg) = check_rate_limit(&email) {
            return Err(msg);
        }

        let users_json = get_stored_string("fa_users").unwrap_or_else(|| "[]".to_string());
        let users: Vec<StoredUser> = serde_json::from_str(&users_json).unwrap_or_default();

        match users.iter().find(|u| u.email == email) {
            Some(u) => {
                let valid = if !u.password_hash.is_empty() {
                    // New hashed password
                    verify_password(&password, &u.salt, &u.password_hash)
                } else if let Some(ref legacy_pw) = u.password {
                    // Legacy plaintext (should be migrated, but fallback)
                    legacy_pw == &password
                } else {
                    false
                };

                if valid {
                    reset_login_attempts(&email);
                    // Update last_login timestamp
                    update_last_login(&email);
                    self.login(u.email.clone(), u.name.clone(), u.paid, u.company_id.clone());
                    // Ensure company setup exists (handles cross-browser login)
                    crate::models::company::CompanySetup::ensure_exists();
                    Ok(())
                } else {
                    record_failed_attempt(&email);
                    Err("Invalid email or password".to_string())
                }
            }
            None => {
                record_failed_attempt(&email);
                Err("Invalid email or password".to_string())
            }
        }
    }

    /// Touch session to extend timeout
    pub fn touch_session(&self) {
        if let Some(user) = self.user.get() {
            save_session(&user);
        }
    }

    /// Check if session is still valid
    pub fn check_session_timeout(&self) -> bool {
        if self.user.get().is_none() {
            return false;
        }
        // Re-verify from storage
        if load_session().is_none() {
            self.user.set(None);
            return false;
        }
        true
    }

    /// Update account: name, email, and optionally password.
    /// Requires current_password for verification.
    /// new_password can be empty/None to keep current password.
    pub fn update_account(
        &self,
        current_password: String,
        new_name: String,
        new_email: String,
        new_password: Option<String>,
    ) -> Result<(), String> {
        let current_user = self.user.get().ok_or("Not logged in")?;
        let current_email = current_user.email.clone();

        let users_json = get_stored_string("fa_users").unwrap_or_else(|| "[]".to_string());
        let mut users: Vec<StoredUser> = serde_json::from_str(&users_json).unwrap_or_default();

        // Find current user and verify password
        let user_idx = users.iter().position(|u| u.email == current_email)
            .ok_or("User not found")?;

        let valid = if !users[user_idx].password_hash.is_empty() {
            verify_password(&current_password, &users[user_idx].salt, &users[user_idx].password_hash)
        } else if let Some(ref legacy_pw) = users[user_idx].password {
            legacy_pw == &current_password
        } else {
            false
        };

        if !valid {
            return Err("wrong_password".to_string());
        }

        // Check if new email is taken by another user
        if new_email != current_email {
            if users.iter().any(|u| u.email == new_email && u.email != current_email) {
                return Err("email_taken".to_string());
            }
        }

        // Validate new password if provided
        if let Some(ref pw) = new_password {
            if !pw.is_empty() {
                validate_password_strength(pw)?;
            }
        }

        // Apply changes
        users[user_idx].name = new_name.clone();
        users[user_idx].email = new_email.clone();

        if let Some(ref pw) = new_password {
            if !pw.is_empty() {
                let salt = generate_salt();
                users[user_idx].password_hash = hash_password(pw, &salt);
                users[user_idx].salt = salt;
                users[user_idx].password = None;
            }
        }

        // Save users
        if let Ok(json) = serde_json::to_string(&users) {
            store_string("fa_users", &json);
        }

        // Update session
        self.login(
            new_email,
            new_name,
            users[user_idx].paid,
            users[user_idx].company_id.clone(),
        );

        Ok(())
    }

    pub fn logout(&self) {
        // Clear session
        remove_stored("fa_user");
        // Clear login attempts
        remove_stored("fa_login_attempts");
        self.user.set(None);
    }
}

// ─── Session persistence with timeout ───────────────────────────────
fn save_session(user: &User) {
    let session = StoredSession {
        user: user.clone(),
        last_active: now_ms(),
    };
    if let Ok(json) = serde_json::to_string(&session) {
        store_string("fa_user", &json);
    }
}

fn load_session() -> Option<User> {
    let json = get_stored_string("fa_user")?;

    // Try new format (with timeout)
    if let Ok(session) = serde_json::from_str::<StoredSession>(&json) {
        let elapsed = now_ms() - session.last_active;
        if elapsed > SESSION_TIMEOUT_MS {
            // Session expired
            remove_stored("fa_user");
            return None;
        }
        // Refresh session timestamp
        save_session(&session.user);
        return Some(session.user);
    }

    // Legacy format (plain User without timestamp) — migrate
    if let Ok(user) = serde_json::from_str::<User>(&json) {
        save_session(&user);
        return Some(user);
    }

    None
}

// ─── Rate limiting (scoped per email) ───────────────────────────────
fn login_attempts_key(email: &str) -> String {
    // Use a sanitized email as part of the key for per-user rate limiting
    let safe_email = email.to_lowercase().replace(|c: char| !c.is_alphanumeric() && c != '@' && c != '.', "_");
    format!("fa_login_attempts_{}", safe_email)
}

fn get_login_attempts(email: &str) -> LoginAttempts {
    get_stored_string(&login_attempts_key(email))
        .and_then(|json| serde_json::from_str(&json).ok())
        .unwrap_or(LoginAttempts {
            count: 0,
            first_attempt_at: 0.0,
            locked_until: None,
        })
}

fn save_login_attempts(email: &str, attempts: &LoginAttempts) {
    if let Ok(json) = serde_json::to_string(attempts) {
        store_string(&login_attempts_key(email), &json);
    }
}

fn check_rate_limit(email: &str) -> Result<(), String> {
    let attempts = get_login_attempts(email);
    let now = now_ms();

    // Check if locked
    if let Some(locked_until) = attempts.locked_until {
        if now < locked_until {
            let remaining_sec = ((locked_until - now) / 1000.0).ceil() as u32;
            let remaining_min = remaining_sec / 60;
            return Err(format!(
                "Too many login attempts. Try again in {} min {} sec.",
                remaining_min,
                remaining_sec % 60
            ));
        }
        // Lockout expired, reset
        reset_login_attempts(email);
    }

    Ok(())
}

fn record_failed_attempt(email: &str) {
    let mut attempts = get_login_attempts(email);
    let now = now_ms();

    // Reset if the window has passed (15 min)
    if now - attempts.first_attempt_at > LOCKOUT_DURATION_MS {
        attempts = LoginAttempts {
            count: 0,
            first_attempt_at: now,
            locked_until: None,
        };
    }

    if attempts.count == 0 {
        attempts.first_attempt_at = now;
    }

    attempts.count += 1;

    if attempts.count >= MAX_LOGIN_ATTEMPTS {
        attempts.locked_until = Some(now + LOCKOUT_DURATION_MS);
    }

    save_login_attempts(email, &attempts);
}

fn reset_login_attempts(email: &str) {
    remove_stored(&login_attempts_key(email));
}

// ─── Demo accounts (hashed) ─────────────────────────────────────────
fn seed_demo_accounts() {
    let users_json = get_stored_string("fa_users").unwrap_or_else(|| "[]".to_string());
    let users: Vec<StoredUser> = serde_json::from_str(&users_json).unwrap_or_default();
    if !users.is_empty() {
        return;
    }

    let demo_accounts = vec![
        ("demo@example.com", "Demo User", "Demo1234"),
        ("admin@example.com", "Admin", "Admin1234"),
        ("tanaka@example.com", "田中太郎", "Tanaka1234"),
    ];

    let demo_users: Vec<StoredUser> = demo_accounts
        .into_iter()
        .map(|(email, name, pw)| {
            let salt = generate_salt();
            let password_hash = hash_password(pw, &salt);
            StoredUser {
                email: email.to_string(),
                name: name.to_string(),
                password_hash,
                salt,
                paid: false,
                company_id: uuid::Uuid::new_v4().to_string(),
                password: None,
                last_login: now_ms(),
            }
        })
        .collect();

    if let Ok(json) = serde_json::to_string(&demo_users) {
        store_string("fa_users", &json);
    }
}

/// New demo passwords (used for reset during migration)
const DEMO_PASSWORDS: &[(&str, &str)] = &[
    ("demo@example.com", "Demo1234"),
    ("admin@example.com", "Admin1234"),
    ("tanaka@example.com", "Tanaka1234"),
];

/// Migrate legacy plaintext passwords to hashed.
/// Demo accounts get reset to the NEW strong passwords.
/// Non-demo accounts keep their existing password (just hashed).
fn migrate_legacy_passwords() {
    let users_json = get_stored_string("fa_users").unwrap_or_else(|| "[]".to_string());
    let mut users: Vec<StoredUser> = serde_json::from_str(&users_json).unwrap_or_default();
    let mut changed = false;

    for user in users.iter_mut() {
        // Migrate missing company_id
        if user.company_id.is_empty() {
            user.company_id = uuid::Uuid::new_v4().to_string();
            changed = true;
        }

        let needs_migration = user.password.is_some()
            || user.password_hash.is_empty()
            || user.salt.is_empty();

        if needs_migration {
            // Check if this is a demo account — reset to new password
            let new_pw = DEMO_PASSWORDS
                .iter()
                .find(|(email, _)| *email == user.email)
                .map(|(_, pw)| *pw);

            let password_to_hash = if let Some(demo_pw) = new_pw {
                // Demo account: use new strong password
                demo_pw.to_string()
            } else if let Some(ref legacy_pw) = user.password {
                // Non-demo: preserve their existing password (just hash it)
                legacy_pw.clone()
            } else {
                // No password at all — skip (user must re-register)
                continue;
            };

            let salt = generate_salt();
            user.password_hash = hash_password(&password_to_hash, &salt);
            user.salt = salt;
            user.password = None;
            changed = true;
        }
    }

    if changed {
        if let Ok(json) = serde_json::to_string(&users) {
            store_string("fa_users", &json);
        }
    }
}

// ─── Public helpers ─────────────────────────────────────────────────
pub fn get_all_stored_users() -> Vec<StoredUser> {
    let json = get_stored_string("fa_users").unwrap_or_else(|| "[]".to_string());
    serde_json::from_str(&json).unwrap_or_default()
}

pub fn toggle_user_paid(email: &str) -> bool {
    let mut users = get_all_stored_users();
    let mut new_paid = false;
    for u in users.iter_mut() {
        if u.email == email {
            u.paid = !u.paid;
            new_paid = u.paid;
        }
    }
    if let Ok(json) = serde_json::to_string(&users) {
        store_string("fa_users", &json);
    }
    new_paid
}

/// Verify admin password (hashed comparison)
pub fn verify_admin_password(admin_email: &str, password: &str) -> bool {
    // Rate limiting check (scoped per email)
    if check_rate_limit(admin_email).is_err() {
        return false;
    }

    let users = get_all_stored_users();
    if let Some(admin) = users.iter().find(|u| u.email == admin_email) {
        let valid = if !admin.password_hash.is_empty() {
            verify_password(password, &admin.salt, &admin.password_hash)
        } else if let Some(ref legacy_pw) = admin.password {
            legacy_pw == password
        } else {
            false
        };

        if valid {
            reset_login_attempts(admin_email);
            true
        } else {
            record_failed_attempt(admin_email);
            false
        }
    } else {
        record_failed_attempt(admin_email);
        false
    }
}

pub fn use_auth() -> AuthState {
    expect_context::<AuthState>()
}

/// Get current company_id from session (for use in stores without AuthState context)
pub fn get_current_company_id() -> String {
    load_session().map(|u| u.company_id).unwrap_or_default()
}

// ─── localStorage helpers ───────────────────────────────────────────
fn get_stored_string(key: &str) -> Option<String> {
    let window = web_sys::window()?;
    let storage = window.local_storage().ok()??;
    storage.get_item(key).ok()?
}

fn store_string(key: &str, value: &str) {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.set_item(key, value);
        }
    }
}

fn remove_stored(key: &str) {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.remove_item(key);
        }
    }
}

// ─── Last login tracking ────────────────────────────────────────────
fn update_last_login(email: &str) {
    let users_json = get_stored_string("fa_users").unwrap_or_else(|| "[]".to_string());
    let mut users: Vec<StoredUser> = serde_json::from_str(&users_json).unwrap_or_default();
    let mut changed = false;
    for user in users.iter_mut() {
        if user.email == email {
            user.last_login = now_ms();
            changed = true;
        }
    }
    if changed {
        if let Ok(json) = serde_json::to_string(&users) {
            store_string("fa_users", &json);
        }
    }
}

// ─── Inactive account cleanup job ───────────────────────────────────
const CLEANUP_LAST_RUN_KEY: &str = "fa_cleanup_last_run";
const INACTIVE_THRESHOLD_DAYS: f64 = 40.0;
const CLEANUP_INTERVAL_MS: f64 = 7.0 * 24.0 * 60.0 * 60.0 * 1000.0; // 7 days

/// Weekly cleanup job: removes free accounts inactive for 40+ days.
/// Runs at most once per week (on Sunday, or if 7+ days since last run).
/// Skips paid accounts and demo/admin accounts.
pub fn run_inactive_account_cleanup() {
    let now = now_ms();

    // Check if it's Sunday (day 0 in JS)
    let today = js_sys::Date::new_0();
    let is_sunday = today.get_day() == 0;

    // Check last run time
    let last_run: f64 = get_stored_string(CLEANUP_LAST_RUN_KEY)
        .and_then(|v| v.parse().ok())
        .unwrap_or(0.0);

    let elapsed = now - last_run;

    // Only run on Sundays, and at most once per 7 days
    if !is_sunday || elapsed < CLEANUP_INTERVAL_MS {
        return;
    }

    let users_json = get_stored_string("fa_users").unwrap_or_else(|| "[]".to_string());
    let users: Vec<StoredUser> = serde_json::from_str(&users_json).unwrap_or_default();

    let threshold_ms = INACTIVE_THRESHOLD_DAYS * 24.0 * 60.0 * 60.0 * 1000.0;
    let mut removed_company_ids: Vec<String> = Vec::new();

    let remaining: Vec<StoredUser> = users
        .into_iter()
        .filter(|u| {
            // Keep paid accounts
            if u.paid {
                return true;
            }
            // Keep accounts with no last_login set (legacy — treat as active)
            if u.last_login == 0.0 {
                return true;
            }
            // Check if inactive for 40+ days
            let inactive_ms = now - u.last_login;
            if inactive_ms > threshold_ms {
                removed_company_ids.push(u.company_id.clone());
                false // Remove
            } else {
                true // Keep
            }
        })
        .collect();

    if !removed_company_ids.is_empty() {
        // Save updated user list
        if let Ok(json) = serde_json::to_string(&remaining) {
            store_string("fa_users", &json);
        }

        // Clean up all company-scoped data
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                for cid in &removed_company_ids {
                    let _ = storage.remove_item(&format!("fa_company_setup_{}", cid));
                    let _ = storage.remove_item(&format!("fa_departments_{}", cid));
                    let _ = storage.remove_item(&format!("fa_accounting_standard_{}", cid));
                }
            }
        }

        web_sys::console::log_1(
            &format!(
                "[Ledgea] Cleanup: removed {} inactive free account(s)",
                removed_company_ids.len()
            ).into()
        );
    }

    // Record last run time
    store_string(CLEANUP_LAST_RUN_KEY, &now.to_string());
}
