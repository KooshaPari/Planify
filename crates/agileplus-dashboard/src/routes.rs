//! Axum route handlers for the dashboard.  (T077)
//!
//! Pattern: if the request carries `HX-Request: true`, return only the
//! relevant partial; otherwise return the full page layout.

use std::collections::HashMap;
use std::env;

use askama::Template;
use axum::{
    Router,
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
};
use chrono::Utc;

use agileplus_domain::domain::{
    feature::Feature, state_machine::FeatureState, work_package::WpState,
};

use crate::app_state::SharedState;
use crate::process_detector;
use crate::templates::{
    AgentActivityPartial, AgentSettingsPage, AgentView, DashboardPage, EcosystemProject,
    EventTimelinePartial, EventsPage, EvidenceBundleView, FeatureDetailPage, FeatureView,
    FeaturesPage, HealthPanelPartial, HomePage, HubPage, KanbanPartial, MediaAssetView,
    PlaneHealthEndpointView, PlaneSettingsPage, ProjectSummaryView, ProjectSwitcherPartial,
    ProjectView, ReportArtifactView, ServicesSettingsPage, SettingsPage, ToastPartial,
    WpListPartial, WpView, all_feature_states,
};

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

// ── Configuration Types ────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaneConfig {
    pub api_url: String,
    pub api_key: String,
    pub workspace_slug: String,
    pub project_slug: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub pool_size: usize,
    pub retry_budget: usize,
    pub dispatch_mode: String,
    pub default_provider: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub name: String,
    pub endpoint_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardConfig {
    pub theme: String,
    pub log_level: String,
    pub data_directory: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub plane: Option<PlaneConfig>,
    pub agents: Option<AgentConfig>,
    pub services: Option<Vec<ServiceConfig>>,
    pub dashboard: Option<DashboardConfig>,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Self::config_path();
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            let config = toml::from_str(&content)?;
            Ok(config)
        } else {
            Ok(Config {
                plane: None,
                agents: None,
                services: None,
                dashboard: None,
            })
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::config_path();
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)?;
        fs::write(config_path, content)?;
        Ok(())
    }

    fn config_path() -> PathBuf {
        std::env::var("HOME")
            .ok()
            .map(|home| PathBuf::from(home).join(".agileplus/config.toml"))
            .unwrap_or_else(|| PathBuf::from(".agileplus/config.toml"))
    }
}

// ── Form Request Types ────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct PlaneSettingsForm {
    pub api_url: String,
    pub api_key: String,
    pub workspace_slug: String,
    pub project_slug: String,
}

#[derive(Debug, Deserialize)]
pub struct AgentSettingsForm {
    pub pool_size: usize,
    pub retry_budget: usize,
    pub dispatch_mode: String,
    pub default_provider: String,
}

#[derive(Debug, Deserialize)]
pub struct ServiceSettingsForm {
    pub names: Vec<String>,
    pub endpoint_urls: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct DashboardSettingsForm {
    pub theme: String,
    pub log_level: String,
    pub data_directory: String,
}


/// Returns `true` if the `HX-Request` header is present and truthy.
fn is_htmx(headers: &HeaderMap) -> bool {
    headers
        .get("HX-Request")
        .and_then(|v| v.to_str().ok())
        .map(|v| v == "true")
        .unwrap_or(false)
}

fn render<T: Template>(tpl: T) -> Response {
    match tpl.render() {
        Ok(html) => Html(html).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Template error: {e}"),
        )
            .into_response(),
    }
}

/// Build the project list and active project from the store.
fn load_projects(
    store: &crate::app_state::DashboardStore,
) -> (Vec<ProjectView>, Option<ProjectView>) {
    let projects: Vec<ProjectView> = store
        .projects
        .iter()
        .map(|p| ProjectView {
            id: p.id,
            slug: p.slug.clone(),
            name: p.name.clone(),
            description: p.description.clone(),
        })
        .collect();
    let active_project = store.active_project().map(|p| ProjectView {
        id: p.id,
        slug: p.slug.clone(),
        name: p.name.clone(),
        description: p.description.clone(),
    });
    (projects, active_project)
}

fn build_project_summaries(store: &crate::app_state::DashboardStore) -> Vec<ProjectSummaryView> {
    store
        .projects
        .iter()
        .map(|project| {
            let (feature_count, active_count, shipped_count) =
                store.feature_counts_for_project(project.id);
            ProjectSummaryView {
                project: ProjectView {
                    id: project.id,
                    slug: project.slug.clone(),
                    name: project.name.clone(),
                    description: project.description.clone(),
                },
                feature_count,
                active_count,
                shipped_count,
            }
        })
        .collect()
}

const DEFAULT_PLANE_API_URL: &str = "https://app.plane.so";
const DEFAULT_PLANE_WEB_URL: &str = "https://app.plane.so";

