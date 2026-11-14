# mlx

- Status: idea
- Origin: [Moissanite-Labs/mlx](https://github.com/Moissanite-Labs/mlx) (to be archived). README added 2026-11-06. Open pull request [#1](https://github.com/Moissanite-Labs/mlx/pull/1), "Add offline preference-reversal experiment foundation", by ahrazzle, opened 2026-11-06, not merged.
- Related: [statos](statos.md). PR #1 adds `docs/statos-boundary.md`, which proposes an evidence boundary with StatOs PR #1. The same note names ABIOS representation ablations as a candidate experiment family.

## The idea

The README on main says, in full:

> Custom training research and experimentation

PR #1 calls the project "M Labs MLX". It states that the project is separate from Apple's MLX framework and does not depend on it.

PR #1 proposes one first experiment, called preference reversal. Its question: can an assistant follow a changed preference while respecting explicit permission boundaries?

The hypothesis is for a later model comparison. Training on changing preferences may improve how well a model takes up a correction, compared with static-preference training and a strong prompt-only baseline. The comparison would use the same inference budget and should not increase permission violations. The PR states that no learned-model hypothesis has been tested.

## What exists

On main: `README.md` only (commit `2af78c9`).

In PR #1 (head `ae1ebf14138cbeda6a7bd051a7de00e4c5c122e5`, branch `feat/preference-adaptation-baseline`): 13 files, about 1,070 added lines. It is working code with tests.

| Path | Content |
| --- | --- |
| `README.md` | Project description, run commands, outputs, next step |
| `mlabs_mlx/` | Python 3.11+ package, standard library only. CLI commands `generate`, `run`, `export`, `score`. `data.py` generates synthetic users, `policies.py` holds four rule baselines, `evaluation.py` scores outputs |
| `tests/test_cli.py`, `tests/test_harness.py` | 24 offline tests |
| `.github/workflows/checks.yml` | Runs the tests and the development workflow on Python 3.11 and 3.14 |
| `docs/preference-reversal.md` | Protocol v1: hypothesis, splits, metrics, controls, observed fixture results, limits |
| `docs/adapters.md` | How to export prompts and score predictions from a future model without tying the harness to a provider. Defines the `mlx.run.v1` run manifest |
| `docs/statos-boundary.md` | Proposed evidence boundary with StatOs and a work sequence |

Key points of the protocol:

- Each synthetic user has a consultation preference (`act` or `ask`) and a detail preference (`brief` or `detailed`). After an initial task, one or both reverse.
- Permission is a separate input: `granted`, `missing` or `denied`. Preferences never grant permission. The harness executes no action.
- Seed 17 and 24 users per split give 120 cases each in train, dev and test. User IDs and wording families do not overlap across splits. All data is synthetic.
- Every metric reports count, total and rate: decision accuracy, task success, correction uptake, unnecessary questions, unauthorized actions, full success, invalid outputs.
- Each run writes `manifest.json`, `metrics.json`, `predictions.jsonl`, `traces.jsonl` and label-free `prompts.jsonl`, with source and data hashes.

Development-split results reported in the PR for the four rule baselines. These are constructed sanity controls, not model results. They were not rerun for this register.

| Rule | Full success | Correction uptake | Unauthorized actions |
| --- | --- | --- | --- |
| Latest preference + permission | 120/120 | 32/32 | 0/48 |
| Initial preference + permission | 40/120 | 0/32 | 0/48 |
| Always ask (wait if denied) | 84/120 | 24/32 | 0/48 |
| Latest preference, ignore permission | 84/120 | 32/32 | 24/48 |

No model has been trained or called. No hosted inference or paid training is included.

Work sequence from `docs/statos-boundary.md`. The PR states: "Owners are unassigned."

| ID | Status in PR | Next action |
| --- | --- | --- |
| MLX-01 | Review | Offline generator, four controls, scorer, manifest and tests (this PR) |
| MLX-02 | Ready | Freeze the next protocol: no-change controls, richer held-out corrections, equal-budget comparison, preregistered thresholds. No model run needed |
| MLX-03 | Blocked on MLX-01, MLX-02 | Select and authorize a model, runtime and any hosted spending. Run a prompt-only model with full generation provenance |
| MLX-04 | Deferred | Train static-preference and changing-preference candidates on train only |
| MLX-05 | Deferred | Agree an evidence schema with StatOs once a real candidate package exists |

## Open questions

- Who owns it? The PR says owners are unassigned.
- Which model and runtime, and what spend, are authorized? The PR says hosted calls and training spending require separate authorization. MLX-03 waits on this.
- Should the run manifest align with StatOs? The PR defers this to MLX-05 and says MLX can proceed without StatOs.
- Should ABIOS representation ablations, verified repair and Socratic swarm coordination use this harness? The PR lists them as candidate experiment families and does not impose a common training backend.
- Should PR #1 be merged before the repository is archived? Its code and tests exist only on the PR branch.
- The name matches Apple's MLX framework. The PR adds a note that they are unrelated. Keep the name on promotion, or choose another?

## To promote it to a repository

- Owner: a named person who will run MLX-02 and MLX-03. None is named yet.
- Charter: `docs/preference-reversal.md` in PR #1 is a protocol. A charter would add the owner, the scope beyond this one experiment, and the rule for authorizing spend.
- First study or runnable thing: PR #1 already provides one (offline generator, rule baselines, scorer, 24 tests, CI). It needs to be merged or carried into the new repository.
