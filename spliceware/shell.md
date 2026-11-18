A self-unpacking POSIX shell archive (bootstrap_gauntlet.sh) is the most efficient delivery mechanism across token cost, UI compatibility, and zero-friction deployment.
Instead of fighting binary attachment filters or clicking download on seven separate source files, a plain-text shell script renders natively as a clean file download and unpacks the entire modular directory tree (pyproject.toml, arena.rs, probe.py, forge.py, worktree.py, server.py) in a single command.

Delivery Mechanism Comparison


Method	Chat UI Reliability	Auditability	Setup Friction	Token & Byte Overhead
Self-Unpacking Script (.sh)	100% (UTF-8 plain text never gets stripped)	Instant (cat or read before running)	1 command (sh bootstrap.sh)	Lowest (raw text + 10 lines of mkdir/cat)
Binary Archive (.tar.gz / .zip)	Fragile (often blocked or hidden by UI sandboxes)	Opaque (must download and extract just to inspect)	2–3 commands (download, move, tar -xzf)	Medium (binary generation + base64 wrapper overhead)
Separate Module Downloads	100%	High	Tedious (7 manual downloads + manual folder creation)	High (per-file metadata repetition)


Why bootstrap_gauntlet.sh Fits the Splicer Ethos

• Zero binary opacity. You should never run an opaque binary blob that touches git and rustc without seeing what is inside. A standard cat << 'EOF' heredoc script lets you scan every line in two seconds before executing.
• Preserves true modularity on disk. Once executed (sh bootstrap_gauntlet.sh ./splicer-gauntlet), it vanishes and leaves behind a clean, standard Python package ready for uv tool install -e ./splicer-gauntlet or pip install -e ./splicer-gauntlet.
• Built-in smoke test. The tail end of the script can verify rustc + wasm32-unknown-unknown availability and run a 200ms self-check on the unpacked server automatically.
Ready for me to generate bootstrap_gauntlet.sh as a downloadable file with the full modular layout (probe, forge, worktree, server, arena.rs, plus splice_eject) wired up and ready to unpack?
