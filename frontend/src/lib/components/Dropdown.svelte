<script lang="ts">
  type Option = { value: string; label: string };

  let { options, value, onchange }: {
    options: Option[];
    value: string;
    onchange: (value: string) => void;
  } = $props();

  let open = $state(false);
  let container: HTMLDivElement;

  let selectedLabel = $derived(options.find(o => o.value === value)?.label ?? value);

  function toggle() { open = !open; }

  function select(opt: Option) {
    onchange(opt.value);
    open = false;
  }

  function handleClickOutside(e: MouseEvent) {
    if (container && !container.contains(e.target as Node)) {
      open = false;
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') open = false;
  }
</script>

<svelte:window onclick={handleClickOutside} onkeydown={handleKeydown} />

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="dropdown" bind:this={container}>
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div class="dropdown-trigger" onclick={toggle}>
    <span class="dropdown-value">{selectedLabel}</span>
    <svg class="dropdown-chevron" class:open width="12" height="12" viewBox="0 0 12 12">
      <path fill="currentColor" d="M3 5l3 3 3-3"/>
    </svg>
  </div>
  {#if open}
    <div class="dropdown-menu">
      {#each options as opt}
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <div
          class="dropdown-item"
          class:selected={opt.value === value}
          onclick={() => select(opt)}
        >
          {opt.label}
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .dropdown {
    position: relative;
    min-width: 160px;
  }
  .dropdown-trigger {
    height: 28px;
    padding: 0 28px 0 8px;
    background: var(--bg3);
    border: 1px solid var(--border2);
    border-radius: var(--radius);
    color: var(--text);
    font-size: 12px;
    cursor: pointer;
    display: flex;
    align-items: center;
    position: relative;
    user-select: none;
  }
  .dropdown-trigger:hover {
    border-color: var(--accent);
  }
  .dropdown-chevron {
    position: absolute;
    right: 8px;
    top: 50%;
    transform: translateY(-50%);
    color: var(--text3);
    transition: transform .15s;
  }
  .dropdown-chevron.open {
    transform: translateY(-50%) rotate(180deg);
  }
  .dropdown-menu {
    position: absolute;
    top: calc(100% + 4px);
    left: 0;
    right: 0;
    background: var(--bg3);
    border: 1px solid var(--border2);
    border-radius: var(--radius);
    padding: 4px 0;
    z-index: 1100;
    box-shadow: 0 8px 24px rgba(0,0,0,.4);
    max-height: 200px;
    overflow-y: auto;
  }
  .dropdown-item {
    padding: 6px 10px;
    font-size: 12px;
    color: var(--text2);
    cursor: pointer;
    transition: background .1s, color .1s;
  }
  .dropdown-item:hover {
    background: var(--bg4);
    color: var(--text);
  }
  .dropdown-item.selected {
    color: var(--accent);
    font-weight: 500;
  }
</style>
