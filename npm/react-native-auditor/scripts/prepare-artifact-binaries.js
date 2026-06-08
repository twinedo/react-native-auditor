#!/usr/bin/env node

"use strict";

const fs = require("node:fs");
const path = require("node:path");

const packageRoot = path.resolve(__dirname, "..");

const platforms = [
  {
    platformKey: "darwin-arm64",
    fileName: "rn-auditor",
    executable: true,
  },
  {
    platformKey: "darwin-x64",
    fileName: "rn-auditor",
    executable: true,
  },
  {
    platformKey: "linux-x64",
    fileName: "rn-auditor",
    executable: true,
  },
  {
    platformKey: "win32-x64",
    fileName: "rn-auditor.exe",
    executable: false,
  },
];

function printUsage() {
  console.error(
    [
      "Usage:",
      "  node scripts/prepare-artifact-binaries.js <artifacts-root>",
      "",
      "Example:",
      "  node scripts/prepare-artifact-binaries.js ../../dist-artifacts",
    ].join("\n"),
  );
}

function isFile(filePath) {
  try {
    return fs.statSync(filePath).isFile();
  } catch {
    return false;
  }
}

function getCandidatePaths(artifactsRoot, platform) {
  const artifactName = `rn-auditor-${platform.platformKey}`;

  return [
    path.join(artifactsRoot, artifactName, platform.fileName),
    path.join(
      artifactsRoot,
      artifactName,
      platform.platformKey,
      platform.fileName,
    ),
    path.join(artifactsRoot, platform.platformKey, platform.fileName),
  ];
}

function findSourceBinary(artifactsRoot, platform) {
  return getCandidatePaths(artifactsRoot, platform).find(isFile) ?? null;
}

function prepareBinary(sourcePath, platform) {
  const destinationPath = path.join(
    packageRoot,
    "vendor",
    platform.platformKey,
    platform.fileName,
  );

  fs.mkdirSync(path.dirname(destinationPath), { recursive: true });
  fs.copyFileSync(sourcePath, destinationPath);

  if (platform.executable) {
    fs.chmodSync(destinationPath, 0o755);
  }

  return destinationPath;
}

const args = process.argv.slice(2);

if (args.length !== 1) {
  printUsage();
  process.exitCode = 1;
  return;
}

const artifactsRoot = path.resolve(process.cwd(), args[0]);

if (!fs.existsSync(artifactsRoot) || !fs.statSync(artifactsRoot).isDirectory()) {
  console.error(`Artifacts folder not found or not a directory: ${artifactsRoot}`);
  process.exitCode = 1;
  return;
}

const resolvedBinaries = platforms.map((platform) => ({
  platform,
  sourcePath: findSourceBinary(artifactsRoot, platform),
}));

let hasMissingBinary = false;

for (const { platform, sourcePath } of resolvedBinaries) {
  if (sourcePath) {
    console.log(`Found ${platform.platformKey}: ${sourcePath}`);
    continue;
  }

  hasMissingBinary = true;
  console.error(
    [
      `Missing ${platform.platformKey} binary (${platform.fileName}).`,
      "Checked:",
      ...getCandidatePaths(artifactsRoot, platform).map(
        (candidatePath) => `  ${candidatePath}`,
      ),
    ].join("\n"),
  );
}

if (hasMissingBinary) {
  console.error(
    "\nArtifact preparation stopped. No vendor binaries were copied.",
  );
  process.exitCode = 1;
  return;
}

for (const { platform, sourcePath } of resolvedBinaries) {
  try {
    const destinationPath = prepareBinary(sourcePath, platform);
    console.log(`Prepared ${platform.platformKey}: ${destinationPath}`);
  } catch (error) {
    console.error(
      `Failed to prepare ${platform.platformKey}: ${error.message}`,
    );
    process.exitCode = 1;
    return;
  }
}

console.log("Prepared all release artifact binaries for npm packaging.");
