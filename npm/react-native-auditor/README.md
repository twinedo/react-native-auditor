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

Then prepare the current platform binary and run the launcher directly:

```bash
cd npm/react-native-auditor
node scripts/prepare-local-binary.js
node bin/rn-auditor.js audit /path/to/project
```

The helper only copies the current platform's existing binary from `target/release`. It does not
run `cargo build`, download anything, or publish anything. Generated vendor binaries and npm pack
tarballs are local artifacts and should not be committed.

## Prepare a package from GitHub Actions artifacts

This workflow validates the complete multi-platform npm package before publishing. It does not
publish to npm or create a GitHub Release.

1. Run the GitHub Actions workflow named `Release Binaries`.
2. Download these artifacts manually:
   - `rn-auditor-darwin-arm64`
   - `rn-auditor-darwin-x64`
   - `rn-auditor-linux-x64`
   - `rn-auditor-win32-x64`
3. Extract the artifacts into a local folder, for example `dist-artifacts/` at the repository root.
4. Prepare the npm vendor layout:

```bash
cd npm/react-native-auditor
node scripts/prepare-artifact-binaries.js ../../dist-artifacts
```

## Final prepublish validation

This is a manual prepublish validation workflow. It checks the package but does not publish it,
create a GitHub Release, tag a release, or upload artifacts.

After preparing all four platform binaries with `prepare-artifact-binaries.js`, run:

```bash
node scripts/check-package.js
npm pack --dry-run
npm pack
npm install -g ./react-native-auditor-0.1.0.tgz
rn-auditor audit ../../
npm uninstall -g react-native-auditor
```

`npm run check:package` is an alias for `node scripts/check-package.js`. The checker validates the
package metadata, launcher permissions and shebang, required platform binaries, and the JSON output
from `npm pack --dry-run`. It fails if required runtime files would be omitted or if local scripts,
tarballs, `dist-artifacts`, `target`, or `.local` content would be included.

The published tarball is intentionally limited by `package.json` to:

- `bin/`
- `vendor/`
- `README.md`
- `package.json`

The local `scripts/` directory remains in the repository for package preparation and validation,
but is not included in the tarball.

To test the generated tarball manually:

```bash
npm install -g ./react-native-auditor-0.1.0.tgz
rn-auditor audit /path/to/project
npm uninstall -g react-native-auditor
```

The artifact helper accepts common extracted layouts such as:

- `<artifacts-root>/rn-auditor-darwin-arm64/rn-auditor`
- `<artifacts-root>/rn-auditor-darwin-arm64/darwin-arm64/rn-auditor`
- `<artifacts-root>/darwin-arm64/rn-auditor`

It requires all four platform binaries before copying any of them. It only copies the expected
binary files into `vendor/` and marks the macOS and Linux binaries executable. It does not access
the network, build Rust, run Cargo, publish to npm, create a GitHub Release, or execute code from a
target React Native project.

The extracted `dist-artifacts/` folder, generated vendor binaries, and npm tarballs are local
pre-publish validation artifacts and should not be committed.
