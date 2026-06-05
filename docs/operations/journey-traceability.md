# Journey Traceability

Implements the phenotype-infra journey-traceability standard for Planify as a self-hosted project planning surface.

## Traceability Model

Every user-facing or operator-facing flow should be traceable across:

1. **FR/NFR** — requirement ID and user story.
2. **Spec** — acceptance criteria, planning invariant, and non-regression constraint.
3. **Docs** — operator/user documentation and rich media placeholders.
4. **Code** — web, admin, space, API, worker, or package surface implementing the flow.
5. **Tests/Gates** — unit, integration, BDD, lint, typecheck, and journey verification acting as autograders.
6. **Evidence** — journey manifest, recording/keyframes, and evaluation verdict.

## User-Facing and Operator-Facing Flows

| Flow | Requirement | Implementation surface | Autograder gates | Evidence status |
| --- | --- | --- | --- | --- |
| User creates an issue and adds it to a planning board | FR-PLANIFY-ISSUE-001, NFR-PLANIFY-USABILITY-001 | `apps/web`, issue models, board views | UI component tests, API fixture tests, BDD journey, eval verdict | Stubbed |
| Team plans work into cycles/sprints | FR-PLANIFY-CYCLE-001, NFR-PLANIFY-TRACEABILITY-001 | `apps/web`, cycle views, planning services | cycle fixture tests, board smoke, journey manifest | Stubbed |
| Product owner reviews roadmap progress | FR-PLANIFY-ROADMAP-001, NFR-PLANIFY-VISIBILITY-001 | roadmap/project views, analytics widgets | route smoke tests, snapshot/a11y checks, journey eval | Stubbed |
| Admin configures self-hosted workspace safely | FR-PLANIFY-ADMIN-001, NFR-PLANIFY-OPERABILITY-001 | `apps/admin`, proxy/deployment config, environment docs | config validation, admin smoke, secret-scan gates | Stubbed |
| Space user tracks personal/team work state | FR-PLANIFY-SPACE-001, NFR-PLANIFY-RESPONSIVENESS-001 | `apps/space`, personal workspace UI | app smoke, state fixture tests, screenshot journey | Stubbed |

## Rich Media Stubs

<!-- RICH-MEDIA-STUB type="animated-gif" subject="Issue creation and board placement" journey="issue-create-board-place" status="TODO" -->
![Planify issue creation — issue form, saved issue, board placement, and traceability ID](../assets/rich-media/planify/issue-create-board-place.gif)

*Expected capture: create a deterministic issue, place it on a board, show the linked planning state, and verify the issue appears in the expected column.*

<!-- RICH-MEDIA-STUB type="annotated-screenshot" subject="Cycle planning board" journey="cycle-planning-board" status="TODO" -->
![Planify cycle planning — selected cycle, included issues, capacity state, and progress rollup](../assets/rich-media/planify/cycle-planning-board.png)

*Expected capture: open a seeded cycle, annotate included work, capacity/progress state, and the tests that prove the planning invariant.*

<!-- RICH-MEDIA-STUB type="annotated-screenshot" subject="Roadmap progress review" journey="roadmap-progress-review" status="TODO" -->
![Planify roadmap — initiatives, progress, risk indicators, and next action](../assets/rich-media/planify/roadmap-progress-review.png)

*Expected capture: review a seeded roadmap, show progress/risk signals, and verify the displayed state matches fixture data.*

<!-- RICH-MEDIA-STUB type="journey-eval" subject="Self-hosted admin configuration verdict" journey="self-hosted-admin-config" status="TODO" -->
![Planify admin config verdict — environment checks, workspace settings, and eval result](../assets/rich-media/planify/self-hosted-admin-config.png)

*Expected capture: configure a self-hosted workspace with fixture settings, validate environment/config checks, and attach a pass/fail eval verdict.*

<!-- RICH-MEDIA-STUB type="animated-gif" subject="Space work tracking flow" journey="space-work-tracking" status="TODO" -->
![Planify Space work tracking — personal work view, status update, and team visibility](../assets/rich-media/planify/space-work-tracking.gif)

*Expected capture: update a personal/team work item in Space and show the state reflected in the planning surface without stale UI data.*

## Journey Manifests

Journey manifests should live in `docs/journeys/manifests/` and include:

- FR/NFR IDs covered by the journey;
- app route, API fixture, or admin action used to reproduce the flow;
- seeded workspace/project data required for deterministic replay;
- expected screenshots/GIFs/keyframes;
- tests and gates that must pass before the journey is accepted;
- eval verdict schema and pass/fail criteria.

## Autograder Gates

Minimum gates before marking a journey complete:

- package lint/typecheck for affected apps and packages;
- route/component smoke tests for user-visible planning flows;
- deterministic fixture tests for issues, cycles, roadmaps, and workspace config;
- BDD journey replay for user stories;
- accessibility checks for browser-facing flows;
- secret-scan and deployment/config validation for self-hosted admin flows;
- doc link validation for every referenced rich media asset;
- journey manifest validation via `phenotype-journey verify` when available;
- eval verdict linked to the FR/NFR IDs in the manifest.

## Status

- [x] Identify initial planning and operator-facing flows
- [x] Stub rich media embeds for expected screenshots/GIFs/evals
- [ ] Author manifests in `docs/journeys/manifests/`
- [ ] Record journey captures for each flow
- [ ] Run `phenotype-journey verify` in CI
