<script lang="ts">
  import { settings } from '../stores/settings';
  import Dropdown from './Dropdown.svelte';

  let { onclose }: { onclose: () => void } = $props();

  let activeTab = $state('rendering');
  let s = $derived($settings);

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

  function set(key: string, value: any) {
    settings.update({ [key]: value });
  }

  function toggleBool(key: string) {
    settings.update({ [key]: !(s as any)[key] });
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
            <Dropdown
              value={s.renderBackend}
              options={[
                { value: 'asciidoctor-js', label: 'Asciidoctor.js (browser)' },
                { value: 'asciidoctor-native', label: 'Asciidoctor (native)' },
              ]}
              onchange={(v) => set('renderBackend', v)}
            />
          </div>
          <div class="s-row">
            <div class="s-label"><p>Safe mode</p><span>Controls which features are allowed</span></div>
            <Dropdown
              value={s.safeMode}
              options={[
                { value: 'unsafe', label: 'unsafe' },
                { value: 'safe', label: 'safe' },
                { value: 'server', label: 'server' },
                { value: 'secure', label: 'secure' },
              ]}
              onchange={(v) => set('safeMode', v)}
            />
          </div>
          <div class="s-row">
            <div class="s-label"><p>Resolve include:: directives</p><span>Follow include paths relative to repo root</span></div>
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <div class="toggle" class:on={s.resolveIncludes} onclick={() => toggleBool('resolveIncludes')} onkeydown={() => {}}><div class="toggle-knob"></div></div>
          </div>
          <div class="s-row">
            <div class="s-label"><p>Evaluate ifdef:: conditionals</p><span>Process preprocessor conditionals before diffing</span></div>
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <div class="toggle" class:on={s.evaluateIfdefs} onclick={() => toggleBool('evaluateIfdefs')} onkeydown={() => {}}><div class="toggle-knob"></div></div>
          </div>
        </div>
      {:else if activeTab === 'includes'}
        <div class="s-section">
          <div class="s-section-title">Include resolution</div>
          <div class="s-row">
            <div class="s-label"><p>Base directory</p><span>Root path used to resolve include:: paths</span></div>
            <input type="text" class="s-input" value={s.includeBaseDir} onchange={(e) => set('includeBaseDir', e.currentTarget.value)}>
          </div>
          <div class="s-row">
            <div class="s-label"><p>Resolve from git object store</p><span>Read include targets from the git history, not disk</span></div>
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <div class="toggle" class:on={s.resolveFromGit} onclick={() => toggleBool('resolveFromGit')} onkeydown={() => {}}><div class="toggle-knob"></div></div>
          </div>
          <div class="s-row">
            <div class="s-label"><p>Show unresolved includes</p><span>Display a warning banner for broken include paths</span></div>
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <div class="toggle" class:on={s.showUnresolvedIncludes} onclick={() => toggleBool('showUnresolvedIncludes')} onkeydown={() => {}}><div class="toggle-knob"></div></div>
          </div>
          <div class="s-row">
            <div class="s-label"><p>Max include depth</p><span>Prevent infinite include loops</span></div>
            <Dropdown
              value={s.maxIncludeDepth === Infinity ? 'Infinity' : String(s.maxIncludeDepth)}
              options={[
                { value: '4', label: '4' },
                { value: '8', label: '8' },
                { value: '16', label: '16' },
                { value: 'Infinity', label: 'Unlimited' },
              ]}
              onchange={(v) => set('maxIncludeDepth', v === 'Infinity' ? Infinity : Number(v))}
            />
          </div>
        </div>
      {:else if activeTab === 'git'}
        <div class="s-section">
          <div class="s-section-title">Git integration</div>
          <div class="s-row">
            <div class="s-label"><p>Default base branch</p><span>Branch to compare against by default</span></div>
            <input type="text" class="s-input" value={s.defaultBaseBranch} onchange={(e) => set('defaultBaseBranch', e.currentTarget.value)}>
          </div>
          <div class="s-row">
            <div class="s-label"><p>Auto-refresh on branch change</p><span>Re-render when the git HEAD changes</span></div>
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <div class="toggle" class:on={s.autoRefreshOnBranchChange} onclick={() => toggleBool('autoRefreshOnBranchChange')} onkeydown={() => {}}><div class="toggle-knob"></div></div>
          </div>
          <div class="s-row">
            <div class="s-label"><p>Show commit metadata</p><span>Display author, date, and message in the header</span></div>
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <div class="toggle" class:on={s.showCommitMetadata} onclick={() => toggleBool('showCommitMetadata')} onkeydown={() => {}}><div class="toggle-knob"></div></div>
          </div>
        </div>
      {:else if activeTab === 'syntax'}
        <div class="s-section">
          <div class="s-section-title">Syntax highlighting</div>
          <div class="s-row">
            <div class="s-label"><p>Highlighter</p><span>Library used for code block coloring</span></div>
            <Dropdown
              value={s.highlighter}
              options={[
                { value: 'rouge', label: 'Rouge (built-in)' },
                { value: 'highlightjs', label: 'highlight.js' },
                { value: 'prism', label: 'Prism' },
                { value: 'none', label: 'None' },
              ]}
              onchange={(v) => set('highlighter', v)}
            />
          </div>
          <div class="s-row">
            <div class="s-label"><p>Theme</p><span>Color theme for code blocks</span></div>
            <Dropdown
              value={s.syntaxTheme}
              options={[
                { value: 'monokai', label: 'Monokai' },
                { value: 'github-dark', label: 'GitHub Dark' },
                { value: 'one-dark', label: 'One Dark' },
                { value: 'dracula', label: 'Dracula' },
              ]}
              onchange={(v) => set('syntaxTheme', v)}
            />
          </div>
          <div class="s-row">
            <div class="s-label"><p>Show line numbers</p><span>Prefix code lines with their line number</span></div>
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <div class="toggle" class:on={s.showLineNumbers} onclick={() => toggleBool('showLineNumbers')} onkeydown={() => {}}><div class="toggle-knob"></div></div>
          </div>
          <div class="s-row">
            <div class="s-label"><p>Word-wrap in code blocks</p><span>Wrap long lines instead of scrolling</span></div>
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <div class="toggle" class:on={s.wordWrapCode} onclick={() => toggleBool('wordWrapCode')} onkeydown={() => {}}><div class="toggle-knob"></div></div>
          </div>
        </div>
      {:else if activeTab === 'diff'}
        <div class="s-section">
          <div class="s-section-title">Diff display</div>
          <div class="s-row">
            <div class="s-label"><p>Granularity</p><span>How precisely inline changes are highlighted</span></div>
            <Dropdown
              value={s.diffGranularity}
              options={[
                { value: 'word', label: 'Word-level' },
                { value: 'character', label: 'Character-level' },
                { value: 'block', label: 'Block-level' },
              ]}
              onchange={(v) => set('diffGranularity', v)}
            />
          </div>
          <div class="s-row">
            <div class="s-label"><p>Ignore whitespace</p><span>Treat whitespace-only changes as equal</span></div>
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <div class="toggle" class:on={s.ignoreWhitespace} onclick={() => toggleBool('ignoreWhitespace')} onkeydown={() => {}}><div class="toggle-knob"></div></div>
          </div>
          <div class="s-row">
            <div class="s-label"><p>Context lines</p><span>Unchanged lines shown around each change</span></div>
            <Dropdown
              value={s.contextLines === Infinity ? 'all' : String(s.contextLines)}
              options={[
                { value: '3', label: '3' },
                { value: '5', label: '5' },
                { value: '10', label: '10' },
                { value: 'all', label: 'All' },
              ]}
              onchange={(v) => set('contextLines', v === 'all' ? Infinity : Number(v))}
            />
          </div>
          <div class="s-row">
            <div class="s-label"><p>Synchronized scrolling</p><span>Keep both panels at the same scroll position</span></div>
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <div class="toggle" class:on={s.syncScrolling} onclick={() => toggleBool('syncScrolling')} onkeydown={() => {}}><div class="toggle-knob"></div></div>
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
            <Dropdown
              value={s.theme}
              options={[
                { value: 'dark', label: 'Dark (default)' },
                { value: 'light', label: 'Light' },
                { value: 'system', label: 'System' },
              ]}
              onchange={(v) => set('theme', v)}
            />
          </div>
          <div class="s-row">
            <div class="s-label"><p>Font size</p><span>Base font size for rendered documents</span></div>
            <Dropdown
              value={String(s.fontSize)}
              options={[
                { value: '12', label: '12px' },
                { value: '13', label: '13px' },
                { value: '14', label: '14px' },
                { value: '15', label: '15px' },
              ]}
              onchange={(v) => set('fontSize', Number(v))}
            />
          </div>
          <div class="s-row">
            <div class="s-label"><p>Line height</p><span>Spacing between lines in rendered text</span></div>
            <Dropdown
              value={String(s.lineHeight)}
              options={[
                { value: '1.5', label: '1.5' },
                { value: '1.75', label: '1.75' },
                { value: '2', label: '2.0' },
              ]}
              onchange={(v) => set('lineHeight', Number(v))}
            />
          </div>
          <div class="s-row">
            <div class="s-label"><p>Sidebar width</p><span>Width of the file tree sidebar in pixels</span></div>
            <Dropdown
              value={String(s.sidebarWidth)}
              options={[
                { value: '180', label: '180px' },
                { value: '220', label: '220px' },
                { value: '260', label: '260px' },
              ]}
              onchange={(v) => set('sidebarWidth', Number(v))}
            />
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
  .s-input { height: 28px; padding: 0 8px; background: var(--bg3); border: 1px solid var(--border2); border-radius: var(--radius); color: var(--text); font-family: var(--mono); font-size: 11px; width: 200px; outline: none; }
  .s-input:focus { border-color: var(--accent); }
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
