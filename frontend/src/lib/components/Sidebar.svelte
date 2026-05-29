<script lang="ts">
  import { appState } from '../stores/app';
  import { invoke } from '@tauri-apps/api/core';

  let { collapsed }: { collapsed: boolean } = $props();
  let state = $derived($appState);

  async function selectFile(idx: number) {
    appState.update(s => ({ ...s, activeFileIdx: idx }));

    const currentState = $appState;
    const file = currentState.changedFiles[idx];
    if (!file || !currentState.repoPath) return;

    try {
      const result: any = await invoke('render_diff', {
        repoPath: currentState.repoPath,
        leftRef: currentState.leftBranch,
        rightRef: currentState.rightBranch,
        filePath: file.path,
      });
      appState.update(s => ({
        ...s,
        leftContent: result.left_html,
        rightContent: result.right_html,
        leftExists: result.left_exists,
        rightExists: result.right_exists,
      }));
    } catch (e) {
      console.error('Failed to render diff:', e);
    }
  }

  const iconMap: Record<string, string> = {
    added: 'ti-file-plus',
    deleted: 'ti-file-minus',
    modified: 'ti-file-text',
  };

  const colorMap: Record<string, string> = {
    added: 'var(--green)',
    deleted: 'var(--red)',
    modified: 'var(--text2)',
  };

  const badgeClass: Record<string, string> = {
    added: 'tb-add',
    deleted: 'tb-del',
    modified: 'tb-mod',
  };

  const badgeText: Record<string, string> = {
    added: 'new',
    deleted: 'del',
    modified: '~',
  };

  interface GroupedFiles {
    label: string;
    type: string;
    files: { path: string; status: string; idx: number }[];
  }

  let grouped = $derived((): GroupedFiles[] => {
    const groups: GroupedFiles[] = [];
    const order = ['modified', 'added', 'deleted'];
    const labels: Record<string, string> = { modified: 'Modified', added: 'Added', deleted: 'Deleted' };
    for (const type of order) {
      const files = state.changedFiles
        .map((f, i) => ({ ...f, idx: i }))
        .filter(f => f.status === type);
      if (files.length > 0) {
        groups.push({ label: labels[type], type, files });
      }
    }
    return groups;
  });
</script>

<div id="sidebar" class:collapsed>
  <div class="sidebar-head">
    <i class="ti ti-files" style="font-size:13px;color:var(--text3)"></i>
    <span>Changed files</span>
  </div>
  <div class="tree">
    {#each grouped() as group}
      <div class="tree-section">{group.label}</div>
      {#each group.files as file}
        <button
          class="tree-item"
          class:active={file.idx === state.activeFileIdx}
          onclick={() => selectFile(file.idx)}
        >
          <i class="ti {iconMap[file.status]}" style="color:{colorMap[file.status]};font-size:13px"></i>
          <span class="fname" class:strike={file.status === 'deleted'}>{file.path}</span>
          <span class="tree-badge {badgeClass[file.status]}">{badgeText[file.status]}</span>
        </button>
      {/each}
    {/each}
    {#if state.changedFiles.length === 0}
      <div class="empty-state">
        <i class="ti ti-git-compare" style="font-size:24px;color:var(--text3);opacity:.5"></i>
        <p>Open a repository and select branches to compare</p>
      </div>
    {/if}
  </div>
</div>

<style>
  #sidebar {
    width: var(--sidebar-w);
    flex-shrink: 0;
    background: var(--bg2);
    border-right: 1px solid var(--border);
    display: flex;
    flex-direction: column;
    overflow: hidden;
    transition: width .2s;
  }
  #sidebar.collapsed { width: 0; }
  .sidebar-head {
    height: 34px; display: flex; align-items: center; padding: 0 12px; gap: 6px;
    border-bottom: 1px solid var(--border); flex-shrink: 0;
  }
  .sidebar-head span { font-size: 10px; font-weight: 600; color: var(--text3); letter-spacing: .08em; text-transform: uppercase; }
  .tree { overflow-y: auto; flex: 1; padding: 4px 0; }
  .tree-section { padding: 8px 12px 3px; font-size: 9px; font-weight: 600; letter-spacing: .1em; text-transform: uppercase; color: var(--text3); }
  .tree-item {
    display: flex; align-items: center; gap: 7px; padding: 5px 12px; cursor: pointer;
    font-size: 12px; color: var(--text2); border-left: 2px solid transparent;
    transition: all .1s; width: 100%; text-align: left;
  }
  .tree-item:hover { background: var(--bg3); color: var(--text); }
  .tree-item.active { background: var(--bg3); color: var(--text); border-left-color: var(--accent); }
  .fname { flex: 1; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; font-family: var(--mono); font-size: 11px; }
  .fname.strike { text-decoration: line-through; opacity: .6; }
  .tree-badge { font-size: 9px; font-weight: 600; padding: 1px 5px; border-radius: 8px; flex-shrink: 0; }
  .tb-mod { background: var(--amber-dim); color: var(--amber); }
  .tb-add { background: var(--green-dim); color: var(--green); }
  .tb-del { background: var(--red-dim); color: var(--red); }
  .empty-state {
    display: flex; flex-direction: column; align-items: center; justify-content: center;
    padding: 32px 12px; gap: 8px; text-align: center;
  }
  .empty-state p { font-size: 11px; color: var(--text3); line-height: 1.5; }
</style>
