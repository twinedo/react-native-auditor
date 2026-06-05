# React Native Auditor

A local-first React Native & Expo project auditor for release readiness, dependency health, and risk reports.

React Native Auditor (`react-native-auditor`) is a rule-based CLI for finding project-level configuration, dependency setup, and release-readiness risks. The CLI command is `rn-auditor`, with `rn-auditor audit` as the main command and `rn-auditor scan` as its alias.

## What it is

React Native Auditor inspects a React Native or Expo project using conservative static checks. It reports what it can determine from files such as `package.json`, `app.json`, `eas.json`, lockfiles, and selected configuration files.

The v0.1 scope is deliberately small: practical checks, clear issue codes, terminal output, and a local HTML report.

## What it is not

React Native Auditor is not a replacement for `react-native doctor`. Doctor-style tools focus on the development environment; React Native Auditor focuses on project-level files and risks.

It is also not a SaaS dashboard, cloud service, command runner, automatic fixer, or plugin platform. Those capabilities are not part of v0.1.

## Current features

- Detects Expo, React Native, and unknown project types.
- Detects Yarn, npm, pnpm, Bun, unknown, and ambiguous package manager states from root lockfiles.
- Reports configuration and release-readiness findings with issue codes.
- Parses static JSON configuration without evaluating JavaScript or TypeScript.
- Prints a terminal report.
- Writes a static HTML report locally.
- Accepts an optional project path and defaults to the current directory.

## Installation and usage

The npm package is not published yet. Once v0.1 is available on npm, the supported installation paths will be:

### Quick run

```bash
npx react-native-auditor audit
```

### Global install

```bash
npm install -g react-native-auditor
rn-auditor audit
```

### Project install

```bash
yarn add -D react-native-auditor
yarn rn-auditor audit
```

To use the current repository before the npm release, install the Rust binary from the repository root:

```bash
cargo install --path crates/rn-auditor
rn-auditor audit
```

### Audit another project

```bash
rn-auditor audit /path/to/project
rn-auditor scan /path/to/project
```

### Generate an HTML report

```bash
rn-auditor report --html
rn-auditor report --html /path/to/project
rn-auditor report --html /path/to/project --output report.html
```

Without `--output`, the report is written to `rn-auditor-report.html` in the current directory.

## Commands

### `rn-auditor audit [path]`

Runs the current audit rules and prints a terminal report. If `path` is omitted, the current directory is audited.

### `rn-auditor scan [path]`

Alias for `rn-auditor audit`.

### `rn-auditor report --html [path] [--output <file>]`

Runs the same audit and writes a local HTML report. The `--html` flag is required.

The following commands and options are roadmap ideas, not available in v0.1:

- `rn-auditor analyze-log`
- `rn-auditor release-check`
- `rn-auditor audit --fix --interactive`

## Example output

```text
React Native Auditor

Scanning path:
  /path/to/project

Project summary:
  Project type: Expo
  Package manager: Multiple / Ambiguous

Issues:
  [Warning] RNA_LOCKFILE_001 - Multiple lockfiles detected
      Multiple package manager lockfiles were found. This can cause dependency installs to differ between local machines and CI.
```

Output varies by project. The example is shortened for readability.

## Checks included in v0.1

### Project and package manager detection

- Project type: Expo, React Native, or Unknown.
- Missing `package.json` (`RNA_PROJECT_001`).
- Invalid `package.json` (`RNA_PACKAGE_001`).
- Package manager: Yarn, npm, pnpm, Bun, Unknown, or Multiple / Ambiguous.
- Multiple package manager lockfiles (`RNA_LOCKFILE_001`).

### Environment files

- `.env` exists without `.env.example` (`RNA_ENV_001`).
- Environment variable values are not parsed or printed.

### Expo app configuration

- Static `app.json` parsing.
- Invalid `app.json` (`RNA_APP_JSON_001`).
- Missing `expo.ios.bundleIdentifier` (`RNA_EXPO_IOS_001`).
- Missing `expo.android.package` (`RNA_EXPO_ANDROID_001`).
- Dynamic `app.config.js` or `app.config.ts` detection (`RNA_EXPO_CONFIG_001`).

Dynamic app config is detected but not evaluated, so related static checks may be limited.

### EAS configuration

- Static `eas.json` parsing.
- Invalid `eas.json` (`RNA_EAS_JSON_001`).
- Missing `eas.json` for an Expo project (`RNA_EAS_001`).
- Missing `build.production` profile (`RNA_EAS_002`).

### Reanimated configuration

- Warns when `react-native-reanimated` is installed but a Babel config or the expected Reanimated Babel plugin cannot be found (`RNA_REANIMATED_001`).

### Reports

- Terminal report with project summary, detected files, lockfiles, and issues.
- Static local HTML report with the same audit results.

## Security model

React Native Auditor is local-first:

- It does not upload project data.
- It does not require a SaaS account or service.
- It does not execute commands from the target project.
- It does not run npm, Yarn, pnpm, or Bun.
- It does not run Expo CLI, EAS CLI, or React Native CLI.
- It does not evaluate `app.config.js` or `app.config.ts`.
- It does not execute `babel.config.js`.
- It does not print `.env` values.
- It uses static JSON parsing and conservative text scanning.

JavaScript and TypeScript configuration files are treated as potentially executable input. v0.1 only detects their presence or scans narrowly for known text patterns where a rule requires it.

## Roadmap

Near-term work will continue to add focused, high-value project rules and strengthen test coverage and reporting. Potential later commands include log analysis, release checks, and conservative interactive fixes.

The project does not plan to add a SaaS dashboard or plugin system for v1. See [docs/ROADMAP.md](docs/ROADMAP.md) for the current development plan.

## Contributing

Small, focused rules are preferred over broad frameworks. Useful rule categories include:

- Universal project rules.
- React Native and Expo rules.
- Pattern-based dependency rules.
- Dedicated rules for popular, high-impact libraries.

v1 intentionally avoids a plugin system. Keep changes reviewable, avoid executing target project code, and run the Rust checks before submitting:

```bash
cargo fmt --check
cargo check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```
