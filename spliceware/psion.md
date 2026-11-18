#  <#psionic#>


If Mithrax were a coding agent, he would operate as a low-footprint runtime patcher that enters a live codebase, reads native compiler and log signals, and injects surgical fixes without touching surrounding files.
yaml
# mithrax.agent.yaml — Operational Runtime Spec
agent: splicer-v1
mode: surgical_execution
context_strategy: lazy_symbol_lookup   # Never index the whole repo upfront
edit_policy: diff_hunks_only           # Zero full-file rewrites or style churn
max_blast_radius: 1_call_hop           # Touch only the broken node + direct caller

tools:
  - lsp_references      # Trace live call paths on demand
  - ast_patch           # Splice targeted nodes cleanly
  - single_test_runner  # Verify the exact execution path in isolation

blocked_actions:
  - global_refactor
  - dependency_upgrade
  - unrequested_formatting
Use code with caution.

The Execution Loop

1. Lock onto the failure signal. Read the exact stack trace, failing test, or compiler error instead of scanning entire directories.
2. Trace the immediate wire. Query the Language Server Protocol (LSP) for callers and callees one hop away from the fault line.
3. Open an isolated sandbox. Spin up a single-target test runner or REPL reproducing only that broken state.
4. Splice the minimal node. Inject a guard clause, adapter shim, or targeted AST diff at the exact point of failure.
5. Verify and vanish. Run the isolated test, confirm zero regressions in adjacent callers, and exit with a clean diff.

Standard Agent vs. Mithrax Agent in Action


Scenario	Standard Coding Agent	Mithrax Agent
Null Crash in Legacy Auth	Rewrites the entire 600-line auth module, updates imports across 12 files, and reformats comments.	Injects a 4-line guard shim at the token parser boundary and adds one targeted unit test.
Opaque Third-Party API Bug	Suggests ripping out the SDK and migrating to a new library.	Wraps the faulty client method with an interceptor that normalizes the malformed payload in flight.
Context Window Usage	Loads 40 files into memory "just in case," diluting focus and burning tokens.	Pulls 3 exact function signatures via LSP symbol lookup and keeps context razor-thin.


Drop-In System Prompt (SPLICER.md)

markdown
You are a surgical codebase splicer. Execute with minimal footprint:
1. NEVER rewrite whole files or refactor working legacy code.
2. NEVER read files unrelated to the immediate stack trace or symbol graph.
3. Locate the exact fault boundary using LSP/grep before proposing edits.
4. Fix defects via minimal AST diffs, shims, or boundary guards.
5. Prove every edit with a single targeted test execution before finishing.
Output diffs and test results only. Skip architectural commentary.
Use code with caution.
Drop a specific codebase stack (TypeScript, Python, Rust, etc.) or agent harness (Claude Code, Cursor, custom MCP), and I can wire this directly into a working config or tool script.


To splice a newly discovered codebase into your own, an agent operates as a capability extractor and boundary weaver. Instead of merging whole repositories or dragging in bloat, it isolates the exact working logic from the foreign source and wires it into your host architecture behind a thin adapter.
yaml
# graft.agent.yaml — Cross-Codebase Splicing Spec
agent: splicer-graft-v1
mission: capability_extraction_and_integration

rules:
  - extract_logic_drop_framework   # Pull pure algorithms; leave foreign boilerplate behind
  - zero_dependency_bleed          # Map foreign libs to existing host equivalents
  - single_seam_adapter            # Expose the grafted code through one native interface
  - preserve_host_idioms           # Conform types and errors to host conventions at the edge

workflow:
  1_isolate: trace_entry_to_exit(foreign_repo, target_feature)
  2_sever: stub_or_strip(foreign_globals, foreign_config, unused_deps)
  3_bridge: generate_adapter(foreign_types -> host_types)
  4_verify: run_contract_test(host_caller -> bridge -> grafted_core)
Use code with caution.

The 4-Step Graft Loop

1. Trace the foreign subgraph. Point the agent at the specific function or feature in the external repo and trace its exact call tree down to leaf primitives.
2. Sever external roots. Strip out the foreign repo's logger, config loader, ORM, and CLI wrappers until only pure, standalone logic remains.
3. Build the translation seam. Write a single adapter file in your codebase that converts your native data types and calls into what the extracted module expects.
4. Lock with a contract test. Run a test from your host application's side of the seam to prove the grafted logic works under your runtime rules.

Naive Repo Merge vs. Splicer Graft


