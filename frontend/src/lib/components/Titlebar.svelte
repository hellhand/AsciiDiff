<script lang="ts">
  import { appState } from '../stores/app';

  let state = $derived($appState);
  let filename = $derived(
    state.changedFiles[state.activeFileIdx]?.path ?? 'No file selected'
  );
</script>

<div id="titlebar">
  <span class="app-name">AsciiDiff</span>
  <span class="sep">&rsaquo;</span>
  <span class="file-name">{filename}</span>
</div>

<style>
  #titlebar {
    height: var(--titlebar-h);
    flex-shrink: 0;
    background: var(--bg2);
    border-bottom: 1px solid var(--border);
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 0 14px;
    user-select: none;
  }
  .app-name { font-size: 12px; font-weight: 500; color: var(--text2); letter-spacing: .02em; }
  .file-name { font-size: 12px; color: var(--text3); }
  .sep { color: var(--text3); font-size: 10px; }
</style>
