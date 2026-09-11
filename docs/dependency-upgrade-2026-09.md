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

## Security remediation

The 27 npm audit findings were resolved on 2026-09-11. Vercel CLI remains at
59.16.0; package.json overrides replace vulnerable transitive dependencies.

| Dependency | Resolved version |
| --- | --- |
| tar | 7.5.22 |
| @tootallnate/once | 2.0.1 |
| ajv | 8.20.0 |
| js-yaml | 4.3.2 |
| minimatch (10.x) | 10.2.6 |
| path-to-regexp (6.x / 8.x) | 6.3.0 / 8.4.2 |
| smol-toml | 1.8.0 |
| undici (previously 5.x) | 6.28.1 |

Overrides are scoped by major where multiple major versions coexist. Undici 5.x
requires a major upgrade to 6.28.1; the existing 7.x dependency is preserved.
Undici release notes: https://github.com/nodejs/undici/releases/tag/v6.28.1
Review these overrides on future Vercel upgrades and remove them once upstream
requirements resolve to secure versions without overrides.

Validation after remediation:

- npm audit: 0 vulnerabilities, including development dependencies.
- npm ci --ignore-scripts: clean reinstall also reports 0 vulnerabilities.
- npm ls: dependency overrides resolve without invalid dependency errors.
- Vercel inspect successfully fetched the existing production deployment (Ready).
- Vercel-resolved Undici: local HTTP fetch POST and request GET passed.
- path-to-regexp 6.x through @vercel/node and @vercel/remix-builder, and 8.x:
  route matching passed.
- No new deployment was made as part of security remediation. Upload/deployment
  execution was not repeated; these changes affect local deployment tools, not
  the already deployed Rust/WASM assets.

This is a clean npm advisory audit at the time of checking, not a comprehensive
application security audit. The deprecated stream-to-promise warning remains;
it is not an npm vulnerability finding.

## Remaining Rust warning

Rust reports a future-compatibility warning in proc-macro-error2 2.0.1.
Current Rust 1.98.1 builds successfully; track upstream before the next toolchain upgrade.
