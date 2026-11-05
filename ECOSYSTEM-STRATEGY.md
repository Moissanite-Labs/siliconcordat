# Ecosystem strategy: one local development workflow

Moissanite Labs builds tools for people and coding agents to develop software together. This proposal connects those tools through one reproducible local workflow for developers and maintainers. It recommends component responsibilities, review gates, and a shared community entry point.

## Home and status

[Compass](https://github.com/Moissanite-Labs/compass/blob/main/README.md) already describes the ecosystem, consent, independent components, and reviewable delivery. Its README calls these directions a working proposal, not adopted policy.

Keep this strategy beside its existing [architecture and roadmap](https://github.com/Moissanite-Labs/compass/blob/main/SILICONCORDAT.md), with implementation decisions owned by each component repository.

This document proposes priorities, not permissions, adopted governance, funding commitments, or a release announcement. Maturity statements are dated 2026-11-05 and reflect repository documentation and bounded offline review evidence. They do not establish publication, installation, or working cross-repository integration.

The [dated ecosystem inventory](https://github.com/Moissanite-Labs/compass/blob/6a3cf9ae62a0c50107580eb831699503f0f94ff7/ecosystem-map/2026-11-03-map.md) supports the repository roles below. Component links identify source repositories, not reviewed implementation revisions. Acceptance requires separate evidence pinned to exact commits.

## Product direction

Prove one path from a small development request to a tested artifact, independent review, and an explicit operator decision. ExoFab should be the operator front door. Synbios should provide the deeper build and collaboration workspace. Exo should enforce host-authorized runtime scope. Swarm coordinates work inside those limits.

Keep the parts independently reusable. Versioned interfaces should connect them without shared private databases, a mandatory hosted service, or a single application that owns everything. A user should be able to use Exo without ExoFab, review Synbios evidence without executing it, and use Abios outside this ecosystem.

## Responsibilities and exchanges

The exchanges below are proposed integration boundaries unless identified as existing. Each component's maintainers own its implementation and acceptance evidence.

| Component and source | Responsibility | What it receives and returns | Evidence boundary |
| --- | --- | --- | --- |
| [Exo / Core / Swarm](https://github.com/Moissanite-Labs/exo) | Core enforces host permissions and manages execution, memory and task state. Swarm routes work. | Receives authorized requests and separately verified evidence. Returns task events, artifacts and bounded execution results. | Core exists. Swarm recovery and integration acceptance remain unverified. |
| [exo-os](https://github.com/Moissanite-Labs/exo-os) | OS integration around pinned shared Exo, not a core fork. | Supplies OS requirements upstream. Consumes core protocol and returns lifecycle evidence. | Documentation and source guards do not establish image, native Stop or systemd acceptance. |
| [ExoFab](https://github.com/Moissanite-Labs/exofab) | Operator workspace for requests, progress, review and explicit decisions. | Sends authorized commands to Exo and build requests to Synbios. Displays their separate evidence and outcomes. | Design/proposal repository. Reviewed synthetic examples are not a running application. |
| [Synbios](https://github.com/Moissanite-Labs/synbios) | Local build collaboration, artifact records, tests and independent acceptance. | Receives scoped work and authorized artifacts. Returns pinned evidence for operator review. | Local reference forge exists. Offline evidence checks are not a live Exo adapter or execution grant. |
| [Ethos](https://github.com/Moissanite-Labs/ethos) | Portable preferences with user inspection and correction. | Receives consented corrections. Supplies preferences with origin, revision and scope for separately authorized context use. | Specification and synthetic boundary model. No verified runtime import/export or two-adapter demonstration. |
| [Abios](https://github.com/Moissanite-Labs/abios) | Checkable agent message formats and validators. | Receives compatibility failures and fixtures. Returns versioned protocol rules and validation results. | Validator implementation exists. Structural validation does not establish authenticated permission or live interoperability. |
| Compass | Cross-project principles, priorities and decisions. | Receives component constraints and review results. Returns proposals and explicit decision records. | Working manifesto. Neither a runtime nor an authorization service. |
| [synmerit / Koinos](https://github.com/Moissanite-Labs/synmerit) | Experimental signed collaboration records and agreement history. | Receives explicitly shared contribution evidence. Returns verifiable records for separate local acceptance. | Reference CLI/workbench exists. Demo does not connect to ExoFab or Synbios. Signatures do not establish identity, truth or legal rights. |
| [exyber](https://github.com/Moissanite-Labs/exyber) | Intended replaceable transport. | Proposed adapter carries original signed messages and delivery status. Core returns acceptance or refusal under its own policy. | README describes upstream iroh. Exo-specific adapter maturity is unverified. Transport identity is not principal identity. |
| [tokenpage](https://github.com/Moissanite-Labs/tokenpage), [scry](https://github.com/Moissanite-Labs/scry), [nullmodem](https://github.com/Moissanite-Labs/nullmodem) | Tokenizer measurement, model/translator research, and communication below text for controlled local models, respectively. | Receive synthetic or explicitly consented fixtures. Return reproducible measurements and failure reports for separate promotion review. | Research directions, not production dependencies or proven model-quality improvements. |

[Exorien](https://github.com/Moissanite-Labs/exorien/blob/HEAD/README.md) has no actionable interface established by the inspected description. Assign no integration role without a concrete contract and evidence.

## Authority and evidence rules

Coordination, learned preferences, signatures and protocol validity never grant permission. External acceptance cannot complete a local task or answer a human approval. Exo must enforce the actual principal, action, scope, destination, version and expiry. Keep external verification, local acceptance and execution authorization as separate facts.

An acknowledgement means a request was received, not that work finished. Stop requests, observer disconnects and service shutdowns have different meanings. Show terminal results only with matching execution evidence. Missing, stale, partial or conflicting evidence must remain visible rather than becoming success.

The Synbios handoff needs trusted task and artifact pins supplied independently of the receipt. A matching digest proves byte equality, not who produced the artifact or whether tests really ran. Ethos preferences remain untrusted context, not instructions. Sharing a preference needs a separate publication decision from using it privately.

## First reproducible workflow

The integration owner should publish a small local example with pinned revisions, setup commands, synthetic inputs and expected outputs. This is an acceptance target, not a claim that a runnable combined workflow exists today.

1. An operator selects a small repository change through ExoFab. Until an application runtime is selected, record that prerequisite rather than treating a mock interface as product completion.
2. Exo verifies the request against existing host-authorized scope. Orchestration cannot grant permission. Swarm assigns bounded build and review tasks. Synbios records artifacts, byte digests, tests and an independent reviewer.
3. ExoFab shows the artifact and evidence returned by Synbios. The operator can inspect the exact revision, reject it or separately authorize the next action. Review readiness cannot trigger execution by itself.
4. Exo performs the authorized local action and returns correlated progress and terminal events. The workflow records refusal, Stop, failure and unknown outcomes as well as success.
5. A fresh local checkout reproduces the result using the documented commands. Tests include altered receipts, revoked permission, duplicate starts, transport loss and Stop during work. No provider credential or private memory is required for the offline acceptance run.

Keep Ethos, Koinos and experimental transports outside this first critical path. Add each through a small optional adapter with its own negative tests. Research can return useful findings without entering runtime execution.

## What is ready, and what comes next

As of 2026-11-05, the available evidence covers separate offline checks for message validation, preference boundaries, interface examples, artifact receipts and OS source guards. These do not establish connected applications, real identity verification, native OS behavior or deployed safety.

Independent review of the latest executor and task-ledger revisions remains pending in this assessment. Acceptance must cover competing processes, failed cleanup and restart behavior at exact revisions. A task ledger alone does not establish retry safety, crash recovery or exactly-once external effects.

Proposed delivery order:

- **Runtime owners:** pin executor and ledger revisions, obtain independent reviews, and resolve any remaining findings before enabling dependent behavior.
- **Integration owners:** select the ExoFab runtime and demonstrate the single local workflow against pinned Exo and Synbios revisions. Establish trusted identity and pin transport before any live handoff.
- **OS owners:** obtain acceptance for the exact OS/core/image composition separately. Source inspection cannot substitute for native execution evidence.
- **Repository maintainers:** preserve each reviewed scope and record dependencies. Publication requires approved ancestry, current byte-level review and separate authorization.

## Community and sustainability

Use one community entry point with routes for getting help, contributing and reporting defects. Keep GitHub as the durable record for proposals, decisions, examples and acceptance evidence. An optional Discord can host help sessions and build clinics, with decisions summarized back to GitHub. Avoid separate communities for every component before there is demand.

Recruit coding-agent developers, maintainers with real review workloads, and local-model users. Ask participants to reproduce the local workflow and report a concrete failure or improvement. Contribution routes should include small code changes, test fixtures, documentation and independent review.

Seek paid design partners first: teams willing to fund a bounded development workflow and test its usefulness. Consider optional compute credits, sponsors and grants next, then investment aligned with independent components and user control. No funding offers, programme eligibility or investor commitments have been verified.

Keep execution and private memory local by default. Hosted collaboration should be optional and portable, with explicit sharing controls and export paths. No mandatory provider should be required. Recommend keeping basic permission checks, Stop, privacy controls and review evidence available without payment.

## Knowledge stewardship

Keep one canonical repository for each artifact. Compass owns cross-project strategy and decision indexes. Component repositories own implementation details and acceptance evidence. Link to canonical records rather than copying them across repositories.

Preserve decisions, research, negative findings, review evidence and the backlog. Each record should identify its owner, date, status and exact source revision. Evidence should name the tested commit, artifact digest where relevant, commands, results and limits. Later changes need renewed review rather than inheriting an earlier verdict.

Keep private evidence local unless separately authorized for publication. Public indexes should contain only approved summaries and safe links. Each task records its outputs, canonical destination and open gaps, including unfinished work and the next review needed.

## Working method

Use smaller bounded assignments with one owner, allowed files, a runnable acceptance target and a short dependency receipt. Coordinate locally and batch GitHub reads and publication requests through one authorized lane. Batch coordination, not unrelated changes into a giant pull request.

Review exact bytes and record hashes, commands, results and limits. Changed bytes need renewed review. Preserve partial work and resume it rather than repeating discovery. Once a contract is clear, assign a named consumer and runtime test instead of producing another contract variant. No efficiency recommendation relaxes approval, privacy or execution controls.
