---
name: diapers
description: Run the agent in an uninterrupted, self-looping loop (Diapers Mode) to perform continuous coding and testing tasks without pausing for user input, structured around the 12 ADDIECRAPEYE phases.
---

# Diapers Mode (Continuous ADDIECRAPEYE Workflow)

When the user triggers this skill (by invoking `/diapers`, mentioning "diapers mode", or asking you to run in diapers mode):

## 1. Structure the Tasks by ADDIECRAPEYE Phase
Your `task.md` checklist must map to the relevant stations of the 12 **ADDIECRAPEYE** phases:
* **Analysis**: Define requirements, audience, and system integrations.
* **Design**: Layout architecture, API routing, and module relationships.
* **Develop**: Write core modules, systems, and structures.
* **Implement**: Hook assets, register plugins, and establish run targets.
* **Evaluate**: Run test suites (`cargo test` / lint checks).
* **Contrast / Repetition / Alignment / Proximity**: Perform UI consistency, layout grouping, and code structure audits.
* **Envision / Yoke / Evolve**: Reflect on performance, build production outputs (`trunk build`), and package deliverables.

## 2. Execute and Self-Chain Loops
* At the end of each turn, if tasks remain in the checklist, **do not stop or wait for user input**.
* Programmatically invoke the `schedule` tool to trigger an immediate callback wakeup for the next turn:
  - `DurationSeconds`: `"1"`
  - `Prompt`: `"Diapers Mode: Continue to the next task in the ADDIECRAPEYE queue. Next Phase: [Active Phase], Task: [Next Task Name]"`
  - `TimerCondition`: `"never"`
* This wakes the agent up in a clean turn, bypassing standard user-input pauses.

## 3. Scope Limits Per Turn
* **One task per turn**: Do not rush through multiple checklist items in a single turn. Complete one task, verify it with `cargo test`, check it off, then chain to the next turn.
* **Depth over breadth**: It is better to implement one feature thoroughly (with tests, error handling, and documentation) than to scaffold three features superficially.

## 4. Context Budget Monitoring
* Be aware that context windows fill up over long runs. If you have been chaining for more than 8-10 turns:
  - **Commit progress**: Run `git add -A && git commit -m "Diapers checkpoint: [phase] [task]"` to preserve work.
  - **Update task.md**: Ensure task.md reflects exactly what is done and what remains.
  - **Consider a subagent handoff**: Spawn a `self` subagent with a fresh context, passing it the task.md as instructions. The subagent reads task.md and resumes from the first unchecked item.

## 5. Session Recovery Protocol
If you are starting a new session (context was truncated or the user returned after a break):
1. Read `task.md` from the artifact directory — it is the single source of truth for progress.
2. Read `walkthrough.md` for context on what was built and how.
3. Read `AGENTS.md` for workspace layout and build commands.
4. Resume from the first unchecked `[ ]` item in task.md.
5. Do NOT re-do completed work. Trust the checkmarks.

## 6. Git Commit Checkpoints
* Commit after completing each **phase** (not each task — that's too noisy).
* Use descriptive commit messages: `"Phase 11 complete: curriculum manager and NPC dialogue trees"`
* This ensures that if context resets, no code is lost.

## 7. Sandboxed Heuristics & Resolution
* Do not yield control to the user for cargo warnings, minor build errors, or test failures.
* Solve compile issues, imports, type conflicts, and path dependencies autonomously.
* Keep the `task.md` and `implementation_plan.md` updated as a single source of truth, so that if the context is truncated over long runs, you immediately recover your active state from these artifacts.

## 8. Graceful Termination
* When all items in task.md are checked off (`[x]`), **stop chaining**.
* Update `walkthrough.md` with a summary of everything accomplished.
* Do NOT schedule another callback — the loop is done.
