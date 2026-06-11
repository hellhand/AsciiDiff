use git2::{DiffOptions, Repository};
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct BranchInfo {
    pub name: String,
    pub hash: String,
    pub ref_type: String, // "branch" or "tag"
    pub is_head: bool,
}

#[derive(Debug, Serialize, Clone)]
pub struct ChangedFile {
    pub path: String,
    pub status: String, // "added", "deleted", "modified"
}

#[derive(Debug, Serialize, Clone)]
pub struct FileContent {
    pub content: String,
    pub exists: bool,
}

pub fn list_refs(repo_path: &str) -> Result<Vec<BranchInfo>, git2::Error> {
    let repo = Repository::open(repo_path)?;
    let mut refs = Vec::new();

    // Local branches
    for branch in repo.branches(Some(git2::BranchType::Local))? {
        let (branch, _) = branch?;
        let name = branch.name()?.unwrap_or("").to_string();
        let commit = branch.get().peel_to_commit()?;
        let hash = commit.id().to_string()[..7].to_string();
        let is_head = branch.is_head();
        refs.push(BranchInfo {
            name,
            hash,
            ref_type: "branch".to_string(),
            is_head,
        });
    }

    // Tags
    repo.tag_foreach(|oid, name_bytes| {
        let name = String::from_utf8_lossy(name_bytes)
            .trim_start_matches("refs/tags/")
            .to_string();
        let hash = oid.to_string()[..7].to_string();
        refs.push(BranchInfo {
            name,
            hash,
            ref_type: "tag".to_string(),
            is_head: false,
        });
        true
    })?;

    Ok(refs)
}

pub fn diff_refs(
    repo_path: &str,
    left_ref: &str,
    right_ref: &str,
) -> Result<Vec<ChangedFile>, git2::Error> {
    let repo = Repository::open(repo_path)?;
    let left_tree = resolve_tree(&repo, left_ref)?;
    let right_tree = resolve_tree(&repo, right_ref)?;

    let mut opts = DiffOptions::new();
    let diff = repo.diff_tree_to_tree(Some(&left_tree), Some(&right_tree), Some(&mut opts))?;

    let mut files = Vec::new();
    diff.foreach(
        &mut |delta, _| {
            let path = delta
                .new_file()
                .path()
                .or_else(|| delta.old_file().path())
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_default();
            let status = match delta.status() {
                git2::Delta::Added => "added",
                git2::Delta::Deleted => "deleted",
                _ => "modified",
            };
            files.push(ChangedFile {
                path,
                status: status.to_string(),
            });
            true
        },
        None,
        None,
        None,
    )?;

    Ok(files)
}

pub fn read_file_at_ref(
    repo_path: &str,
    git_ref: &str,
    file_path: &str,
) -> Result<FileContent, git2::Error> {
    let repo = Repository::open(repo_path)?;
    let tree = resolve_tree(&repo, git_ref)?;

    match tree.get_path(std::path::Path::new(file_path)) {
        Ok(entry) => {
            let blob = repo.find_blob(entry.id())?;
            let content = String::from_utf8_lossy(blob.content()).to_string();
            Ok(FileContent {
                content,
                exists: true,
            })
        }
        Err(_) => Ok(FileContent {
            content: String::new(),
            exists: false,
        }),
    }
}

/// Result of a per-file diff: which lines (1-indexed) are deleted on the left and added on the right.
#[derive(Debug, Serialize, Clone, Default)]
pub struct FileDiffLines {
    /// Line numbers in the old file that were deleted or modified (1-indexed)
    pub left_changed: Vec<u32>,
    /// Line numbers in the new file that were added or modified (1-indexed)
    pub right_changed: Vec<u32>,
}

/// Uses git2's diff to get the exact changed line numbers for a specific file
/// between two refs. This matches `git diff` output precisely.
pub fn diff_file_lines(
    repo_path: &str,
    left_ref: &str,
    right_ref: &str,
    file_path: &str,
) -> Result<FileDiffLines, git2::Error> {
    let repo = Repository::open(repo_path)?;
    let left_tree = resolve_tree(&repo, left_ref)?;
    let right_tree = resolve_tree(&repo, right_ref)?;

    let mut opts = DiffOptions::new();
    opts.pathspec(file_path);

    let diff = repo.diff_tree_to_tree(Some(&left_tree), Some(&right_tree), Some(&mut opts))?;

    let mut result = FileDiffLines::default();

    diff.foreach(
        &mut |_delta, _progress| true,
        None,
        None,
        Some(&mut |_delta, _hunk, line| {
            match line.origin() {
                '-' => {
                    if let Some(n) = line.old_lineno() {
                        result.left_changed.push(n);
                    }
                }
                '+' => {
                    if let Some(n) = line.new_lineno() {
                        result.right_changed.push(n);
                    }
                }
                _ => {} // context lines, no-newline markers, etc.
            }
            true
        }),
    )?;

    Ok(result)
}

fn resolve_tree<'a>(
    repo: &'a Repository,
    refspec: &str,
) -> Result<git2::Tree<'a>, git2::Error> {
    let obj = repo.revparse_single(refspec)?;
    obj.peel_to_tree()
}
