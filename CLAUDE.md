# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

---

| Document | Purpose | Summary |
|----------|---------|---------|
| [`docs/PROJ-LAYOUT.md`](docs/PROJ-LAYOUT.md) | Directory tree with descriptions of what each directory and key file contains | `docs/PROJ-LAYOUT.summary.md` |
| [`docs/PROJ-ARCH.md`](docs/PROJ-ARCH.md) | High-level architecture: components, diagrams, design decisions | `docs/PROJ-ARCH.summary.md` |

### When to Reference

- **PROJ-LAYOUT** — Finding files, understanding directory organization, locating entry points.
- **PROJ-ARCH** — Understanding component relationships, layered design, state management, or terminal abstraction.

### Overflow Structure

Each doc stays concise by extracting detail into subdirectories:

```
docs/
├── PROJ-LAYOUT.md          + layout/*.md
├── PROJ-ARCH.md            + arch/*.md
└── *.summary.md            (compact versions for quick reference)
```

Summary files contain the same structure in condensed form — prefer these for fast orientation, then read the full doc when detail is needed.

### Maintenance — Auto-Sync Documentation

After completing code changes, **you MUST run the appropriate update skill(s)** if your changes match any trigger condition below. Do not merely flag that docs need updating — actually run the update.

| Doc | Skill | Trigger: run when you... |
|-----|-------|--------------------------|
| PROJ-LAYOUT | `update-layout-doc` | Create, delete, rename, or move any file or directory |
| PROJ-ARCH | `update-arch-doc` | Add a library, command, integration, or change data flow / state management |

**Do NOT trigger** for: editing logic within existing files, fixing bugs, updating tests, changing comments, or modifying configuration values.

**How to run**: Use the `Skill` tool to invoke the skill by name (e.g., `skill: "update-layout-doc"`). Run the skill **after** all code changes are complete, so the update reflects the final state.

- If you have **no remaining work**: invoke the Skill directly (blocking is fine).
- If you **still have other tasks**: dispatch a background subagent to run the Skill, then continue your work. Mention the background update to the user.
- If **multiple docs** need updating: invoke each skill in parallel.

---

## Response Protocol

### Assumptions Table

Open every response with a table of assumptions made to resolve ambiguities, followed by a mermaid diagram response plan. Restate the request, show how context/knowledge/assumptions shape the response, lay out review steps, then follow the plan.

### Reflection Block

Append a self-review reflection block to the **end of every response**:

```
<block type="reflection">
[one issue per line, emoji prefix, < 80 chars each]
</block>
```

**Emoji indicators**: `✅` Verified | `🐛` Bug | `🔒` Security | `⚠️` Pitfall | `🚀` Improvement | `🧩` Edge Case | `📝` TODO | `🔄` Refactor | `❓` Question

Review for: correctness, security, edge cases, improvements, completeness. Always include at least one `✅`. Never skip this block.

---

## Scratchpad Directory Rule

**ALWAYS use `.tmp/` for temporary files, NOT `/tmp/`** (except plan files which cannot be saved here in plan mode).

`.tmp/` is project-scoped and persists across sessions. `/tmp/` is system-wide and ephemeral.

---

## Architecture

**Three-layer design** — understand this before modifying anything:

1. **POSIX Libraries** (`lib/*.sh`) — Pure POSIX sh. All functions prefixed `_tabbing_*`. No bash/zsh-isms. This is where core logic lives.
2. **Shell Adapters** (`shell/tabbing.{bash,zsh}`) — Source all five libraries, define user-facing functions. Handle array indexing differences (0-based vs 1-based) and prompt hooks (`PROMPT_COMMAND` vs `precmd`).
3. **Bootstrap** (`bin/tabbing-init`) — POSIX `/bin/sh`. Resolves `TABBING_ROOT`, outputs `source` command.

**Critical constraint**: When adding functionality, put shared logic in `lib/*.sh` using POSIX sh, then add shell-specific wrappers in *both* `shell/tabbing.bash` and `shell/tabbing.zsh`. Never put bash-isms or zsh-isms in `lib/`.

## State

- **Runtime**: Environment variables (`TAB_TITLE`, `TAB_STATUS`, `TAB_ID`, `TAB_HIGHLIGHT`, `TAB_URGENCY`, `TAB_EMOJI`, `TAB_TERMINAL`, `TAB_RECORDING`)
- **Persistent**: YAML under `$XDG_STATE_HOME/tabbing/` — parsed with `sed`/`awk`, no external YAML tools

## Development

| Goal | Command |
|------|---------|
| Install (zsh) | `eval "$(bin/tabbing-init zsh)"` |
| Install (bash) | `eval "$(bin/tabbing-init bash)"` |
| Run demo | `bin/demo-runner` |
| Run demo (fast) | `bin/demo-runner --fast` |

No build system, no package manager, no test framework yet (`tests/` is empty). Shell scripts are executed directly.

**Dependencies**: Only POSIX utilities (`sed`, `awk`, `date`, `mkdir`, `printf`). Optional: `asciinema` (recording), `agg` (GIF conversion).

---

*End of CLAUDE.md*
