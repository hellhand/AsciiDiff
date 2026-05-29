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
  layout: 'split' | 'unified';
  showSource: boolean;
  highlight: boolean;
  syncScroll: boolean;
  leftContent: string;
  rightContent: string;
  leftSource: string;
  rightSource: string;
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
  layout: 'split',
  showSource: false,
  highlight: true,
  syncScroll: true,
  leftContent: '',
  rightContent: '',
  leftSource: '',
  rightSource: '',
  leftExists: false,
  rightExists: false,
};

export const appState = writable<AppState>(initialState);

export const diffStats = derived(appState, ($s) => {
  const files = $s.changedFiles;
  return {
    added: files.filter(f => f.status === 'added').length,
    deleted: files.filter(f => f.status === 'deleted').length,
    modified: files.filter(f => f.status === 'modified').length,
  };
});
