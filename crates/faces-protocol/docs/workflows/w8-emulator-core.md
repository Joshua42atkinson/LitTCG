---
description: W8 — Terminal UI emulator with state display, detection simulation, and multiple render modes
---

# W8: Emulator Core

## Objective

Build a terminal-based emulator that displays FACES states in real time,
simulates text-to-FACES detection, and supports multiple render modes. This
is the primary development and demonstration tool for the FACES protocol.

## Prerequisites

- W1-W5 complete (Gate 1 passed)
- W6-W7 complete (Gate 2 passed, eval harness available)
- All protocol, detection, transition, consent, and phase features working

## Steps

1. **Create `emulator/` directory in `crates/faces-protocol/`**:
   - `emulator/mod.rs` — module root
   - `emulator/ui.rs` — terminal UI rendering
   - `emulator/input.rs` — keyboard input handling
   - `emulator/session.rs` — session state management
   - `emulator/render_modes.rs` — multiple display modes

2. **Implement session management** (`session.rs`):
   - `Session` struct: current_state, profile, gate, phase_manager, history
   - `Session::new()` — neutral state, empty profile, locked gate
   - `process_text(&mut self, text: &str)` — detect + update state
   - `process_tick(&mut self)` — decay, auto-lock, phase transitions
   - `state_display() -> String` — formatted current state

3. **Implement render modes** (`render_modes.rs`):
   - `RenderMode::Plain` — ASCII face only: `{^^~}` (no color)
   - `RenderMode::Color` — ASCII face with ANSI color from Aura
   - `RenderMode::Big` — large multi-line ASCII art face
   - `RenderMode::Telemetry` — hex + face + all dimension values
   - `RenderMode::Debug` — everything: face, hex, RGB, consonance, harmonic distance, phase, gate
   - `render(mode: RenderMode, state: &FacesState, session: &Session) -> String`

4. **Implement terminal UI** (`ui.rs`):
   - Split-screen layout:
     - Top: current FACES state display (render mode selectable)
     - Middle: input text + detected state preview
     - Bottom: transition log (last 5 transitions with magnitude + harmonic distance)
   - `draw(&self, session: &Session) -> String` — full screen render
   - No external TUI library — use ANSI escape codes directly (zero-dep)

5. **Implement input handling** (`input.rs`):
   - `InputMode` enum: Text, Command, Navigate
   - Text mode: type text, Enter to process
   - Command mode: `:set <mode>`, `:gate lock`, `:gate unlock`, `:phase static`, `:phase dynamic`, `:quit`
   - Navigate mode: arrow keys to browse history
   - `handle_input(&mut self, key: char) -> InputResult`

6. **Implement transition log**:
   - Ring buffer of last 10 transitions
   - Each entry: timestamp, from_state, to_state, magnitude, harmonic_distance
   - `log_transition(from: &FacesState, to: &FacesState)`
   - `format_log() -> String` — formatted log for display

7. **Add emulator as optional feature**:
   - `Cargo.toml`: `[features] emulator = ["eval"]`
   - Emulator depends on eval module for benchmark integration
   - Default build remains zero-dep

8. **Implement main loop**:
   - Read input → process → update state → render → repeat
   - `run()` function: main emulator loop
   - Graceful shutdown on `:quit` command
   - Ctrl-C handling (if possible with std only)

## Testing

- Session: text processing updates state, tick triggers decay
- Render modes: each mode produces non-empty, correct output
- UI layout: draw produces multi-line string with expected sections
- Input: text mode processes input, command mode executes commands
- Transition log: entries added correctly, ring buffer wraps
- Command parsing: `:set`, `:gate`, `:phase`, `:quit` all work
- Integration: type text → see state change → see transition logged

## Completion Criteria

- `emulator/` module with session, UI, input, render modes
- 5 render modes: Plain, Color, Big, Telemetry, Debug
- Split-screen terminal UI with ANSI codes (zero-dep)
- Command system for gate/phase/render mode control
- Transition log with magnitude and harmonic distance
- Emulator feature flag (opt-in)
- All tests pass (target: 250+ tests)
- Zero dependencies maintained in default build
- PROGRESS.md updated
- **Gate 3: Joshua reviews emulator**
