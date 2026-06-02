# React Native Auditor Roadmap

## Product

React Native Auditor is a local-first React Native & Expo project auditor for release readiness, dependency health, and risk reports.

It is not a replacement for `react-native doctor`.

It focuses on project-level risks:

- dependency setup
- lockfile consistency
- Expo / React Native config
- release readiness
- risky patterns
- common setup mistakes

## Current status

Done:

- Rust CLI foundation
- `rn-auditor audit`
- `rn-auditor scan`
- `scan` alias for `audit`
- Optional path argument
- Default to current directory
- Path exists validation
- Path is directory validation
- `package.json` parser using `serde` / `serde_json`
- Project type detection:
  - Expo
  - React Native
  - Unknown
- Terminal report displays project type
- `cargo fmt`, `cargo check`, and `cargo clippy` clean

## MVP execution plan

### Step 1 — Package manager and lockfile detection

Detect package manager from root lockfiles:

- `yarn.lock` => Yarn
- `package-lock.json` => npm
- `pnpm-lock.yaml` => pnpm
- `bun.lock` => Bun
- `bun.lockb` => Bun

Behavior:

- no lockfile => Package manager: Unknown
- one package manager lockfile => show package manager
- more than one package manager lockfile => Package manager: Multiple / Ambiguous
- create warning issue for multiple lockfiles
- do not auto-delete lockfiles

### Step 2 — Env rule

Detect environment file risk:

- `.env` exists
- `.env.example` missing

Behavior:

- create warning issue if `.env` exists but `.env.example` is missing
- never print secret values
- do not parse or display `.env` values

### Step 3 — Static app.json parser

Parse `app.json` safely as JSON.

Goal:

- support static Expo config only
- do not execute JS/TS config files
- prepare for app identity checks

### Step 4 — Expo app identity rules

Check static `app.json` for:

- missing `expo.ios.bundleIdentifier`
- missing `expo.android.package`

Behavior:

- warn when release identity config is missing
- skip deep validation if project uses dynamic app config

### Step 5 — Dynamic app config detection

Detect:

- `app.config.js`
- `app.config.ts`

Behavior:

- do not execute these files
- report that dynamic config exists
- explain that some static checks may be skipped or limited

### Step 6 — eas.json parser

Parse `eas.json` safely as JSON.

Goal:

- prepare for EAS release readiness checks
- do not execute anything

### Step 7 — EAS production profile rules

Check:

- missing `eas.json`
- missing `build.production`
- weak or incomplete production profile where possible

Keep the rule conservative.

### Step 8 — Reanimated Babel setup rule

If `react-native-reanimated` is installed, check whether `babel.config.js` appears to include:

- `react-native-reanimated/plugin`

Behavior:

- use safe text scanning only
- do not execute Babel config
- warn if likely missing

### Step 9 — Basic HTML report

Add:

- `rn-auditor report --html`

or prepare reporter architecture for HTML output.

Keep HTML static and local.

### Step 10 — Integration tests

Add CLI integration tests for:

- default path
- custom path
- invalid path
- path is file
- package manager detection
- multiple lockfile warning
- env warning

### Step 11 — Architecture cleanup

Refactor only when useful.

Possible future structure:

```text
src/
  main.rs
  cli.rs
  project.rs
  issue.rs
  scanner.rs
  parsers/
    mod.rs
    package_json.rs
    app_json.rs
    eas_json.rs
  rules/
    mod.rs
    lockfiles.rs
    env.rs
    app_config.rs
    eas.rs
    reanimated.rs
  reporters/
    mod.rs
    terminal.rs
    html.rs