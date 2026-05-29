<script lang="ts">
  import { appState, diffStats } from '../stores/app';

  let state = $derived($appState);
  let stats = $derived($diffStats);
  let activeFile = $derived(state.changedFiles[state.activeFileIdx]?.path ?? '—');
</script>

<div id="statusbar">
  <div class="sb-item"><div class="sb-dot" style="background:var(--green)"></div> {stats.added} additions</div>
  <div class="sb-item"><div class="sb-dot" style="background:var(--red)"></div> {stats.deleted} deletions</div>
  <div class="sb-item"><div class="sb-dot" style="background:var(--amber)"></div> {stats.modified} modifications</div>
  <div class="sb-spacer"></div>
  <div class="sb-item"><i class="ti ti-file-text" style="font-size:11px"></i> {activeFile}</div>
  <div class="sb-item"><i class="ti ti-git-branch" style="font-size:11px"></i> {state.leftBranch || '—'} &lrarr; {state.rightBranch || '—'}</div>
</div>

<style>
  #statusbar {
    height: var(--statusbar-h);
    flex-shrink: 0;
    background: var(--bg2);
    border-top: 1px solid var(--border);
    display: flex;
    align-items: center;
    padding: 0 12px;
    gap: 16px;
    font-size: 11px;
    color: var(--text3);
  }
  .sb-item { display: flex; align-items: center; gap: 4px; }
  .sb-dot { width: 6px; height: 6px; border-radius: 50%; }
  .sb-spacer { flex: 1; }
</style>
