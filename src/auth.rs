use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub email: String,
    pub name: String,
}

#[derive(Clone, Copy, Debug)]
pub struct AuthState {
    pub user: RwSignal<Option<User>>,
}

impl AuthState {
    pub fn new() -> Self {
        seed_demo_accounts();
        let stored_user = get_stored_user();
        AuthState {
            user: RwSignal::new(stored_user),
        }
    }

    pub fn is_logged_in(&self) -> bool {
        self.user.get().is_some()
    }

    pub fn login(&self, email: String, name: String) {
        let user = User { email, name };
        store_user(&user);
        self.user.set(Some(user));
    }

    pub fn signup(&self, email: String, name: String, _password: String) -> Result<(), String> {
        // For now, store credentials in localStorage
        // Later replace with server API call
        let users_json = get_stored_string("fa_users").unwrap_or_else(|| "[]".to_string());
        let mut users: Vec<StoredUser> = serde_json::from_str(&users_json).unwrap_or_default();

        if users.iter().any(|u| u.email == email) {
            return Err("Email already registered".to_string());
        }

        users.push(StoredUser {
            email: email.clone(),
            name: name.clone(),
            password: _password,
        });

        let json = serde_json::to_string(&users).unwrap_or_default();
        store_string("fa_users", &json);

        self.login(email, name);
        Ok(())
    }

    pub fn authenticate(&self, email: String, password: String) -> Result<(), String> {
        let users_json = get_stored_string("fa_users").unwrap_or_else(|| "[]".to_string());
        let users: Vec<StoredUser> = serde_json::from_str(&users_json).unwrap_or_default();

        match users.iter().find(|u| u.email == email && u.password == password) {
            Some(u) => {
                self.login(u.email.clone(), u.name.clone());
                Ok(())
            }
            None => Err("Invalid email or password".to_string()),
        }
    }

    pub fn logout(&self) {
        remove_stored("fa_user");
        self.user.set(None);
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct StoredUser {
    email: String,
    name: String,
    password: String,
}

fn get_stored_user() -> Option<User> {
    let json = get_stored_string("fa_user")?;
    serde_json::from_str(&json).ok()
}

fn store_user(user: &User) {
    if let Ok(json) = serde_json::to_string(user) {
        store_string("fa_user", &json);
    }
}

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

fn seed_demo_accounts() {
    let users_json = get_stored_string("fa_users").unwrap_or_else(|| "[]".to_string());
    let users: Vec<StoredUser> = serde_json::from_str(&users_json).unwrap_or_default();
    if !users.is_empty() {
        return;
    }

    let demo_users = vec![
        StoredUser {
            email: "demo@example.com".to_string(),
            name: "Demo User".to_string(),
            password: "demo123".to_string(),
        },
        StoredUser {
            email: "admin@example.com".to_string(),
            name: "Admin".to_string(),
            password: "admin123".to_string(),
        },
        StoredUser {
            email: "tanaka@example.com".to_string(),
            name: "田中太郎".to_string(),
            password: "tanaka123".to_string(),
        },
    ];

    if let Ok(json) = serde_json::to_string(&demo_users) {
        store_string("fa_users", &json);
    }
}

pub fn use_auth() -> AuthState {
    expect_context::<AuthState>()
}
