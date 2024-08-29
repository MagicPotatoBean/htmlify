# [zoe.soutter.com](/)
---

# HTMLify

[Github repo](https://github.com/MagicPotatoBean/htmlify)

HTMLify is a project I made specifically for this website, as I wanted a way to ensure all of my pages were formatted identically and so I didnt need to do tons of manual HTML editing, so i wrote this markdown parser to convert markdown into HTML with this styling!

Lots of markdown is easy to test for, i.e. headers were super simple to detect:
``` rs
// --snip--
} else if line.starts_with("# ") {
    final_text.push_str("<div><h1>");
    final_text.push_str(&line[2..]);
    final_text.push_str("</div></h1>");
} else if line.starts_with("## ") {
// --snip--
```
But for some things, like links, I might have several per line, or could have none, so had to write the following code to replace all instances of a link, with the HTML equivalent (and this happens before the rest of the parsing i.e. headers and such)
``` rs
// --snip--
while let Ok(parsed) = prse::try_parse!(line, "{}[{}]({}){}") {
    let (before, display, link, after): (&str, &str, &str, &str) = parsed;
    if before.ends_with("!") {
        panic!("Image not embedded correctly, this is likely because it has text before the [... or after the ...)")
    }
    line = format!("{before}<a href=\"{link}\">{display}</a>{after}");
}
// --snip--
```
Then at the end, I stick all of the HTMLified markdown into an HTML template I made earlier with embedded styling (and syntax highlighting thanks to Google's [code-prettify](https://github.com/googlearchive/code-prettify))
