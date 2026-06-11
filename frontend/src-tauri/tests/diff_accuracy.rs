use app_lib::git;
use app_lib::git::FileDiffLines;
use app_lib::render;

const TEST_REPO: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../test-repo");

#[test]
fn git_diff_lines_matches_git_cli() {
    // Expected from: git diff main feature/v2 -- architecture-guide.adoc
    // Left (main) deleted lines: 19, 21, 22, 23, 24
    // Right (feature/v2) added lines: 16, 17, 21, 23, 24, 25, 26, 27
    let result = git::diff_file_lines(
        TEST_REPO,
        "main",
        "feature/v2",
        "architecture-guide.adoc",
    )
    .expect("diff_file_lines should succeed");

    let expected_left: Vec<u32> = vec![19, 21, 22, 23, 24];
    let expected_right: Vec<u32> = vec![16, 17, 21, 23, 24, 25, 26, 27];

    assert_eq!(
        result.left_changed, expected_left,
        "Left (deleted) lines don't match git diff.\nGot: {:?}\nExpected: {:?}",
        result.left_changed, expected_left
    );
    assert_eq!(
        result.right_changed, expected_right,
        "Right (added) lines don't match git diff.\nGot: {:?}\nExpected: {:?}",
        result.right_changed, expected_right
    );
}

#[test]
fn diff_hotfix_branch() {
    let result = git::diff_file_lines(
        TEST_REPO,
        "main",
        "hotfix/auth-fix",
        "architecture-guide.adoc",
    )
    .expect("diff_file_lines should succeed");

    assert!(
        result.left_changed.is_empty() && result.right_changed.is_empty(),
        "Expected no changes for architecture-guide.adoc between main and hotfix/auth-fix, got left={:?} right={:?}",
        result.left_changed, result.right_changed
    );
}

#[test]
fn diff_include_expansion_marks_correct_expanded_lines() {
    let result = git::diff_file_lines(
        TEST_REPO,
        "main",
        "feature/v2",
        "architecture-guide.adoc",
    )
    .expect("diff_file_lines should succeed");

    assert!(
        result.right_changed.contains(&16),
        "Line 16 (observability include) should be marked as added"
    );
    assert!(
        result.right_changed.contains(&17),
        "Line 17 (blank line after include) should be marked as added"
    );
}

#[test]
fn diff_api_reference_error_codes_table() {
    // git diff main v2.0.0 -- api-reference.adoc
    // In the Error Codes table, only 3 rows are added (lines 66, 67, 70 in new file).
    // The existing rows (400, 401, 403, 404, 429, 500) should NOT be marked.
    let result = git::diff_file_lines(
        TEST_REPO,
        "main",
        "v2.0.0",
        "api-reference.adoc",
    )
    .expect("diff_file_lines should succeed");

    // From the git diff output, in the new file (right/v2.0.0):
    // The error codes table: added lines are 66 (409), 67 (422), 70 (503)
    assert!(
        result.right_changed.contains(&66),
        "Line 66 (409 row) should be marked as added"
    );
    assert!(
        result.right_changed.contains(&67),
        "Line 67 (422 row) should be marked as added"
    );
    assert!(
        result.right_changed.contains(&70),
        "Line 70 (503 row) should be marked as added"
    );

    // Lines that should NOT be marked (existing unchanged rows):
    // 62: | 400 | Bad request ...
    // 63: | 401 | Unauthorized ...
    // 64: | 403 | Forbidden ...
    // 65: | 404 | Not found ...
    // 68: | 429 | Rate limited ...
    // 69: | 500 | Internal error ...
    assert!(
        !result.right_changed.contains(&62),
        "Line 62 (400 row) should NOT be marked - it's unchanged"
    );
    assert!(
        !result.right_changed.contains(&63),
        "Line 63 (401 row) should NOT be marked - it's unchanged"
    );
    assert!(
        !result.right_changed.contains(&68),
        "Line 68 (429 row) should NOT be marked - it's unchanged"
    );
    assert!(
        !result.right_changed.contains(&69),
        "Line 69 (500 row) should NOT be marked - it's unchanged"
    );
}

#[test]
fn render_table_highlights_only_changed_rows() {
    // Simulate a table where only row 2 (line 5, 1-indexed) is added
    let left = "|===\n| Name | Port\n\n| API | 8080\n|===";
    let right = "|===\n| Name | Port\n\n| API | 8080\n| Auth | 9001\n|===";
    // In right, line 5 is "| Auth | 9001" (added)
    let diff = FileDiffLines {
        left_changed: vec![],
        right_changed: vec![5],
    };
    let (_, right_html) = render::compute_diff_html(left, right, &diff);

    // The added row should have the diff class
    assert!(
        right_html.contains("diff-wrap-add"),
        "Added row should have diff-wrap-add class:\n{}", right_html
    );

    // Count how many <tr> tags have diff-wrap-add - should be exactly 1
    let marked_rows = right_html.matches("diff-wrap-add").count();
    assert_eq!(
        marked_rows, 1,
        "Only 1 row should be highlighted, got {}:\n{}", marked_rows, right_html
    );

    // The existing row (API | 8080) should NOT be marked
    // Find the <tr> containing "API" and verify it doesn't have diff class
    assert!(
        !right_html.contains("<tr class=\"diff-wrap-add\"><td>API</td>"),
        "Existing API row should NOT be highlighted:\n{}", right_html
    );
}

#[test]
fn diff_included_file_changes_detected() {
    // chapters/overview.adoc is included by architecture-guide.adoc
    // It has changes between main and feature/v2 (line 3 modified, lines 8-9 modified, etc.)
    // These should be detected by diff_file_lines on the included file.
    let result = git::diff_file_lines(
        TEST_REPO,
        "main",
        "feature/v2",
        "chapters/overview.adoc",
    )
    .expect("diff_file_lines should succeed on included file");

    // Line 3 was modified (description text changed)
    assert!(
        result.left_changed.contains(&3),
        "Line 3 in overview.adoc should be marked as changed on left side: {:?}",
        result.left_changed
    );
    assert!(
        result.right_changed.contains(&3),
        "Line 3 in overview.adoc should be marked as changed on right side: {:?}",
        result.right_changed
    );

    // feature/v2 has new lines added (mTLS, OpenTelemetry, v2 comparison table)
    assert!(
        result.right_changed.len() > result.left_changed.len(),
        "Right side should have more changed lines due to additions"
    );
}

