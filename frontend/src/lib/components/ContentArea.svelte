<script lang="ts">
  import { appState } from '../stores/app';
  import { onMount } from 'svelte';

  let state = $derived($appState);
  let panelLeft: HTMLElement;
  let panelRight: HTMLElement;
  let syncing = false;

  function setupScrollSync() {
    if (!panelLeft || !panelRight) return;

    panelLeft.onscroll = () => {
      if (!state.syncScroll || syncing) return;
      syncing = true;
      const ratio = panelLeft.scrollTop / (panelLeft.scrollHeight - panelLeft.clientHeight || 1);
      panelRight.scrollTop = ratio * (panelRight.scrollHeight - panelRight.clientHeight);
      requestAnimationFrame(() => syncing = false);
    };

    panelRight.onscroll = () => {
      if (!state.syncScroll || syncing) return;
      syncing = true;
      const ratio = panelRight.scrollTop / (panelRight.scrollHeight - panelRight.clientHeight || 1);
      panelLeft.scrollTop = ratio * (panelLeft.scrollHeight - panelLeft.clientHeight);
      requestAnimationFrame(() => syncing = false);
    };
  }

  $effect(() => {
    // Re-setup scroll sync when syncScroll changes
    state.syncScroll;
    setupScrollSync();
  });

  let activeFile = $derived(state.changedFiles[state.activeFileIdx] ?? null);
  let leftRef = $derived(() => {
    const b = state.branches.find(b => b.name === state.leftBranch);
    return b ? `${state.leftBranch} \u00b7 ${b.hash}` : state.leftBranch || '\u2014';
  });
  let rightRef = $derived(() => {
    const b = state.branches.find(b => b.name === state.rightBranch);
    return b ? `${state.rightBranch} \u00b7 ${b.hash}` : state.rightBranch || '\u2014';
  });

  let currentDiffIdx = $state(-1);

  function jumpDiff(dir: number) {
    if (!panelRight) return;
    const diffs = panelRight.querySelectorAll('.diff-wrap-add, .diff-wrap-del, .diff-wrap-mod');
    if (!diffs.length) return;
    currentDiffIdx = Math.max(0, Math.min(diffs.length - 1, currentDiffIdx + dir));
    const el = diffs[currentDiffIdx] as HTMLElement;
    el.scrollIntoView({ behavior: 'smooth', block: 'center' });
    el.style.outline = '2px solid var(--amber)';
    setTimeout(() => { el.style.outline = ''; }, 600);
  }

  // Listen for keyboard diff navigation
  function handleKeydown(e: KeyboardEvent) {
    const mod = e.metaKey || e.ctrlKey;
    if (mod && e.key === 'ArrowDown') { e.preventDefault(); jumpDiff(1); }
    if (mod && e.key === 'ArrowUp') { e.preventDefault(); jumpDiff(-1); }
  }

  onMount(() => {
    document.addEventListener('keydown', handleKeydown);
    return () => document.removeEventListener('keydown', handleKeydown);
  });
</script>

<div id="content">
  <div id="panel-header">
    <div class="ph-side">
      <i class="ti ti-git-commit" style="font-size:12px;color:var(--text3)"></i>
      <span class="commit-tag ct-l">{leftRef()}</span>
      <div class="ph-nav">
        <button class="nav-arrow" onclick={() => jumpDiff(-1)}>
          <i class="ti ti-chevron-up"></i>
        </button>
        <button class="nav-arrow" onclick={() => jumpDiff(1)}>
          <i class="ti ti-chevron-down"></i>
        </button>
      </div>
    </div>
    <div class="ph-side">
      <i class="ti ti-git-commit" style="font-size:12px;color:var(--text3)"></i>
      <span class="commit-tag ct-r">{rightRef()}</span>
    </div>
  </div>
  <div id="panels">
    {#if state.viewMode !== 'preview' && state.viewMode !== 'unified'}
      <div class="panel" bind:this={panelLeft}>
        {#if state.leftExists}
          {@html state.leftContent}
        {:else if activeFile}
          <div class="empty-panel">
            <i class="ti ti-file-off"></i>
            <p>File does not exist in <strong>{state.leftBranch}</strong></p>
          </div>
        {:else}
          <div class="empty-panel">
            <i class="ti ti-file-text"></i>
            <p>Select a file to view</p>
          </div>
        {/if}
      </div>
    {/if}
    <div class="panel" bind:this={panelRight}>
      {#if state.rightExists}
        {@html state.rightContent}
      {:else if activeFile}
        <div class="empty-panel">
          <i class="ti ti-file-off"></i>
          <p>File does not exist in <strong>{state.rightBranch}</strong></p>
        </div>
      {:else}
        <div class="empty-panel">
          <i class="ti ti-file-text"></i>
          <p>Select a file to view</p>
        </div>
      {/if}
    </div>
  </div>
</div>

<style>
  #content { flex: 1; display: flex; flex-direction: column; overflow: hidden; min-width: 0; }
  #panel-header {
    height: 34px; display: flex; flex-shrink: 0;
    background: var(--bg2); border-bottom: 1px solid var(--border);
  }
  .ph-side {
    flex: 1; display: flex; align-items: center; padding: 0 14px; gap: 8px; font-size: 11px;
  }
  .ph-side:first-child { border-right: 1px solid var(--border); }
  .commit-tag {
    padding: 2px 8px; border-radius: 10px; font-size: 10px; font-weight: 500;
    font-family: var(--mono);
  }
  .ct-l { background: var(--accent-dim); color: #a5b4fc; border: 1px solid var(--accent-border); }
  .ct-r { background: var(--teal-dim); color: #5eead4; border: 1px solid rgba(20,184,166,.3); }
  .ph-nav { display: flex; gap: 2px; margin-left: auto; }
  .nav-arrow {
    width: 22px; height: 22px; border: 1px solid var(--border); border-radius: var(--radius);
    background: transparent; color: var(--text3); display: flex; align-items: center; justify-content: center;
    font-size: 12px; cursor: pointer; transition: all .1s;
  }
  .nav-arrow:hover { background: var(--bg4); color: var(--text); }
  #panels { display: flex; flex: 1; overflow: hidden; }
  .panel {
    flex: 1; overflow-y: auto; overflow-x: hidden;
    background: var(--bg); min-width: 0;
  }
  .panel:first-child { border-right: 1px solid var(--border); }
  .empty-panel {
    display: flex; flex-direction: column; align-items: center; justify-content: center;
    min-height: 300px; color: var(--text3); gap: 12px;
  }
  .empty-panel i { font-size: 40px; opacity: .4; }
  .empty-panel p { font-size: 13px; }
  .empty-panel strong { color: var(--text2); }
</style>