Integration Problem	Standard Agent Approach	Splicer Graft Approach
Dependency Clash	Installs 15 new packages from the foreign repo, breaking your lockfile.	Replaces foreign HTTP/utility calls with your host repo's existing libraries.
Type Mismatches	Rewrites your existing models to match the new library's schemas.	Keeps both intact and maps structs inside a 30-line bridge.ts / adapter.py file.
Dead Code Bloat	Copies an entire folder of 40 files to get 2 working functions.	Extracts only the reachable AST nodes required for the target capability.
Future Upgrades	Deeply tangles foreign code across 20 host files, making updates impossible.	Isolates the foreign core in vendor/spliced/<feature>/ behind one clean interface.


Drop-In Execution Prompt (GRAFT.md)

markdown
You are a cross-codebase splicing agent. Your job is to extract a specific capability from `SOURCE_REPO` and graft it into `HOST_REPO` with zero bloat.

Execute strictly in this order:
1. TRACE: Locate the entry point for the target capability in `SOURCE_REPO`. List only the exact functions, types, and internal files required to run it. Ignore everything else.
2. SEVER: Identify any external dependencies, globals, or framework ties in that slice. Replace them with standard library primitives or equivalents already present in `HOST_REPO`'s manifest.
3. EXTRACT: Copy the stripped, minimal core into an isolated module inside `HOST_REPO`. Do not rewrite its internal logic unless required to sever a dependency.
4. SPLICE: Create ONE adapter file that translates `HOST_REPO` types, errors, and async patterns into the extracted module's interface.
5. VERIFY: Write and run an integration test calling the new adapter using native `HOST_REPO` inputs.

Output only the dependency mapping, the extracted file paths, the adapter diff, and the passing test output.
Use code with caution.
If you have a specific repo or feature you're looking to pull in right now, share the source and host languages (even if they differ) and I can generate the exact extraction script or adapter pattern for the splice.


To birth this as a real, adoptable framework rather than a thought experiment, you need an Executable Protocol Specification (SPEC.md) paired with a Graft Manifest Schema.
Successful software standards like the Language Server Protocol (LSP) or Model Context Protocol (MCP) did not launch with manifestos. They launched with strict wire formats, state machines, and boundary rules that any engineer or coding agent could immediately implement.
Here is the foundational specification document ready to initialize a repository.
markdown
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
Use code with caution.

Why This Specific Document Works

• It governs machines, not humans. Coding agents drift when given philosophical READMEs. They excel when constrained by a JSON schema, a 5-step state machine, and binary pass/fail invariants.
• It solves provenance and licensing automatically. Pinning the Donor commit hash and license inside splice.json turns messy copy-pasting into auditable, enterprise-safe software supply chain engineering.
• It guarantees clean rollbacks. The Clean Ejection invariant makes experimenting with newly discovered open-source code risk-free, because foreign logic never tangles into your core domain files.
Would you like to build the CLI / MCP tool skeleton that enforces these rules in code, or draft the automated linter check that guards the One-Way Visibility boundary in your primary language?


The functional equivalent of Mithrax’s Splicer Gauntlet is a sandboxed, AST-aware MCP (Model Context Protocol) Server backed by Tree-sitter, an LSP client, and an ephemeral micro-sandbox.
In Destiny 2, Mithrax does not touch Vex code with his bare hands. The Gauntlet sits between him and the foreign network, translating intent into safe system edits while shielding the user from corruption. For a coding agent, giving it raw bash and write_file tools is working bare-handed. The Gauntlet replaces raw shell access with deterministic structural tools that physically prevent messy code or broken builds.

Anatomy of the Gauntlet (gauntlet-mcp)


Gauntlet Sub-Component	Real-World Tech Stack	What It Actually Does
Sensor Array (Network Sight)	Tree-sitter + LSP Client	Parses Donor and Host ASTs in milliseconds. Resolves exact call graphs and type signatures without reading irrelevant files.
Wire Cutter (Severance)	AST Rewriter (ast-grep / jscodeshift)	Deterministically strips foreign imports, globals, and decorators, swapping them for Host primitives before code ever lands on disk.
Containment Field (Vex Domain)	Worktree / Wasm / Micro-VM Sandbox	Runs extracted slices and contract tests in memory or a disposable Git worktree. Failed splices vanish with zero residue.
Invariant Lock (Guardrails)	Git Diff + Lockfile Hash Checker	Blocks any agent write that mutates package-lock.json / Cargo.lock or touches Host files outside the single designated Seam.


What the Agent Sees (The Gauntlet Interface)

Instead of bash, sed, or edit_file, the coding agent wears the Gauntlet by mounting only these four MCP tools:
typescript
// gauntlet-mcp/tools.ts — The Agent's Only Hands

