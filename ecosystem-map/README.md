# Ecosystem map

This folder maps the Moissanite-Labs repos: what each one is for, who works on it, how they depend on each other, and what to do next. It is a dated snapshot. The current map is from 2026-11-03.

## Files

- [2026-11-03-map.md](2026-11-03-map.md): the map. **[F]** marks a fact read from GitHub. **[I]** marks an inference.
- [2026-11-03-log.md](2026-11-03-log.md): coordination log. Secret protection, the exo main ruleset, the blocked PR backlog and the agreed work sequence.
- [2026-11-03-exo-blocked-prs.tsv](2026-11-03-exo-blocked-prs.tsv): exo PRs blocked by the main ruleset, with the reason for each.
- [2026-11-03-raw/](2026-11-03-raw/): raw survey data, one folder per repo (metadata, PRs, issues, recent commits, CI runs, file tree, rulesets).

## Refresh

1. Re-run the read-only survey across all org repos. Make no writes during the survey.
2. Add new files with the new date, such as `YYYY-MM-DD-map.md` and `YYYY-MM-DD-raw/`.
3. Never overwrite or edit an older dated file.
4. Update this README to point at the newest files.
