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
  import { onMount } from 'svelte';

  let branchModalOpen = $state(false);
  let settingsOpen = $state(false);
  let sidebarCollapsed = $state(false);

  function handleKeydown(e: KeyboardEvent) {
    const mod = e.metaKey || e.ctrlKey;
    if (mod && e.key === 'b') { e.preventDefault(); branchModalOpen = true; }
    if (mod && e.key === ',') { e.preventDefault(); settingsOpen = true; }
    if (mod && e.key === 'e') { e.preventDefault(); sidebarCollapsed = !sidebarCollapsed; }
    if (mod && e.key === 's') { e.preventDefault(); swapBranches(); }
    if (e.key === 'Escape') { branchModalOpen = false; settingsOpen = false; }
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
    return () => document.removeEventListener('keydown', handleKeydown);
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
