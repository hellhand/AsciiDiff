import { writable, derived } from 'svelte/store';

export interface BranchInfo {
  name: string;
  hash: string;
  ref_type: 'branch' | 'tag';
  is_head: boolean;
}

export interface ChangedFile {
  path: string;
  status: 'added' | 'deleted' | 'modified';
}

export interface AppState {
  repoPath: string | null;
  branches: BranchInfo[];
  leftBranch: string;
  rightBranch: string;
  changedFiles: ChangedFile[];
  activeFileIdx: number;
  viewMode: 'split' | 'unified' | 'preview';
  highlight: boolean;
  collapse: boolean;
  syncScroll: boolean;
  leftContent: string;
  rightContent: string;
  leftExists: boolean;
  rightExists: boolean;
}

const initialState: AppState = {
  repoPath: null,
  branches: [],
  leftBranch: 'main',
  rightBranch: '',
  changedFiles: [],
  activeFileIdx: 0,
  viewMode: 'split',
  highlight: true,
  collapse: false,
  syncScroll: true,
  leftContent: '',
  rightContent: '',
  leftExists: false,
  rightExists: false,
};

export const appState = writable<AppState>(initialState);

export const activeFile = derived(appState, ($s) =>
  $s.changedFiles[$s.activeFileIdx] ?? null
);

export const diffStats = derived(appState, ($s) => {
  const files = $s.changedFiles;
  return {
    added: files.filter(f => f.status === 'added').length,
    deleted: files.filter(f => f.status === 'deleted').length,
    modified: files.filter(f => f.status === 'modified').length,
  };
});
