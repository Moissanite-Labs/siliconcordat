Passing inline code strings (stripped_rust_code, host_seam_code) instead of file paths in gauntlet_mcp.py enforces physical air-gapping and atomic transactions, though it hits real limits once a splice grows past a single module.
In the Gauntlet architecture, the agent does not have general filesystem write permissions. If the MCP tools accepted a file_path for the stripped Rust code or Seam adapter, the agent would first need a raw write_file tool to stage those files on disk—breaking the containment boundary before verification even starts.

Why Inline Payloads Work for Surgical Splices

• Zero Workspace Pollution. When an agent iterates three times on splice_forge_preview to fix a borrow-checker error, inline strings live purely in RAM and vanish inside tempfile.TemporaryDirectory(). Your working tree never sees .tmp_slice_v2.rs litter.
• Hard Containment Enforcement. Withhold raw filesystem write tools and force payloads through the JSON-RPC call. The agent physically cannot mutate unauthorized Host files or skip the RUST_ARENA_PRELUDE injection.
• Remote & Container Portability. An MCP server running inside Docker or over SSH does not share a local filesystem path namespace with the client IDE or orchestrator. Inline strings cross process and container boundaries with zero volume-mount mismatches.

Where Inline Strings Break (And Why Modularity Wins at Scale)

You spotted the exact architectural seam where the single-file MVP starts to strain:

Dimension	Inline String Payload (Current MVP)	File / Staged Directory Reference
Token Cost	High. The LLM must re-emit the entire 150-line slice on every retry if 1 line fails to compile.	Low. Agent applies a 3-line surgical diff to a staged file without regenerating unchanged code.
JSON Escaping	Fragile. Escaping nested quotes, regexes, and raw string literals (r#"..."#) inside JSON arguments causes occasional syntax fumbles.	Zero Escaping. Files sit cleanly on disk as native UTF-8 source files with IDE/LSP syntax support.
Multi-File Splices	Awkward. Grafting a Donor module split across parser.rs + tables.rs requires jamming everything into one blob.	Modular. Preserves clean sub-module boundaries (mod tables;) inside the isolated graft folder.
Auditability	Ephemeral until the final commit step succeeds.	Humans can inspect the staging directory live while the agent works.


The Best of Both Worlds: Managed Staging Handles

To get true modularity without surrendering the Gauntlet's containment field, you upgrade the protocol from inline strings to Gauntlet-owned Staging Sessions (session_id + modular relative file paths).
Instead of handing the agent raw write_file access to your repo, the Gauntlet opens an isolated staging directory outside your working tree (.git/gauntlet-stage/<session_id>/) and exposes modular file operations strictly inside that sandbox:
python
# Upgraded Modular Gauntlet Flow — Zero Inline Blobs, Full Containment

# 1. Open an isolated workspace (returns session_id, initializes conduit/arena.rs separately!)
splice_init_session(capability="myers_diff", donor_uri="...") 
# ->Creates .git/gauntlet-stage/s_9f8a/
#     ├── arena.rs      (Modular allocator file — never mixed into Donor code!)
#     ├── lib.rs        (Entry point wiring `mod arena; mod core;`)
#     └── core/         (Clean folder for 1 or N modular Donor files)

# 2. Extract or patch individual files inside the sandbox (no JSON mega-blobs)
splice_stage_file(session_id="s_9f8a", target="core/table.rs", donor_source="src/table.rs")
splice_patch_stage(session_id="s_9f8a", target="core/table.rs", old="std::fs", new="core::slice")

# 3. Compile the modular crate directory as a real multi-file unit
splice_forge_session(session_id="s_9f8a")

# 4. Commit atomically from stage -> worktree verify -> vendor/splices/myers_diff/
splice_commit_session(session_id="s_9f8a", verify_command="npm test")
Use code with caution.
