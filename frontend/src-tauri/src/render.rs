use similar::{ChangeTag, TextDiff};

/// Renders raw AsciiDoc text into basic HTML matching the mock's CSS classes.
pub fn render_asciidoc(source: &str) -> String {
    let mut html = String::from("<div class=\"rendered\">");
    let lines: Vec<&str> = source.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i];

        // Document title (= Title)
        if line.starts_with("= ") && !line.starts_with("== ") {
            let title = &line[2..];
            html.push_str(&format!("<div class=\"adoc-title\">{}</div>", escape_html(title)));
        }
        // Author line (typically after title, contains email or plain name)
        else if i > 0 && lines[i - 1].starts_with("= ") && !line.is_empty() && !line.starts_with(':') && !line.starts_with('[') {
            html.push_str(&format!("<div class=\"adoc-author\"><span>{}</span></div>", escape_html(line)));
        }
        // Attribute definitions (skip them in output)
        else if line.starts_with(':') && line.contains(':') && line.len() > 1 {
            // skip attribute definitions
        }
        // Section headings
        else if line.starts_with("==== ") {
            html.push_str(&format!("<div class=\"adoc-h3\">{}</div>", escape_html(&line[5..])));
        } else if line.starts_with("=== ") {
            html.push_str(&format!("<div class=\"adoc-h2\">{}</div>", escape_html(&line[4..])));
        } else if line.starts_with("== ") {
            html.push_str(&format!("<div class=\"adoc-h1\">{}</div>", escape_html(&line[3..])));
        }
        // Admonition blocks
        else if line.starts_with("NOTE: ") {
            html.push_str(&format!("<div class=\"adoc-note\"><strong>Note:</strong> {}</div>", escape_html(&line[6..])));
        } else if line.starts_with("WARNING: ") {
            html.push_str(&format!("<div class=\"adoc-warn\"><strong>Warning:</strong> {}</div>", escape_html(&line[9..])));
        } else if line.starts_with("TIP: ") {
            html.push_str(&format!("<div class=\"adoc-tip\"><strong>Tip:</strong> {}</div>", escape_html(&line[5..])));
        } else if line.starts_with("IMPORTANT: ") {
            html.push_str(&format!("<div class=\"adoc-important\"><strong>Important:</strong> {}</div>", escape_html(&line[11..])));
        }
        // Code block (listing)
        else if line.starts_with("[source") || line == "----" {
            let lang = if line.starts_with("[source") {
                line.trim_start_matches("[source,")
                    .trim_end_matches(']')
                    .trim()
                    .to_string()
            } else {
                String::new()
            };

            // Find the start delimiter
            if line.starts_with("[source") {
                i += 1;
                if i >= lines.len() || lines[i] != "----" {
                    // Not a proper code block, treat as paragraph
                    html.push_str(&format!("<p class=\"adoc-p\">{}</p>", inline_format(line)));
                    i += 1;
                    continue;
                }
            }

            // Collect code block content
            i += 1;
            let mut code = String::new();
            while i < lines.len() && lines[i] != "----" {
                if !code.is_empty() {
                    code.push('\n');
                }
                code.push_str(lines[i]);
                i += 1;
            }

            let lang_badge = if !lang.is_empty() {
                format!("<span class=\"lang-badge\">{}</span>", escape_html(&lang))
            } else {
                String::new()
            };

            html.push_str(&format!(
                "<div class=\"adoc-listing\"><div class=\"adoc-listing-title\">{}</div><pre class=\"adoc-pre\">{}</pre></div>",
                lang_badge,
                escape_html(&code)
            ));
        }
        // Unordered list
        else if line.starts_with("* ") || line.starts_with("- ") {
            html.push_str("<ul class=\"adoc-ul\">");
            while i < lines.len() && (lines[i].starts_with("* ") || lines[i].starts_with("- ")) {
                let item = &lines[i][2..];
                html.push_str(&format!("<li>{}</li>", inline_format(item)));
                i += 1;
            }
            html.push_str("</ul>");
            continue; // don't increment i again
        }
        // Ordered list
        else if line.starts_with(". ") {
            html.push_str("<ol class=\"adoc-ol\">");
            while i < lines.len() && lines[i].starts_with(". ") {
                let item = &lines[i][2..];
                html.push_str(&format!("<li>{}</li>", inline_format(item)));
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
                "<div class=\"adoc-include\"><i class=\"ti ti-file-symlink\"></i><span style=\"color:var(--text3)\">include::</span><span class=\"inc-path\">{}</span></div>",
                escape_html(path)
            ));
        }
        // Table (simple)
        else if line.starts_with("|===") {
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
            if !rows.is_empty() {
                html.push_str("<table class=\"adoc-table\">");
                // First row is header
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
        // Empty line (skip)
        else if line.is_empty() {
            // skip
        }
        // Regular paragraph
        else {
            let mut para = String::from(line);
            // Collect continuation lines
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
            html.push_str(&format!("<p class=\"adoc-p\">{}</p>", inline_format(&para)));
        }

        i += 1;
    }

    html.push_str("</div>");
    html
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

    // Now render with diff classes
    let mut html = String::from("<div class=\"rendered\">");
    let mut i = 0;

    while i < source_lines.len() {
        let line = source_lines[i];
        let is_changed = changed_source_lines.get(i).copied().unwrap_or(false);
        let wrap_class = if is_changed {
            match change_type {
                "del" => " diff-wrap-del",
                "add" => " diff-wrap-add",
                _ => " diff-wrap-mod",
            }
        } else {
            ""
        };

        // Simplified rendering with diff class injection
        if line.starts_with("= ") && !line.starts_with("== ") {
            html.push_str(&format!("<div class=\"adoc-title{}\">{}</div>", wrap_class, escape_html(&line[2..])));
        } else if line.starts_with("== ") {
            html.push_str(&format!("<div class=\"adoc-h1{}\">{}</div>", wrap_class, escape_html(&line[3..])));
        } else if line.starts_with("=== ") {
            html.push_str(&format!("<div class=\"adoc-h2{}\">{}</div>", wrap_class, escape_html(&line[4..])));
        } else if line.starts_with("==== ") {
            html.push_str(&format!("<div class=\"adoc-h3{}\">{}</div>", wrap_class, escape_html(&line[5..])));
        } else if !line.is_empty() && !line.starts_with(':') && !line.starts_with('[') && !line.starts_with("|===") && !line.starts_with("----") {
            html.push_str(&format!("<p class=\"adoc-p{}\">{}</p>", wrap_class, inline_format(line)));
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
    fn test_render_headings() {
        let html = render_asciidoc("== Section One\n=== Subsection");
        assert!(html.contains("adoc-h1"));
        assert!(html.contains("adoc-h2"));
    }

    #[test]
    fn test_render_code_block() {
        let src = "[source,rust]\n----\nfn main() {}\n----";
        let html = render_asciidoc(src);
        assert!(html.contains("adoc-listing"));
        assert!(html.contains("fn main()"));
    }

    #[test]
    fn test_render_note() {
        let html = render_asciidoc("NOTE: This is important");
        assert!(html.contains("adoc-note"));
    }

    #[test]
    fn test_render_include() {
        let html = render_asciidoc("include::partials/_header.adoc[]");
        assert!(html.contains("adoc-include"));
        assert!(html.contains("partials/_header.adoc"));
    }

    #[test]
    fn test_escape_html() {
        assert_eq!(escape_html("<script>"), "&lt;script&gt;");
    }

    #[test]
    fn test_diff_produces_markers() {
        let left = "= Title\n\nOld paragraph";
        let right = "= Title\n\nNew paragraph";
        let (_, right_html) = compute_diff_html(left, right);
        assert!(right_html.contains("diff-wrap-add"));
    }
}
