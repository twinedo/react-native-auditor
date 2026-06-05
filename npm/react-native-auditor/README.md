# React Native Auditor npm wrapper

This directory contains the unpublished npm distribution wrapper for React Native Auditor.
The JavaScript launcher selects the bundled Rust binary for the current platform and forwards
all CLI arguments to it. It does not download binaries, run Cargo, build during npm install, or
execute project scripts.

Supported package layouts:

- `vendor/darwin-arm64/rn-auditor`
- `vendor/darwin-x64/rn-auditor`
- `vendor/linux-x64/rn-auditor`
- `vendor/win32-x64/rn-auditor.exe`

## Local packaging and testing

This workflow is for local package preparation and testing only. It does not publish the package
to npm.

From the repository root, build the release binary:

```bash
cargo build -p react-native-auditor --release
```

Then prepare and test the npm package:

```bash
cd npm/react-native-auditor
node scripts/prepare-local-binary.js
node bin/rn-auditor.js audit /path/to/project
npm pack
npm install -g ./react-native-auditor-0.1.0.tgz
rn-auditor audit /path/to/project
npm uninstall -g react-native-auditor
```

The helper only copies the current platform's existing binary from `target/release`. It does not
run `cargo build`, download anything, or publish anything. Generated vendor binaries and npm pack
tarballs are local artifacts and should not be committed.
