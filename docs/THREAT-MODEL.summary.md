# Threat Model — Summary

> Single-user local terminal tool. No network listener, no privileged
> execution, no cross-user trust. Jewels: user shell (eval/sourced code),
> terminal (escape emission), egress secrets + voice-memo audio.
> Full reference: [THREAT-MODEL.md](THREAT-MODEL.md)

## Trust Boundaries

1. Local same-user processes ↔ shell adapters / state files (`.env` files are
   sourced shell code; Claude FIFO writable by any local process)
2. User ↔ other local users on shared hosts (state files use default umask)
3. Workstation ↔ internet (HTTPS egress: Toggl API token, LiteLLM key + audio)

## Register at a glance

- **Open (3)**: T-001 terminal escape injection via title/status text (Medium);
  T-003 state files readable on multi-user hosts (Low); T-006 voice-memo audio
  + transcript leave machine to LiteLLM endpoint (Medium)
- **Accepted (4)**: T-002 session `.env` = stored shell code; T-004 Claude
  FIFO spoofing; T-007 secrets in env vars; T-009 daemon DoS; T-010 editable
  history log
- **Mitigated (2)**: T-005 tabbing-doctor config patching (partial);
  T-008 cargo supply chain (locked deps, native-tls)

## Residual Risk

Terminal escape injection (T-001) dominates: arbitrary strings are
interpolated into OSC/CSI sequences. Bounded by modern terminal sandboxing; a
sanitize-on-render pass would close it. Same-user compromise adds no new
capability via tabbing-on.
