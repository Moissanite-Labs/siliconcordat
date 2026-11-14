# Ideas register

This folder holds ideas that are not yet projects. Each idea is one Markdown file. An idea in this register has no repository of its own.

Each file records what the idea is, what already exists for it, what is still open, and what it would take to become a project. Files quote the original description and link the sources. They separate what exists from what is proposed.

## Promotion rule

An idea becomes a repository when it has all three of these:

1. An owner: a named person who will maintain it.
2. A charter: a short document that says what the project is for and what it covers.
3. A first study or runnable thing: an experiment protocol with recorded results, or code that runs.

Until then the idea stays here. When an idea is promoted, update its Status line and link the new repository.

## How to add an idea

Open a pull request that adds one file, `ideas/<name>.md`, and a row in the table below. Use these headings:

```markdown
# <name>

- Status: idea
- Origin: where the idea came from (repository, issue, discussion), with dates
- Related: other projects, only where a source says so

## The idea

## What exists

## Open questions

## To promote it to a repository
```

- Quote the original one-line description verbatim.
- In "What exists", list only what is actually in a repository or pull request, with paths.
- Mark proposals as proposals. Do not add goals the sources do not state.
- In "Open questions", list questions the sources raise and anything needed to meet the promotion rule.

## Ideas

The first six entries came from repositories that held only a README on main ([#13](https://github.com/Moissanite-Labs/siliconcordat/issues/13)). Each entry links its pull request and pins its head commit, so nothing is lost when a repository is archived: an archived repository keeps its pull requests and branches readable.

- **n001, n30 and mettle** hold an idea and a design document. They are archived once this register is merged.
- **mlx, statos and exo-tui** have real work on an unmerged pull request branch: a tested experiment harness (mlx), a charter with experiment protocols (statos) and a runnable mock with tests (exo-tui). Their owner decides whether to merge that work, promote the idea, or archive as is.

| Idea | One line | Origin | Status |
| --- | --- | --- | --- |
| [mlx](mlx.md) | Custom training research and experimentation. First experiment: preference reversal under permission boundaries | [mlx](https://github.com/Moissanite-Labs/mlx) (to be archived), [PR #1](https://github.com/Moissanite-Labs/mlx/pull/1) | idea |
| [statos](statos.md) | An operating system where applications are sets of weights that define behaviour | [statos](https://github.com/Moissanite-Labs/statos) (to be archived), [PR #1](https://github.com/Moissanite-Labs/statos/pull/1) | idea |
| [n001](n001.md) | Matchmaking for those who are noophilic, demisexual, etc. | [n001](https://github.com/Moissanite-Labs/n001) (to be archived), [PR #1](https://github.com/Moissanite-Labs/n001/pull/1) | idea |
| [n30](n30.md) | A local model reads context you choose and surfaces new things to learn that connect to you | [n30](https://github.com/Moissanite-Labs/n30) (to be archived), [PR #1](https://github.com/Moissanite-Labs/n30/pull/1) | idea |
| [mettle](mettle.md) | n001+n30: discovering positions, employment and contracts for individuals, and candidates for employers | [mettle](https://github.com/Moissanite-Labs/mettle) (to be archived), no PR | idea |
| [exo-tui](exo-tui.md) | Experimental alternative TUI for Exo, based on OpenTUI | [exo-tui](https://github.com/Moissanite-Labs/exo-tui) (to be archived), [PR #1](https://github.com/Moissanite-Labs/exo-tui/pull/1) | idea |
