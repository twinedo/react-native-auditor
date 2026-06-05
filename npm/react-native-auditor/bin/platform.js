"use strict";

const path = require("node:path");

const supportedBinaries = {
  "darwin-arm64": ["darwin-arm64", "rn-auditor"],
  "darwin-x64": ["darwin-x64", "rn-auditor"],
  "linux-x64": ["linux-x64", "rn-auditor"],
  "win32-x64": ["win32-x64", "rn-auditor.exe"],
};

function getPlatformBinary(platform = process.platform, arch = process.arch) {
  const platformKey = `${platform}-${arch}`;
  const binaryParts = supportedBinaries[platformKey];

  if (!binaryParts) {
    return null;
  }

  return {
    platformKey,
    relativePath: path.join(...binaryParts),
    fileName: binaryParts[1],
  };
}

module.exports = {
  getPlatformBinary,
  supportedPlatformKeys: Object.keys(supportedBinaries),
};
