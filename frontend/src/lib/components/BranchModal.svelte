<script lang="ts">
  import { appState } from '../stores/app';
  import { invoke } from '@tauri-apps/api/core';

  let { onclose }: { onclose: () => void } = $props();

  let state = $derived($appState);
  let tempLeft = $state(state.leftBranch);
  let tempRight = $state(state.rightBranch);
  let filterLeft = $state('');
  let filterRight = $state('');

  let branches = $derived(state.branches.filter(b => b.ref_type === 'branch'));
  let tags = $derived(state.branches.filter(b => b.ref_type === 'tag'));

  let filteredBranchesLeft = $derived(branches.filter(b => b.name.includes(filterLeft)));
  let filteredTagsLeft = $derived(tags.filter(b => b.name.includes(filterLeft)));
  let filteredBranchesRight = $derived(branches.filter(b => b.name.includes(filterRight)));
  let filteredTagsRight = $derived(tags.filter(b => b.name.includes(filterRight)));

  let leftHash = $derived(state.branches.find(b => b.name === tempLeft)?.hash ?? '—');
  let rightHash = $derived(state.branches.find(b => b.name === tempRight)?.hash ?? '—');

  async function apply() {
    appState.update(s => {
      s.leftBranch = tempLeft;
      s.rightBranch = tempRight;
      return s;
    });

    // Fetch changed files
    const currentState = $appState;
    if (currentState.repoPath && tempLeft && tempRight) {
      try {
        const files: any[] = await invoke('list_changed_files', {
          repoPath: currentState.repoPath,
          leftRef: tempLeft,
          rightRef: tempRight,
        });
        appState.update(s => {
          s.changedFiles = files;
          s.activeFileIdx = 0;
          return s;
        });

        // Auto-render the first file
        if (files.length > 0) {
          const result: any = await invoke('render_diff', {
            repoPath: currentState.repoPath,
            leftRef: tempLeft,
            rightRef: tempRight,
            filePath: files[0].path,
          });
          appState.update(s => ({
            ...s,
            leftContent: result.left_html,
            rightContent: result.right_html,
            leftSource: result.left_source,
            rightSource: result.right_source,
            leftExists: result.left_exists,
            rightExists: result.right_exists,
          }));
        }
      } catch (e) {
        console.error('Failed to list changed files:', e);
      }
    }
    onclose();
  }

  function handleBackdrop(e: MouseEvent) {
    if (e.target === e.currentTarget) onclose();
  }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="modal-overlay open" onclick={handleBackdrop}>
  <div class="modal">
    <div class="modal-head">
      <h2><i class="ti ti-git-compare" style="font-size:14px;margin-right:6px;vertical-align:-2px"></i>Select branches to compare</h2>
      <button class="modal-close" onclick={onclose}><i class="ti ti-x" style="font-size:12px"></i></button>
    </div>
    <div class="modal-body">
      <div class="modal-cols">
        <div>
          <div class="col-label"><div class="branch-dot bd-l"></div> Base (left panel)</div>
          <div class="search-wrap">
            <i class="ti ti-search"></i>
            <input type="text" placeholder="Filter branches..." bind:value={filterLeft}>
          </div>
          <div class="branch-list">
            {#if filteredBranchesLeft.length}
              <div class="bl-section">Branches</div>
              {#each filteredBranchesLeft as b}
                <button class="bl-item" class:sel-l={b.name === tempLeft} onclick={() => tempLeft = b.name}>
                  <div class="check-icon" class:ci-l={b.name === tempLeft}>{b.name === tempLeft ? '✓' : ''}</div>
                  <span class="bl-name">{b.name}</span>
                  <span class="bl-hash">{b.hash}</span>
                </button>
              {/each}
            {/if}
            {#if filteredTagsLeft.length}
              <div class="bl-section">Tags</div>
              {#each filteredTagsLeft as b}
                <button class="bl-item" class:sel-l={b.name === tempLeft} onclick={() => tempLeft = b.name}>
                  <div class="check-icon" class:ci-l={b.name === tempLeft}>{b.name === tempLeft ? '✓' : ''}</div>
                  <span class="bl-name">{b.name}</span>
                  <span class="bl-hash">{b.hash}</span>
                </button>
              {/each}
            {/if}
          </div>
        </div>
        <div>
          <div class="col-label"><div class="branch-dot bd-r"></div> Compare (right panel)</div>
          <div class="search-wrap">
            <i class="ti ti-search"></i>
            <input type="text" placeholder="Filter branches..." bind:value={filterRight}>
          </div>
          <div class="branch-list">
            {#if filteredBranchesRight.length}
              <div class="bl-section">Branches</div>
              {#each filteredBranchesRight as b}
                <button class="bl-item" class:sel-r={b.name === tempRight} onclick={() => tempRight = b.name}>
                  <div class="check-icon" class:ci-r={b.name === tempRight}>{b.name === tempRight ? '✓' : ''}</div>
                  <span class="bl-name">{b.name}</span>
                  <span class="bl-hash">{b.hash}</span>
                </button>
              {/each}
            {/if}
            {#if filteredTagsRight.length}
              <div class="bl-section">Tags</div>
              {#each filteredTagsRight as b}
                <button class="bl-item" class:sel-r={b.name === tempRight} onclick={() => tempRight = b.name}>
                  <div class="check-icon" class:ci-r={b.name === tempRight}>{b.name === tempRight ? '✓' : ''}</div>
                  <span class="bl-name">{b.name}</span>
                  <span class="bl-hash">{b.hash}</span>
                </button>
              {/each}
            {/if}
          </div>
        </div>
      </div>
      <div class="commit-summary">
        <div class="commit-summary-title">Commit range</div>
        <div class="cs-chips">
          <span class="cs-chip cs-l">{leftHash}</span>
          <i class="ti ti-arrow-right" style="font-size:12px"></i>
          <span class="cs-chip cs-r">{rightHash}</span>
        </div>
      </div>
    </div>
    <div class="modal-footer">
      <div class="files-note"><i class="ti ti-files" style="font-size:12px"></i> Select branches to see changed files</div>
      <div style="display:flex;gap:8px">
        <button class="btn-ghost" onclick={onclose}>Cancel</button>
        <button class="btn-primary" onclick={apply}>Compare branches</button>
      </div>
    </div>
  </div>
</div>

<style>
  .modal-overlay {
    position: fixed; inset: 0; background: rgba(0,0,0,.6); backdrop-filter: blur(4px);
    display: flex; align-items: center; justify-content: center; z-index: 1000;
  }
  .modal {
    width: 540px; background: var(--bg2); border: 1px solid var(--border2);
    border-radius: var(--radius-lg); overflow: hidden;
  }
  .modal-head {
    padding: 16px 18px 12px; border-bottom: 1px solid var(--border);
    display: flex; align-items: center; justify-content: space-between;
  }
  .modal-head h2 { font-size: 14px; font-weight: 600; }
  .modal-close {
    width: 26px; height: 26px; border: 1px solid var(--border); border-radius: var(--radius);
    background: transparent; color: var(--text3); display: flex; align-items: center; justify-content: center;
    cursor: pointer; transition: all .1s;
  }
  .modal-close:hover { background: var(--bg4); color: var(--text); }
  .modal-body { padding: 16px 18px; }
  .modal-footer { padding: 12px 18px; border-top: 1px solid var(--border); display: flex; align-items: center; justify-content: space-between; gap: 10px; }
  .modal-cols { display: grid; grid-template-columns: 1fr 1fr; gap: 14px; margin-bottom: 14px; }
  .col-label { font-size: 10px; font-weight: 600; letter-spacing: .08em; text-transform: uppercase; color: var(--text3); margin-bottom: 7px; display: flex; align-items: center; gap: 5px; }
  .branch-dot { width: 7px; height: 7px; border-radius: 50%; }
  .bd-l { background: var(--accent); }
  .bd-r { background: var(--teal); }
  .search-wrap { position: relative; margin-bottom: 7px; }
  .search-wrap i { position: absolute; left: 8px; top: 50%; transform: translateY(-50%); font-size: 13px; color: var(--text3); pointer-events: none; }
  .search-wrap input {
    width: 100%; height: 30px; padding: 0 8px 0 28px;
    background: var(--bg3); border: 1px solid var(--border2);
    border-radius: var(--radius); color: var(--text); font-size: 12px; outline: none;
  }
  .search-wrap input:focus { border-color: var(--accent); }
  .branch-list { border: 1px solid var(--border); border-radius: var(--radius); overflow: hidden; max-height: 190px; overflow-y: auto; }
  .bl-section { padding: 4px 10px; font-size: 9px; font-weight: 600; letter-spacing: .1em; text-transform: uppercase; color: var(--text3); background: var(--bg3); border-bottom: 1px solid var(--border); }
  .bl-item {
    display: flex; align-items: center; gap: 7px; padding: 7px 10px; cursor: pointer;
    border-bottom: 1px solid var(--border); font-size: 12px; transition: background .1s;
    width: 100%; text-align: left;
  }
  .bl-item:last-child { border-bottom: none; }
  .bl-item:hover { background: var(--bg3); }
  .bl-item.sel-l { background: var(--accent-dim); }
  .bl-item.sel-r { background: var(--teal-dim); }
  .bl-name { flex: 1; font-family: var(--mono); font-size: 11px; color: var(--text); }
  .bl-hash { font-family: var(--mono); font-size: 10px; color: var(--text3); background: var(--bg4); padding: 1px 5px; border-radius: 3px; }
  .check-icon { width: 14px; height: 14px; border-radius: 50%; display: flex; align-items: center; justify-content: center; font-size: 9px; flex-shrink: 0; }
  .ci-l { background: var(--accent-dim); color: #a5b4fc; }
  .ci-r { background: var(--teal-dim); color: #5eead4; }
  .commit-summary { background: var(--bg3); border: 1px solid var(--border); border-radius: var(--radius); padding: 10px 12px; font-size: 11px; }
  .commit-summary-title { font-weight: 600; margin-bottom: 6px; color: var(--text2); font-size: 11px; }
  .cs-chips { display: flex; align-items: center; gap: 7px; color: var(--text3); }
  .cs-chip { font-family: var(--mono); font-size: 10px; padding: 2px 7px; border-radius: 8px; }
  .cs-l { background: var(--accent-dim); color: #a5b4fc; border: 1px solid var(--accent-border); }
  .cs-r { background: var(--teal-dim); color: #5eead4; border: 1px solid rgba(20,184,166,.3); }
  .btn-primary { height: 32px; padding: 0 18px; border: none; border-radius: var(--radius); background: var(--accent); color: #fff; font-size: 12px; font-weight: 600; cursor: pointer; transition: opacity .15s; }
  .btn-primary:hover { opacity: .9; }
  .btn-ghost { height: 32px; padding: 0 14px; border: 1px solid var(--border2); border-radius: var(--radius); background: transparent; color: var(--text2); font-size: 12px; cursor: pointer; transition: all .15s; }
  .btn-ghost:hover { background: var(--bg4); color: var(--text); }
  .files-note { font-size: 11px; color: var(--text3); display: flex; align-items: center; gap: 5px; }
</style>