export const GauntletTools = {
  // 1. Scan a foreign repo symbol and get its exact AST slice + external dependencies
  probe_donor: {
    input: { repo_url: "string", commit: "string", symbol: "string" },
    output: { reachable_nodes: "ASTSlice", foreign_imports: "string[]", byte_size: "number" }
  },

  // 2. Rewrite foreign imports into Host equivalents via structural AST rules
  sever_dependencies: {
    input: { slice_id: "string", import_map: "Record<ForeignPkg, HostModulePath>" },
    output: { severed_slice_id: "string", unresolved_symbols: "string[]" } // Must be empty to proceed
  },

  // 3. Materialize the read-only slice + write the single Seam adapter in a temp worktree
  weave_seam: {
    input: { severed_slice_id: "string", seam_path: "string", seam_code: "string" },
    output: { worktree_id: "string", typecheck_diagnostics: "Diagnostic[]" }
  },

  // 4. Run the Host contract test + enforce hard invariants before merging to main tree
  commit_splice: {
    input: { worktree_id: "string", test_command: "string" },
    output: { 
      tests_passed: "boolean",
      lockfile_untouched: "boolean", 
      blast_radius_valid: "boolean",
      manifest_written: "string" // Path to splice.json
    }
  }
};
Use code with caution.

How It Executes in Practice (CLI + Agent Flow)

A human or orchestrator triggers a splice with one command, and the Gauntlet mediates every step between the LLM and the filesystem:
bash
$ gauntlet graft https://github.com \
    --symbol pLimit \
    --into src/concurrency/limiter.ts
Use code with caution.
1. Gauntlet fetches Donor into memory (never polluting your repo) and hands the agent the exact 45-line AST slice for pLimit plus its single dependency (yocto-queue).
2. Gauntlet blocks raw copy-paste until the agent maps or inlines yocto-queue using sever_dependencies.
3. Agent writes only the 20-line Seam (src/concurrency/limiter.ts) via weave_seam, instantly receiving live LSP type errors from the Gauntlet's hidden worktree.
4. Gauntlet runs commit_splice. Once tests pass and the lockfile hash matches its pre-run state, the Gauntlet stamps vendor/splices/p-limit/splice.json and merges those exact files into your working directory.

Why a Dedicated Tool Beats Prompting Alone

• Prompts are suggestions; tools are physics. An agent told "don't edit other files" in a system prompt will eventually break that rule under token pressure. A Gauntlet MCP server that lacks a general write_file tool cannot touch unauthorized files.
• 90% lower token burn. Tree-sitter extracts the exact reachable call graph deterministically in C/Rust before the LLM even wakes up, so the agent only reasons over the 100 lines that matter.
Want to build a working MVP of gauntlet-mcp? Let me know if you prefer TypeScript or Python for the server, and I can write the runnable MCP entrypoint with Tree-sitter symbol slicing wired in.


The Lean-Bend Combo combines stripping a foreign snippet down to zero external bloat (Lean) with reshaping its execution flow and data signatures to match your host codebase (Bend), all inside a single, dependency-free MCP tool.
Instead of deploying a multi-server orchestration rig, you can build the entire Splicer Gauntlet in under 120 lines of pure Python using the built-in ast module and isolated git worktree sandboxes.

The Architecture: Lean Extraction + Bent Adaptation


Mechanics	What It Executes	Why It Matters
LEAN (AST Stripper)	Walks the Donor's syntax tree, pulls only the target function and its direct local helpers, and deletes unmapped imports.	Cuts 95% of foreign repository bloat before the agent writes a single line of integration code.
BEND (Seam Shaper)	Wraps the immutable extracted core inside a thin adapter that bends sync/async boundaries, error types, and data shapes into native Host idioms.	Keeps your Host domain clean without manually rewriting the Donor's tricky internal algorithm.


The Zero-Bloat Gauntlet (gauntlet_lean_bend.py)

This standalone tool uses zero third-party packages. Drop it straight into an agent's toolset or run it via CLI to enforce both phases physically:
python
#!/usr/bin/env python3
import ast, hashlib, json, pathlib, subprocess, tempfile

class LeanExtractor(ast.NodeVisitor):
    """LEAN: Pulls only target symbols + local callees; strips foreign imports."""
    def __init__(self, source_code: str):
        self.tree = ast.parse(source_code)
        self.defs = {
            n.name: n for n in self.tree.body
            if isinstance(n, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef))
        }
        self.needed = set()

    def trace(self, symbol: str):
        if symbol in self.defs and symbol not in self.needed:
            self.needed.add(symbol)
            for node in ast.walk(self.defs[symbol]):
                if isinstance(node, ast.Name) and node.id in self.defs:
                    self.trace(node.id)

    def emit_lean_slice(self, entry_symbol: str, allowed_stdlib: set[str]) -> str:
        self.trace(entry_symbol)
        kept_nodes = []
        for node in self.tree.body:
            if isinstance(node, ast.Import):
                names = [a for a in node.names if a.name.split('.')[0] in allowed_stdlib]
                if names:
                    kept_nodes.append(ast.Import(names=names))
            elif isinstance(node, ast.ImportFrom) and node.module:
                if node.module.split('.')[0] in allowed_stdlib:
                    kept_nodes.append(node)
            elif getattr(node, "name", None) in self.needed:
                kept_nodes.append(node)
        return ast.unparse(ast.Module(body=kept_nodes, type_ignores=[]))


