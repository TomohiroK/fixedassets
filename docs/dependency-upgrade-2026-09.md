# Dependency upgrade — 2026-09-11

## Versions

- Rust: 1.94.0 → 1.98.1 (repository-pinned; host default unchanged).
- Leptos: 0.7.8 → 0.8.20; leptos_router: 0.7.8 → 0.8.15.
- wasm-bindgen: 0.2.114 → 0.2.128; web-sys/js-sys: 0.3.91 → 0.3.105.
- rust_decimal: 1.40.0 → 1.43.0; chrono: 0.4.44 → 0.4.45.
- sha2: 0.10 → 0.11.0; getrandom: 0.2 → 0.4.3 with wasm_js.
- Trunk: 0.21.14 (already current); Tailwind: explicitly pinned to 3.4.19.
- Vercel CLI: 59.13.1 globally → 59.16.0 locally via npm scripts/npx.

Tailwind remains on v3 to avoid changing styles and browser compatibility as part of this upgrade.
The existing Vercel prebuilt deployment and Basic Auth configuration remain in place.
Use `npm ci`, `npm run build`, and `npm run deploy` from the repository root.
The deploy command publishes to production, as before; it was not executed during validation.
If NO_COLOR is set to 1 by the calling environment, use `NO_COLOR=true npm run build` for Trunk 0.21.14.

## Compatibility changes

Leptos 0.8 LocalResource returns its data without SendWrapper. Removed eight obsolete
unwrap/dereference operations across dashboard, asset list/detail/register, photos and depreciation.
Reference: https://github.com/leptos-rs/leptos/releases/tag/v0.8.0

The getrandom API now uses fill. Randomness failure aborts salt generation instead of
silently using a zero-filled salt. Existing hash format and storage keys are unchanged.

## Validation

- Release Trunk build with locked Cargo dependencies: passed, including generated JS/WASM/CSS.
- cargo test --locked: 3 passed (legacy hash compatibility, salt generation, stored-user compatibility).
- cargo clippy --locked --target wasm32-unknown-unknown: passed with 54 warnings.
- Shell deployment script and middleware JavaScript syntax: passed.
- Local browser: initial page, demo login, asset registration, asset list/card/detail,
  empty photo gallery; no console errors in the successful localhost session.
- Browser used a dedicated localhost origin and a synthetic asset. Production and existing user data were not used.
- Financial calculation correctness, photo upload, and production Vercel execution were not revalidated.

## Remaining upstream warnings

Vercel CLI 59.16.0 pins tar 7.5.7 through its tool dependencies. A same-major override to
7.5.22 removes the critical advisory; re-check this override when upgrading Vercel again.
After the override, npm reports 27 development dependency vulnerabilities
(1 low, 11 moderate, 15 high; 0 critical). These dependencies are deployment tools,
not bundled into the browser WASM application. Do not use npm audit fix --force blindly:
it proposes a Vercel downgrade and other potentially incompatible changes.

Rust reports a future-compatibility warning in proc-macro-error2 2.0.1.
Current Rust 1.98.1 builds successfully; track upstream before the next toolchain upgrade.
