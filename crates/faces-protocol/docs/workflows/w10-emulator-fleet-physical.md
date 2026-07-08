---
description: W10 — Multi-agent emulator fleet with behavior mapping and LED matrix physical simulation
---

# W10: Emulator Fleet + Physical Simulation

## Objective

Extend the emulator to support multiple FACES agents simultaneously (fleet
mode), map FACES states to physical behaviors, and simulate LED matrix output
using the `to_rgb()` conversion from W1. This is the final workflow and the
bridge to physical AI deployment.

## Prerequisites

- W9 complete (emulator interaction with Consent Gate, VAAM, transitions)
- W1 complete (to_rgb() and safety presets available)

## Steps

1. **Multi-agent fleet support**:
   - `Fleet` struct: manages multiple `Session` instances
   - `Agent` struct: id, name, session, role (leader/follower/observer)
   - `Fleet::new()` — empty fleet
   - `add_agent(name: &str, role: Role) -> AgentId`
   - `remove_agent(id: AgentId)`
   - `agents() -> &[Agent]`
   - `agent_mut(id: AgentId) -> &mut Agent`

2. **Fleet display**:
   - Grid layout: each agent gets a cell showing its FACES state
   - Agent cells arranged in configurable grid (rows x cols)
   - `render_fleet(mode: RenderMode) -> String` — grid of agent states
   - Highlight agent with focus (`:focus <agent_name>`)
   - Show fleet-wide metrics: average aura, dominant container, etc.

3. **Behavior mapping**:
   - `BehaviorMap` struct: maps FACES dimensions to physical actions
   - Aura → LED color (via `to_rgb()`)
   - Container → posture (neutral/rigid/fluid/defensive/sharp)
   - Focus → attention direction (neutral/intense/open/distant/happy/tired)
   - Action → movement (withheld/assertive/playful/thoughtful/hesitant)
   - `map_state(state: &FacesState) -> Behavior`
   - `Behavior` struct: led_color, posture, attention, movement (all as enums/strings)

4. **LED matrix simulation**:
   - `LedMatrix` struct: width, height, pixels (Vec of (r, g, b))
   - `LedMatrix::new(width: usize, height: usize)`
   - `set_pixel(x: usize, y: usize, color: (u8, u8, u8))`
   - `from_state(state: &FacesState) -> LedMatrix` — Aura color fills matrix
   - `from_fleet(fleet: &Fleet) -> LedMatrix` — each agent gets a region
   - `render_ascii() -> String` — ASCII art representation of LED matrix
   - Brightness modulation: consonance level affects brightness

5. **Safety preset integration**:
   - `:safety emergency` — all agents set to EMERGENCY preset
   - `:safety low_power` — all agents set to LOW_POWER preset
   - `:safety human_proximity` — all agents set to HUMAN_PROXIMITY preset
   - `:safety clear` — return all agents to their previous states
   - Visual alarm when any agent enters EMERGENCY

6. **Fleet transition visualization**:
   - When any agent transitions, show it in a fleet-wide transition log
   - `fleet_transition_log` — last 20 fleet transitions
   - Show agent name, transition, magnitude, harmonic distance
   - Highlight volatile transitions across the fleet

7. **Physical simulation display**:
   - `:physical` command toggles physical simulation panel
   - Shows LED matrix ASCII art for focused agent or whole fleet
   - Shows behavior mapping: posture, attention, movement
   - Shows RGB values: `RGB: (215, 0, 0)` for urgent

8. **Fleet demo scenarios**:
   - `:demo fleet` — spawn 4 agents, assign different VAAM profiles, show grid
   - `:demo swarm` — all agents process same text, show divergent states
   - `:demo safety` — trigger emergency preset across fleet, show LED alarm
   - `:demo led` — cycle through all 10 named Auras, show LED matrix colors

## Testing

- Fleet: add/remove agents, agent count correct
- Fleet display: grid renders, each agent cell non-empty
- Behavior mapping: each FACES dimension maps to correct behavior
- LED matrix: from_state produces correct color, from_fleet divides correctly
- LED matrix: ASCII render produces non-empty string
- Safety presets: all agents update, clear restores previous states
- Fleet transitions: logged correctly, volatile transitions highlighted
- Physical simulation: LED matrix and behavior mapping display correctly
- Demo scenarios: run without panic, produce expected output

## Completion Criteria

- `Fleet` and `Agent` structs with multi-agent management
- Fleet grid display with per-agent cells
- `BehaviorMap` with FACES → physical behavior mapping
- `LedMatrix` with RGB output via `to_rgb()`
- Safety preset fleet-wide commands
- Fleet transition log
- Physical simulation panel with LED ASCII art
- 4 fleet demo scenarios
- All tests pass (target: 300+ tests)
- Zero dependencies maintained in default build
- PROGRESS.md updated
- **Final review: full W1-W10 complete, crate ready for open-source release**
