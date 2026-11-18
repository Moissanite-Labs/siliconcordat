#  grafting

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

