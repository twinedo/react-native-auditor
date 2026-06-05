#!/usr/bin/env node

"use strict";

const fs = require("node:fs");
const path = require("node:path");
const {
  getPlatformBinary,
  supportedPlatformKeys,
} = require("../bin/platform");

const packageRoot = path.resolve(__dirname, "..");
const repositoryRoot = path.resolve(packageRoot, "..", "..");
const platformBinary = getPlatformBinary();

if (!platformBinary) {
  console.error(
    [
      `Cannot prepare a local binary for ${process.platform}-${process.arch}.`,
      `Supported platforms: ${supportedPlatformKeys.join(", ")}.`,
    ].join("\n"),
  );
  process.exitCode = 1;
} else {
  const sourcePath = path.join(
    repositoryRoot,
    "target",
    "release",
    platformBinary.fileName,
  );
  const destinationPath = path.join(
    packageRoot,
    "vendor",
    platformBinary.relativePath,
  );

  if (!fs.existsSync(sourcePath) || !fs.statSync(sourcePath).isFile()) {
    console.error(
      [
        `Rust release binary not found: ${sourcePath}`,
        "",
        "Build it from the repository root, then run this helper again:",
        "  cargo build -p react-native-auditor --release",
      ].join("\n"),
    );
    process.exitCode = 1;
  } else {
    try {
      fs.mkdirSync(path.dirname(destinationPath), { recursive: true });
      fs.copyFileSync(sourcePath, destinationPath);

      if (process.platform !== "win32") {
        fs.chmodSync(destinationPath, 0o755);
      }

      console.log(
        `Prepared ${platformBinary.platformKey} binary: ${destinationPath}`,
      );
    } catch (error) {
      console.error(
        `Failed to prepare ${platformBinary.platformKey} binary: ${error.message}`,
      );
      process.exitCode = 1;
    }
  }
}
