#!/usr/bin/env node

"use strict";

const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const { spawnSync } = require("node:child_process");

const packageRoot = path.resolve(__dirname, "..");
const packageJsonPath = path.join(packageRoot, "package.json");

const requiredPackageFiles = [
  "package.json",
  "README.md",
  "bin/rn-auditor.js",
  "bin/platform.js",
  "vendor/darwin-arm64/rn-auditor",
  "vendor/darwin-x64/rn-auditor",
  "vendor/linux-x64/rn-auditor",
  "vendor/win32-x64/rn-auditor.exe",
];

const requiredFilesEntries = ["bin", "vendor", "README.md", "package.json"];

const forbiddenTarballPaths = [
  ".gitignore",
  "scripts",
  "dist-artifacts",
  "target",
  ".local",
];

const failures = [];

function pass(message) {
  console.log(`PASS ${message}`);
}

function fail(message) {
  failures.push(message);
  console.error(`FAIL ${message}`);
}

function isFile(relativePath) {
  try {
    return fs.statSync(path.join(packageRoot, relativePath)).isFile();
  } catch {
    return false;
  }
}

function validateRequiredFiles() {
  for (const relativePath of requiredPackageFiles) {
    if (isFile(relativePath)) {
      pass(`Required file exists: ${relativePath}`);
    } else {
      fail(`Required file is missing: ${relativePath}`);
    }
  }
}

function validateLauncher() {
  const launcherPath = path.join(packageRoot, "bin", "rn-auditor.js");

  if (!isFile("bin/rn-auditor.js")) {
    return;
  }

  const launcher = fs.readFileSync(launcherPath, "utf8");
  const hasShebang = launcher.startsWith("#!/usr/bin/env node");
  const isExecutable = process.platform === "win32"
    || (fs.statSync(launcherPath).mode & 0o111) !== 0;

  if (hasShebang) {
    pass("bin/rn-auditor.js has a Node.js shebang");
  } else {
    fail("bin/rn-auditor.js must start with '#!/usr/bin/env node'");
  }

  if (isExecutable) {
    pass("bin/rn-auditor.js is executable");
  } else {
    fail("bin/rn-auditor.js is not executable");
  }
}

function readPackageJson() {
  try {
    return JSON.parse(fs.readFileSync(packageJsonPath, "utf8"));
  } catch (error) {
    fail(`package.json is not valid JSON: ${error.message}`);
    return null;
  }
}

function validatePackageJson(packageJson) {
  if (!packageJson) {
    return;
  }

  const checks = [
    [
      packageJson.name === "react-native-auditor",
      "package.json name is react-native-auditor",
    ],
    [packageJson.version === "0.1.0", "package.json version is 0.1.0"],
    [
      packageJson.bin?.["rn-auditor"] === "bin/rn-auditor.js",
      "package.json bin.rn-auditor points to bin/rn-auditor.js",
    ],
    [packageJson.private !== true, "package.json private is not true"],
    [Array.isArray(packageJson.files), "package.json files is an array"],
  ];

  for (const [condition, message] of checks) {
    if (condition) {
      pass(message);
    } else {
      fail(message);
    }
  }

  if (!Array.isArray(packageJson.files)) {
    return;
  }

  for (const entry of requiredFilesEntries) {
    if (packageJson.files.includes(entry)) {
      pass(`package.json files includes ${entry}`);
    } else {
      fail(`package.json files must include ${entry}`);
    }
  }

  if (packageJson.files.includes("scripts")) {
    fail("package.json files must not include local packaging scripts");
  } else {
    pass("package.json files excludes local packaging scripts");
  }
}

function isForbiddenTarballPath(relativePath) {
  if (relativePath.endsWith(".tgz")) {
    return true;
  }

  return forbiddenTarballPaths.some(
    (forbiddenPath) =>
      relativePath === forbiddenPath
      || relativePath.startsWith(`${forbiddenPath}/`),
  );
}

function validateTarballContents() {
  const npmCache = fs.mkdtempSync(
    path.join(os.tmpdir(), "react-native-auditor-npm-"),
  );
  let result;

  try {
    result = spawnSync(
      process.platform === "win32" ? "npm.cmd" : "npm",
      ["pack", "--dry-run", "--json", "--ignore-scripts"],
      {
        cwd: packageRoot,
        encoding: "utf8",
        env: {
          ...process.env,
          npm_config_audit: "false",
          npm_config_cache: npmCache,
          npm_config_fund: "false",
          npm_config_offline: "true",
          npm_config_update_notifier: "false",
        },
      },
    );
  } finally {
    fs.rmSync(npmCache, { force: true, recursive: true });
  }

  if (result.error) {
    fail(`Could not run npm pack --dry-run: ${result.error.message}`);
    return;
  }

  if (result.status !== 0) {
    const details = (result.stderr || result.stdout).trim();
    fail(
      `npm pack --dry-run exited with status ${result.status}${
        details ? `: ${details}` : ""
      }`,
    );
    return;
  }

  let packResult;

  try {
    const parsed = JSON.parse(result.stdout);
    packResult = Array.isArray(parsed) ? parsed[0] : null;
  } catch (error) {
    fail(`Could not parse npm pack --dry-run JSON: ${error.message}`);
    return;
  }

  if (!packResult || !Array.isArray(packResult.files)) {
    fail("npm pack --dry-run did not return a package file list");
    return;
  }

  pass("npm pack --dry-run completed successfully");

  const tarballFiles = new Set(
    packResult.files.map((file) => file.path.replaceAll("\\", "/")),
  );

  for (const relativePath of requiredPackageFiles) {
    if (tarballFiles.has(relativePath)) {
      pass(`Tarball includes ${relativePath}`);
    } else {
      fail(`Tarball would omit required file: ${relativePath}`);
    }
  }

  const forbiddenFiles = [...tarballFiles].filter(isForbiddenTarballPath);

  if (forbiddenFiles.length === 0) {
    pass("Tarball excludes local scripts and generated repository artifacts");
  } else {
    for (const relativePath of forbiddenFiles) {
      fail(`Tarball would include forbidden file: ${relativePath}`);
    }
  }
}

console.log("Checking React Native Auditor npm package...\n");

validateRequiredFiles();
validateLauncher();
validatePackageJson(readPackageJson());

if (failures.length === 0) {
  validateTarballContents();
} else {
  console.error("\nSkipping npm pack dry-run until filesystem and metadata checks pass.");
}

if (failures.length > 0) {
  console.error(`\nPackage validation failed with ${failures.length} error(s).`);
  process.exitCode = 1;
} else {
  console.log("\nPackage validation passed.");
}
