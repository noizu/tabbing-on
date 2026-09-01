# merge-notes — tabbing-on (sep-1 branch sweep, 2026-09-01)

Utility repo (`the-robot-lives/tabbing-on`; dual checkout `Portfolio/Utilities/source/tabbing-on` + `utilities/...`, same remote — swept from the source checkout). 

- Base: `origin/mono-repo-dev` @ cfabf65 (freshest tip by commit date; mono-repo-dev carries the real unmerged content vs a stale `main`).
- `develop` cut from mono-repo-dev cfabf65 and pushed; **mono-repo-dev retained as historical** — develop IS its continuation; do not open a PR for mono-repo-dev.
- `sep-1` tag created on cfabf65 (base tip).
- Local `main` left as-is / ff-synced to `origin/main` (was stale); main never pushed. Fast-forward `develop` → `main` when adopting.
- Branches pruned: none (repo held only `main` + `mono-repo-dev`; mono-repo-dev kept per sweep policy).

## Review/merge sequence
1. Review `develop` (== mono-repo-dev cfabf65) vs `main`; merge `develop` → `main` (FF) when validated. No other pending work; no open PRs.

## Open PRs
None.
