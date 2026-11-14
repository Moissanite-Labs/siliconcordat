# statos

- Status: idea
- Origin: [Moissanite-Labs/statos](https://github.com/Moissanite-Labs/statos) (to be archived). README added 2026-11-06. Open pull request [#1](https://github.com/Moissanite-Labs/statos/pull/1), "Establish StatOs: outcomes, weight applications, and verification research", by ahrazzle, opened 2026-11-06, not merged.
- Related: [mlx](mlx.md), whose PR #1 proposes an evidence boundary with this PR. PR #1 here names proposed relationships with [Synbios](https://github.com/Moissanite-Labs/synbios), Exo core and desktop, and Exoframe / Exofab Crystal Lattice. It cites research records in aska-digital/ingestion (issue #452, PR #464).

## The idea

The README on main says, in full:

> operating system based on statistics and parameters, where “applications” are just sets of weights that defined behaviour.

PR #1 writes the name as StatOs. Its `docs/vision.md` quotes a founding definition dated 2026-11-06:

> StatOs - an operating system based on statistically modelling your desired outcomes and achieving them through dynamic custom weight writing/tuning

and a founding verification requirement:

> I want every single aspect of the system, every single mechanism to be formally verified or verifiable from circuit pathways all the way up to interface.

The PR describes the idea this way. A user describes an outcome. The system builds candidate behaviours as parameter sets, by direct construction or by tuning. It measures how well each candidate meets the outcome, and keeps the evidence and the alternatives. An application package is the tuple (parameters, execution specification, interface, outcome contract, provenance, evidence).

The long-term scope in the PR covers model weights, runtime and system architecture, and reconfigurable hardware (FPGA). The proposed starting point is a research environment hosted on an existing OS.

Hypotheses the PR lists:

- Parameter packages can provide useful application boundaries on a shared execution substrate.
- Large models can construct or improve small parameterized applications at useful cost.
- Compatible representations can support richer inter-model communication.
- Application composition can preserve declared properties, with failures detected before activation.
- Joint optimization of model and execution substrate can outperform optimizing each separately.

The PR lists these as hypotheses to investigate. No experiment has been run.

## What exists

On main: `README.md` only (commit `b928091`).

In PR #1 (head `5a147c12547bb4ef306b44fbb330d687522b0393`, branch `research/weights-as-applications-foundation`): 9 files, about 400 added lines. Documentation and protocols only. The PR states: "There is no implemented runtime, trained application, or completed formal proof yet." No paid service was used.

| Path | Content |
| --- | --- |
| `README.md` | Status, file index, core proposition diagram |
| `docs/vision.md` | Research charter: founding definition, design image, controllable surfaces, verification requirement, hypotheses |
| `docs/architecture.md` | Outcome contracts, application packages, a seven-step loop (observe, deliberate, construct, evaluate, verify, activate, remember), composition, relationships to other projects |
| `docs/verification.md` | Kinds of evidence and verification obligations by layer, from outcome specification to physical substrate |
| `docs/roadmap.md` | Milestones M0 to M6, decisions dated 2026-11-06, open research questions |
| `experiments/README.md` | Protocols E001 and E002. Neither has been run |
| `examples/boolean-outcome.json` | Illustrative E001 input, marked `proposed_protocol_not_executable` |
| `research/inkling-2-small.md` | Assessment of Inkling 2 Small and Tinker as a candidate backend for E002. Recommendation: "prototype, bounded and staged". Nothing adopted or run |
| `CONTRIBUTING.md` | Rules for labelling evidence, recording sources, and paid runs |

E001, finite-domain weight applications (proposed):

- Question: can one fixed runtime run different behaviours only by swapping parameter packages, with every legal input checked?
- Task: all 16 Boolean functions of two inputs. A fixed network has four threshold hidden units, one per input pattern, and four output weights in `{0,1}`.
- Arms: table lookup in ordinary code, direct weight construction, and tuning of the output weights (five seeds, at most 1,000 steps per function per seed).
- Acceptance: one unchanged runtime loads every candidate; direct construction passes all 64 checks; swap AND to XOR and back; a mutated weight is rejected before activation; record size, time and cost. A result that favours ordinary code is valid.
- Budget: local CPU, no network or hosted model calls. The PR estimates 1 to 2 days of work.

E002, outcome-driven statistical adaptation (proposed): an agent adapts a small model on a licensed or synthetic forecasting task, judged by an independent evaluator within a fixed budget. Candidate provider: Tinker. Candidate orchestrator: Inkling 2 Small. A spend cap must be recorded before any paid run. The PR does not set or authorize one.

Roadmap from `docs/roadmap.md` (proposed):

| Milestone | Deliverable |
| --- | --- |
| M0 | Research foundation (this PR) |
| M1 | E001: several behaviours as parameters on one runtime, with replacement and rollback |
| M2 | Formal model of package activation and concurrent candidate integration |
| M3 | E002 with a small model and a pinned evaluation contract |
| M4 | Agent-directed adaptation with preserved alternatives |
| M5 | Routing and composition of two or more learned applications |
| M6 | Numerical format or kernel optimization, then a supported FPGA region |

## Open questions

Raised in PR #1 (`docs/roadmap.md`):

1. Which useful behaviours are cheaper or more reliable to construct directly than to train or implement conventionally?
2. What is the smallest application boundary that remains stable under tuning and composition?
3. Can a generated evaluator be checked independently enough to prevent optimizing an incorrect objective?
4. Which properties of learned behaviours are tractably verifiable, and at what numerical precision?
5. How do representations evolve without invalidating dependent applications?
6. How should expensive alternatives be retained: complete checkpoints, reproducible recipes, deltas, or some combination?
7. When does adaptation cost exceed the future benefit of the resulting application?

Also raised in the PR:

- No proof assistant has been selected. Lean is a candidate. Bend is recorded as an unverified lead.
- No spend budget is set for E002.
- The relationships to Synbios, Exo and Crystal Lattice describe intended ownership only.
- No distribution license has been selected.

Needed to promote:

- Who owns it? No owner is named.
- Should PR #1 be merged before the repository is archived, so the charter and protocols are on a main branch?

## To promote it to a repository

- Owner: a named person who will implement E001.
- Charter: `docs/vision.md` in PR #1 is written as a research charter. Merge it or carry it into the new repository.
- First study or runnable thing: E001 is a written protocol that needs only a local CPU and no spend. Running it and recording the results meets the rule.
