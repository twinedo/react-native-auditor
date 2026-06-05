#!/usr/bin/env node

"use strict";

const fs = require("node:fs");
const path = require("node:path");
const { spawn } = require("node:child_process");

const args = process.argv.slice(2);
const vendorDirectory = path.resolve(__dirname, "..", "vendor");

const platformBinaryPaths = {
  "darwin-arm64": path.join(vendorDirectory, "darwin-arm64", "rn-auditor"),
  "darwin-x64": path.join(vendorDirectory, "darwin-x64", "rn-auditor"),
  "linux-x64": path.join(vendorDirectory, "linux-x64", "rn-auditor"),
  "win32-x64": path.join(vendorDirectory, "win32-x64", "rn-auditor.exe"),
};

const platformKey = `${process.platform}-${process.arch}`;
const binaryCandidates = [
  platformBinaryPaths[platformKey],
  path.join(
    vendorDirectory,
    process.platform === "win32" ? "rn-auditor.exe" : "rn-auditor",
  ),
].filter(Boolean);

const binaryPath = binaryCandidates.find((candidate) => fs.existsSync(candidate));

if (!binaryPath) {
  console.error(
    [
      "React Native Auditor npm wrapper is installed, but its native rn-auditor binary is missing.",
      `Expected a bundled binary for ${platformKey} under:`,
      `  ${vendorDirectory}`,
      "",
      "This npm package skeleton is not ready for published npm use until native binaries are bundled.",
      "The wrapper will not download a binary, run Cargo, or build anything automatically.",
    ].join("\n"),
  );
  process.exitCode = 1;
} else {
  const child = spawn(binaryPath, args, { stdio: "inherit" });

  child.on("error", (error) => {
    console.error(`Failed to start the bundled rn-auditor binary: ${error.message}`);
    process.exitCode = 1;
  });

  child.on("exit", (code, signal) => {
    if (signal) {
      console.error(`The bundled rn-auditor binary exited due to signal ${signal}.`);
      process.exitCode = 1;
      return;
    }

    process.exitCode = code ?? 1;
  });
}
