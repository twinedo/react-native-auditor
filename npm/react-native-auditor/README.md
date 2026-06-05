# React Native Auditor npm wrapper

This directory contains the unpublished npm distribution wrapper for React Native Auditor.
The JavaScript launcher only forwards arguments to the bundled Rust `rn-auditor` binary.
It does not download, install, build, or execute project scripts.

## Local testing

From the repository root:

```bash
cargo build -p react-native-auditor --release
cp target/release/rn-auditor npm/react-native-auditor/vendor/rn-auditor
chmod +x npm/react-native-auditor/bin/rn-auditor.js
node npm/react-native-auditor/bin/rn-auditor.js audit
```

To inspect the npm package tarball:

```bash
cd npm/react-native-auditor
npm pack
```

The package is not published. Native binaries must be bundled before npm distribution is
ready for `npx react-native-auditor audit` or global `rn-auditor audit` usage.
