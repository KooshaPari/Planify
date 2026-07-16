-- Migration 025: Create intent graph tables
-- Maps the AgilePlus Intent Graph Ontology to SQLite schema

-- ---------------------------------------------------------------------------
-- intent_nodes: typed nodes in the intent graph
-- ---------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS intent_nodes (
    id TEXT PRIMARY KEY NOT NULL,
    node_type TEXT NOT NULL,
    dag_stage TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL,
    tags TEXT DEFAULT '[]',         -- JSON array of strings
    meta TEXT DEFAULT '{}' NOT NULL, -- JSON object: required {timestamp, source}
    properties TEXT DEFAULT '{}',    -- JSON object: free-form key/value
    table_ref TEXT,                 -- foreign table name (e.g., "features")
    table_id TEXT,                  -- foreign row id within table_ref
    created_at TEXT NOT NULL,       -- ISO 8601
    updated_at TEXT NOT NULL,       -- ISO 8601

    -- referential integrity: table_ref + table_id must exist in target
    -- (enforced via application layer or deferred triggers if needed)

    CONSTRAINT valid_node_id CHECK (id REGEXP '^[A-Z][a-z]+#[a-z0-9\-]+$'),
    CONSTRAINT valid_node_type CHECK (node_type IN (
        'Intent','Plan','Feature','Story','Task',
        'Spec','Commit','Test','PR','Bug','Artifact'
    )),
    CONSTRAINT valid_dag_stage CHECK (dag_stage IN (
        'intent','plan','feature','story','task',
        'spec','commit','test','pr','bug','artifact'
    )),
    CONSTRAINT valid_status CHECK (status IN (
        'draft','active','completed','deprecated','rejected',
        'open','in_progress','blocked','deferred','cancelled'
    ))
);

-- ---------------------------------------------------------------------------
-- intent_edges: typed relationships between nodes
-- ---------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS intent_edges (
    id TEXT PRIMARY KEY NOT NULL,         -- UUID
    source TEXT NOT NULL,
    target TEXT NOT NULL,
    relationship_type TEXT NOT NULL,
    canonical_map TEXT DEFAULT '{}',      -- JSON object: {link_type, direction}
    meta TEXT DEFAULT '{}' NOT NULL,      -- JSON object: required {timestamp, source}
    properties TEXT DEFAULT '{}',         -- JSON object: free-form key/value
    created_at TEXT NOT NULL,             -- ISO 8601

    -- soft foreign keys to intent_nodes
    CONSTRAINT fk_edge_source FOREIGN KEY (source) REFERENCES intent_nodes(id) ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT fk_edge_target FOREIGN KEY (target) REFERENCES intent_nodes(id) ON DELETE CASCADE ON UPDATE CASCADE,

    CONSTRAINT valid_relationship CHECK (relationship_type IN (
        'implements','tests','covers','traces-to',
        'derives-from','resolves','blocks','depends-on'
    )),
    CONSTRAINT no_self_loop CHECK (source != target)
);

-- ---------------------------------------------------------------------------
-- intent_graph_metadata: singleton-ish record for graph versioning
-- ---------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS intent_graph_metadata (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    version TEXT NOT NULL,
    schema_uri TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    node_count INTEGER NOT NULL DEFAULT 0,
    edge_count INTEGER NOT NULL DEFAULT 0,
    dag_valid INTEGER NOT NULL DEFAULT 0,  -- BOOLEAN (0/1)
    source_system TEXT
);

-- ---------------------------------------------------------------------------
-- Indexes for common lookup patterns
-- ---------------------------------------------------------------------------
CREATE INDEX IF NOT EXISTS idx_nodes_type ON intent_nodes(node_type);
CREATE INDEX IF NOT EXISTS idx_nodes_stage ON intent_nodes(dag_stage);
CREATE INDEX IF NOT EXISTS idx_nodes_status ON intent_nodes(status);
CREATE INDEX IF NOT EXISTS idx_nodes_table_ref ON intent_nodes(table_ref, table_id);
CREATE INDEX IF NOT EXISTS idx_nodes_created_at ON intent_nodes(created_at);
CREATE INDEX IF NOT EXISTS idx_nodes_updated_at ON intent_nodes(updated_at);

CREATE INDEX IF NOT EXISTS idx_edges_source ON intent_edges(source);
CREATE INDEX IF NOT EXISTS idx_edges_target ON intent_edges(target);
CREATE INDEX IF NOT EXISTS idx_edges_relationship ON intent_edges(relationship_type);
CREATE INDEX IF NOT EXISTS idx_edges_created_at ON intent_edges(created_at);

CREATE UNIQUE INDEX IF NOT EXISTS idx_edges_pair ON intent_edges(source, target, relationship_type);

-- ---------------------------------------------------------------------------
-- Triggers: maintain metadata counts on insert/delete
-- ---------------------------------------------------------------------------
CREATE TRIGGER IF NOT EXISTS tg_nodes_insert
AFTER INSERT ON intent_nodes
BEGIN
    UPDATE intent_graph_metadata
    SET node_count = node_count + 1,
        updated_at = datetime('now')
    WHERE id = (SELECT id FROM intent_graph_metadata ORDER BY id DESC LIMIT 1);
END;

CREATE TRIGGER IF NOT EXISTS tg_nodes_delete
AFTER DELETE ON intent_nodes
BEGIN
    UPDATE intent_graph_metadata
    SET node_count = node_count - 1,
        updated_at = datetime('now')
    WHERE id = (SELECT id FROM intent_graph_metadata ORDER BY id DESC LIMIT 1);
END;

CREATE TRIGGER IF NOT EXISTS tg_edges_insert
AFTER INSERT ON intent_edges
BEGIN
    UPDATE intent_graph_metadata
    SET edge_count = edge_count + 1,
        updated_at = datetime('now')
    WHERE id = (SELECT id FROM intent_graph_metadata ORDER BY id DESC LIMIT 1);
END;

CREATE TRIGGER IF NOT EXISTS tg_edges_delete
AFTER DELETE ON intent_edges
BEGIN
    UPDATE intent_graph_metadata
    SET edge_count = edge_count - 1,
        updated_at = datetime('now')
    WHERE id = (SELECT id FROM intent_graph_metadata ORDER BY id DESC LIMIT 1);
END;
