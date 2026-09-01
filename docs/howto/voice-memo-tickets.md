# How to: turn a spoken thought into a project ticket file

**Goal:** Record a quick voice memo and get back a structured user-story/bug/task
ticket file, without opening an editor or a PM tool.
**Prereqs:** microphone access; an LLM/Whisper-compatible endpoint (defaults to
Noizu's internal `inference.noizu.com` LiteLLM gateway — override for your own).

1. Launch the interactive recorder (same binary, two names — `tabbing-plan`
   for scripting clarity, `task-memo` for muscle memory):
   ```bash
   tabbing-plan
   # or
   task-memo -p my-project -t bug
   ```
   It walks: record → transcribe (Whisper) → classify/draft (LLM) → preview →
   write ticket file.

2. Point it at a different LLM gateway, or fix the project/type up front:
   ```bash
   tabbing-plan --project my-project --type user-story \
                --api-url https://api.openai.com/v1 \
                --api-key "$OPENAI_API_KEY" \
                --model gpt-4o \
                --whisper-model whisper-1
   ```
   Flags win over env vars (`LITELLM_API_URL`, `LITELLM_API_KEY`, `M2T_MODEL`,
   `M2T_WHISPER_MODEL`) which win over built-in defaults.

3. Skip the tab-title integration (useful in scripted/CI contexts):
   ```bash
   tabbing-plan --no-tabbing
   ```

**Verify:** `tabbing-plan --help` prints usage instantly (no mic needed) to
confirm the binary and flags resolve; a completed session writes a ticket file
under `--output-dir` (default: inferred from the project's ticket directory
convention).

**Gotchas:**
- **Project not detected:** without `-p/--project`, it infers the project name
  from your `cwd` by looking for a `projects/<name>/` path segment — pass
  `-p` explicitly if your project isn't under a `projects/` parent.
- **No `-t/--type` given:** the LLM classifies the ticket type
  (user-story/bug/task) from the transcript itself — only pass `--type` to
  force one.
- **This is the Rust-native successor to the retained `ink-plan/` (Node/Ink)
  prototype** — use `tabbing-plan`/`task-memo`, not the Ink prototype, unless
  you're specifically working on the prototype itself.
