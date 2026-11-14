# exo-tui

- Status: idea
- Origin: [Moissanite-Labs/exo-tui](https://github.com/Moissanite-Labs/exo-tui) (to be archived). README added 2026-10-30. Open pull request [#1](https://github.com/Moissanite-Labs/exo-tui/pull/1), "Start a mock-only OpenTUI frontend", by ahrazzle, opened 2026-10-30, not merged.
- Related: [Exo](https://github.com/Moissanite-Labs/exo). The README names Exo, and PR #1 models its message shapes on Exo protocol v1.

## The idea

The README on main says, in full:

> Experimental alternative TUI for Exo, based off OpenTUI by `@anomalyco`

PR #1 describes OpenTUI as "an independent project by Anomaly", MIT licensed.

PR #1 starts this as a mock-only TypeScript frontend. It has a streaming transcript, a composer, keyboard focus and a scripted approval queue. A replaceable adapter marks where a real Exo v1 transport would plug in. There is no engine, network connection, provider call or real approval action.

## What exists

On main: `README.md` only (commit `c5eb3a3`).

In PR #1 (head `8fc8b16b79965672d4f540c9558a5e8e55ec3fe7`, branch `codex/mock-opentui-bootstrap`): 14 files, about 540 added lines, from two commits on 2026-10-30. It is runnable mock code with tests.

| Path | Content |
| --- | --- |
| `README.md` | How to run it, keys, protocol boundary, what it is not, verification, roadmap |
| `src/protocol.ts` | Adapter interface (`command`, `subscribe(since)`, `close`) and Exo v1 command, event and response types |
| `src/mock.ts` | `MockAdapter`, the only adapter. Local fixtures only; opens no socket and starts no process |
| `src/controller.ts` | `hello` handshake for protocol 1, subscribe and replay, send, approval response |
| `src/state.ts` | Transcript and approval state, ordered by event sequence |
| `src/ui.ts` | OpenTUI renderer: transcript, approval card, help line, composer |
| `src/index.ts` | Entry point. Requires `--mock`; `--plain` prints a fixed fixture |
| `test/controller.test.ts`, `test/ui.test.ts`, `test/terminal_pty.py` | 9 tests, including a POSIX pseudo-terminal test |
| `package.json`, `bun.lock`, `tsconfig.json`, `.gitignore` | Bun 1.3.14 or newer, `@opentui/core` pinned to 0.7.0 |

PR #1 reports that on macOS arm64 with Bun 1.4.2, `bun test` passed 9 tests and `bun run typecheck` passed. This was not rerun for this register.

The PR README is explicit about limits. The mock is "not a compatible Exo client yet". Its session, messages, ordering and timing are invented. There is no real handshake, SSE or stdio transport. It omits session lifecycle, message completion, signing, reconciliation through `approvals.list`, reconnection across server instance changes, and many event types. "Never use this mock for real approvals."

Roadmap in the PR README:

1. Broaden terminal and accessibility testing.
2. Implement and test a v1 transport with replay and server-instance handling.
3. Reconcile pending approvals against the core.
4. Add session selection and richer transcript events.

It also says to preserve Exo's existing CLI and service entry points.

## Open questions

Raised in PR #1:

- Screen-reader support, other terminals and platforms, and stderr handling for future subprocesses and transports are unverified.
- The repository has no project license. The PR notes that OpenTUI's MIT license does not grant one.

Needed to promote:

- Who owns it? No owner is named.
- Name. Moissanite-Labs/exo already ships a terminal client packaged as `exo-tui` (`clients/tui/pyproject.toml`, built on Textual). Decision D-031 (2026-10-12) in Exo's `DECISIONS.md` covers its look. This idea and that client share a name.
- Is a second, alternative TUI still wanted alongside Exo's own client? The README says "alternative" but does not say why.

## To promote it to a repository

- Owner: a named person.
- Charter: state why an alternative TUI is wanted next to Exo's existing client, and choose a name that does not clash with it.
- First study or runnable thing: PR #1 is already a runnable mock with tests. The next step in its roadmap is a tested v1 transport against a real Exo server.