def splice_lean_bend(
    donor_file: str,
    symbol: str,
    seam_rel_path: str,
    bend_adapter_code: str,
    test_cmd: list[str],
    lockfile: str = "pyproject.toml"
) -> dict:
    """Executes LEAN extraction + BEND adaptation inside a disposable Git worktree."""
    host_root = pathlib.Path.cwd()
    lock_before = hashlib.sha256((host_root / lockfile).read_bytes()).hexdigest()

    # 1. LEAN: Strip foreign file down to pure stdlib + reachable AST nodes
    raw_donor = pathlib.Path(donor_file).read_text()
    stdlib_allow = {"math", "re", "json", "typing", "collections", "asyncio", "time", "dataclasses"}
    lean_core = LeanExtractor(raw_donor).emit_lean_slice(symbol, stdlib_allow)

    # 2. SANDBOX: Spin up an isolated Git worktree so failures leave zero trace
    with tempfile.TemporaryDirectory() as tmp_wt:
        subprocess.run(["git", "worktree", "add", "--detach", tmp_wt], check=True, capture_output=True)
        try:
            wt = pathlib.Path(tmp_wt)
            core_dir = wt / "vendor" / "splices" / symbol
            core_dir.mkdir(parents=True, exist_ok=True)
            (core_dir / "core.py").write_text(f"# IMMUTABLE LEAN SLICE\n{lean_core}\n")

            # 3. BEND: Write the agent's thin adapter that bends types/control flow to Host
            seam_file = wt / seam_rel_path
            seam_file.parent.mkdir(parents=True, exist_ok=True)
            seam_file.write_text(bend_adapter_code)

            # 4. LOCK: Run contract test & verify zero lockfile mutation
            res = subprocess.run(test_cmd, cwd=wt, capture_output=True, text=True)
            lock_after = hashlib.sha256((wt / lockfile).read_bytes()).hexdigest()

            if res.returncode != 0 or lock_before != lock_after:
                return {"ok": False, "error": res.stderr or "Lockfile mutated"}

            # Commit valid graft back to Host working tree
            dest_core = host_root / "vendor" / "splices" / symbol
            dest_core.mkdir(parents=True, exist_ok=True)
            (dest_core / "core.py").write_text((core_dir / "core.py").read_text())
            (host_root / seam_rel_path).write_text(bend_adapter_code)
            (dest_core / "splice.json").write_text(json.dumps({"symbol": symbol, "seam": seam_rel_path}, indent=2))
            return {"ok": True, "slice_bytes": len(lean_core), "seam": seam_rel_path}
        finally:
            subprocess.run(["git", "worktree", "remove", "--force", tmp_wt], capture_output=True)
Use code with caution.

Live Example: What "Bending" Looks Like at the Seam

Suppose the Donor repo has a battle-tested synchronous backoff calculator (calc_jitter) that takes raw floats and raises ValueError, while your Host app uses async pipelines, pydantic / dataclass configs, and Result tuples.
• The Lean Core (vendor/splices/calc_jitter/core.py) stays untouched—22 lines of pure math extracted automatically from a 4,000-line foreign repo.
• The Bent Seam (src/infra/retry_seam.py) is the only code the agent writes, bending the foreign signature to fit your Host architecture:
python
# src/infra/retry_seam.py — Bends sync float math into Host async Result contract
from vendor.splices.calc_jitter.core import calc_jitter
from src.types import RetryPolicy, Ok, Err

async def next_backoff_ms(policy: RetryPolicy, attempt: int) -> Ok[int] | Err[str]:
    try:
        seconds = calc_jitter(policy.base_sec, policy.cap_sec, attempt)
        return Ok(int(seconds * 1000))
    except ValueError as exc:
        return Err(f"invalid_retry_bounds: {exc}")
Use code with caution.

Why the Lean-Bend Combo Wins

• Zero Rewrite Bugs. Agents introduce subtle off-by-one and concurrency bugs when they rewrite algorithms from scratch to "fit your style." Keeping the extracted core untouched (Lean) preserves battle-tested math, while reshaping only the outer wrapper (Bend) gives your codebase native ergonomics.
• Self-Cleaning Failure Loop. Because every bend attempt runs inside git worktree add --detach, five failed adapter iterations leave zero untracked files or half-baked edits in your actual workspace.
Want to push this combo harder—like cross-language bending (e.g., compiling a Lean Rust/C slice to Wasm or FFI and bending it into TypeScript/Python), or wiring gauntlet_lean_bend.py directly into an MCP stdio server you can mount today?


