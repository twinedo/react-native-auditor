<div align="center">

# React Native Auditor

**A local-first React Native & Expo project auditor for release readiness, dependency health, and risk reports.**

Audit React Native and Expo projects before release issues waste your time.

[![npm version](https://img.shields.io/npm/v/react-native-auditor?style=flat-square)](https://www.npmjs.com/package/react-native-auditor)
[![CI](https://img.shields.io/github/actions/workflow/status/twinedo/react-native-auditor/ci.yml?branch=main&style=flat-square&label=CI)](https://github.com/twinedo/react-native-auditor/actions/workflows/ci.yml)
[![license](https://img.shields.io/github/license/twinedo/react-native-auditor?style=flat-square)](LICENSE)
![platforms](https://img.shields.io/badge/platform-macOS%20%7C%20Linux%20%7C%20Windows-4c566a?style=flat-square)
<a href="https://www.npmjs.com/package/react-native-auditor">
    <img src="https://img.shields.io/npm/dm/react-native-auditor.svg" alt="npm downloads" />
  </a>
</div>

## Quick start

Run an audit in the current project:

```bash
npx react-native-auditor audit
```

Audit a specific project:

```bash
npx react-native-auditor audit /path/to/project
```

Or install the CLI globally:

```bash
npm install -g react-native-auditor
rn-auditor audit
```

`scan` is an alias for `audit`:

```bash
rn-auditor scan
```

Generate a local HTML report:

```bash
rn-auditor report --html
rn-auditor report --html /path/to/project --output report.html
```

Generate a JSON report for CI or other tooling:

```bash
rn-auditor report --json
rn-auditor report --json --output rn-auditor-report.json
rn-auditor report --json /path/to/project
```

## What it does

React Native Auditor uses conservative static checks to:

- Detect Expo, React Native, and unknown project types.
- Detect the package manager and conflicting root lockfiles.
- Check Expo and EAS release-readiness configuration.
- Find missing environment variable documentation.
- Flag a common Reanimated and Babel setup risk.
- Generate readable terminal output and a local static HTML report.

## Why React Native Auditor

`react-native doctor` is useful for checking the development environment. React Native Auditor has a different scope: it audits project-level files for dependency, configuration, and release-readiness risks.

- **Project-focused:** inspects the repository state that travels with the app.
- **Local-first:** project data stays on the machine running the audit.
- **Static by default:** reads selected files without running the target project.
- **Native CLI:** the npm package launches a bundled Rust binary for fast, predictable execution.
- **No service dependency:** no account, dashboard, or SaaS connection is required.

React Native Auditor is intentionally focused. It does not replace React Native Doctor, Expo Doctor, platform build validation, or a real release build.

## Example output

```text
React Native Auditor

Scanning path:
  /work/mobile-app

Project summary:
  Project type: Expo
  Package manager: Multiple / Ambiguous

Issues:
  [Warning] RNA_LOCKFILE_001 - Multiple lockfiles detected
      Multiple package manager lockfiles were found.
  [Warning] RNA_ENV_001 - Missing .env.example
      This project uses a .env file but does not document required variables.
```

Output varies by project. This example is shortened for readability.

## Checks included in v0.1

### Project detection

- Detects Expo, React Native, and unknown project types from `package.json`.
- Reports a missing or invalid `package.json`.
- Accepts the current directory or an explicit project path.

### Dependency and lockfile health

- Detects npm, Yarn, pnpm, and Bun from root lockfiles.
- Reports multiple package manager lockfiles as an ambiguous setup.

### Environment files

- Reports when `.env` exists but `.env.example` is missing.
- Never parses or prints environment variable values.

### Expo app config

- Parses static `app.json` as JSON.
- Checks `expo.ios.bundleIdentifier` and `expo.android.package`.
- Detects dynamic `app.config.js` and `app.config.ts` without evaluating them.

Dynamic app config limits the static checks that can be performed.

### EAS release readiness

- Parses static `eas.json` as JSON.
- Reports a missing `eas.json` for Expo projects.
- Reports a missing `build.production` profile.

### Reanimated setup

- Checks for `react-native-reanimated` in project dependencies.
- Uses narrow text scanning to look for `react-native-reanimated/plugin` in `babel.config.js`.
- Reports when the expected Babel setup cannot be found.

### Reports

- Terminal report with project summary, detected files, lockfiles, and issues.
- Static local HTML report containing the same audit results.
- Pretty-printed JSON report with detected files, lockfiles, issue counts by severity, and the full issue list.

## HTML report

```bash
rn-auditor report --html /path/to/project --output report.html
```

The report is a static HTML file written locally. It does not require a server or upload project data. Without `--output`, the file is written as `rn-auditor-report.html` in the current directory.

## JSON report

```bash
rn-auditor report --json
rn-auditor report --json --output rn-auditor-report.json
rn-auditor report --json /path/to/project
```

Without `--output`, the pretty-printed JSON report is written to standard output. The report includes detected files, lockfiles, issue counts by severity, and the full issue list.

## Security model

React Native Auditor treats project configuration as untrusted input.

- Does not upload project data.
- Does not require a SaaS account or service.
- Does not execute commands from the target project.
- Does not run npm, Yarn, pnpm, or Bun commands from the target project.
- Does not run Expo CLI, EAS CLI, or React Native CLI.
- Does not evaluate `app.config.js` or `app.config.ts`.
- Does not execute `babel.config.js`.
- Does not print `.env` values.

Static JSON parsing and narrow text scanning are used where a rule needs file contents. JavaScript and TypeScript configuration files are never loaded as executable modules.

## Installation

React Native Auditor requires Node.js 18 or newer when installed through npm.

### npx

```bash
npx react-native-auditor audit
```

### Global npm install

```bash
npm install -g react-native-auditor
rn-auditor audit
```

### Other package runners

The published package exposes the standard `rn-auditor` npm binary and can also be launched with:

```bash
yarn dlx react-native-auditor audit
pnpm dlx react-native-auditor audit
bunx react-native-auditor audit
```

The npm wrapper selects the bundled Rust binary for the current platform and forwards CLI arguments to it. The v0.1 package includes binaries for macOS arm64/x64, Linux x64, and Windows x64.

## Commands

| Command | Description |
| --- | --- |
| `rn-auditor audit [path]` | Audit a project and print the terminal report. |
| `rn-auditor scan [path]` | Alias for `audit`. |
| `rn-auditor report --html [path] [--output <file>]` | Write a local static HTML report. |
| `rn-auditor report --json [path] [--output <file>]` | Print or write a pretty-printed JSON report. |

If no path is provided, the current directory is audited. Exactly one of `--html` or `--json` is required for the `report` command.

## Roadmap

Planned work remains deliberately focused:

- More high-value static rules.
- Stronger release-readiness checks.
- Log analysis as a later capability.
- Conservative, interactive fixes at a later stage.

There is no plugin system planned for v1. Commands such as `analyze-log`, `release-check`, and interactive fixes are roadmap items, not v0.1 features. See [docs/ROADMAP.md](docs/ROADMAP.md) for current development notes.

## Contributing

Contributions should prefer small, focused rules with clear findings. Checks must remain static-safe and must not execute code or commands from the target project.

Before submitting a Rust change, run:

```bash
cargo fmt --all -- --check
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## License

[MIT](LICENSE)
