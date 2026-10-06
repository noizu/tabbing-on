# Threat Model

## Overview

tabbing-on is a **purely local, single-user terminal tool**: a CLI binary plus
sourced shell adapters running as the user, with flat-file state under
`~/.local/state/tabbing/`, optional egress to the Toggl Track API and a
LiteLLM/Whisper endpoint, and no listening network surface of its own. The
crown jewels are (1) the user's shell — adapters are `eval`/`source`d and
state files are shell code, (2) the terminal itself — the tool's whole job is
emitting escape sequences, and (3) secrets in env vars (`TAB_TOGGL_TOKEN`,
`LITELLM_API_KEY`) plus voice-memo audio sent to the LLM endpoint.

There is no multi-user service boundary. The trust boundaries that exist are:

1. **Local processes ↔ user shell/state**: any process running as the same
   user (compromised or not) can read/write state, write the Claude FIFO, or
   tamper with session `.env` files that wrappers `source`.
2. **User ↔ other local users** (shared hosts): state files are created with
   the default umask — no explicit `chmod` hardening.
3. **Workstation ↔ internet**: HTTPS egress with bearer-token auth to Toggl
   and the LiteLLM endpoint (audio + transcription text leave the machine).

Grounding: components and data flow in [PROJ-ARCH.md](PROJ-ARCH.md); the
implementing directories in [PROJ-LAYOUT.md](PROJ-LAYOUT.md).

## Attack Surface

```mermaid
graph LR
    U[User shell prompt] -->|eval "| INIT["tabbing-init / tabbing-ssh-shim<br/>emitted shell code"]
    CLI[CLI applets + adapters] --> ESC[Terminal via OSC/CSI escapes]
    CLI --> STATE[(~/.local/state/tabbing/<br/>YAML, .env, FIFO, pid)]
    CLI -->|token over HTTPS| TGL[Toggl Track API]
    PLAN[tabbing-plan / task-memo] -->|API key, audio| LLM[LiteLLM / Whisper endpoint]
    DAEMON[dc-mode daemon] -->|polls| DC[(direnv-config store)]
    CC[Claude Code IDE] -->|reads| FIFO[claude-*.pipe FIFO]
    OTHER[Other local processes] -.write.-> FIFO
    OTHER -.tamper.-> STATE
    DOCTOR[tabbing-doctor] -->|patches| CFG[Kitty / Ghostty config files]
```

Notable boundary-crossing components:

| Surface | Where | Concern |
|---------|-------|---------|
| `eval "$(tabbing-init bash\|zsh)"` and eval-able `emit`/shim output | `rust/src/init.rs`, `render.rs` | shell-code injection if emitted output is attacker-influenced |
| Terminal escape emission (OSC 0/4/6/10/11/12/1337) | `rust/src/render.rs`, `terminal.rs`, `lib/render.sh` | escape-sequence injection via title/status text |
| Session `.env` files `source`d by wrappers | `shell-impl/lib/session.sh`, `_tabbing-wrapper` | stored shell code = code execution if file tampered |
| Claude bridge FIFO + state in XDG dirs | `rust/src/claude.rs`, `lib/claude.sh` | any local process can write the pipe / read state |
| Config auto-patching | `tabbing-doctor` | rewrites user terminal config files |
| HTTPS egress with token/key | `toggl.rs`, `plan/llm.rs` | credential handling; audio privacy |
| Supply chain | `rust/Cargo.lock` (reqwest + native-tls, clap, ratatui, ...) | dependency compromise |

## Vulnerability Register

| ID | Severity | STRIDE | Component | Status |
|----|----------|--------|-----------|--------|
| T-001 | Medium | Tampering | Terminal escape emission | Open — no explicit control-char sanitization of title/status text found in `render.rs` / `render.sh`; malicious text (e.g. from a script that reads a repo/branch name into a title) could inject escapes |
| T-002 | Medium | Tampering/EoP | Session `.env` sourcing | Accepted — `sessions/{TAB_SESSION}.env` is shell code by design; same-user trust domain, file tamper ⇒ already game over |
| T-003 | Low | Info disclosure | State files on multi-user hosts | Open — files created with default umask; history/todos may contain sensitive titles. Mitigate with umask 077 or chmod if shared-host use matters |
| T-004 | Low | Spoofing | Claude bridge FIFO | Accepted — any same-user process can write `claude-*.pipe`; scoped to statusline display |
| T-005 | Low | Tampering | `tabbing-doctor` config patching | Mitigated (partial) — patches only Kitty/Ghostty title-related settings, but writes user config files; no backup/rollback contract documented |
| T-006 | Medium | Info disclosure | Voice-memo pipeline | Open — audio + transcript sent to `LITELLM_API_URL` with no data-handling guarantees; key in env. Endpoint trust is a user decision, not a code control |
| T-007 | Low | Info disclosure | Secrets in env vars (`TAB_TOGGL_TOKEN`, `LITELLM_API_KEY`) | Accepted — inherited from the Toggl/CLI ecosystem convention; env leaks via `/proc` on shared hosts noted |
| T-008 | Low | Tampering | Supply chain (cargo deps) | Mitigated — `Cargo.lock` checked in; reqwest uses native-tls (system CA store) |
| T-009 | Low | DoS | dc-mode daemon | Accepted — local, single instance per session, pid-file guarded |
| T-010 | Low | Repudiation | History log | Accepted — YAML append log is user-editable by design; not an audit control |

## Mitigation Coverage

0 mitigated/partial · 3 accepted · 3 open (T-001, T-003, T-006). The open
items are low-effort hardening candidates, not active exposures: there is no
network listener, no privileged execution, and no cross-user trust anywhere in
the tool.

## Residual Risk

The dominant residual risk is **terminal escape injection (T-001)**: the tool
interpolates arbitrary strings (titles, statuses, git-derived text) into
escape sequences that the terminal interprets. Modern terminals sandbox most
OSC payload effects, so impact is bounded to rendering/clipboard-adjacent
quirks, but a sanitize-on-render pass would close it. Everything crossing the
same-user boundary is accepted: on a compromised same-user host, tabbing-on
adds no new capability the attacker lacks.
