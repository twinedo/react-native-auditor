#!/usr/bin/env node

"use strict";

const fs = require("node:fs");
const path = require("node:path");
const { spawn } = require("node:child_process");
const { getPlatformBinary, supportedPlatformKeys } = require("./platform");

const args = process.argv.slice(2);
const vendorDirectory = path.resolve(__dirname, "..", "vendor");
const platformBinary = getPlatformBinary();

if (!platformBinary) {
  console.error(
    [
      `React Native Auditor does not support ${process.platform}-${process.arch}.`,
      `Supported platforms: ${supportedPlatformKeys.join(", ")}.`,
      "",
      "The npm wrapper only executes a bundled Rust binary.",
      "It will not download a binary, run Cargo, or build anything automatically.",
    ].join("\n"),
  );
  process.exitCode = 1;
} else {
  const binaryPath = path.join(vendorDirectory, platformBinary.relativePath);

  if (!fs.existsSync(binaryPath)) {
    console.error(
      [
        `The bundled rn-auditor binary for ${platformBinary.platformKey} is missing.`,
        `Expected binary: ${binaryPath}`,
        "",
        "For local packaging from the repository checkout, build the Rust release binary and run:",
        "  node scripts/prepare-local-binary.js",
        "",
        "The npm wrapper will not fall back to Cargo, download a binary, or build automatically.",
      ].join("\n"),
    );
    process.exitCode = 1;
    return;
  }

  const child = spawn(binaryPath, args, { stdio: "inherit" });

  child.on("error", (error) => {
    console.error(`Failed to start the bundled rn-auditor binary: ${error.message}`);
    process.exitCode = 1;
  });

  child.on("exit", (code, signal) => {
    if (signal) {
      process.kill(process.pid, signal);
      return;
    }

    process.exitCode = code ?? 1;
  });
}
