import { writable, get } from 'svelte/store';

export interface Settings {
  // Rendering
  renderBackend: 'asciidoctor-js' | 'asciidoctor-native';
  safeMode: 'unsafe' | 'safe' | 'server' | 'secure';
  resolveIncludes: boolean;
  evaluateIfdefs: boolean;

  // Includes
  includeBaseDir: string;
  resolveFromGit: boolean;
  showUnresolvedIncludes: boolean;
  maxIncludeDepth: number;

  // Git
  defaultBaseBranch: string;
  autoRefreshOnBranchChange: boolean;
  showCommitMetadata: boolean;

  // Syntax
  highlighter: 'rouge' | 'highlightjs' | 'prism' | 'none';
  syntaxTheme: 'monokai' | 'github-dark' | 'one-dark' | 'dracula';
  showLineNumbers: boolean;
  wordWrapCode: boolean;

  // Diff
  diffGranularity: 'word' | 'character' | 'block';
  ignoreWhitespace: boolean;
  contextLines: number;
  syncScrolling: boolean;

  // Appearance
  theme: 'dark' | 'light' | 'system';
  fontSize: number;
  lineHeight: number;
  sidebarWidth: number;
}

const defaults: Settings = {
  renderBackend: 'asciidoctor-js',
  safeMode: 'unsafe',
  resolveIncludes: true,
  evaluateIfdefs: true,

  includeBaseDir: './docs',
  resolveFromGit: true,
  showUnresolvedIncludes: true,
  maxIncludeDepth: 8,

  defaultBaseBranch: 'main',
  autoRefreshOnBranchChange: true,
  showCommitMetadata: true,

  highlighter: 'rouge',
  syntaxTheme: 'monokai',
  showLineNumbers: false,
  wordWrapCode: false,

  diffGranularity: 'word',
  ignoreWhitespace: true,
  contextLines: 3,
  syncScrolling: true,

  theme: 'dark',
  fontSize: 13,
  lineHeight: 1.75,
  sidebarWidth: 220,
};

const STORAGE_KEY = 'asciidiff-settings';

function loadSettings(): Settings {
  try {
    const stored = localStorage.getItem(STORAGE_KEY);
    if (stored) {
      return { ...defaults, ...JSON.parse(stored) };
    }
  } catch {}
  return { ...defaults };
}

function createSettingsStore() {
  const store = writable<Settings>(loadSettings());

  store.subscribe((value) => {
    try {
      localStorage.setItem(STORAGE_KEY, JSON.stringify(value));
    } catch {}
  });

  function update(partial: Partial<Settings>) {
    store.update((s) => ({ ...s, ...partial }));
  }

  function reset() {
    store.set({ ...defaults });
  }

  return {
    subscribe: store.subscribe,
    set: store.set,
    update,
    reset,
  };
}

export const settings = createSettingsStore();
