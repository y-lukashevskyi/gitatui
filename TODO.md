# TODO

## Current progress

- [x] Run `git diff` and display output in a TUI
- [x] Split screen into header and body with borders
- [x] Parse diff into a `Vec<Line>` once at startup
- [x] Colorize diff lines (`+` green, `-` red, `@@` cyan, `diff --git` yellow/bold)

## Diff viewer

- [ ] Add scrolling (`j`/`k` and arrow keys, `q` to quit)
- [ ] Add a status bar at the bottom with keybinding hints
- [x] Fix line styling order so `+++`/`---` and `diff --git` headers don't get
      miscolored as additions/deletions
- [ ] Clamp scroll position to the actual diff length
- [ ] Refactor: extract state into an `App` struct
- [ ] Refactor: move line-styling logic into its own function

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
