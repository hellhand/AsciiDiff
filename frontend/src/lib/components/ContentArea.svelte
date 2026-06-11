<script lang="ts">
  import { appState } from '../stores/app';
  import { onMount } from 'svelte';

  let state = $derived($appState);
  let panelLeft = $state<HTMLElement | null>(null);
  let panelRight = $state<HTMLElement | null>(null);
  let splitPercent = $state(50);
  let dragging = $state(false);
  let panelsEl: HTMLElement;

  function startDrag(e: MouseEvent) {
    e.preventDefault();
    dragging = true;

    function onMove(ev: MouseEvent) {
      if (!panelsEl) return;
      const rect = panelsEl.getBoundingClientRect();
      const pct = ((ev.clientX - rect.left) / rect.width) * 100;
      splitPercent = Math.max(20, Math.min(80, pct));
    }

    function onUp() {
      dragging = false;
      document.removeEventListener('mousemove', onMove);
      document.removeEventListener('mouseup', onUp);
    }

    document.addEventListener('mousemove', onMove);
    document.addEventListener('mouseup', onUp);
  }

  $effect(() => {
    const left = panelLeft;
    const right = panelRight;
    if (!left || !right) return;

    // Sync scroll using wheel events — these only fire on user input,
    // not on programmatic scrollTop changes, so no feedback loop is possible.
    function onWheelLeft(e: WheelEvent) {
      if (!state.syncScroll) return;
      right.scrollTop = left.scrollTop + e.deltaY;
    }

    function onWheelRight(e: WheelEvent) {
      if (!state.syncScroll) return;
      left.scrollTop = right.scrollTop + e.deltaY;
    }

    left.addEventListener('wheel', onWheelLeft, { passive: true });
    right.addEventListener('wheel', onWheelRight, { passive: true });

    return () => {
      left.removeEventListener('wheel', onWheelLeft);
      right.removeEventListener('wheel', onWheelRight);
    };
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
  <div id="panels" bind:this={panelsEl} class:dragging>
    {#if state.layout === 'split'}
      <div class="panel" bind:this={panelLeft} style="flex: 0 0 {splitPercent}%">
        {#if state.leftExists}
          {#if state.showSource}
            <pre class="source-view">{state.leftSource}</pre>
          {:else}
            {@html state.leftContent}
          {/if}
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
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div class="split-handle" onmousedown={startDrag}>
        <div class="split-handle-line"></div>
      </div>
    {/if}
    <div class="panel" bind:this={panelRight} style={state.layout === 'split' ? `flex: 1` : ''}>
      {#if state.rightExists}
        {#if state.showSource}
          <pre class="source-view">{state.rightSource}</pre>
        {:else}
          {@html state.rightContent}
        {/if}
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
  #panels.dragging { cursor: col-resize; user-select: none; }
  .panel {
    flex: 1; overflow-y: auto; overflow-x: hidden;
    background: var(--bg); min-width: 0;
  }
  .split-handle {
    width: 5px; flex-shrink: 0; cursor: col-resize;
    display: flex; align-items: center; justify-content: center;
    background: var(--bg2); border-left: 1px solid var(--border); border-right: 1px solid var(--border);
    transition: background .15s;
  }
  .split-handle:hover, #panels.dragging .split-handle {
    background: var(--bg4);
  }
  .split-handle-line {
    width: 1px; height: 32px; background: var(--border2); border-radius: 1px;
  }
  .empty-panel {
    display: flex; flex-direction: column; align-items: center; justify-content: center;
    min-height: 300px; color: var(--text3); gap: 12px;
  }
  .empty-panel i { font-size: 40px; opacity: .4; }
  .empty-panel p { font-size: 13px; }
  .empty-panel strong { color: var(--text2); }
  .source-view {
    padding: 20px 24px;
    font-family: var(--mono);
    font-size: var(--content-font-size);
    line-height: var(--content-line-height);
    color: var(--text2);
    white-space: pre-wrap;
    word-wrap: break-word;
    margin: 0;
    background: transparent;
  }
</style>
