<script lang="ts">
  import { appState } from '../stores/app';
  import { invoke } from '@tauri-apps/api/core';
  import { open } from '@tauri-apps/plugin-dialog';

  let { branchModalOpen = $bindable(), settingsOpen = $bindable(), onswap }: {
    branchModalOpen: boolean;
    settingsOpen: boolean;
    onswap: () => void;
  } = $props();

  let state = $derived($appState);

  async function openRepo() {
    const selected = await open({ directory: true, multiple: false });
    if (selected) {
      try {
        const branches = await invoke('open_repository', { path: selected });
        appState.update(s => {
          s.repoPath = selected as string;
          s.branches = branches as any[];
          // auto-select head branch as left
          const head = (branches as any[]).find(b => b.is_head);
          if (head) s.leftBranch = head.name;
          return s;
        });
      } catch (e) {
        console.error('Failed to open repo:', e);
      }
    }
  }

  function setLayout(layout: 'split' | 'unified') {
    appState.update(s => ({ ...s, layout }));
  }

  function toggleSource() {
    appState.update(s => ({ ...s, showSource: !s.showSource }));
  }

  function toggleHighlight() {
    appState.update(s => ({ ...s, highlight: !s.highlight }));
  }

  function toggleSync() {
    appState.update(s => ({ ...s, syncScroll: !s.syncScroll }));
  }
</script>

<div id="toolbar">
  <div class="tb-group">
    <button class="tb-btn" onclick={openRepo}>
      <i class="ti ti-folder-open"></i><span>Open repo</span>
    </button>
  </div>
  <div class="tb-sep"></div>

  <!-- Branch pickers -->
  <button class="branch-pill" onclick={() => branchModalOpen = true}>
    <div class="branch-dot bd-l"></div>
    <i class="ti ti-git-branch"></i>
    <span>{state.leftBranch || '—'}</span>
    <i class="ti ti-chevron-down"></i>
  </button>
  <button class="swap-btn" onclick={onswap} title="Swap branches">⇄</button>
  <button class="branch-pill" onclick={() => branchModalOpen = true}>
    <div class="branch-dot bd-r"></div>
    <i class="ti ti-git-branch"></i>
    <span>{state.rightBranch || '—'}</span>
    <i class="ti ti-chevron-down"></i>
  </button>
  <div class="tb-sep"></div>

  <!-- View modes -->
  <div class="view-mode-group">
    <button class="tb-btn" class:active={state.layout === 'split'} onclick={() => setLayout('split')}>
      <i class="ti ti-layout-columns"></i> Split
    </button>
    <button class="tb-btn" class:active={state.layout === 'unified'} onclick={() => setLayout('unified')}>
      <i class="ti ti-file-diff"></i> Unified
    </button>
  </div>
  <div class="tb-sep"></div>

  <button class="tb-btn" class:active={state.showSource} onclick={toggleSource}>
    <i class="ti ti-code"></i> Source
  </button>
  <div class="tb-sep"></div>

  <button class="tb-btn" class:active={state.highlight} onclick={toggleHighlight}>
    <i class="ti ti-highlight"></i> Highlight
  </button>
  <button class="tb-btn" class:active={state.syncScroll} onclick={toggleSync}>
    <i class="ti ti-arrows-right-left"></i> Sync scroll
  </button>

  <div class="tb-spacer"></div>

  <button class="tb-btn" onclick={() => settingsOpen = true}>
    <i class="ti ti-settings"></i>
  </button>
</div>

<style>
  #toolbar {
    height: var(--toolbar-h);
    flex-shrink: 0;
    background: var(--bg2);
    border-bottom: 1px solid var(--border);
    display: flex;
    align-items: center;
    gap: 4px;
    padding: 0 10px;
  }
  .tb-group { display: flex; gap: 3px; align-items: center; }
  .tb-sep { width: 1px; height: 22px; background: var(--border); margin: 0 5px; flex-shrink: 0; }
  .tb-btn {
    height: 28px; padding: 0 9px; border: 1px solid transparent;
    border-radius: var(--radius); background: transparent; color: var(--text2);
    font-size: 12px; display: flex; align-items: center; gap: 5px; white-space: nowrap;
    transition: all .15s;
  }
  .tb-btn:hover { background: var(--bg4); border-color: var(--border); color: var(--text); }
  .tb-btn.active { background: var(--bg4); border-color: var(--border2); color: var(--text); }
  .tb-btn i { font-size: 14px; }
  .tb-spacer { flex: 1; }
  .branch-pill {
    display: flex; align-items: center; gap: 6px; height: 28px; padding: 0 10px;
    border: 1px solid var(--border2); border-radius: 14px; background: var(--bg3);
    font-size: 11px; font-weight: 500; color: var(--text); cursor: pointer;
    transition: all .15s;
  }
  .branch-pill:hover { border-color: var(--border2); background: var(--bg4); }
  .branch-dot { width: 7px; height: 7px; border-radius: 50%; flex-shrink: 0; }
  .bd-l { background: var(--accent); }
  .bd-r { background: var(--teal); }
  .branch-pill i { font-size: 13px; color: var(--text3); }
  .swap-btn {
    width: 28px; height: 28px; border: 1px solid var(--border); border-radius: var(--radius);
    background: transparent; color: var(--text3); display: flex; align-items: center; justify-content: center;
    transition: all .15s; font-size: 16px;
  }
  .swap-btn:hover { background: var(--bg4); color: var(--text); }
  .view-mode-group { display: flex; gap: 2px; }
</style>
