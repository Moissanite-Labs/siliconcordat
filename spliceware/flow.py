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
