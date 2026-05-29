<script lang="ts">
  import './app.css';
  import Titlebar from './lib/components/Titlebar.svelte';
  import Toolbar from './lib/components/Toolbar.svelte';
  import Sidebar from './lib/components/Sidebar.svelte';
  import ContentArea from './lib/components/ContentArea.svelte';
  import Statusbar from './lib/components/Statusbar.svelte';
  import BranchModal from './lib/components/BranchModal.svelte';
  import SettingsModal from './lib/components/SettingsModal.svelte';
  import { appState } from './lib/stores/app';
  import { settings } from './lib/stores/settings';
  import { onMount } from 'svelte';

  let branchModalOpen = $state(false);
  let settingsOpen = $state(false);
  let sidebarCollapsed = $state(false);

  // Apply settings as CSS custom properties
  $effect(() => {
    const s = $settings;
    const root = document.documentElement;
    root.style.setProperty('--content-font-size', `${s.fontSize}px`);
    root.style.setProperty('--content-line-height', String(s.lineHeight));
    root.style.setProperty('--sidebar-w', `${s.sidebarWidth}px`);

    // Theme
    root.setAttribute('data-theme', s.theme === 'system'
      ? (window.matchMedia('(prefers-color-scheme: light)').matches ? 'light' : 'dark')
      : s.theme);

    // Sync scrolling setting to app state
    appState.update(state => ({ ...state, syncScroll: s.syncScrolling }));
  });

  function handleKeydown(e: KeyboardEvent) {
    const mod = e.metaKey || e.ctrlKey;
    if (mod && e.key === 'b') { e.preventDefault(); branchModalOpen = true; }
    if (mod && e.key === ',') { e.preventDefault(); settingsOpen = true; }
    if (mod && e.key === 'e') { e.preventDefault(); sidebarCollapsed = !sidebarCollapsed; }
    if (mod && e.key === 's') { e.preventDefault(); swapBranches(); }
    if (e.key === 'Escape') { branchModalOpen = false; settingsOpen = false; }

    // Block browser shortcuts that break desktop-app feel
    if (mod && e.key === 'r') e.preventDefault();         // reload
    if (mod && e.key === 'u') e.preventDefault();         // view source
    if (mod && e.key === 'p') e.preventDefault();         // print
    if (mod && e.shiftKey && e.key === 'I') e.preventDefault(); // devtools
    if (e.key === 'F5') e.preventDefault();               // reload
  }

  function swapBranches() {
    appState.update(s => {
      const tmp = s.leftBranch;
      s.leftBranch = s.rightBranch;
      s.rightBranch = tmp;
      return s;
    });
  }

  onMount(() => {
    document.addEventListener('keydown', handleKeydown);

    // Disable context menu (right-click) globally
    document.addEventListener('contextmenu', (e) => e.preventDefault());

    // Disable drag-and-drop of links/images (browser behavior)
    document.addEventListener('dragstart', (e) => e.preventDefault());

    return () => {
      document.removeEventListener('keydown', handleKeydown);
    };
  });
</script>

<div id="app">
  <Titlebar />
  <Toolbar
    bind:branchModalOpen
    bind:settingsOpen
    onswap={swapBranches}
  />
  <div id="main">
    <Sidebar collapsed={sidebarCollapsed} />
    <ContentArea />
  </div>
  <Statusbar />
</div>

{#if branchModalOpen}
  <BranchModal onclose={() => branchModalOpen = false} />
{/if}

{#if settingsOpen}
  <SettingsModal onclose={() => settingsOpen = false} />
{/if}

<style>
  #app {
    display: flex;
    flex-direction: column;
    height: 100vh;
  }
  #main {
    display: flex;
    flex: 1;
    overflow: hidden;
  }
</style>
