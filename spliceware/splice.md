#  spliceware



# SPLICER PROTOCOL SPECIFICATION (v0.1.0)
Standard for Surgical Capability Extraction and Cross-Codebase Grafting

## 1. Scope
Splicer defines a deterministic workflow and manifest schema for extracting isolated functional capabilities from an external codebase (`Donor`) and integrating them into an existing system (`Host`) with zero dependency bleed and strict boundary isolation.

## 2. Core Primitives
* **Donor (`source`)**: The external repository containing the target capability.
* **Host (`target`)**: The destination repository receiving the capability.
* **Slice**: The minimal reachable Abstract Syntax Tree (AST) subgraph required to execute the capability.
* **Severance**: The replacement of Donor globals, I/O, and third-party packages with Host-native primitives or standard library calls.
* **Seam**: The single adapter file translating Host types and control flow into the Slice.
* **Lock**: The contract test suite verifying the Seam using strictly Host-native inputs.

## 3. The Graft Manifest (`splice.json`)
Every splice operation MUST be governed by a declarative manifest stored inside the Host's isolated graft directory (e.g., `vendor/splices/<name>/splice.json`).

```json
{
  "splice_version": "0.1.0",
  "capability": "rate_limiter_token_bucket",
  "donor": {
    "uri": "https://github.com",
    "commit": "a1b2c3d4e5f6",
    "license": "MIT",
    "entry_symbols": ["TokenBucket.consume", "TokenBucket.refill"]
  },
  "severance_map": {
    "donor_logger": "host://src/core/logger",
    "redis_client": "host://src/infra/cache",
    "axios": "stdlib://fetch"
  },
  "seam": {
    "path": "src/adapters/rate_limiter.ts",
    "exports": ["checkRateLimit"]
  },
  "verification": {
    "command": "npm test -- src/adapters/rate_limiter.test.ts",
    "max_host_files_touched": 2
  }
}
```

## 4. State Machine (Execution Lifecycle)
An agent or CLI implementing Splicer MUST execute five sequential states. Failure at any state halts execution cleanly without mutating the Host working tree.

1. `PROBE`: Parse Donor AST from `entry_symbols`. Output the exact dependency graph of reachable functions, types, and external imports.
2. `SEVER`: Check external imports against Host's package manifest. Reject any unmapped third-party package unless explicitly permitted. Rewrite severed calls per `severance_map`.
3. `EXTRACT`: Write the stripped Slice to `vendor/splices/<capability>/core`. Preserve Donor license headers and commit provenance.
4. `WEAVE`: Generate the Seam file at `seam.path`. Enforce Host typing, error handling, and async conventions at this boundary.
5. `LOCK`: Execute `verification.command`. If exit code != 0, revert `WEAVE` and `EXTRACT` or iterate strictly inside the Seam.

## 5. Hard Invariants (Compliance Rules)
A tool or agent is Splicer-compliant ONLY if it enforces all four invariants:

* **Zero Lockfile Mutation**: A splice MUST NOT add dependencies to the Host's package manager (`package.json`, `Cargo.toml`, `pyproject.toml`) without explicit human override.
* **One-Way Visibility**: Host code may import ONLY from the Seam. Direct imports from `vendor/splices/<capability>/core` are forbidden by linter rule.
* **Immutable Core**: Once extracted and severed, Slice files are read-only. Behavioral adaptations happen exclusively inside the Seam.
* **Clean Ejection**: Deleting `vendor/splices/<capability>/` and the single Seam file MUST leave the Host in a valid, buildable state (aside from callers of that Seam).

## 6. Agent Tool Interface (MCP / Function Signatures)
Coding agents implementing Splicer expose four deterministic tools rather than raw shell access:

* `splice_probe(donor_uri, entry_symbols)` -> Returns reachable AST slice + foreign dependency list.
* `splice_extract(slice_id, severance_map)` -> Writes isolated core into Host sandbox.
* `splice_weave(slice_id, host_interface_spec)` -> Generates the single Seam adapter.
* `splice_verify(splice_manifest_path)` -> Runs contract tests + checks invariant compliance.
