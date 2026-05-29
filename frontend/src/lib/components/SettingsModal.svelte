<script lang="ts">
  let { onclose }: { onclose: () => void } = $props();

  let activeTab = $state('rendering');

  const tabs = [
    { id: 'rendering', icon: 'ti-eye', label: 'Rendering' },
    { id: 'includes', icon: 'ti-file-symlink', label: 'Includes' },
    { id: 'git', icon: 'ti-git-branch', label: 'Git' },
    { id: 'syntax', icon: 'ti-code', label: 'Syntax highlight' },
    { id: 'diff', icon: 'ti-layout-columns', label: 'Diff display' },
    { id: 'keys', icon: 'ti-keyboard', label: 'Keybindings' },
    { id: 'appearance', icon: 'ti-palette', label: 'Appearance' },
  ];

  function handleBackdrop(e: MouseEvent) {
    if (e.target === e.currentTarget) onclose();
  }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="settings-overlay" onclick={handleBackdrop}>
  <div class="settings-modal">
    <div class="settings-nav">
      <div class="settings-nav-title">Preferences</div>
      {#each tabs as tab}
        <button class="sn-item" class:active={activeTab === tab.id} onclick={() => activeTab = tab.id}>
          <i class="ti {tab.icon}"></i> {tab.label}
        </button>
      {/each}
    </div>
    <div class="settings-content">
      {#if activeTab === 'rendering'}
        <div class="s-section">
          <div class="s-section-title">AsciiDoc rendering</div>
          <div class="s-row">
            <div class="s-label"><p>Backend</p><span>Rendering engine for HTML output</span></div>
            <select class="s-select"><option>Asciidoctor.js (browser)</option><option>Asciidoctor (native)</option></select>
          </div>
          <div class="s-row">
            <div class="s-label"><p>Safe mode</p><span>Controls which features are allowed</span></div>
            <select class="s-select"><option>unsafe</option><option>safe</option><option>server</option><option>secure</option></select>
          </div>
          <div class="s-row">
            <div class="s-label"><p>Resolve include:: directives</p><span>Follow include paths relative to repo root</span></div>
            <div class="toggle on"><div class="toggle-knob"></div></div>
          </div>
          <div class="s-row">
            <div class="s-label"><p>Evaluate ifdef:: conditionals</p><span>Process preprocessor conditionals before diffing</span></div>
            <div class="toggle on"><div class="toggle-knob"></div></div>
          </div>
        </div>
      {:else if activeTab === 'includes'}
        <div class="s-section">
          <div class="s-section-title">Include resolution</div>
          <div class="s-row">
            <div class="s-label"><p>Base directory</p><span>Root path used to resolve include:: paths</span></div>
            <input type="text" class="s-input" value="./docs">
          </div>
          <div class="s-row">
            <div class="s-label"><p>Resolve from git object store</p><span>Read include targets from the git history, not disk</span></div>
            <div class="toggle on"><div class="toggle-knob"></div></div>
          </div>
          <div class="s-row">
            <div class="s-label"><p>Show unresolved includes</p><span>Display a warning banner for broken include paths</span></div>
            <div class="toggle on"><div class="toggle-knob"></div></div>
          </div>
          <div class="s-row">
            <div class="s-label"><p>Max include depth</p><span>Prevent infinite include loops</span></div>
            <select class="s-select"><option>4</option><option>8</option><option>16</option><option>Unlimited</option></select>
          </div>
        </div>
      {:else if activeTab === 'git'}
        <div class="s-section">
          <div class="s-section-title">Git integration</div>
          <div class="s-row">
            <div class="s-label"><p>Default base branch</p><span>Branch to compare against by default</span></div>
            <input type="text" class="s-input" value="main">
          </div>
          <div class="s-row">
            <div class="s-label"><p>Auto-refresh on branch change</p><span>Re-render when the git HEAD changes</span></div>
            <div class="toggle on"><div class="toggle-knob"></div></div>
          </div>
          <div class="s-row">
            <div class="s-label"><p>Show commit metadata</p><span>Display author, date, and message in the header</span></div>
            <div class="toggle on"><div class="toggle-knob"></div></div>
          </div>
        </div>
      {:else if activeTab === 'syntax'}
        <div class="s-section">
          <div class="s-section-title">Syntax highlighting</div>
          <div class="s-row">
            <div class="s-label"><p>Highlighter</p><span>Library used for code block coloring</span></div>
            <select class="s-select"><option>Rouge (built-in)</option><option>highlight.js</option><option>Prism</option><option>None</option></select>
          </div>
          <div class="s-row">
            <div class="s-label"><p>Theme</p><span>Color theme for code blocks</span></div>
            <select class="s-select"><option>Monokai</option><option>GitHub Dark</option><option>One Dark</option><option>Dracula</option></select>
          </div>
          <div class="s-row">
            <div class="s-label"><p>Show line numbers</p><span>Prefix code lines with their line number</span></div>
            <div class="toggle"><div class="toggle-knob"></div></div>
          </div>
          <div class="s-row">
            <div class="s-label"><p>Word-wrap in code blocks</p><span>Wrap long lines instead of scrolling</span></div>
            <div class="toggle"><div class="toggle-knob"></div></div>
          </div>
        </div>
      {:else if activeTab === 'diff'}
        <div class="s-section">
          <div class="s-section-title">Diff display</div>
          <div class="s-row">
            <div class="s-label"><p>Granularity</p><span>How precisely inline changes are highlighted</span></div>
            <select class="s-select"><option>Word-level</option><option>Character-level</option><option>Block-level</option></select>
          </div>
          <div class="s-row">
            <div class="s-label"><p>Ignore whitespace</p><span>Treat whitespace-only changes as equal</span></div>
            <div class="toggle on"><div class="toggle-knob"></div></div>
          </div>
          <div class="s-row">
            <div class="s-label"><p>Context lines</p><span>Unchanged lines shown around each change</span></div>
            <select class="s-select"><option>3</option><option>5</option><option>10</option><option>All</option></select>
          </div>
          <div class="s-row">
            <div class="s-label"><p>Synchronized scrolling</p><span>Keep both panels at the same scroll position</span></div>
            <div class="toggle on"><div class="toggle-knob"></div></div>
          </div>
        </div>
      {:else if activeTab === 'keys'}
        <div class="s-section">
          <div class="s-section-title">Keyboard shortcuts</div>
          <div class="shortcuts-grid">
            <div class="shortcut-row"><span>Next diff</span><span><span class="kbd">Ctrl</span> <span class="kbd">&darr;</span></span></div>
            <div class="shortcut-row"><span>Previous diff</span><span><span class="kbd">Ctrl</span> <span class="kbd">&uarr;</span></span></div>
            <div class="shortcut-row"><span>Toggle view mode</span><span><span class="kbd">Ctrl</span> <span class="kbd">\</span></span></div>
            <div class="shortcut-row"><span>Open branch picker</span><span><span class="kbd">Ctrl</span> <span class="kbd">B</span></span></div>
            <div class="shortcut-row"><span>Swap branches</span><span><span class="kbd">Ctrl</span> <span class="kbd">S</span></span></div>
            <div class="shortcut-row"><span>Toggle highlight</span><span><span class="kbd">Ctrl</span> <span class="kbd">H</span></span></div>
            <div class="shortcut-row"><span>Collapse unchanged</span><span><span class="kbd">Ctrl</span> <span class="kbd">U</span></span></div>
            <div class="shortcut-row"><span>Toggle sidebar</span><span><span class="kbd">Ctrl</span> <span class="kbd">E</span></span></div>
            <div class="shortcut-row"><span>Settings</span><span><span class="kbd">Ctrl</span> <span class="kbd">,</span></span></div>
            <div class="shortcut-row"><span>Close modal</span><span><span class="kbd">Esc</span></span></div>
          </div>
        </div>
      {:else if activeTab === 'appearance'}
        <div class="s-section">
          <div class="s-section-title">Appearance</div>
          <div class="s-row">
            <div class="s-label"><p>Theme</p><span>Application color theme</span></div>
            <select class="s-select"><option>Dark (default)</option><option>Light</option><option>System</option></select>
          </div>
          <div class="s-row">
            <div class="s-label"><p>Font size</p><span>Base font size for rendered documents</span></div>
            <select class="s-select"><option>12px</option><option selected>13px</option><option>14px</option><option>15px</option></select>
          </div>
          <div class="s-row">
            <div class="s-label"><p>Line height</p><span>Spacing between lines in rendered text</span></div>
            <select class="s-select"><option>1.5</option><option selected>1.75</option><option>2.0</option></select>
          </div>
          <div class="s-row">
            <div class="s-label"><p>Sidebar width</p><span>Width of the file tree sidebar in pixels</span></div>
            <select class="s-select"><option>180px</option><option selected>220px</option><option>260px</option></select>
          </div>
        </div>
      {/if}
    </div>
  </div>
</div>

<style>
  .settings-overlay {
    position: fixed; inset: 0; background: rgba(0,0,0,.6); backdrop-filter: blur(4px);
    display: flex; align-items: center; justify-content: center; z-index: 1000;
  }
  .settings-modal {
    width: 620px; height: 480px; background: var(--bg2); border: 1px solid var(--border2);
    border-radius: var(--radius-lg); overflow: hidden; display: flex;
  }
  .settings-nav { width: 168px; background: var(--bg3); border-right: 1px solid var(--border); padding: 10px 0; flex-shrink: 0; }
  .settings-nav-title { font-size: 9px; font-weight: 600; letter-spacing: .1em; text-transform: uppercase; color: var(--text3); padding: 4px 12px 8px; }
  .sn-item {
    display: flex; align-items: center; gap: 7px; padding: 7px 12px; cursor: pointer;
    font-size: 12px; color: var(--text2); border-left: 2px solid transparent; transition: all .1s;
    width: 100%; text-align: left;
  }
  .sn-item:hover { background: var(--bg4); color: var(--text); }
  .sn-item.active { color: var(--text); border-left-color: var(--accent); background: var(--bg4); }
  .sn-item i { font-size: 14px; }
  .settings-content { flex: 1; overflow-y: auto; padding: 20px 22px; }
  .s-section { margin-bottom: 22px; }
  .s-section-title { font-size: 12px; font-weight: 600; color: var(--text); margin-bottom: 12px; padding-bottom: 6px; border-bottom: 1px solid var(--border); }
  .s-row { display: flex; align-items: flex-start; justify-content: space-between; gap: 16px; margin-bottom: 12px; }
  .s-label p { font-size: 12px; font-weight: 500; color: var(--text); margin-bottom: 2px; }
  .s-label span { font-size: 11px; color: var(--text3); line-height: 1.4; }
  .s-select { height: 28px; padding: 0 8px; background: var(--bg3); border: 1px solid var(--border2); border-radius: var(--radius); color: var(--text); font-size: 12px; min-width: 160px; outline: none; cursor: pointer; }
  .s-select:focus { border-color: var(--accent); }
  .s-input { height: 28px; padding: 0 8px; background: var(--bg3); border: 1px solid var(--border2); border-radius: var(--radius); color: var(--text); font-family: var(--mono); font-size: 11px; width: 200px; outline: none; }
  .toggle { width: 34px; height: 18px; border-radius: 9px; background: var(--bg4); border: 1px solid var(--border2); position: relative; cursor: pointer; flex-shrink: 0; margin-top: 2px; transition: background .2s; }
  .toggle.on { background: var(--accent); border-color: var(--accent); }
  .toggle-knob { width: 12px; height: 12px; border-radius: 50%; background: var(--text3); position: absolute; top: 2px; left: 2px; transition: left .2s, background .2s; }
  .toggle.on .toggle-knob { left: 18px; background: #fff; }
  .shortcuts-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 0; }
  .shortcut-row { display: flex; justify-content: space-between; align-items: center; padding: 6px 0; border-bottom: 1px solid var(--border); font-size: 12px; }
  .shortcut-row:last-child { border-bottom: none; }
  .shortcut-row span { color: var(--text2); }
  .kbd { display: inline-block; font-family: var(--mono); font-size: 10px; background: var(--bg3); border: 1px solid var(--border2); border-radius: 3px; padding: 1px 5px; color: var(--text2); }
</style>
