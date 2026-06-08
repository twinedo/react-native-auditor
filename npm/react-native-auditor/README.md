<div align="center">

# React Native Auditor

**A local-first React Native & Expo project auditor for release readiness, dependency health, and risk reports.**

Audit React Native and Expo projects before release issues waste your time.

[![npm version](https://img.shields.io/npm/v/react-native-auditor?style=flat-square)](https://www.npmjs.com/package/react-native-auditor)
[![license](https://img.shields.io/npm/l/react-native-auditor?style=flat-square)](https://github.com/twinedo/react-native-auditor/blob/main/LICENSE)
![platforms](https://img.shields.io/badge/platform-macOS%20%7C%20Linux%20%7C%20Windows-4c566a?style=flat-square)

</div>

## Quick start

```bash
npx react-native-auditor audit
```

Audit another project:

```bash
npx react-native-auditor audit /path/to/project
```

Or install the CLI globally:

```bash
npm install -g react-native-auditor
rn-auditor audit
```

`rn-auditor scan` is an alias for `rn-auditor audit`.

## What it checks

- React Native, Expo, and unknown project detection.
- Package manager detection and conflicting lockfiles.
- Missing `.env.example` documentation.
- Static Expo app identity configuration.
- EAS production profile readiness.
- Common Reanimated and Babel setup risk.
- Terminal and local static HTML reports.

React Native Auditor complements `react-native doctor`: doctor-style tools check the development environment, while this CLI focuses on project-level files and release risks.

## HTML report

```bash
rn-auditor report --html
rn-auditor report --html /path/to/project --output report.html
```

Without `--output`, the report is written to `rn-auditor-report.html` in the current directory.

## Commands

| Command | Description |
| --- | --- |
| `rn-auditor audit [path]` | Audit a project and print the terminal report. |
| `rn-auditor scan [path]` | Alias for `audit`. |
| `rn-auditor report --html [path] [--output <file>]` | Write a local static HTML report. |

## Native binary distribution

This npm package is a small JavaScript launcher around the native React Native Auditor CLI, written in Rust. It selects the bundled binary for the current platform and forwards the CLI arguments.

The package does not download a binary during installation and does not build Rust code on the user's machine. v0.1 includes macOS arm64/x64, Linux x64, and Windows x64 binaries. Node.js 18 or newer is required by the npm wrapper.

The package can also be launched through other npm-compatible package runners:

```bash
yarn dlx react-native-auditor audit
pnpm dlx react-native-auditor audit
bunx react-native-auditor audit
```

## Security

React Native Auditor is local-first and uses conservative static checks:

- Does not upload project data or require a SaaS service.
- Does not execute target project commands.
- Does not run npm, Yarn, pnpm, Bun, Expo CLI, EAS CLI, or React Native CLI.
- Does not evaluate `app.config.js` or `app.config.ts`.
- Does not execute `babel.config.js`.
- Does not print `.env` values.

## Documentation

See the [GitHub repository](https://github.com/twinedo/react-native-auditor) for the full rule list, roadmap, security model, and contributing guide.

## License

[MIT](https://github.com/twinedo/react-native-auditor/blob/main/LICENSE)
