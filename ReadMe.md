# Mafia TUI

A terminal-based Mafia game engine focused on **explicit game flow**,
**deterministic state transitions**, and **command-driven interaction**.

This project models the Mafia game as a well-defined engine rather transitions
a UI-driven toy app. The terminal UI is a thin layer on top of the engine,
not the source of truth.

> Current version: **0.1.beta**  
> Status: foundation complete, gameplay description and refinement in progress

---

## What this is

- A **game engine** for Mafia
- A **terminal UI (TUI)** for hosting and controlling the game
- A **command-based interface** instead of keybinding-heavy interaction
- A foundation for future extensions (roles, rules, automation)

---

## What this is not (yet)

- Not a polished end-user game
- Not multiplayer or networked
- Not feature-complete in terms of Mafia variants
- Not focused on visuals

---

## Core ideas

- **Commands are the public API** of the game
- **Game flow is explicit**, not implicit
- **Phases drive behavior**, not ad-hoc conditionals
- **Rules are enforced by the engine**, not by the host remembering them

---

## High-level game flow

Lobby → Night → Day → Vote → Resolution → Night → ...
Each phase:

- Allows a specific set of commands
- Restricts invalid actions
- Emits events that are rendered by the TUI

---

## Interaction model

- One command input line
- Commands are typed and executed explicitly
- No modal keybindings or hidden shortcuts
- The UI reflects engine state, never mutates it directly

Example commands:

```text
join Alice
join Bob
start
next
assign
next
assign
next

---

## Interaction model

- Single command input line

- Explicit typed commands (no modal keybindings)

- Event log and engine state rendered in the TUI

--- 

## Running the application

```bash
cargo run
```

The application will start in the terminal with a TUI interface.

---

## Key bindings

### Normal mode

| Key | Action |
|-----|--------|
| `:` | Enter command mode |
| `j` | Join player (opens popup) |
| `l` | Leave player (opens popup) |
| `n` | Next phase |
| `b` | Start game |
| `w` | Warn player (opens popup) |
| `p` | Pardon player (opens popup) |
| `o` | Nominate player (opens popup) |
| `c` | Check player (opens popup) |
| `g` | Guess targets (opens popup) |
| `v` | Vote (opens popup) |
| `s` | Shoot player (opens popup) |
| `h` | Show help dialog |
| `Esc` | Quit game |

### Command mode

| Key | Action |
|-----|--------|
| `Enter` | Execute typed command |
| `Esc` | Return to normal mode |
| `Backspace` | Delete last character |

### Popup mode

| Key | Action |
|-----|--------|
| `Enter` | Confirm popup action |
| `Esc` | Close popup without applying |
| `Backspace` | Delete last character |
| Any character | Type input |

---

## Project structure
├── engine/     # Core game logic, state, commands, phases
├── tui/        # Terminal UI (layout, widgets, input)
├── app/        # Application glue code
└── docs/       # Documentation (work in progress)