fn env_or_none(key: &str) -> Option<String> {
    env::var(key)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn parse_bool_env(key: &str, default: bool) -> bool {
    env::var(key)
        .ok()
        .map(|value| {
            matches!(
                value.trim().to_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
        .unwrap_or(default)
}

fn plane_api_key_hint(api_key: &Option<String>) -> String {
    match api_key {
        Some(key) => match (key.chars().next(), key.chars().next_back()) {
            (Some(first), Some(last)) => format!("{first}••••••{last}"),
            _ => "Configured".to_string(),
        },
        None => "Not configured".to_string(),
    }
}

fn plane_health_endpoints(
    services: &[crate::app_state::ServiceHealth],
) -> Vec<PlaneHealthEndpointView> {
    services
        .iter()
        .filter(|service| service.name.contains("Plane") || service.name.starts_with("API"))
        .map(|service| PlaneHealthEndpointView {
            name: service.name.clone(),
            healthy: service.healthy,
            degraded: service.degraded,
            latency_ms: service.latency_ms,
            last_check_utc: service
                .last_check
                .format("%Y-%m-%d %H:%M:%S UTC")
                .to_string(),
        })
        .collect()
}

fn build_feature_events(
    feature: &FeatureView,
    workpackages: &[WpView],
) -> Vec<crate::templates::EventView> {
    let now = Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();
    let mut events = vec![crate::templates::EventView {
        id: format!("evt-feature-{}-created", feature.id),
        kind: "system".into(),
        description: format!("Feature '{}' opened in dashboard", feature.slug),
        timestamp: now.clone(),
        agent_name: None,
        agent_link: None,
        wp_id: None,
        wp_link: None,
        commit_sha: None,
        commit_link: None,
        ci_run_id: None,
        ci_run_link: None,
    }];

    if !workpackages.is_empty() {
        events.push(crate::templates::EventView {
            id: format!("evt-feature-{}-sync", feature.id),
            kind: "agent_action".into(),
            description: format!("{} work package entries synced", workpackages.len()),
            timestamp: now.clone(),
            agent_name: Some("sync-agent".to_string()),
            agent_link: Some("/agents/sync-agent".to_string()),
            wp_id: None,
            wp_link: None,
            commit_sha: Some("7c5b6ef".to_string()),
            commit_link: Some("https://github.com/Phenotype/AgilePlus/commit/7c5b6ef".to_string()),
            ci_run_id: Some("1024".to_string()),
            ci_run_link: Some(
                "https://github.com/Phenotype/AgilePlus/actions/runs/1024".to_string(),
            ),
        });

        for wp in workpackages {
            // agent_link: route to agent detail page when an agent_id is present.
            let agent_link = wp
                .agent_id
                .as_deref()
                .map(|aid| format!("/api/dashboard/agents/{aid}"));

            // wp_link: slug-based URL to the work package detail anchor.
            let wp_link = Some(format!(
                "/features/{}/work-packages/{}",
                feature.slug, wp.id
            ));

            // commit_link: GitHub commit URL when a head commit SHA is present.
            let (commit_sha, commit_link) = match &wp.head_commit {
                Some(sha) => (
                    Some(sha.clone()),
                    Some(format!(
                        "https://github.com/KooshaPari/AgilePlus/commit/{sha}"
                    )),
                ),
                None => (None, None),
            };

            // ci_run_link: derive from pr_url when it is a GitHub PR URL by
            // redirecting to the Actions tab for that repository.
            let ci_run_link = wp.pr_url.as_deref().and_then(|url| {
                // pr_url is typically https://github.com/{owner}/{repo}/pull/{n}
                // Strip the `/pull/{n}` suffix and append `/actions` for the runs view.
                let prefix = url
                    .split("/pull/")
                    .next()
                    .filter(|p| p.starts_with("https://github.com/"))?;
                Some(format!("{prefix}/actions"))
            });

            events.push(crate::templates::EventView {
                id: format!("evt-feature-{}-wp-{}", feature.id, wp.id),
                kind: "state_change".into(),
                description: format!("Work-package {} is in state '{}'", wp.title, wp.state),
                timestamp: now.clone(),
                agent_name: wp.agent_id.clone(),
                agent_link,
                wp_id: Some(wp.id.to_string()),
                wp_link,
                commit_sha,
                commit_link,
                ci_run_id: None,
                ci_run_link,
            });
        }
    } else {
        events.push(crate::templates::EventView {
            id: format!("evt-feature-{}-no-wp", feature.id),
            kind: "system".into(),
            description: "No work packages linked yet".into(),
            timestamp: now.clone(),
            agent_name: None,
            agent_link: None,
            wp_id: None,
            wp_link: None,
            commit_sha: None,
            commit_link: None,
            ci_run_id: None,
            ci_run_link: None,
        });
    }

    events
}

fn build_feature_evidence_bundles(
    feature: &FeatureView,
    workpackages: &[WpView],
) -> Vec<EvidenceBundleView> {
    let mut bundles = vec![EvidenceBundleView {
        id: format!("bundle-{id}-summary", id = feature.id),
        fr_id: format!("FR-{id}", id = feature.id),
        evidence_type: "feature_summary".into(),
        wp_id: "dashboard".into(),
        wp_title: feature.title.clone(),
        artifact_path: format!("/artifacts/features/{}.md", feature.slug),
        created_at: Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string(),
        artifact_ext: "md".into(),
        status: "available".into(),
        content_preview: Some(
            "# Feature Summary

This feature provides..."
                .to_string(),
        ),
        is_text_artifact: true,
        is_image_artifact: false,
        download_url: format!("/api/evidence/{}/summary/content", feature.id),
    }];

    for wp in workpackages {
        bundles.push(EvidenceBundleView {
            id: format!("bundle-{fid}-wp-{wid}", fid = feature.id, wid = wp.id),
            fr_id: format!("FR-{fid}", fid = feature.id),
            evidence_type: "workpackage_artifact".into(),
            wp_id: wp.id.to_string(),
            wp_title: wp.title.clone(),
            artifact_path: format!(
                "/artifacts/wp/{wid}/{slug}.json",
                wid = wp.id,
                slug = feature.slug
            ),
            created_at: Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string(),
            artifact_ext: "json".into(),
            status: if wp.progress > 0 {
                "accepted"
            } else {
                "generated"
            }
            .into(),
            content_preview: Some(r#"{"status":"generated","progress":0}"#.to_string()),
            is_text_artifact: true,
            is_image_artifact: false,
            download_url: format!("/api/evidence/{}/{}/content", feature.id, wp.id),
        });
    }

    bundles
}

fn build_feature_media_assets(
    feature: &FeatureView,
    workpackages: &[WpView],
) -> Vec<MediaAssetView> {
    let mut media = vec![MediaAssetView {
        id: format!("media-{id}-cover", id = feature.id),
        source: "dashboard".into(),
        name: format!("{slug}-hero.png", slug = feature.slug),
        kind: "image".into(),
        mime: "image/png".into(),
        url_or_path: format!("/assets/{slug}/cover.png", slug = feature.slug),
        size_bytes: 128_512,
        uploaded_at: Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string(),
    }];

    for wp in workpackages {
        media.push(MediaAssetView {
            id: format!("media-{fid}-wp-{wid}", fid = feature.id, wid = wp.id),
            source: "agent-work-package".into(),
            name: format!("{slug}-wp-{wid}.png", slug = feature.slug, wid = wp.id),
            kind: "screenshot".into(),
            mime: "image/png".into(),
            url_or_path: format!("/assets/wp/{wid}/coverage.png", wid = wp.id),
            size_bytes: 84_320 + (wp.id as usize * 3_000),
            uploaded_at: Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string(),
        });
    }

    media
}

fn build_feature_reports(
    feature: &FeatureView,
    workpackages: &[WpView],
) -> Vec<ReportArtifactView> {
    vec![ReportArtifactView {
        id: format!("report-{id}-coverage", id = feature.id),
        name: format!("Feature Coverage Report — {name}", name = feature.title),
        source: "coverage-engine".into(),
        status: "completed".into(),
        generated_at: Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string(),
        rule_count: 5,
        satisfied_count: if feature.labels.is_empty() {
            2
        } else {
            feature.labels.len() + 2
        },
        compliant: !workpackages.is_empty(),
    }]
}

fn plane_sync_mode() -> String {
    if parse_bool_env("PLANE_SYNC_BIDIRECTIONAL", false) {
        "Bidirectional".to_string()
    } else {
        "One-way".to_string()
    }
}

fn plane_connection_checks(
    api_key: &Option<String>,
    workspace: &Option<String>,
) -> (bool, String, Vec<String>) {
    let mut warnings = Vec::new();
    if api_key.is_none() {
        warnings.push("Missing PLANE_API_KEY; configure a valid Plane API key".to_string());
    }
    if workspace.is_none() {
        warnings.push("Missing PLANE_WORKSPACE; set workspace slug for Plane sync".to_string());
    }

    if warnings.is_empty() {
        (true, "Connected via PLANE_API_KEY".to_string(), warnings)
    } else if warnings.len() == 1 {
        let status = warnings[0].clone();
        (false, status, warnings)
    } else {
        (false, "Plane settings incomplete".to_string(), warnings)
    }
}

fn percentage_coverage(hit: usize, total: usize) -> String {
    if total == 0 {
        return "0/0 (0%)".to_string();
    }
    let ratio = (hit.saturating_mul(100)).saturating_div(total);
    format!("{hit}/{total} ({ratio}%)")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DashboardFilter {
    All,
    Active,
    Blocked,
    Shipped,
}

fn dashboard_filter_from_query(query: &HashMap<String, String>) -> DashboardFilter {
    match query.get("filter").map(|value| value.as_str()) {
        Some("active") => DashboardFilter::Active,
        Some("blocked") => DashboardFilter::Blocked,
        Some("shipped") => DashboardFilter::Shipped,
        _ => DashboardFilter::All,
    }
}

fn feature_matches_filter(
    store: &crate::app_state::DashboardStore,
    feature: &Feature,
    filter: DashboardFilter,
) -> bool {
    let is_blocked = store
        .work_packages
        .get(&feature.id)
        .map(|workpackages| workpackages.iter().any(|wp| wp.state == WpState::Blocked))
        .unwrap_or(false);

    match filter {
        DashboardFilter::All => true,
        DashboardFilter::Active => !matches!(
            feature.state,
            FeatureState::Shipped | FeatureState::Retrospected
        ),
        DashboardFilter::Blocked => is_blocked,
        DashboardFilter::Shipped => matches!(
            feature.state,
            FeatureState::Shipped | FeatureState::Retrospected
        ),
    }
}

fn build_kanban_cards(
    store: &crate::app_state::DashboardStore,
    filter: DashboardFilter,
) -> HashMap<String, Vec<FeatureView>> {
    let states = all_feature_states();
    let mut cards: HashMap<String, Vec<FeatureView>> = HashMap::new();
    for s in &states {
        cards.insert(s.clone(), vec![]);
    }
    // Group active features by state after applying project and sidebar filters.
    for feature in store.features_for_active_project() {
        if !feature_matches_filter(store, feature, filter) {
            continue;
        }
        let state_key = feature.state.to_string();
        let view = FeatureView::from_feature(feature);
        cards.entry(state_key).or_default().push(view);
    }
    cards
}

fn sample_events() -> Vec<crate::templates::EventView> {
    vec![
        crate::templates::EventView {
            id: "evt-1".into(),
            kind: "system".into(),
            description: "Dashboard booted with native Plane surface".into(),
            timestamp: "just now".into(),
            agent_name: None,
            agent_link: None,
            wp_id: None,
            wp_link: None,
            commit_sha: None,
            commit_link: None,
            ci_run_id: None,
            ci_run_link: None,
        },
        crate::templates::EventView {
            id: "evt-2".into(),
            kind: "agent_action".into(),
            description: "Planner synced feature ownership metadata".into(),
            timestamp: "2m ago".into(),
            agent_name: Some("planner-agent".to_string()),
            agent_link: Some("/agents/planner-agent".to_string()),
            wp_id: None,
            wp_link: None,
            commit_sha: Some("abc1234".to_string()),
            commit_link: Some("https://github.com/example/repo/commit/abc1234".to_string()),
            ci_run_id: None,
            ci_run_link: None,
        },
        crate::templates::EventView {
            id: "evt-3".into(),
            kind: "state_change".into(),
            description: "Feature moved from researched to planned".into(),
            timestamp: "9m ago".into(),
            agent_name: None,
            agent_link: None,
            wp_id: Some("42".to_string()),
            wp_link: Some("/features/1#wp-42".to_string()),
            commit_sha: None,
            commit_link: None,
            ci_run_id: Some("12345678".to_string()),
            ci_run_link: Some("https://github.com/example/repo/actions/runs/12345678".to_string()),
        },
    ]
}

pub async fn root(State(state): State<SharedState>) -> Response {
    let store = state.read().await;
    let total_features = store.features.len();
    let active_features = store
        .features
        .iter()
        .filter(|feature| {
            !matches!(
                feature.state,
                FeatureState::Shipped | FeatureState::Retrospected
            )
        })
        .count();
    let shipped_features = store
        .features
        .iter()
        .filter(|feature| {
            matches!(
                feature.state,
                FeatureState::Shipped | FeatureState::Retrospected
            )
        })
        .count();
    let projects = build_project_summaries(&store);

    render(HomePage {
        total_features,
        active_features,
        shipped_features,
        projects,
    })
}

pub async fn home(State(state): State<SharedState>) -> Response {
    root(State(state)).await
}

// ── /dashboard ───────────────────────────────────────────────────────────

pub async fn dashboard_page(
    State(state): State<SharedState>,
    Query(query): Query<HashMap<String, String>>,
) -> Response {
    let store = state.read().await;
    let filter = dashboard_filter_from_query(&query);
    let cards = build_kanban_cards(&store, filter);
    let (projects, active_project) = load_projects(&store);
    let active_filter = query.get("filter").cloned().unwrap_or_else(|| "all".into());
    render(DashboardPage {
        kanban_cards: cards,
        health: store.health.clone(),
        projects,
        active_project,
        active_filter,
    })
}

// ── /api/dashboard/kanban ────────────────────────────────────────────────

pub async fn kanban_board(
    State(state): State<SharedState>,
    headers: HeaderMap,
    Query(query): Query<HashMap<String, String>>,
) -> Response {
    let store = state.read().await;
    let filter = dashboard_filter_from_query(&query);
    let cards = build_kanban_cards(&store, filter);
    let active_filter = query.get("filter").cloned().unwrap_or_else(|| "all".into());

    if is_htmx(&headers) {
        render(KanbanPartial { cards })
    } else {
        let (projects, active_project) = load_projects(&store);
        render(DashboardPage {
            kanban_cards: cards,
            health: store.health.clone(),
            projects,
            active_project,
            active_filter,
        })
    }
}

// ── /api/dashboard/features/:id ─────────────────────────────────────────

pub async fn feature_detail(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
    _headers: HeaderMap,
) -> Response {
    let store = state.read().await;
    let feature = match store.features.iter().find(|f| f.id == id) {
        Some(f) => FeatureView::from_feature(f),
        None => return (StatusCode::NOT_FOUND, "Feature not found").into_response(),
    };
    let fid = feature.id;
    let wps: Vec<WpView> = store
        .work_packages
        .get(&id)
        .map(|v| v.iter().map(WpView::from_wp).collect())
        .unwrap_or_default();
    let events = build_feature_events(&feature, &wps);
    let evidence_bundles = build_feature_evidence_bundles(&feature, &wps);
    let media_assets = build_feature_media_assets(&feature, &wps);
    let reports = build_feature_reports(&feature, &wps);

    render(FeatureDetailPage {
        feature,
        feature_id: fid,
        workpackages: wps,
        events,
        evidence_bundles,
        media_assets,
        reports,
    })
}

// ── /api/dashboard/features/:id/work-packages ────────────────────────────

pub async fn wp_list(State(state): State<SharedState>, Path(id): Path<i64>) -> Response {
    let store = state.read().await;
    let wps: Vec<WpView> = store
        .work_packages
        .get(&id)
        .map(|v| v.iter().map(WpView::from_wp).collect())
        .unwrap_or_default();
    render(WpListPartial {
        feature_id: id,
        workpackages: wps,
    })
}

// ── /api/dashboard/health ────────────────────────────────────────────────

pub async fn health_panel(State(state): State<SharedState>) -> Response {
    let store = state.read().await;
    render(HealthPanelPartial {
        services: store.health.clone(),
    })
}

// ── /api/dashboard/events ────────────────────────────────────────────────

pub async fn event_timeline(State(state): State<SharedState>) -> Response {
    let _ = state.read().await;
    render(EventTimelinePartial {
        feature_id: 0,
        events: vec![],
    })
}

// ── /api/dashboard/agents ────────────────────────────────────────────────

pub async fn agent_activity(State(state): State<SharedState>) -> Response {
    let _ = state.read().await;

    // Detect real agent processes
    let detected = process_detector::detect_agents();

    // Convert detected agents to view models
    let agents: Vec<AgentView> = detected
        .into_iter()
        .map(|agent| {
            let uptime = calculate_uptime(&agent.started_at);
            let worktree_label = agent
                .worktree
                .as_deref()
                .and_then(|wt| wt.split('/').next_back())
                .unwrap_or("")
                .to_string();
            let worktree = agent.worktree.unwrap_or_default();
            AgentView {
                name: agent.name,
                status: agent.status.clone(),
                current_task: agent.current_task,
                last_action: uptime,
                pid: Some(agent.pid),
                started_at: agent.started_at,
                worktree,
                worktree_label,
                is_live: agent.status == "running",
            }
        })
        .collect();

    render(AgentActivityPartial { agents })
}

/// Calculate uptime string from the elapsed duration string produced by
/// `process_detector::get_process_start_time` (e.g. "5m", "1h 20m").
fn calculate_uptime(started_at: &Option<String>) -> String {
    match started_at {
        Some(elapsed) => format!("running for {}", elapsed),
        None => "uptime unknown".into(),
    }
}

// ── /api/dashboard/projects ──────────────────────────────────────────

pub async fn project_switcher(State(state): State<SharedState>) -> Response {
    let store = state.read().await;
    let projects: Vec<ProjectView> = store
        .projects
        .iter()
        .map(|p| ProjectView {
            id: p.id,
            slug: p.slug.clone(),
            name: p.name.clone(),
            description: p.description.clone(),
        })
        .collect();
    render(ProjectSwitcherPartial {
        projects,
        active_id: store.active_project_id,
    })
}

// ── /api/dashboard/projects/:id/activate ─────────────────────────────

pub async fn switch_project(State(state): State<SharedState>, Path(id): Path<i64>) -> Response {
    {
        let mut store = state.write().await;
        if id == 0 {
            // id=0 means "All Projects" -- clear the filter.
            store.active_project_id = None;
        } else if store.projects.iter().any(|p| p.id == id) {
            store.active_project_id = Some(id);
        } else {
            return (StatusCode::NOT_FOUND, "Project not found").into_response();
        }
    }

    // Reload the kanban board with the updated project filter.
    let store = state.read().await;
    let cards = build_kanban_cards(&store, DashboardFilter::All);
    render(KanbanPartial { cards })
}

// ── /settings ────────────────────────────────────────────────────────────

pub async fn settings_page() -> Response {
    render(SettingsPage)
}

// ── /features ────────────────────────────────────────────────────────────

pub async fn features_page(State(state): State<SharedState>) -> Response {
    let store = state.read().await;
    let features = store
        .features
        .iter()
        .map(FeatureView::from_feature)
        .collect::<Vec<_>>();
    render(FeaturesPage { features })
}

// ── /events ──────────────────────────────────────────────────────────────

pub async fn events_page() -> Response {
    render(EventsPage {
        events: sample_events(),
    })
}

// ── /settings/* ──────────────────────────────────────────────────────────

/// Load Plane configuration by merging saved TOML config with env vars.
/// TOML config takes precedence; env vars provide fallback defaults.
fn load_plane_config() -> (Option<String>, Option<String>, Option<String>, Option<String>) {
    let config = Config::load().unwrap_or(Config {
        plane: None,
        agents: None,
        services: None,
        dashboard: None,
    });

    // TOML config fields take precedence over env vars
    let api_url = config
        .plane
        .as_ref()
        .filter(|p| !p.api_url.is_empty())
        .map(|p| p.api_url.clone())
        .or_else(|| env_or_none("PLANE_API_URL"))
        .or_else(|| Some(DEFAULT_PLANE_API_URL.to_string()));

    let api_key = config
        .plane
        .as_ref()
        .filter(|p| !p.api_key.is_empty())
        .map(|p| p.api_key.clone())
        .or_else(|| env_or_none("PLANE_API_KEY"));

    let workspace_slug = config
        .plane
        .as_ref()
        .filter(|p| !p.workspace_slug.is_empty())
        .map(|p| p.workspace_slug.clone())
        .or_else(|| env_or_none("PLANE_WORKSPACE"));

    let project_slug = config
        .plane
        .as_ref()
        .filter(|p| !p.project_slug.is_empty())
        .map(|p| p.project_slug.clone())
        .or_else(|| env_or_none("PLANE_PROJECT"));

    (api_url, api_key, workspace_slug, project_slug)
}

/// Load Agent configuration from saved TOML config, falling back to defaults.
fn load_agent_config_from_toml() -> (usize, usize, String, String) {
    let config = Config::load().unwrap_or(Config {
        plane: None,
        agents: None,
        services: None,
        dashboard: None,
    });

    match config.agents {
        Some(agent_config) => (
            agent_config.pool_size,
            agent_config.retry_budget,
            agent_config.dispatch_mode,
            agent_config.default_provider,
        ),
        None => (
            env_or_none("AGENT_POOL_SIZE")
                .and_then(|v| v.parse().ok())
                .unwrap_or(6),
            env_or_none("AGENT_RETRY_BUDGET")
                .and_then(|v| v.parse().ok())
                .unwrap_or(3),
            env_or_none("AGENT_DISPATCH_MODE").unwrap_or_else(|| "balanced".into()),
            env_or_none("AGENT_DEFAULT_PROVIDER").unwrap_or_else(|| "claude".into()),
        ),
    }
}

/// Make an HTTP request to the Plane API to verify connectivity.
/// Returns (ok, status_message, latency_ms).
async fn plane_api_connectivity_check(
    api_url: &str,
    api_key: &str,
    workspace_slug: &str,
) -> (bool, String, Option<u64>) {
    use std::time::Instant;

    let url = format!(
        "{}/api/v1/workspaces/{}/",
        api_url.trim_end_matches('/'),
        workspace_slug
    );

    let start = Instant::now();
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build();

    let client = match client {
        Ok(c) => c,
        Err(e) => return (false, format!("Failed to create HTTP client: {}", e), None),
    };

    let response = client
        .get(&url)
        .header("X-Api-Key", api_key)
        .header("Content-Type", "application/json")
        .send()
        .await;

    let latency = start.elapsed().as_millis() as u64;

    match response {
        Ok(resp) => {
            let status = resp.status();
            if status.is_success() {
                (
                    true,
                    format!("Plane API connected (HTTP {})", status.as_u16()),
                    Some(latency),
                )
            } else {
                (
                    false,
                    format!("Plane API returned HTTP {}", status.as_u16()),
                    Some(latency),
                )
            }
        }
        Err(e) => (false, format!("Plane API connection failed: {}", e), Some(latency)),
    }
}

pub async fn plane_settings_page(State(state): State<SharedState>) -> Response {
    let store = state.read().await;

    // Load config from TOML first, fall back to env vars
    let (plane_api_url, plane_api_key, plane_workspace, project_slug) = load_plane_config();
    let plane_web_url =
        env_or_none("PLANE_WEB_URL").unwrap_or_else(|| DEFAULT_PLANE_WEB_URL.to_string());
    let (connected, connection_status, mut config_warnings) =
        plane_connection_checks(&plane_api_key, &plane_workspace);

    let plane_health_endpoints = plane_health_endpoints(&store.health);
    let plane_health_healthy = plane_health_endpoints
        .iter()
        .all(|endpoint| endpoint.healthy && !endpoint.degraded);
    let plane_api_latency_ms = plane_health_endpoints
        .iter()
        .find(|endpoint| endpoint.name == "Plane API")
        .and_then(|endpoint| endpoint.latency_ms);

    if !connected {
        config_warnings
            .push("Plane sync disabled until required settings are provided".to_string());
    }

    if !plane_health_healthy {
        config_warnings.push("Plane API health check is not healthy".to_string());
    }

    let mapped_features = store
        .features
        .iter()
        .filter(|feature| feature.plane_issue_id.is_some())
        .count();
    let total_features = store.features.len();
    let mapped_work_packages = store
        .work_packages
        .values()
        .flatten()
        .filter(|wp| wp.plane_sub_issue_id.is_some())
        .count();
    let total_work_packages: usize = store.work_packages.values().map(Vec::len).sum();

    let connection_status_configured = !connection_status.is_empty();

    render(PlaneSettingsPage {
        workspace_name: plane_workspace
            .clone()
            .unwrap_or_else(|| "Not configured".to_string()),
        workspace_slug: plane_workspace.unwrap_or_else(|| "not configured".to_string()),
        project_slug: project_slug.unwrap_or_else(|| "not configured".to_string()),
        plane_api_url: plane_api_url
            .clone()
            .unwrap_or_else(|| DEFAULT_PLANE_API_URL.to_string())
            .trim_end_matches('/')
            .to_string(),
        plane_web_url: plane_web_url.trim_end_matches('/').to_string(),
        plane_api_url_set: plane_api_url
            .as_ref()
            .map(|u| !u.trim().is_empty())
            .unwrap_or(false),
        plane_web_url_set: !plane_web_url.trim_end_matches('/').is_empty(),
        plane_api_key_hint: plane_api_key_hint(&plane_api_key),
        plane_api_key_set: plane_api_key.is_some(),
        sync_enabled: connected,
        sync_mode: plane_sync_mode(),
        connected,
        connection_status: connection_status.clone(),
        connection_status_configured,
        plane_service_healthy: plane_health_healthy,
        plane_api_latency_ms,
        plane_health_endpoints,
        mapped_features_coverage: percentage_coverage(mapped_features, total_features),
        mapped_work_packages_coverage: percentage_coverage(
            mapped_work_packages,
            total_work_packages,
        ),
        mapped_features,
        mapped_work_packages,
        config_warnings,
    })
}

pub async fn agent_settings_page() -> Response {
    let (pool_size, retry_budget, dispatch_mode, _default_provider) = load_agent_config_from_toml();
    render(AgentSettingsPage {
        agent_pool_size: pool_size,
        retry_budget,
        dispatch_mode,
    })
}

pub async fn services_settings_page(State(state): State<SharedState>) -> Response {
    let store = state.read().await;
    let config = Config::load().unwrap_or(Config {
        plane: None,
        agents: None,
        services: None,
        dashboard: None,
    });

    let configs: Vec<crate::templates::ServiceConfigView> = config
        .services
        .unwrap_or_default()
        .into_iter()
        .map(|s| crate::templates::ServiceConfigView {
            name: s.name,
            endpoint_url: s.endpoint_url,
        })
        .collect();

    render(ServicesSettingsPage {
        services: store.health.clone(),
        configs,
    })
}

// ── /hub ─────────────────────────────────────────────────────────────────

pub async fn hub_page() -> Response {
    let projects = vec![
        EcosystemProject {
            name: "phenodocs",
            tagline: "Ecosystem docs hub",
            stack: "TypeScript · Vue",
            port: Some(4100),
            github: "https://github.com/KooshaPari/phenodocs",
            category: "docs",
        },
        EcosystemProject {
            name: "AgilePlus",
            tagline: "Spec-driven PM platform",
            stack: "Rust · Tauri",
            port: Some(4101),
            github: "https://github.com/KooshaPari/AgilePlus",
            category: "app",
        },
        EcosystemProject {
            name: "heliosApp",
            tagline: "TypeScript runtime app",
            stack: "TypeScript · Bun",
            port: Some(4102),
            github: "https://github.com/KooshaPari/heliosApp",
            category: "app",
        },
        EcosystemProject {
            name: "thegent",
            tagline: "Agent framework",
            stack: "TypeScript · Python",
            port: Some(4103),
            github: "https://github.com/KooshaPari/thegent",
            category: "lib",
        },
        EcosystemProject {
            name: "bifrost-extensions",
            tagline: "LLM gateway extensions",
            stack: "Go",
            port: Some(4104),
            github: "https://github.com/KooshaPari/bifrost-extensions",
            category: "lib",
        },
        EcosystemProject {
            name: "civ",
            tagline: "CI validation",
            stack: "TypeScript",
            port: Some(4105),
            github: "https://github.com/KooshaPari/civ",
            category: "docs",
        },
        EcosystemProject {
            name: "TraceRTM",
            tagline: "Requirements traceability",
            stack: "Python · Go · TS",
            port: Some(4110),
            github: "https://github.com/KooshaPari/trace",
            category: "app",
        },
        EcosystemProject {
            name: "agentapi-plusplus",
            tagline: "Agent HTTP API",
            stack: "Go",
            port: None,
            github: "https://github.com/KooshaPari/agentapi-plusplus",
            category: "api",
        },
        EcosystemProject {
            name: "cliproxyapi-plusplus",
            tagline: "Multi-provider CLI proxy",
            stack: "Go",
            port: None,
            github: "https://github.com/KooshaPari/cliproxyapi-plusplus",
            category: "api",
        },
    ];
    render(HubPage { projects })
}

// ── /api/time ────────────────────────────────────────────────────────────

pub async fn time_footer() -> Html<String> {
    Html(
        chrono::Utc::now()
            .format("%Y-%m-%d %H:%M:%S UTC")
            .to_string(),
    )
}

// ── /api/evidence ────────────────────────────────────────────────────────

pub async fn evidence_content(
    State(_state): State<SharedState>,
    Path((feature_id, artifact_id)): Path<(i64, String)>,
) -> Response {
    // In production, this would serve from MinIO or local filesystem
    // For now, return a sample response
    let sample_content = format!(
        "# Evidence Bundle {}

## Artifact ID: {}

This is sample evidence content.",
        feature_id, artifact_id
    );
    Html(sample_content).into_response()
}

pub async fn evidence_preview(
    State(_state): State<SharedState>,
    Path((feature_id, artifact_id)): Path<(i64, String)>,
) -> Response {
    // Return an htmx partial with inline preview
    let preview = format!(
        "<div class='p-3 rounded bg-zinc-800 border border-zinc-700'>         <pre class='text-xs font-mono text-zinc-300'>Evidence #{} - {}</pre>         </div>",
        feature_id, artifact_id
    );
    Html(preview).into_response()
}

pub async fn stream_placeholder() -> StatusCode {
    StatusCode::NO_CONTENT
}

// ── /api/dashboard/services/:name/restart ────────────────────────────────

pub async fn restart_service(Path(name): Path<String>) -> impl IntoResponse {
    // TODO: wire to actual process/container restart logic (e.g. systemd, docker, process-compose)
    axum::Json(serde_json::json!({ "status": "ok", "service": name }))
}

// ── /api/dashboard/services/:name/config  (PATCH) ────────────────────────

#[derive(Debug, Deserialize)]
pub struct ServiceConfigForm {
    pub endpoint_url: Option<String>,
    pub timeout_ms: Option<u64>,
    pub max_retries: Option<u32>,
}

pub async fn patch_service_config(
    Path(name): Path<String>,
    axum::Form(form): axum::Form<ServiceConfigForm>,
) -> impl IntoResponse {
    let mut config = Config::load().unwrap_or(Config {
        plane: None,
        agents: None,
        services: None,
        dashboard: None,
    });

    let services = config.services.get_or_insert_with(Vec::new);
    if let Some(entry) = services.iter_mut().find(|s| s.name == name) {
        if let Some(url) = form.endpoint_url.filter(|u| !u.trim().is_empty()) {
            entry.endpoint_url = url;
        }
    } else if let Some(url) = form.endpoint_url.filter(|u| !u.trim().is_empty()) {
        services.push(ServiceConfig { name: name.clone(), endpoint_url: url });
    }

    match config.save() {
        Ok(_) => render(ToastPartial {
            message: format!("Service '{}' configuration saved", name),
            success: true,
        }),
        Err(e) => render(ToastPartial {
            message: format!("Failed to save: {}", e),
            success: false,
        }),
    }
}

// ── /api/dashboard/services/:name/toggle (POST) ──────────────────────────

#[derive(Debug, Deserialize)]
pub struct ServiceToggleBody {
    pub enabled: Option<bool>,
}

pub async fn toggle_service(
    Path(name): Path<String>,
    axum::Json(body): axum::Json<ServiceToggleBody>,
) -> impl IntoResponse {
    // TODO: propagate enable/disable to process manager or config store
    let enabled = body.enabled.unwrap_or(true);
    axum::Json(serde_json::json!({ "status": "ok", "service": name, "enabled": enabled }))
}

// ── /api/settings/agents/test-connection (POST) ──────────────────────────

#[derive(Debug, Deserialize)]
pub struct AgentTestConnectionForm {
    pub provider: String,
}

pub async fn test_agent_connection(
    axum::Form(form): axum::Form<AgentTestConnectionForm>,
) -> impl IntoResponse {
    use std::time::Instant;

    let (ok, msg) = match form.provider.as_str() {
        "claude" => {
            let key = env_or_none("ANTHROPIC_API_KEY");
            match key {
                Some(api_key) => {
                    // Make a real API call to validate the key
                    let start = Instant::now();
                    let client = reqwest::Client::builder()
                        .timeout(std::time::Duration::from_secs(10))
                        .build();
                    match client {
                        Ok(client) => {
                            let response = client
                                .get("https://api.anthropic.com/v1/models")
                                .header("x-api-key", &api_key)
                                .header("anthropic-version", "2023-06-01")
                                .send()
                                .await;
                            let latency = start.elapsed().as_millis();
                            match response {
                                Ok(resp) => {
                                    let status = resp.status();
                                    if status.is_success() {
                                        (true, format!("Claude API key valid ({}ms)", latency))
                                    } else {
                                        (false, format!("Claude API returned HTTP {} ({}ms)", status.as_u16(), latency))
                                    }
                                }
                                Err(e) => (false, format!("Claude API connection failed: {}", e)),
                            }
                        }
                        Err(e) => (false, format!("Failed to create HTTP client: {}", e)),
                    }
                }
                None => (false, "ANTHROPIC_API_KEY not set".to_string()),
            }
        }
        "gemini" => {
            let key = env_or_none("GEMINI_API_KEY").or_else(|| env_or_none("GOOGLE_API_KEY"));
            match key {
                Some(api_key) => {
                    // Make a real API call to validate the key
                    let start = Instant::now();
                    let client = reqwest::Client::builder()
                        .timeout(std::time::Duration::from_secs(10))
                        .build();
                    match client {
                        Ok(client) => {
                            let response = client
                                .get("https://generativelanguage.googleapis.com/v1/models")
                                .query(&[("key", &api_key)])
                                .send()
                                .await;
                            let latency = start.elapsed().as_millis();
                            match response {
                                Ok(resp) => {
                                    let status = resp.status();
                                    if status.is_success() {
                                        (true, format!("Gemini API key valid ({}ms)", latency))
                                    } else {
                                        (false, format!("Gemini API returned HTTP {} ({}ms)", status.as_u16(), latency))
                                    }
                                }
                                Err(e) => (false, format!("Gemini API connection failed: {}", e)),
                            }
                        }
                        Err(e) => (false, format!("Failed to create HTTP client: {}", e)),
                    }
                }
                None => (false, "GEMINI_API_KEY / GOOGLE_API_KEY not set".to_string()),
            }
        }
        "local" => (true, "Local provider requires no external credentials".to_string()),
        other => (false, format!("Unknown provider: {}", other)),
    };

    let css = if ok { "text-green-400" } else { "text-red-400" };
    Html(format!(r#"<span class="{}">{}</span>"#, css, msg)).into_response()
}

// ── Router builder ───────────────────────────────────────────────────────

// ── Settings POST Handlers ─────────────────────────────────────────────────

pub async fn save_plane_settings(axum::Form(form): axum::Form<PlaneSettingsForm>) -> Response {
    let mut config = match Config::load() {
        Ok(c) => c,
        Err(_) => Config {
            plane: None,
            agents: None,
            services: None,
            dashboard: None,
        },
    };

    config.plane = Some(PlaneConfig {
        api_url: form.api_url.trim().to_string(),
        api_key: form.api_key.trim().to_string(),
        workspace_slug: form.workspace_slug.trim().to_string(),
        project_slug: form.project_slug.trim().to_string(),
    });

    match config.save() {
        Ok(_) => render(ToastPartial {
            message: "Plane settings saved successfully".to_string(),
            success: true,
        }),
        Err(e) => render(ToastPartial {
            message: format!("Failed to save settings: {}", e),
            success: false,
        }),
    }
}

pub async fn save_agent_settings(axum::Form(form): axum::Form<AgentSettingsForm>) -> Response {
    let mut config = match Config::load() {
        Ok(c) => c,
        Err(_) => Config {
            plane: None,
            agents: None,
            services: None,
            dashboard: None,
        },
    };

    config.agents = Some(AgentConfig {
        pool_size: form.pool_size,
        retry_budget: form.retry_budget,
        dispatch_mode: form.dispatch_mode.trim().to_string(),
        default_provider: form.default_provider.trim().to_string(),
    });

    match config.save() {
        Ok(_) => render(ToastPartial {
            message: "Agent settings saved successfully".to_string(),
            success: true,
        }),
        Err(e) => render(ToastPartial {
            message: format!("Failed to save settings: {}", e),
            success: false,
        }),
    }
}

pub async fn save_dashboard_settings(
    axum::Form(form): axum::Form<DashboardSettingsForm>,
) -> Response {
    let mut config = match Config::load() {
        Ok(c) => c,
        Err(_) => Config {
            plane: None,
            agents: None,
            services: None,
            dashboard: None,
        },
    };

    config.dashboard = Some(DashboardConfig {
        theme: form.theme.trim().to_string(),
        log_level: form.log_level.trim().to_string(),
        data_directory: form.data_directory.trim().to_string(),
    });

    match config.save() {
        Ok(_) => render(ToastPartial {
            message: "Dashboard settings saved successfully".to_string(),
            success: true,
        }),
        Err(e) => render(ToastPartial {
            message: format!("Failed to save settings: {}", e),
            success: false,
        }),
    }
}

pub async fn save_services_settings(axum::Form(form): axum::Form<ServiceSettingsForm>) -> Response {
    let mut config = match Config::load() {
        Ok(c) => c,
        Err(_) => Config {
            plane: None,
            agents: None,
            services: None,
            dashboard: None,
        },
    };

    let mut services = Vec::new();
    for (name, url) in form.names.into_iter().zip(form.endpoint_urls.into_iter()) {
        if !name.trim().is_empty() {
            services.push(ServiceConfig {
                name: name.trim().to_string(),
                endpoint_url: url.trim().to_string(),
            });
        }
    }
    config.services = Some(services);

    match config.save() {
        Ok(_) => render(ToastPartial {
            message: "Service settings saved successfully".to_string(),
            success: true,
        }),
        Err(e) => render(ToastPartial {
            message: format!("Failed to save settings: {}", e),
            success: false,
        }),
    }
}

#[derive(Debug, Deserialize)]
pub struct SingleServiceTestForm {
    pub name: String,
    pub endpoint_url: String,
}

pub async fn test_service_connection(
    axum::Form(form): axum::Form<SingleServiceTestForm>,
) -> Response {
    let is_valid = !form.endpoint_url.trim().is_empty() && form.endpoint_url.starts_with("http");

    if is_valid {
        render(ToastPartial {
            message: format!("Connection to {} successful (mock)", form.name),
            success: true,
        })
    } else {
        render(ToastPartial {
            message: format!("Invalid endpoint for {}: {}", form.name, form.endpoint_url),
            success: false,
        })
    }
}

pub async fn test_plane_connection(
    axum::Form(form): axum::Form<PlaneSettingsForm>,
) -> Response {
    let api_url = form.api_url.trim();
    let api_key = form.api_key.trim();
    let workspace_slug = form.workspace_slug.trim();

    if api_url.is_empty() || api_key.is_empty() || workspace_slug.is_empty() {
        return render(ToastPartial {
            message: "Plane settings are incomplete — fill all required fields".to_string(),
            success: false,
        });
    }

    if !api_url.starts_with("http") {
        return render(ToastPartial {
            message: format!("Invalid API URL: must start with http:// or https://"),
            success: false,
        });
    }

    // Make a real HTTP call to the Plane API
    let (ok, message, latency) = plane_api_connectivity_check(api_url, api_key, workspace_slug).await;

    let latency_str = latency
        .map(|ms| format!(" ({}ms)", ms))
        .unwrap_or_default();

    render(ToastPartial {
        message: format!("{}{}", message, latency_str),
        success: ok,
    })
}

// ── Planify REST Shim Subprocess Management ────────────────────────────────

use std::process::Stdio;
use tokio::process::{Child, Command};

/// Shared state for the Planify REST shim subprocess.
pub struct PlanifyShimState {
    pub child: Option<Child>,
    pub port: u16,
    pub running: bool,
}

impl PlanifyShimState {
    pub fn new() -> Self {
        Self {
            child: None,
            port: 8000,
            running: false,
        }
    }
}

/// Start the Planify REST shim as a subprocess.
pub async fn start_planify_shim(
    State(_state): State<SharedState>,
) -> Response {
    let shim_dir = std::env::current_dir()
        .ok()
        .map(|cwd| cwd.join("tools").join("planify-shim"))
        .filter(|path| path.exists());

    let shim_dir = match shim_dir {
        Some(dir) => dir,
        None => {
            return render(ToastPartial {
                message: "Planify shim directory not found at tools/planify-shim/".to_string(),
                success: false,
            });
        }
    };

    // Check if bun is available for running the shim
    let bun_available = Command::new("bun")
        .arg("--version")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .is_ok();

    if !bun_available {
        return render(ToastPartial {
            message: "bun runtime not found — install bun to run the Planify shim".to_string(),
            success: false,
        });
    }

    // Start the shim subprocess
    match Command::new("bun")
        .arg("run")
        .arg("src/server.js")
        .current_dir(&shim_dir)
        .env("PORT", "8000")
        .env(
            "AGILEPLUS_API_URL",
            env_or_none("AGILEPLUS_API_URL").unwrap_or_else(|| "http://127.0.0.1:4000".to_string()),
        )
        .env(
            "AGILEPLUS_API_KEY",
            env_or_none("AGILEPLUS_API_KEY").unwrap_or_else(|| "dev-key".to_string()),
        )
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(_child) => {
            render(ToastPartial {
                message: "Planify REST shim started on :8000".to_string(),
                success: true,
            })
        }
        Err(e) => render(ToastPartial {
            message: format!("Failed to start Planify shim: {}", e),
            success: false,
        }),
    }
}

/// Stop the Planify REST shim subprocess.
pub async fn stop_planify_shim(
    State(_state): State<SharedState>,
) -> Response {
    render(ToastPartial {
        message: "Planify REST shim stop requested (use process manager for full control)".to_string(),
        success: true,
    })
}

/// Check the status of the Planify REST shim.
pub async fn planify_shim_status() -> Response {
    // Check if port 8000 is in use
    let port_in_use = tokio::net::TcpStream::connect("127.0.0.1:8000")
        .await
        .is_ok();

    let status = if port_in_use {
        "running"
    } else {
        "stopped"
    };

    render(ToastPartial {
        message: format!("Planify REST shim is {}", status),
        success: port_in_use,
    })
}

pub fn router(state: SharedState) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/home", get(home))
        .route("/dashboard", get(dashboard_page))
        .route("/features", get(features_page))
        .route("/events", get(events_page))
        .route("/settings", get(settings_page))
        .route("/settings/plane", get(plane_settings_page))
        .route("/settings/agents", get(agent_settings_page))
        .route("/settings/services", get(services_settings_page))
        .route("/api/settings/services", post(save_services_settings))
        .route("/api/settings/services/test", post(test_service_connection))
        .route("/hub", get(hub_page))
        .route("/api/settings/plane", post(save_plane_settings))
        .route("/api/settings/plane/test", post(test_plane_connection))
        .route("/api/settings/agents", post(save_agent_settings))
        .route("/api/settings/agents/test-connection", post(test_agent_connection))
        .route("/api/settings/dashboard", post(save_dashboard_settings))
        .route("/api/settings/services", post(save_services_settings))
        .route("/api/planify-shim/start", post(start_planify_shim))
        .route("/api/planify-shim/stop", post(stop_planify_shim))
        .route("/api/planify-shim/status", get(planify_shim_status))
        .route("/api/dashboard/services/{name}/restart", post(restart_service))
        .route("/api/dashboard/services/{name}/config", axum::routing::patch(patch_service_config))
        .route("/api/dashboard/services/{name}/toggle", post(toggle_service))
        .route("/api/dashboard/kanban", get(kanban_board))
        .route("/api/dashboard/features/{id}", get(feature_detail))
        .route("/api/dashboard/features/{id}/work-packages", get(wp_list))
        .route("/api/dashboard/health", get(health_panel))
        .route("/api/dashboard/events", get(event_timeline))
        .route("/api/dashboard/agents", get(agent_activity))
        .route("/api/dashboard/projects", get(project_switcher))
        .route(
            "/api/dashboard/projects/{id}/activate",
            post(switch_project),
        )
        .route("/api/time", get(time_footer))
        .route("/api/stream-placeholder", get(stream_placeholder))
        .route(
            "/api/evidence/{feature_id}/{artifact_id}/content",
            get(evidence_content),
        )
        .route(
            "/api/evidence/{feature_id}/{artifact_id}/preview",
            get(evidence_preview),
        )
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_state::{DashboardStore, default_health};
    use crate::templates::{AgentActivityPartial, AgentView, EventTimelinePartial};
    use std::sync::Arc;
    use tokio::sync::RwLock;

    fn make_state() -> SharedState {
        let store = DashboardStore {
            health: default_health(),
            ..Default::default()
        };
        Arc::new(RwLock::new(store))
    }

    #[tokio::test]
    async fn health_panel_renders() {
        let state = make_state();
        let store = state.read().await;
        let tpl = HealthPanelPartial {
            services: store.health.clone(),
        };
        let html = tpl.render().expect("template renders");
        assert!(html.contains("NATS"));
    }

    #[tokio::test]
    async fn services_settings_page_renders() {
        let state = make_state();
        let store = state.read().await;
        let tpl = ServicesSettingsPage {
            services: store.health.clone(),
            configs: vec![],
        };
        let html = tpl.render().expect("template renders");
        assert!(html.contains("Service Endpoints"));
    }

    #[tokio::test]
    async fn plane_settings_page_renders() {
        let state = make_state();
        let response = plane_settings_page(State(state)).await;
        let body = response.into_body();
        let bytes = axum::body::to_bytes(body, usize::MAX).await.unwrap();
        let html = String::from_utf8(bytes.to_vec()).unwrap();
        assert!(html.contains("Native Plane Views"));
        assert!(html.contains("Browse Synced Features"));
    }

    #[tokio::test]
    async fn kanban_partial_renders_empty() {
        let states = all_feature_states();
        let cards: HashMap<String, Vec<FeatureView>> =
            states.iter().map(|s| (s.clone(), vec![])).collect();
        let tpl = KanbanPartial { cards };
        let html = tpl.render().expect("template renders");
        assert!(html.contains("kanban-board"));
    }

    #[tokio::test]
    async fn wp_list_renders_empty() {
        let tpl = WpListPartial {
            feature_id: 1,
            workpackages: vec![],
        };
        let html = tpl.render().expect("template renders");
        assert!(html.contains("Title"));
    }

    #[tokio::test]
    async fn event_timeline_renders_empty() {
        let tpl = EventTimelinePartial {
            feature_id: 0,
            events: vec![],
        };
        let html = tpl.render().expect("template renders");
        assert!(html.contains("event-timeline"));
    }

    #[tokio::test]
    async fn agent_activity_renders_empty() {
        let tpl = AgentActivityPartial { agents: vec![] };
        let html = tpl.render().expect("template renders");
        assert!(html.contains("agent-activity"));
    }

    #[tokio::test]
    async fn agent_activity_renders_agents() {
        let tpl = AgentActivityPartial {
            agents: vec![AgentView {
                name: "test-agent".into(),
                status: "running".into(),
                current_task: "doing work".into(),
                last_action: "1m ago".into(),
                pid: Some(12345),
                started_at: None,
                worktree: String::new(),
                worktree_label: String::new(),
                is_live: true,
            }],
        };
        let html = tpl.render().expect("template renders");
        assert!(html.contains("test-agent"));
        assert!(html.contains("running"));
    }
}
