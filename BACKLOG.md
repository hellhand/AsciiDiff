# AsciiDiff - Follow-up Backlog

## High Priority

### 1. Improve AsciiDoc renderer fidelity
- Handle nested lists, definition lists, and complex tables
- Support block-level admonitions (multi-line NOTE/WARNING/TIP/IMPORTANT)
- Add image rendering support (show image dimensions/placeholders)
- Handle conditional directives (ifdef/ifndef)

### 2. Implement inline word-level diff highlighting
- Current diff marks entire lines; mock shows word-level `inline-add`/`inline-del` spans
- Use `similar` crate's word-level diff to produce precise inline markers
- Handle modifications showing both old (strikethrough) and new (highlighted) text

### 3. Implement "Collapse unchanged" functionality with backend support
- Backend should return diff regions with context markers
- Frontend should show collapsible sections with "N unchanged lines" indicator
- Respect the "Context lines" setting from preferences

### 4. Persist and apply settings
- Save settings to a JSON file via Tauri (fs plugin)
- Load settings on startup
- Apply settings reactively (font size, sidebar width, theme, etc.)

### 5. Handle include:: directive resolution from git
- Read included files from git object store at the specified ref
- Expand includes before rendering/diffing
- Show resolved includes in the rendered output

## Medium Priority

### 6. Add file watcher for auto-refresh
- Watch git HEAD for changes
- Auto-refresh the file list and diff when branches change
- Show notification in statusbar

### 7. Implement unified diff view
- Currently unified mode just shows the right panel
- Should show a proper unified diff with interleaved additions/deletions

### 8. Add commit metadata display
- Show author, date, commit message in panel headers
- Add "N commits ahead" info in branch modal
- Show file-level stats (lines added/removed per file)

### 9. Add syntax highlighting for code blocks
- Integrate a syntax highlighter (tree-sitter or highlight.js via wasm)
- Respect the "Highlighter" and "Theme" settings

### 10. Implement theme system
- Add light theme CSS variables
- Add system theme detection
- Persist theme preference

## Low Priority

### 11. Add drag-to-resize for sidebar
- Allow user to resize sidebar by dragging the border
- Persist width preference

### 12. Add file search/filter in sidebar
- Quick filter input above the file tree
- Fuzzy match on file paths

### 13. Export diff as HTML/PDF
- Allow exporting the current diff view as a standalone HTML file
- Optional PDF export via print dialog

### 14. Add custom keybinding configuration
- Allow users to rebind keyboard shortcuts
- Store in settings file
