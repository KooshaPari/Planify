#!/usr/bin/env node
/**
 * Planify2 dual-harness adapter for shared-3task.v1.json (FR-DH-001).
 *
 * Parity with helios-cli harness_runner::dual_harness.
 * Loads fixture, runs planify2 adapter specs, exits non-zero on any failure.
 *
 * Usage:
 *   DUAL_HARNESS_WORKDIR=$(pwd) node scripts/dual-harness/run_shared_3task.mjs
 *   DUAL_HARNESS_FIXTURE=/path/to/shared-3task.v1.json node ...
 */
"use strict";

const { spawnSync } = require("node:child_process");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");

const SCHEMA = "pheno.dual_harness.fixture.v1";

function defaultFixturePath() {
  if (process.env.DUAL_HARNESS_FIXTURE) {
    return process.env.DUAL_HARNESS_FIXTURE;
  }
  // .../worktrees/Planify2/<topic>/scripts/dual-harness → repos/
  const here = __dirname;
  const repos = path.resolve(here, "../../../../../");
  return path.join(
    repos,
    "pheno-harness",
    "plans",
    "2026-07-22-dual-harness-matrix",
    "fixtures",
    "shared-3task.v1.json",
  );
}

function loadFixture(filePath) {
  const raw = fs.readFileSync(filePath, "utf8");
  const fixture = JSON.parse(raw);
  if (fixture.schema_version !== SCHEMA) {
    throw new Error(`unsupported schema_version: ${fixture.schema_version}`);
  }
  return fixture;
}

function runAdapter(adapter) {
  const timeoutMs =
    adapter.timeout_secs != null ? Number(adapter.timeout_secs) * 1000 : 30_000;
  let cwd = process.cwd();
  if (adapter.working_dir_env) {
    const dir = process.env[adapter.working_dir_env];
    if (!dir) {
      return {
        ok: false,
        error: `${adapter.working_dir_env} unset`,
        timedOut: false,
        stdout: "",
        stderr: "",
        status: null,
      };
    }
    cwd = dir;
  }
  const result = spawnSync(adapter.cmd, adapter.args || [], {
    cwd,
    encoding: "utf8",
    timeout: timeoutMs,
    env: process.env,
  });
  const timedOut = Boolean(result.error && result.error.code === "ETIMEDOUT");
  return {
    ok: !timedOut && result.status === 0,
    timedOut,
    status: result.status,
    stdout: result.stdout || "",
    stderr: result.stderr || "",
    error: result.error ? String(result.error.message || result.error) : null,
  };
}

function accept(task, run) {
  const a = task.acceptance || {};
  if (a.must_error && a.error_class === "timeout") {
    return run.timedOut === true;
  }
  if (a.must_error) {
    return false;
  }
  let ok = true;
  if (a.exit_code != null) {
    ok = ok && run.status === a.exit_code;
  }
  if (a.stdout_contains != null) {
    ok = ok && run.stdout.includes(a.stdout_contains);
  }
  if (a.stdout_path_prefix_env) {
    const prefix = process.env[a.stdout_path_prefix_env] || "";
    const out = run.stdout.trim();
    const prefixReal = fs.existsSync(prefix) ? fs.realpathSync(prefix) : prefix;
    const outReal = fs.existsSync(out) ? fs.realpathSync(out) : out;
    ok = ok && Boolean(prefix) && outReal.startsWith(prefixReal);
  }
  return ok;
}

function main() {
  const fixturePath = defaultFixturePath();
  if (!fs.existsSync(fixturePath)) {
    console.error(`FAIL: fixture missing at ${fixturePath}`);
    process.exit(2);
  }
  if (!process.env.DUAL_HARNESS_WORKDIR) {
    const tmp = fs.mkdtempSync(path.join(os.tmpdir(), "dual-harness-planify-"));
    process.env.DUAL_HARNESS_WORKDIR = tmp;
    console.error(`DUAL_HARNESS_WORKDIR defaulted to ${tmp}`);
  }

  const fixture = loadFixture(fixturePath);
  const outcomes = [];
  for (const task of fixture.tasks || []) {
    const adapter = (task.adapters || {}).planify2;
    if (!adapter) {
      outcomes.push({ task_id: task.task_id, passed: false, detail: "missing planify2 adapter" });
      continue;
    }
    const run = runAdapter(adapter);
    const passed = accept(task, run);
    outcomes.push({
      task_id: task.task_id,
      passed,
      detail: passed
        ? "ok"
        : run.timedOut
          ? "timeout"
          : run.error || `status=${run.status} stdout=${JSON.stringify(run.stdout.trim())}`,
    });
  }

  for (const o of outcomes) {
    console.log(`${o.passed ? "PASS" : "FAIL"} ${o.task_id}: ${o.detail}`);
  }
  const failed = outcomes.filter((o) => !o.passed);
  if (failed.length) {
    process.exit(1);
  }
  console.log(`OK: ${outcomes.length} tasks (planify2 / ${fixture.fixture_id})`);
}

main();
