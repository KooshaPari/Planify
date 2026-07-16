-- Helper views for intent graph analysis and reporting

-- ---------------------------------------------------------------------------
-- v_intent_graph_nodes: human-readable node view with parsed JSON
-- ---------------------------------------------------------------------------
CREATE VIEW IF NOT EXISTS v_intent_graph_nodes AS
SELECT
    id,
    node_type,
    dag_stage,
    title,
    description,
    status,
    json_extract(tags, '$') AS tags,
    json_extract(meta, '$.timestamp') AS meta_timestamp,
    json_extract(meta, '$.source') AS meta_source,
    json_extract(meta, '$.confidence') AS meta_confidence,
    json_extract(meta, '$.agent_id') AS meta_agent_id,
    json_extract(properties, '$') AS properties,
    table_ref,
    table_id,
    created_at,
    updated_at
FROM intent_nodes;

-- ---------------------------------------------------------------------------
-- v_intent_graph_edges: human-readable edge view with parsed canonical_map
-- ---------------------------------------------------------------------------
CREATE VIEW IF NOT EXISTS v_intent_graph_edges AS
SELECT
    e.id,
    e.source,
    e.target,
    e.relationship_type,
    json_extract(e.canonical_map, '$.link_type') AS canonical_link_type,
    json_extract(e.canonical_map, '$.direction') AS canonical_direction,
    json_extract(e.meta, '$.timestamp') AS meta_timestamp,
    json_extract(e.meta, '$.source') AS meta_source,
    json_extract(e.meta, '$.confidence') AS meta_confidence,
    json_extract(e.meta, '$.agent_id') AS meta_agent_id,
    json_extract(e.properties, '$') AS properties,
    e.created_at,
    src.node_type AS source_type,
    tgt.node_type AS target_type
FROM intent_edges e
LEFT JOIN intent_nodes src ON src.id = e.source
LEFT JOIN intent_nodes tgt ON tgt.id = e.target;

-- ---------------------------------------------------------------------------
-- v_intent_graph_summary: aggregate counts per node type
-- ---------------------------------------------------------------------------
CREATE VIEW IF NOT EXISTS v_intent_graph_summary AS
SELECT
    node_type,
    COUNT(*) AS count,
    status,
    dag_stage
FROM intent_nodes
GROUP BY node_type, status, dag_stage
ORDER BY count DESC;

-- ---------------------------------------------------------------------------
-- v_intent_graph_dag_stages: stage flow with node counts
-- ---------------------------------------------------------------------------
CREATE VIEW IF NOT EXISTS v_intent_graph_dag_stages AS
SELECT
    dag_stage,
    COUNT(*) AS node_count,
    GROUP_CONCAT(DISTINCT node_type) AS node_types
FROM intent_nodes
GROUP BY dag_stage
ORDER BY
    CASE dag_stage
        WHEN 'intent' THEN 1
        WHEN 'plan' THEN 2
        WHEN 'feature' THEN 3
        WHEN 'story' THEN 4
        WHEN 'task' THEN 5
        WHEN 'spec' THEN 6
        WHEN 'commit' THEN 7
        WHEN 'test' THEN 8
        WHEN 'pr' THEN 9
        WHEN 'bug' THEN 10
        WHEN 'artifact' THEN 11
        ELSE 99
    END;

-- ---------------------------------------------------------------------------
-- v_intent_orphans: nodes with no incoming or outgoing edges
-- ---------------------------------------------------------------------------
CREATE VIEW IF NOT EXISTS v_intent_orphans AS
SELECT n.*
FROM intent_nodes n
LEFT JOIN intent_edges e_src ON e_src.source = n.id
LEFT JOIN intent_edges e_tgt ON e_tgt.target = n.id
WHERE e_src.id IS NULL AND e_tgt.id IS NULL;

-- ---------------------------------------------------------------------------
-- v_intent_cycles: potential cycle pairs (same source/target in both dirs)
-- ---------------------------------------------------------------------------
CREATE VIEW IF NOT EXISTS v_intent_cycles AS
SELECT
    a.id AS edge_a_id,
    a.source AS node_a,
    a.target AS node_b,
    b.id AS edge_b_id,
    a.relationship_type AS rel_a,
    b.relationship_type AS rel_b
FROM intent_edges a
JOIN intent_edges b ON a.source = b.target AND a.target = b.source
WHERE a.id < b.id;
