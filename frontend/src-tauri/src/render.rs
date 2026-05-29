use similar::{ChangeTag, TextDiff};

/// Renders raw AsciiDoc text into basic HTML matching the mock's CSS classes.
pub fn render_asciidoc(source: &str) -> String {
    let lines: Vec<&str> = source.lines().collect();
    let no_changes: Vec<bool> = vec![false; lines.len()];
    render_asciidoc_with_changes(&lines, &no_changes, "")
}

/// Compute a word-level diff between two rendered HTML strings and produce
/// side-by-side HTML with diff highlighting classes matching the mock.
pub fn compute_diff_html(left_text: &str, right_text: &str) -> (String, String) {
    let _left_html = render_asciidoc(left_text);
    let _right_html = render_asciidoc(right_text);

    // For a simple first pass, we diff at the line level of the source text
    // and then mark which rendered sections have changes
    let diff = TextDiff::from_lines(left_text, right_text);

    let mut left_lines: Vec<(ChangeTag, &str)> = Vec::new();
    let mut right_lines: Vec<(ChangeTag, &str)> = Vec::new();

    for change in diff.iter_all_changes() {
        match change.tag() {
            ChangeTag::Equal => {
                left_lines.push((ChangeTag::Equal, change.value()));
                right_lines.push((ChangeTag::Equal, change.value()));
            }
            ChangeTag::Delete => {
                left_lines.push((ChangeTag::Delete, change.value()));
            }
            ChangeTag::Insert => {
                right_lines.push((ChangeTag::Insert, change.value()));
            }
        }
    }

    // Re-render left with deletions marked
    let left_marked = render_with_marks(left_text, &left_lines, "del");
    let right_marked = render_with_marks(right_text, &right_lines, "add");

    (left_marked, right_marked)
}

/// Renders asciidoc with diff markers applied to changed sections
fn render_with_marks(source: &str, marked_lines: &[(ChangeTag, &str)], change_type: &str) -> String {
    // Build a set of line indices that are changed
    let source_lines: Vec<&str> = source.lines().collect();
    let mut changed_source_lines: Vec<bool> = vec![false; source_lines.len()];

    let mut src_idx = 0;
    for (tag, line_content) in marked_lines {
        let trimmed = line_content.trim_end_matches('\n');
        match tag {
            ChangeTag::Equal => {
                if src_idx < source_lines.len() {
                    src_idx += 1;
                }
            }
            ChangeTag::Delete | ChangeTag::Insert => {
                if src_idx < source_lines.len() && source_lines[src_idx] == trimmed {
                    changed_source_lines[src_idx] = true;
                    src_idx += 1;
                } else if src_idx < source_lines.len() {
                    changed_source_lines[src_idx] = true;
                }
            }
        }
    }

    // Render using the full renderer with diff class injection
    render_asciidoc_with_changes(&source_lines, &changed_source_lines, change_type)
}

