use std::{ffi::OsStr, fs, path::Path};

fn main() {
    convert_recursive("./markdown", "./markup", "./markdown");
}
fn convert_recursive<T, U, V>(markdownroot: U, markuproot: T, dir: V)
where
    T: AsRef<Path>,
    U: AsRef<Path>,
    V: AsRef<Path>,
{
    for entry in std::fs::read_dir(dir).unwrap().flatten() {
        if entry.file_type().unwrap().is_dir() {
            convert_recursive(markdownroot.as_ref(), markuproot.as_ref(), &entry.path());
        } else if entry.file_type().unwrap().is_file() {
            if entry.path().extension() == Some(OsStr::new("md")) {
                convert(&markdownroot, &markuproot, &entry.path())
            }
        }
    }
}
fn convert<T, U, V>(markdownroot: T, markuproot: U, file: V)
where
    T: AsRef<Path>,
    U: AsRef<Path>,
    V: AsRef<Path>,
{
    let input = std::fs::read(&file).unwrap();
    let in_str = String::from_utf8_lossy(&input);
    let out_str = htmlify(
        in_str.to_string(),
        &file
            .as_ref()
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string()
            .split_once(".")
            .unwrap_or((
                file.as_ref()
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string()
                    .as_str(),
                "",
            ))
            .0,
    );
    std::fs::write(
        markuproot
            .as_ref()
            .join(file.as_ref().strip_prefix(markdownroot).unwrap())
            .with_extension("html"),
        out_str,
    )
    .unwrap();
}
fn htmlify(text: String, title: &str) -> String {
    let mut final_text = format!("<!DOCTYPE html>
<html>
    <head>
        <title>{title}</title>
        <link href='https://fonts.googleapis.com/css?family=JetBrains Mono' rel='stylesheet'>
        <script src=\"https://cdn.jsdelivr.net/gh/google/code-prettify@master/loader/run_prettify.js\"></script>
        <style>
            span.pln {{color:#84a0e0}}
            span.pun {{color:#c6d0f5}}
            span.str {{color:#9bc380}}
            span.lit {{color:#df946f}}
            span.kwd {{color:#ca9ee6}}
            span.com {{color:#AAAAAA}}
            body {{text-align: center;background-color: #1B1A21;color: white;font-family: 'JetBrains Mono';font-size: 22px}}
            div {{margin:auto;width:50%;text-align:center}}
            h1 {{margin:auto;max-width: 800px}}
            h2, h3, h4, h5, h6 {{margin:auto;max-width: 700px}}
            p {{margin:auto;max-width: 700px}}
            code {{margin:auto;max-width: 700px;word-break:break-all}}
            span.inline_code {{padding-right: 5px;padding-left: 5px;padding-bottom: 2px;padding-top: 2px;max-width: 700px;font-size: 15px;background-color: #23222B;border: 1px solid white;border-radius: 5px;text-align: left;margin-top:20px;margin-bottom:20px;display:inline}}
            div.has_code {{padding: 5px;max-width: 700px;font-size: 15px;background-color: #23222B;border: 1px solid white;border-radius: 5px;text-align: left;margin-top:20px;margin-bottom:20px}}
            img {{width:500px;padding-top:5px;padding-bottom:5px}}
            figure {{max-width: 500px;margin:auto;border-top-width:1px;border-top-style:solid;border-bottom-width:1px;border-bottom-style:solid;margin-bottom:30px;margin-top:30px}}
            figcaption {{margin-bottom:10px}}
            a {{color: aquamarine}}
        </style>
    </head>
    <body>");
    let mut state = MarkdownState::Normal;
    for mut line in text.lines().map(ToOwned::to_owned) {
        //if line == "" {
        //    if !state.is_code() {
        //        state = MarkdownState::Normal
        //    }
        //    final_text.push_str("<br>")
        //} else
        if !state.is_code() {
            let mut is_para = true;
            if let Ok(parsed) = prse::try_parse!(line, "``` {}") {
                is_para = false;
                let lang: &str = parsed;
                state = MarkdownState::Code;
                final_text.push_str(&format!(
                    "<div class=\"has_code\"><code class=\"prettyprint {lang}\">"
                ));
            } else if line.starts_with("```") {
                is_para = false;
                state = MarkdownState::Code;
                final_text.push_str(&format!(
                    "<div class=\"has_code\"><code class=\"prettyprint\">"
                ));
            } else {
                while let Ok(parsed) = prse::try_parse!(line, "{}`{}`{}") {
                    let (before, inside, after): (&str, &str, &str) = parsed;
                    line = format!(
                        "{before}<span class=\"inline_code\"><code>{inside}</code></span>{after}"
                    );
                }
            }
            if let Ok(parsed) = prse::try_parse!(line, "![{}]({})") {
                let (display, link): (&str, &str) = parsed;
                final_text.push_str("<div><figure><img src=\"");
                final_text.push_str(link);
                final_text.push_str("\"><figcaption>");
                final_text.push_str(display);
                final_text.push_str("</figcaption></div>");
                continue;
            } else {
                while let Ok(parsed) = prse::try_parse!(line, "{}[{}]({}){}") {
                    let (before, display, link, after): (&str, &str, &str, &str) = parsed;
                    if before.ends_with("!") {
                        panic!("Image not embedded correctly, this is likely because it has text before the [... or after the ...)")
                    }
                    line = format!("{before}<a href=\"{link}\">{display}</a>{after}");
                }
            }
            if line == "***" || line == "---" {
                final_text.push_str("<hr>");
                continue;
            }
            while let Ok(parsed) = prse::try_parse!(line, "{}***{}***{}") {
                let (before, inside, after): (&str, &str, &str) = parsed;
                line = format!("{before}<strong><i>{inside}</i></strong>{after}");
            }
            while let Ok(parsed) = prse::try_parse!(line, "{}**{}**{}") {
                let (before, inside, after): (&str, &str, &str) = parsed;
                line = format!("{before}<strong>{inside}</strong>{after}");
            }
            while let Ok(parsed) = prse::try_parse!(line, "{}*{}*{}") {
                let (before, inside, after): (&str, &str, &str) = parsed;
                line = format!("{before}<i>{inside}</i>{after}");
            }

            while let Ok(parsed) = prse::try_parse!(line, "{}^{}^{}") {
                let (before, inside, after): (&str, &str, &str) = parsed;
                line = format!("{before}<sup>{inside}</sup>{after}");
            }

            while let Ok(parsed) = prse::try_parse!(line, "{}~~{}~~{}") {
                let (before, inside, after): (&str, &str, &str) = parsed;
                line = format!("{before}<s>{inside}</s>{after}");
            }
            while let Ok(parsed) = prse::try_parse!(line, "{}~{}~{}") {
                let (before, inside, after): (&str, &str, &str) = parsed;
                line = format!("{before}<sub>{inside}</sub>{after}");
            }

            while let Ok(parsed) = prse::try_parse!(line, "{}=={}=={}") {
                let (before, inside, after): (&str, &str, &str) = parsed;
                line = format!("{before}<mark>{inside}</mark>{after}");
            }
            if let Ok(parsed) = prse::try_parse!(line, "- [ ] {}") {
                if !state.is_todo_list() {
                    final_text.push_str("<div><ul style=\"text-align: left;list-style: none;margin: auto;width: 50%;\">");
                    state = MarkdownState::TodoList;
                }
                let name: &str = parsed;
                final_text.push_str(&format!(
                    "<li><input type=\"checkbox\" name=\"{name}\"><label> {name}</label></li><br>"
                ));
                continue;
            } else if let Ok(parsed) = prse::try_parse!(line, "- [{}] {}") {
                if !state.is_todo_list() {
                    final_text.push_str("<div><ul>");
                    state = MarkdownState::TodoList;
                }
                let (_, name): (char, &str) = parsed;
                final_text.push_str(&format!(
                    "<li><input type=\"checkbox\" name=\"{name}\" checked><label> {name}</label></li><br>"
                ));
                continue;
            } else {
                if state.is_todo_list() {
                    final_text.push_str("</ul></div>");
                    state = MarkdownState::Normal;
                }
            }
            if line.starts_with("- ") {
                if !state.is_ul() {
                    final_text.push_str("<div><ul>");
                }
                final_text.push_str("<li>");
                final_text.push_str(&line[2..]);
                final_text.push_str("</li><br>");
                state = MarkdownState::Ul;
                continue;
            } else {
                if state.is_ul() {
                    final_text.push_str("</ul></div>");
                }
            }
            if line.starts_with("1. ") {
                if !state.is_ol() {
                    final_text.push_str("<div><ol>");
                }
                final_text.push_str("<li>");
                final_text.push_str(&line[3..]);
                final_text.push_str("</li><br>");
                state = MarkdownState::Ol;
                continue;
            } else {
                if state.is_ol() {
                    final_text.push_str("</ol></div>");
                }
            }
            if let Ok(parsed) = prse::try_parse!(line, "# {} {{#{}}}") {
                let (header, id): (&str, &str) = parsed;
                final_text.push_str(&format!("<div><h1 id=\"{id}\">"));
                final_text.push_str(&header);
                final_text.push_str("</div></h1>");
            } else if let Ok(parsed) = prse::try_parse!(line, "## {} {{#{}}}") {
                let (header, id): (&str, &str) = parsed;
                final_text.push_str(&format!("<div><h2 id=\"{id}\">"));
                final_text.push_str(&header);
                final_text.push_str("</div></h2>");
            } else if let Ok(parsed) = prse::try_parse!(line, "### {} {{#{}}}") {
                let (header, id): (&str, &str) = parsed;
                final_text.push_str(&format!("<div><h3 id=\"{id}\">"));
                final_text.push_str(&header);
                final_text.push_str("</div></h3>");
            } else if let Ok(parsed) = prse::try_parse!(line, "#### {} {{#{}}}") {
                let (header, id): (&str, &str) = parsed;
                final_text.push_str(&format!("<div><h4 id=\"{id}\">"));
                final_text.push_str(&header);
                final_text.push_str("</div></h4>");
            } else if let Ok(parsed) = prse::try_parse!(line, "##### {} {{#{}}}") {
                let (header, id): (&str, &str) = parsed;
                final_text.push_str(&format!("<div><h5 id=\"{id}\">"));
                final_text.push_str(&header);
                final_text.push_str("</div></h5>");
            } else if let Ok(parsed) = prse::try_parse!(line, "###### {} {{#{}}}") {
                let (header, id): (&str, &str) = parsed;
                final_text.push_str(&format!("<div><h6 id=\"{id}\">"));
                final_text.push_str(&header);
                final_text.push_str("</div></h6>");
            } else if line.starts_with("# ") {
                final_text.push_str("<div><h1>");
                final_text.push_str(&line[2..]);
                final_text.push_str("</div></h1>");
            } else if line.starts_with("## ") {
                final_text.push_str("<div><h2>");
                final_text.push_str(&line[3..]);
                final_text.push_str("</div></h2>");
            } else if line.starts_with("### ") {
                final_text.push_str("<div><h3>");
                final_text.push_str(&line[4..]);
                final_text.push_str("</div></h3>");
            } else if line.starts_with("#### ") {
                final_text.push_str("<div><h4>");
                final_text.push_str(&line[5..]);
                final_text.push_str("</div></h4>");
            } else if line.starts_with("##### ") {
                final_text.push_str("<div><h5>");
                final_text.push_str(&line[6..]);
                final_text.push_str("</div></h5>");
            } else if line.starts_with("###### ") {
                final_text.push_str("<div><h6>");
                final_text.push_str(&line[7..]);
                final_text.push_str("</div></h6>");
            } else if is_para {
                final_text.push_str("<div><p>");
                final_text.push_str(&line);
                final_text.push_str("</p></div>");
            }
        } else {
            // code
            if line == "```" {
                state = MarkdownState::Normal;
                final_text.push_str("</code></div>");
            } else {
                final_text.push_str(
                    &line
                        .replace("<", "&lt;")
                        .replace(">", "&gt;")
                        .replace(" ", "&nbsp;")
                        .replace("\t", "&nbsp;&nbsp;&nbsp;&nbsp;"),
                );
                final_text.push_str("<br>")
            }
        }
        if line == "" {
            final_text.push_str("<br>");
        }
    }
    final_text.push_str("</body></html>");
    final_text
}
enum MarkdownState {
    Normal,
    Code,
    Ol,
    Ul,
    TodoList,
}
impl MarkdownState {
    fn is_normal(&self) -> bool {
        if let MarkdownState::Normal = self {
            true
        } else {
            false
        }
    }
    fn is_code(&self) -> bool {
        if let MarkdownState::Code = self {
            true
        } else {
            false
        }
    }
    fn is_ol(&self) -> bool {
        if let MarkdownState::Ol = self {
            true
        } else {
            false
        }
    }
    fn is_ul(&self) -> bool {
        if let MarkdownState::Ul = self {
            true
        } else {
            false
        }
    }
    fn is_todo_list(&self) -> bool {
        if let MarkdownState::TodoList = self {
            true
        } else {
            false
        }
    }
}
