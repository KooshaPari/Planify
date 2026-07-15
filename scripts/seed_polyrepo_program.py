#!/usr/bin/env python3
"""Seed AgilePlus polyrepo planning records."""

import hashlib
import json
import sqlite3
import sys
from datetime import datetime, timezone
from pathlib import Path


def utc_iso() -> str:
    return datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def make_hash(*parts: str) -> bytes:
    payload = "\n".join(parts).encode("utf-8")
    return hashlib.sha256(payload).digest()


def rows(conn):
    projects = [
        ("OmniRoute", "OmniRoute"),
        ("Tracera", "Tracera"),
        ("AgilePlus/Substrate", "AgilePlus/Substrate"),
        ("DesktopDeploy", "DesktopDeploy"),
        ("Vercel", "Vercel"),
        ("AgentInfra", "AgentInfra"),
    ]

    features = [
        ("omniroute-caddy-planning", "OmniRoute: Caddy ingress planning"),
        ("tracera-caddy-planning", "Tracera: Caddy routing planning"),
        ("agileplus-db-planning", "AgilePlus/Substrate: DB migration planning"),
        ("desktopdeploy-lifecycle-planning", "DesktopDeploy: Workspace lifecycle planning"),
        ("vercel-id-planning", "Vercel: Identity planning"),
        ("agentinfra-gate-planning", "AgentInfra: Gate aggregation planning"),
    ]

    work_packages = [
        (
            "AGILEPLUS-DB-001",
            "AGILEPLUS-DB-001: Define reproducible migration and bootstrap plan",
            "planned",
            1,
            "db-owner",
            "Provide migration provenance, rollback windows, and reproducible bootstrap checks for all environments.",
            "agileplus-db-planning",
        ),
        (
            "OR-CADDY-001",
            "OR-CADDY-001: Define Caddy ingress bootstrapping for OmniRoute",
            "planned",
            1,
            "or-caddy-owner",
            "Define TLS trust chains, startup health probes, and secure config contract for OmniRoute ingress.",
            "omniroute-caddy-planning",
        ),
        (
            "TR-CADDY-001",
            "TR-CADDY-001: Define Caddy routing for trace pipelines",
            "planned",
            1,
            "tr-caddy-owner",
            "Define tracing-aware request pathing, tenant isolation, and route observability defaults for Tracera.",
            "tracera-caddy-planning",
        ),
        (
            "OR-WSLC-001",
            "OR-WSLC-001: Define workspace lifecycle sequencing and handoff",
            "planned",
            1,
            "or-wslc-owner",
            "Define create, suspend, resume, and shutdown behavior with explicit owner approvals and timeouts.",
            "desktopdeploy-lifecycle-planning",
        ),
        (
            "VERCEL-ID-001",
            "VERCEL-ID-001: Plan identity attestation and policy alignment for Vercel",
            "planned",
            1,
            "vercel-id-owner",
            "Define actor identity verification and token audience checks for deployment and runtime trust boundaries.",
            "vercel-id-planning",
        ),
        (
            "GATE-AGG-001",
            "GATE-AGG-001: Define aggregate release gate criteria",
            "planned",
            1,
            "gate-owner",
            "Define pass/fail gating logic that blocks promotion when downstream work packages are incomplete.",
            "agentinfra-gate-planning",
        ),
    ]

    deps = {
        "OR-WSLC-001": ["OR-CADDY-001"],
        "GATE-AGG-001": ["AGILEPLUS-DB-001", "OR-CADDY-001", "TR-CADDY-001", "OR-WSLC-001", "VERCEL-ID-001"],
    }

    return projects, features, work_packages, deps


def get_or_create_project(conn: sqlite3.Connection, slug: str, name: str) -> int:
    row = conn.execute("SELECT id FROM projects WHERE slug = ?", (slug,)).fetchone()
    if row:
        return row[0]
    now = utc_iso()
    conn.execute(
        "INSERT INTO projects (slug, name, description, created_at, updated_at) VALUES (?, ?, '', ?, ?)",
        (slug, name, now, now),
    )
    return int(conn.execute("SELECT last_insert_rowid()").fetchone()[0])


def insert_feature(conn: sqlite3.Connection, slug: str, friendly_name: str) -> int:
    row = conn.execute("SELECT id FROM features WHERE slug = ?", (slug,)).fetchone()
    if row:
        return row[0]
    now = utc_iso()
    spec_hash = make_hash(slug, friendly_name)
    conn.execute(
        "INSERT INTO features (slug, friendly_name, state, spec_hash, target_branch, created_at, updated_at, module_id)"
        " VALUES (?, ?, 'planned', ?, 'main', ?, ?, NULL)",
        (slug, friendly_name, spec_hash, now, now),
    )
    return int(conn.execute("SELECT last_insert_rowid()").fetchone()[0])


