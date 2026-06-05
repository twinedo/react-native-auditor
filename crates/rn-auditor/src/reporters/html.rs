use std::fmt::Write;
use std::fs;
use std::path::Path;

use crate::issue::Issue;
use crate::scanner::ProjectScan;

pub fn write_html_report(
    scan: &ProjectScan,
    issues: &[Issue],
    output_path: &Path,
) -> Result<(), String> {
    let html = render_html_report(scan, issues);

    fs::write(output_path, html).map_err(|error| {
        format!(
            "Failed to write HTML report to {}: {error}",
            output_path.display()
        )
    })
}

fn render_html_report(scan: &ProjectScan, issues: &[Issue]) -> String {
    let mut html = String::new();

    html.push_str(
        "<!doctype html>\n\
         <html lang=\"en\">\n\
         <head>\n\
         <meta charset=\"utf-8\">\n\
         <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n\
         <title>React Native Auditor Report</title>\n\
         <style>\n\
         body{font-family:system-ui,sans-serif;max-width:960px;margin:0 auto;padding:2rem;line-height:1.5;color:#1f2937}\
         h1,h2{color:#111827}\
         table{border-collapse:collapse;width:100%}\
         th,td{border:1px solid #d1d5db;padding:.5rem;text-align:left;vertical-align:top}\
         th{background:#f3f4f6}\
         .issue{border:1px solid #d1d5db;border-radius:.375rem;padding:1rem;margin-bottom:1rem}\
         code{background:#f3f4f6;padding:.125rem .25rem;border-radius:.25rem}\
         </style>\n\
         </head>\n\
         <body>\n\
         <h1>React Native Auditor Report</h1>\n",
    );

    write!(
        html,
        "<p><strong>Project path:</strong> {}</p>\n\
         <h2>Project summary</h2>\n\
         <table>\n\
         <tr><th>Project type</th><td>{}</td></tr>\n\
         <tr><th>Package manager</th><td>{}</td></tr>\n\
         </table>\n",
        escape_html(&scan.root.display().to_string()),
        escape_html(scan.project_type.label()),
        escape_html(&scan.package_manager.label()),
    )
    .expect("writing to a String should not fail");

    html.push_str(
        "<h2>Detected files</h2>\n\
         <table>\n\
         <tr><th>File</th><th>Detected</th></tr>\n",
    );

    for (name, detected) in [
        ("package.json", scan.has_package_json),
        ("app.json", scan.has_app_json),
        ("app.config.js", scan.has_app_config_js),
        ("app.config.ts", scan.has_app_config_ts),
        ("eas.json", scan.has_eas_json),
        (".env", scan.has_env),
        (".env.example", scan.has_env_example),
        ("babel.config.js", scan.has_babel_config_js),
        ("metro.config.js", scan.has_metro_config_js),
    ] {
        writeln!(
            html,
            "<tr><td>{}</td><td>{}</td></tr>",
            escape_html(name),
            if detected { "yes" } else { "no" },
        )
        .expect("writing to a String should not fail");
    }

    html.push_str("</table>\n<h2>Detected lockfiles</h2>\n");

    if scan.lockfiles.is_empty() {
        html.push_str("<p>none</p>\n");
    } else {
        html.push_str("<ul>\n");
        for lockfile in &scan.lockfiles {
            writeln!(
                html,
                "<li>{}</li>",
                escape_html(&lockfile.display().to_string())
            )
            .expect("writing to a String should not fail");
        }
        html.push_str("</ul>\n");
    }

    html.push_str("<h2>Issues</h2>\n");

    if issues.is_empty() {
        html.push_str("<p>No issues found.</p>\n");
    } else {
        for issue in issues {
            html.push_str("<article class=\"issue\">\n");
            write!(
                html,
                "<p><strong>Severity:</strong> {}</p>\n\
                 <p><strong>Code:</strong> <code>{}</code></p>\n\
                 <h3>{}</h3>\n\
                 <p>{}</p>\n",
                escape_html(&format!("{:?}", issue.severity)),
                escape_html(&issue.code),
                escape_html(&issue.title),
                escape_html(&issue.message),
            )
            .expect("writing to a String should not fail");

            if let Some(path) = &issue.file_path {
                writeln!(
                    html,
                    "<p><strong>File:</strong> {}</p>",
                    escape_html(&path.display().to_string())
                )
                .expect("writing to a String should not fail");
            }

            html.push_str("</article>\n");
        }
    }

    html.push_str("</body>\n</html>\n");
    html
}

fn escape_html(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());

    for character in value.chars() {
        match character {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            _ => escaped.push(character),
        }
    }

    escaped
}

#[cfg(test)]
mod tests {
    use super::escape_html;

    #[test]
    fn escapes_html_sensitive_characters() {
        assert_eq!(
            escape_html("<script data-name=\"x\">Tom & 'Sam'</script>"),
            "&lt;script data-name=&quot;x&quot;&gt;Tom &amp; &#39;Sam&#39;&lt;/script&gt;"
        );
    }
}
