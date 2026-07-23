#!/usr/bin/env node
/** Smoke test for Planify dual-harness adapter (FR-DH-001). */
"use strict";

const { spawnSync } = require("node:child_process");
const path = require("node:path");

const script = path.join(__dirname, "run_shared_3task.cjs");
const result = spawnSync(process.execPath, [script], {
  encoding: "utf8",
  env: process.env,
});
process.stdout.write(result.stdout || "");
process.stderr.write(result.stderr || "");
if (result.status !== 0) {
  process.exit(result.status || 1);
}
console.log("test_run_shared_3task: PASS");