/// Full AsciiDoc renderer that optionally wraps changed lines with diff classes.
/// When `changed_lines` is empty, behaves identically to `render_asciidoc`.
fn render_asciidoc_with_changes(lines: &[&str], changed_lines: &[bool], change_type: &str) -> String {
    let mut html = String::from("<div class=\"rendered\">");
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i];
        let is_changed = changed_lines.get(i).copied().unwrap_or(false);
        let wrap_class = if is_changed {
            match change_type {
                "del" => " diff-wrap-del",
                "add" => " diff-wrap-add",
                _ => " diff-wrap-mod",
            }
        } else {
            ""
        };

        // Check if any lines in a range are changed
        let range_changed = |start: usize, end: usize| -> bool {
            (start..end).any(|idx| changed_lines.get(idx).copied().unwrap_or(false))
        };

        // Document title (= Title)
        if line.starts_with("= ") && !line.starts_with("== ") {
            html.push_str(&format!("<div class=\"adoc-title{}\">{}</div>", wrap_class, escape_html(&line[2..])));
        }
        // Author line
        else if i > 0 && lines[i - 1].starts_with("= ") && !line.is_empty() && !line.starts_with(':') && !line.starts_with('[') {
            html.push_str(&format!("<div class=\"adoc-author{}\"><span>{}</span></div>", wrap_class, escape_html(line)));
        }
        // Attribute definitions (skip)
        else if line.starts_with(':') && line.contains(':') && line.len() > 1 {
            // skip
        }
        // Section headings
        else if line.starts_with("==== ") {
            html.push_str(&format!("<div class=\"adoc-h3{}\">{}</div>", wrap_class, escape_html(&line[5..])));
        } else if line.starts_with("=== ") {
            html.push_str(&format!("<div class=\"adoc-h2{}\">{}</div>", wrap_class, escape_html(&line[4..])));
        } else if line.starts_with("== ") {
            html.push_str(&format!("<div class=\"adoc-h1{}\">{}</div>", wrap_class, escape_html(&line[3..])));
        }
        // Admonitions
        else if line.starts_with("NOTE: ") {
            html.push_str(&format!("<div class=\"adoc-note{}\"><strong>Note:</strong> {}</div>", wrap_class, escape_html(&line[6..])));
        } else if line.starts_with("WARNING: ") {
            html.push_str(&format!("<div class=\"adoc-warn{}\"><strong>Warning:</strong> {}</div>", wrap_class, escape_html(&line[9..])));
        } else if line.starts_with("TIP: ") {
            html.push_str(&format!("<div class=\"adoc-tip{}\"><strong>Tip:</strong> {}</div>", wrap_class, escape_html(&line[5..])));
        } else if line.starts_with("IMPORTANT: ") {
            html.push_str(&format!("<div class=\"adoc-important{}\"><strong>Important:</strong> {}</div>", wrap_class, escape_html(&line[11..])));
        }
        // Code block
        else if line.starts_with("[source") || line == "----" {
            let lang = if line.starts_with("[source") {
                line.trim_start_matches("[source,")
                    .trim_end_matches(']')
                    .trim()
                    .to_string()
            } else {
                String::new()
            };

            if line.starts_with("[source") {
                i += 1;
                if i >= lines.len() || lines[i] != "----" {
                    html.push_str(&format!("<p class=\"adoc-p{}\">{}</p>", wrap_class, inline_format(line)));
                    i += 1;
                    continue;
                }
            }

            let block_start = i;
            i += 1;
            let mut code = String::new();
            while i < lines.len() && lines[i] != "----" {
                if !code.is_empty() {
                    code.push('\n');
                }
                code.push_str(lines[i]);
                i += 1;
            }

            let block_class = if range_changed(block_start, i + 1) {
                match change_type {
                    "del" => " diff-wrap-del",
                    "add" => " diff-wrap-add",
                    _ => " diff-wrap-mod",
                }
            } else {
                ""
            };

            let lang_badge = if !lang.is_empty() {
                format!("<span class=\"lang-badge\">{}</span>", escape_html(&lang))
            } else {
                String::new()
            };

            html.push_str(&format!(
                "<div class=\"adoc-listing{}\"><div class=\"adoc-listing-title\">{}</div><pre class=\"adoc-pre\">{}</pre></div>",
                block_class, lang_badge, escape_html(&code)
            ));
        }
        // Unordered list
        else if line.starts_with("* ") || line.starts_with("- ") {
            html.push_str("<ul class=\"adoc-ul\">");
            while i < lines.len() && (lines[i].starts_with("* ") || lines[i].starts_with("- ")) {
                let item_changed = changed_lines.get(i).copied().unwrap_or(false);
                let item_class = if item_changed {
                    match change_type {
                        "del" => " class=\"diff-wrap-del\"",
                        "add" => " class=\"diff-wrap-add\"",
                        _ => " class=\"diff-wrap-mod\"",
                    }
                } else {
                    ""
                };
                let item = &lines[i][2..];
                html.push_str(&format!("<li{}>{}</li>", item_class, inline_format(item)));
                i += 1;
            }
            html.push_str("</ul>");
            continue;
        }
        // Ordered list
        else if line.starts_with(". ") {
            html.push_str("<ol class=\"adoc-ol\">");
            while i < lines.len() && lines[i].starts_with(". ") {
                let item_changed = changed_lines.get(i).copied().unwrap_or(false);
                let item_class = if item_changed {
                    match change_type {
                        "del" => " class=\"diff-wrap-del\"",
                        "add" => " class=\"diff-wrap-add\"",
                        _ => " class=\"diff-wrap-mod\"",
                    }
                } else {
                    ""
                };
                let item = &lines[i][2..];
                html.push_str(&format!("<li{}>{}</li>", item_class, inline_format(item)));
                i += 1;
            }
            html.push_str("</ol>");
            continue;
        }
        // Include directive
        else if line.starts_with("include::") {
            let path = line.trim_start_matches("include::")
                .trim_end_matches("[]")
                .trim();
            html.push_str(&format!(
                "<div class=\"adoc-include{}\"><i class=\"ti ti-file-symlink\"></i><span style=\"color:var(--text3)\">include::</span><span class=\"inc-path\">{}</span></div>",
                wrap_class, escape_html(path)
            ));
        }
        // Table
        else if line.starts_with("|===") {
            let table_start = i;
            i += 1;
            let mut rows: Vec<Vec<String>> = Vec::new();
            while i < lines.len() && !lines[i].starts_with("|===") {
                if lines[i].starts_with('|') {
                    let cells: Vec<String> = lines[i][1..]
                        .split('|')
                        .map(|c| c.trim().to_string())
                        .collect();
                    rows.push(cells);
                }
                i += 1;
            }

            let table_class = if range_changed(table_start, i + 1) {
                match change_type {
                    "del" => " diff-wrap-del",
                    "add" => " diff-wrap-add",
                    _ => " diff-wrap-mod",
                }
            } else {
                ""
            };

            if !rows.is_empty() {
                html.push_str(&format!("<table class=\"adoc-table{}\">", table_class));
                html.push_str("<thead><tr>");
                for cell in &rows[0] {
                    html.push_str(&format!("<th>{}</th>", inline_format(cell)));
                }
                html.push_str("</tr></thead><tbody>");
                for row in &rows[1..] {
                    html.push_str("<tr>");
                    for cell in row {
                        html.push_str(&format!("<td>{}</td>", inline_format(cell)));
                    }
                    html.push_str("</tr>");
                }
                html.push_str("</tbody></table>");
            }
        }
        // Empty line
        else if line.is_empty() {
            // skip
        }
        // Regular paragraph
        else {
            let mut para = String::from(line);
            let para_start = i;
            while i + 1 < lines.len()
                && !lines[i + 1].is_empty()
                && !lines[i + 1].starts_with('=')
                && !lines[i + 1].starts_with('[')
                && !lines[i + 1].starts_with('|')
                && !lines[i + 1].starts_with("* ")
                && !lines[i + 1].starts_with("- ")
                && !lines[i + 1].starts_with(". ")
                && !lines[i + 1].starts_with("----")
                && !lines[i + 1].starts_with("NOTE:")
                && !lines[i + 1].starts_with("WARNING:")
                && !lines[i + 1].starts_with("TIP:")
                && !lines[i + 1].starts_with("IMPORTANT:")
                && !lines[i + 1].starts_with("include::")
            {
                i += 1;
                para.push(' ');
                para.push_str(lines[i]);
            }

            let para_class = if range_changed(para_start, i + 1) {
                match change_type {
                    "del" => " diff-wrap-del",
                    "add" => " diff-wrap-add",
                    _ => " diff-wrap-mod",
                }
            } else {
                ""
            };
            html.push_str(&format!("<p class=\"adoc-p{}\">{}</p>", para_class, inline_format(&para)));
        }

        i += 1;
    }

    html.push_str("</div>");
    html
}

