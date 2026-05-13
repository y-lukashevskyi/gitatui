# TODO

## Current progress

- [x] Run `git diff` and display output in a TUI
- [x] Split screen into header and body with borders
- [x] Parse diff into a `Vec<Line>` once at startup
- [x] Colorize diff lines (`+` green, `-` red, `@@` cyan, `diff --git` yellow/bold)

## Diff viewer

- [x] Add scrolling (`j`/`k` and arrow keys, `q` to quit)
- [x] Add a status bar at the bottom with keybinding hints
- [x] Fix line styling order so `+++`/`---` and `diff --git` headers don't get
      miscolored as additions/deletions
- [x] Clamp scroll position to the actual diff length
- [x] Make scroll clamp viewport-aware so the last line pins to the bottom
      instead of scrolling off the top
- [x] Refactor: extract state into an `App` struct
- [x] Refactor: move line-styling logic into its own function

## Global menu

Approach A: a `View` enum (`Menu`, `LocalDiff`, `RemoteDiff`, `Pulls`) plus
flat state on `App`. Full-screen replacement (no overlay). `q` is contextual —
quits from the menu, returns to the menu from any view.

- [ ] Add `View` enum and `view: View` field on `App`; default to `View::Menu`
- [ ] Add `menu_selected: usize` (or `ListState`) for highlighted item
- [ ] Render menu screen with the `List` widget and `ListState`
- [ ] Dispatch in event loop and renderer with `match app.view`
- [ ] Generalize `get_git_diff` to take args (so local vs `origin/main` reuses it)
- [ ] Wire menu → LocalDiff: shell out, populate `diff_lines`, switch view
- [ ] Wire menu → RemoteDiff (`origin/main`): same flow, different args
- [ ] Add `Pulls` placeholder screen ("Coming soon")
- [ ] Add `back_to_menu` on `q`/`Esc` from any non-menu view
- [ ] Reset `scroll_offset` when entering a diff view
- [ ] Update status bar hints per view

### Deferred (low-priority)

- [ ] Refactor to Approach B: per-variant data on `View` enum
      (e.g. `View::LocalDiff { lines, scroll }`) so unrelated state can't be
      accessed in the wrong view
- [ ] Cache diffs across menu re-entries instead of re-shelling git each time
- [ ] Diff against arbitrary refs, not just `origin/main`
- [ ] Real Pulls implementation (via `gh` CLI or GitHub API)

## File navigation

- [ ] Run `git diff --name-only` to get the list of changed files
- [ ] Split the body horizontally: file list on the left, diff on the right
- [ ] Render the file list using the `List` widget
- [ ] Make the file list selectable with `ListState` (up/down keys)
- [ ] Show the diff only for the currently selected file
- [ ] Cache per-file diffs so they don't get re-fetched on every selection change

## Polish

- [ ] Handle the case where the current directory is not a git repository
- [ ] Handle the case where there are no changes (empty diff)
- [ ] Add horizontal scrolling for long lines
- [ ] Fill `+`/`-` line background to viewport edge (pad lines to width at
      render time so the bg color spans the full row)
- [ ] Add a line-number gutter (parse hunk headers `@@ -X,Y +A,B @@` and
      track old/new line counters per hunk)
- [ ] Add a two-char status column (`--`, `+-`, `+`) next to the gutter
- [ ] Show current branch name in the header
- [ ] Show staged vs. unstaged indicator per file

## Future ideas

- [ ] Stage / unstage files from the UI
- [ ] View commit log in a separate tab
- [ ] Show commit details and diff for a selected commit
- [ ] Neovim-style peek window for previewing files
- [ ] GitHub integration (PRs, issues)
- [ ] Search within diff
- [ ] Syntax highlighting for diff content (per file extension)

## Distribution

- [ ] Add screenshot / GIF to README
- [ ] Add installation instructions to README
- [ ] Add usage and keybindings table to README
- [ ] Publish to crates.io