def upsert_wp(conn: sqlite3.Connection, wp_id: str, title: str, owner: str, acceptance: str, feature_id: int) -> int:
    row = conn.execute("SELECT id FROM work_packages WHERE title = ?", (title,)).fetchone()
    if row:
        return row[0]
    now = utc_iso()
    conn.execute(
        "INSERT INTO work_packages (feature_id, title, state, sequence, acceptance_criteria, agent_id, created_at, updated_at)"
        " VALUES (?, ?, 'planned', 1, ?, ?, ?, ?)",
        (feature_id, title, acceptance, owner, now, now),
    )
    return int(conn.execute("SELECT last_insert_rowid()").fetchone()[0])


def upsert_dependency(conn: sqlite3.Connection, wp_id: int, depends_on: int) -> None:
    exists = conn.execute(
        "SELECT 1 FROM wp_dependencies WHERE wp_id = ? AND depends_on = ? AND dep_type = 'explicit' LIMIT 1",
        (wp_id, depends_on),
    ).fetchone()
    if not exists:
        conn.execute(
            "INSERT INTO wp_dependencies (wp_id, depends_on, dep_type) VALUES (?, ?, 'explicit')",
            (wp_id, depends_on),
        )


def upsert_evidence(conn: sqlite3.Connection, wp_id: int, fr_id: str, artifact_path: str) -> None:
    meta = json.dumps({"notes": "unverified planning evidence"}, separators=(",", ":"))
    row = conn.execute(
        "SELECT id, artifact_path, metadata FROM evidence WHERE wp_id = ? AND fr_id = ? AND evidence_type = 'manual_attestation' LIMIT 1",
        (wp_id, fr_id),
    ).fetchone()
    if row:
        if row[1] != artifact_path or row[2] != meta:
            conn.execute(
                "UPDATE evidence SET artifact_path = ?, metadata = ? WHERE id = ?",
                (artifact_path, meta, row[0]),
            )
        return
    conn.execute(
        "INSERT INTO evidence (wp_id, fr_id, evidence_type, artifact_path, metadata, created_at) VALUES (?, ?, 'manual_attestation', ?, ?, ?)",
        (wp_id, fr_id, artifact_path, meta, utc_iso()),
    )


def table_counts(conn: sqlite3.Connection) -> dict:
    names = ["projects", "features", "work_packages", "wp_dependencies", "evidence"]
    return {name: int(conn.execute(f"SELECT COUNT(*) FROM {name}").fetchone()[0]) for name in names}


def main(argv=None) -> int:
    args = list(sys.argv[1:] if argv is None else argv)
    db_path = (
        Path(args[0]).expanduser()
        if args
        else Path(__file__).resolve().parents[1] / ".agileplus" / "agileplus.db"
    )
    if not db_path.is_file():
        raise FileNotFoundError(f"Database file does not exist: {db_path}")
    conn = sqlite3.connect(db_path)
    projects, features, work_packages, deps = rows(conn)

    try:
        conn.execute("PRAGMA foreign_keys = ON")
        conn.execute("BEGIN IMMEDIATE")
        try:
            for slug, name in projects:
                get_or_create_project(conn, slug, name)

            feature_ids = {slug: insert_feature(conn, slug, friendly_name) for slug, friendly_name in features}

            wp_ids = {}
            for wp_id, title, _, _, owner, acceptance, feature_slug in work_packages:
                if not owner:
                    raise ValueError(f"owner required for {wp_id}")
                wp_ids[wp_id] = upsert_wp(
                    conn,
                    wp_id,
                    title,
                    owner,
                    acceptance,
                    feature_ids[feature_slug],
                )

            for wp_id, dep_ids in deps.items():
                child = wp_ids[wp_id]
                for dep in dep_ids:
                    parent = wp_ids[dep]
                    upsert_dependency(conn, child, parent)

            artifact_path = str(Path(__file__).resolve().parents[2] / "work.md")
            for wp_id, wp_row in wp_ids.items():
                upsert_evidence(conn, wp_row, wp_id, artifact_path)

            conn.commit()
        except Exception:
            conn.rollback()
            raise

        print(json.dumps(table_counts(conn), sort_keys=True))
    finally:
        conn.close()

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