/// Apply inline formatting (bold, italic, monospace)
fn inline_format(text: &str) -> String {
    let escaped = escape_html(text);
    // Bold
    let result = escaped.replace("*", ""); // simplified - would need proper regex
    // Monospace `code`
    let mut output = String::new();
    let mut chars = result.chars().peekable();
    let mut in_code = false;

    while let Some(ch) = chars.next() {
        if ch == '`' {
            if in_code {
                output.push_str("</code>");
                in_code = false;
            } else {
                output.push_str("<code class=\"adoc-code-inline\">");
                in_code = true;
            }
        } else {
            output.push(ch);
        }
    }
    if in_code {
        output.push_str("</code>");
    }

    output
}

fn escape_html(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_title() {
        let html = render_asciidoc("= My Title");
        assert!(html.contains("<div class=\"adoc-title\">My Title</div>"));
    }

    #[test]
    fn test_render_title_with_author() {
        let html = render_asciidoc("= My Title\nJohn Doe <john@example.com>");
        assert!(html.contains("adoc-title"));
        assert!(html.contains("adoc-author"));
        assert!(html.contains("John Doe"));
    }

    #[test]
    fn test_render_headings() {
        let html = render_asciidoc("== Section One\n=== Subsection\n==== Sub-sub");
        assert!(html.contains("adoc-h1"));
        assert!(html.contains("adoc-h2"));
        assert!(html.contains("adoc-h3"));
        assert!(html.contains("Section One"));
        assert!(html.contains("Subsection"));
        assert!(html.contains("Sub-sub"));
    }

    #[test]
    fn test_render_paragraph() {
        let html = render_asciidoc("This is a paragraph.\nThis continues it.");
        assert!(html.contains("<p class=\"adoc-p\">"));
        assert!(html.contains("This is a paragraph. This continues it."));
    }

    #[test]
    fn test_render_code_block() {
        let src = "[source,rust]\n----\nfn main() {\n    println!(\"hello\");\n}\n----";
        let html = render_asciidoc(src);
        assert!(html.contains("adoc-listing"));
        assert!(html.contains("lang-badge"));
        assert!(html.contains("rust"));
        assert!(html.contains("fn main()"));
        assert!(html.contains("println!"));
    }

    #[test]
    fn test_render_code_block_no_language() {
        let src = "----\nsome code\n----";
        let html = render_asciidoc(src);
        assert!(html.contains("adoc-listing"));
        assert!(html.contains("some code"));
        assert!(!html.contains("lang-badge"));
    }

    #[test]
    fn test_render_note() {
        let html = render_asciidoc("NOTE: This is important");
        assert!(html.contains("adoc-note"));
        assert!(html.contains("This is important"));
    }

    #[test]
    fn test_render_warning() {
        let html = render_asciidoc("WARNING: Be careful");
        assert!(html.contains("adoc-warn"));
        assert!(html.contains("Be careful"));
    }

    #[test]
    fn test_render_tip() {
        let html = render_asciidoc("TIP: Try this");
        assert!(html.contains("adoc-tip"));
        assert!(html.contains("Try this"));
    }

    #[test]
    fn test_render_important() {
        let html = render_asciidoc("IMPORTANT: Do not skip");
        assert!(html.contains("adoc-important"));
        assert!(html.contains("Do not skip"));
    }

    #[test]
    fn test_render_unordered_list() {
        let src = "* Item one\n* Item two\n* Item three";
        let html = render_asciidoc(src);
        assert!(html.contains("<ul class=\"adoc-ul\">"));
        assert!(html.contains("<li>Item one</li>"));
        assert!(html.contains("<li>Item two</li>"));
        assert!(html.contains("<li>Item three</li>"));
    }

    #[test]
    fn test_render_ordered_list() {
        let src = ". First\n. Second\n. Third";
        let html = render_asciidoc(src);
        assert!(html.contains("<ol class=\"adoc-ol\">"));
        assert!(html.contains("<li>First</li>"));
        assert!(html.contains("<li>Second</li>"));
    }

    #[test]
    fn test_render_table_with_header_separator() {
        // AsciiDoc tables use a blank line to separate header from body
        let src = "|===\n| Name | Age\n\n| Alice | 30\n| Bob | 25\n|===";
        let html = render_asciidoc(src);
        assert!(html.contains("<table class=\"adoc-table\">"), "No table found in:\n{}", html);
        assert!(html.contains("<th>"), "No header cells in:\n{}", html);
        assert!(html.contains("Name"), "Missing 'Name' in:\n{}", html);
        assert!(html.contains("Alice"), "Missing 'Alice' in:\n{}", html);
        assert!(html.contains("Bob"), "Missing 'Bob' in:\n{}", html);
    }

    #[test]
    fn test_render_table_no_header_separator() {
        // Table without blank line separator (all rows treated uniformly)
        let src = "|===\n| H1 | H2\n| D1 | D2\n|===";
        let html = render_asciidoc(src);
        assert!(html.contains("<table class=\"adoc-table\">"), "No table found in:\n{}", html);
        assert!(html.contains("H1"), "Missing 'H1' in:\n{}", html);
        assert!(html.contains("D1"), "Missing 'D1' in:\n{}", html);
    }

    #[test]
    fn test_render_table_many_columns() {
        let src = "|===\n| Service | Port | Protocol | Owner\n\n| Gateway | 8080 | HTTP | Platform\n| Auth | 9001 | gRPC | Identity\n|===";
        let html = render_asciidoc(src);
        assert!(html.contains("<th>Service</th>"), "Missing header in:\n{}", html);
        assert!(html.contains("<td>Gateway</td>"), "Missing data in:\n{}", html);
        assert!(html.contains("<td>gRPC</td>"), "Missing data in:\n{}", html);
    }

    #[test]
    fn test_render_include() {
        let html = render_asciidoc("include::partials/_header.adoc[]");
        assert!(html.contains("adoc-include"));
        assert!(html.contains("partials/_header.adoc"));
    }

    #[test]
    fn test_render_inline_code() {
        let html = render_asciidoc("Use the `foo` command");
        assert!(html.contains("<code class=\"adoc-code-inline\">foo</code>"));
    }

    #[test]
    fn test_render_attributes_skipped() {
        let html = render_asciidoc(":toc: left\n:icons: font\n\n== Section");
        // Attributes should not appear as visible text
        assert!(!html.contains(":toc:"));
        assert!(html.contains("Section"));
    }

    #[test]
    fn test_render_caution_as_paragraph() {
        // CAUTION is not handled specially, should render as paragraph
        let html = render_asciidoc("CAUTION: This is not handled as admonition yet");
        assert!(html.contains("adoc-p") || html.contains("CAUTION"));
    }

    #[test]
    fn test_escape_html() {
        assert_eq!(escape_html("<script>alert('xss')</script>"), "&lt;script&gt;alert('xss')&lt;/script&gt;");
    }

    #[test]
    fn test_escape_html_basic() {
        assert_eq!(escape_html("a & b"), "a &amp; b");
        assert_eq!(escape_html("\"quoted\""), "&quot;quoted&quot;");
    }

    #[test]
    fn test_diff_produces_markers() {
        let left = "= Title\n\nOld paragraph";
        let right = "= Title\n\nNew paragraph";
        let (_, right_html) = compute_diff_html(left, right);
        assert!(right_html.contains("diff-wrap-add"));
    }

    #[test]
    fn test_diff_equal_content_no_markers() {
        let text = "= Title\n\nSame paragraph";
        let (left_html, right_html) = compute_diff_html(text, text);
        assert!(!left_html.contains("diff-wrap"));
        assert!(!right_html.contains("diff-wrap"));
    }

    #[test]
    fn test_diff_deleted_line() {
        let left = "= Title\n\nFirst\n\nSecond";
        let right = "= Title\n\nFirst";
        let (left_html, _) = compute_diff_html(left, right);
        assert!(left_html.contains("diff-wrap-del"));
    }

    #[test]
    fn test_diff_table_rendered() {
        // When tables change between versions, they should still render as tables in diff mode
        let left = "== Services\n\n|===\n| Name | Port\n\n| API | 8080\n|===";
        let right = "== Services\n\n|===\n| Name | Port\n\n| API | 8080\n| Auth | 9001\n|===";
        let (_, right_html) = compute_diff_html(left, right);
        assert!(right_html.contains("<table"), "Table not rendered in diff mode:\n{}", right_html);
        assert!(right_html.contains("Auth"), "Table data missing in diff mode:\n{}", right_html);
        assert!(right_html.contains("diff-wrap-add"), "Diff marker missing on table:\n{}", right_html);
    }

    #[test]
    fn test_diff_code_block_rendered() {
        let left = "== Config\n\n[source,yaml]\n----\nport: 8080\n----";
        let right = "== Config\n\n[source,yaml]\n----\nport: 9090\ntimeout: 30s\n----";
        let (_, right_html) = compute_diff_html(left, right);
        assert!(right_html.contains("adoc-listing"), "Code block not rendered in diff mode:\n{}", right_html);
        assert!(right_html.contains("9090"), "Code content missing:\n{}", right_html);
    }

    #[test]
    fn test_diff_list_rendered() {
        let left = "* Item one\n* Item two";
        let right = "* Item one\n* Item two\n* Item three";
        let (_, right_html) = compute_diff_html(left, right);
        assert!(right_html.contains("<ul"), "List not rendered in diff mode:\n{}", right_html);
        assert!(right_html.contains("Item three"), "List item missing:\n{}", right_html);
    }

    #[test]
    fn test_render_full_document() {
        let src = "= Architecture Guide\nTeam <team@example.com>\n:toc:\n\n== Overview\n\nThis describes the system.\n\nNOTE: See deployment guide.\n\n== Services\n\n|===\n| Name | Port\n\n| API | 8080\n| Auth | 9001\n|===\n\n[source,yaml]\n----\nport: 8080\n----\n\n* Item one\n* Item two";
        let html = render_asciidoc(src);
        assert!(html.contains("adoc-title"));
        assert!(html.contains("adoc-author"));
        assert!(html.contains("adoc-h1"));
        assert!(html.contains("adoc-note"));
        assert!(html.contains("adoc-table"));
        assert!(html.contains("adoc-listing"));
        assert!(html.contains("adoc-ul"));
        assert!(html.contains("API"));
        assert!(html.contains("8080"));
    }
}
