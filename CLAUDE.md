# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project overview

`tuigseq` is a terminal UI (TUI) client for [Logseq](https://logseq.com), built in Rust with `ratatui` and `crossterm`. It renders a list of "blocks" (Logseq's term for outline nodes) that can be navigated and edited in place.

## Commands

- Build: `cargo build`
- Run: `cargo run`
- Type-check (fast, no codegen): `cargo check`
- Format: `cargo fmt`
- Lint: `cargo clippy`

There are no tests in the repository yet.

## Architecture

The app follows The Elm Architecture (TEA) — Model / Update / View / Message — rather than idiomatic ratatui-style direct rendering.

- `model.rs` — the single `Model` struct (`mode`, `blocks: Vec<String>`, `selected`, `running`). `Mode` is an enum: `Normal` or `Editing(EditData)`, where `EditData` holds the in-progress text and a byte-offset `cursor`. Editing state is only valid/accessible while `Mode::Editing` is active.
- `msg.rs` — the `Message` enum lists every possible action across all modes (`MoveUp`, `InsertChar(char)`, `Quit`, etc.). It is intentionally *not* split per mode; each mode's `update.rs` matches only the subset of variants it cares about and falls through `_ => None` for the rest, relying on `input.rs` to only ever emit mode-appropriate messages.
- `input.rs` — translates raw crossterm key events into `Message`s. The keymap branches on `model.mode`: Normal mode and Editing mode have entirely different bindings (e.g. `j`/`k` to move, `A` to start editing, vs. `Esc`/`Backspace`/arrow keys while editing).
- `modes.rs` and `modes/` — the app is organized per-mode rather than per-layer: `modes/normal/` and `modes/edit/` each own their own `update.rs` (applies a `Message` to the `Model`) and `view.rs` (renders that mode). `modes.rs` is the top-level dispatcher: it matches `model.mode` and delegates both `update` and `view` calls to the active mode's module. `update` returns `Option<Message>`, letting a handler chain into a follow-up message; `main.rs`'s loop keeps calling it until it returns `None`. Editing-specific text manipulation (`InsertChar`, `Backspace`, cursor movement) uses `char_indices()` rather than raw byte indexing to stay UTF-8-safe. Adding a new mode means adding a new `modes/<name>/` directory with its own `update.rs`/`view.rs`, not touching the existing ones.
- `modes/normal/view.rs` renders the plain list; `modes/edit/view.rs` renders the list with the selected row swapped for an editable text field. Both build the same two-pane layout (a bordered menu pane at 25% width, content at 75%) independently — check both when changing shared layout/chrome.
- `widgets/` — small reusable `Widget` impls shared across modes: `ListItemWidget` (enum wrapping either a plain `Line` or the `TextInputWidget`, used as the `tui-widget-list` builder's per-row output in edit mode) and `TextInputWidget` (renders text plus a manually-drawn block-cursor by styling a single buffer cell — there is no real terminal cursor involved).
- `tui.rs` — terminal setup/teardown (raw mode, alternate screen).
- `main.rs` — owns the run loop: draw → poll one input event → drain the `Message` chain through `modes::update` → repeat until `model.running` is false, then restore the terminal.

### Key conventions

- Normal-mode list rendering uses the plain `ratatui::widgets::List`, while edit mode uses `tui-widget-list::ListView` so the selected row can have a different, taller height while editing. Keep this distinction in mind when changing row heights or selection rendering.
- Cursor/text editing logic always works in byte offsets but must advance/retreat by full UTF-8 scalar values (`char_indices`, `len_utf8`) — never assume 1 byte per character.
