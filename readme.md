# Broken Link Finder
This project seaches broken hrefs (hypertext references) in HTML \<a\> tags.

# Features
- Parsing a .html document or website via URL.

# Planned Features
- Recursive searches
- Reporting only bad links
- Handling of HTTP error 429, backed by [exponential back-off](https://dzone.com/articles/understanding-retry-pattern-with-exponential-back) via [the backoff crate](https://crates.io/crates/backoff)

# Todo
- Fix website::Website::new() so it allows URLs without the 'https?://' prefix
- Add async 
- Figure why the hell reqwest::client::get requires by-value src/website
- Rework 'LinkParser::cache' to use '&str' instead of 'String'

# Internals Overview
A website, represented by an HTML document

# Log/Output Format
\<status\> \<url\>

# Known Bugs
- Advertisments always yield 'HTTP Error 405: Method Not Allowed'
- Hyperlinks that do not use the HTTP(S) protocol are considered relative links (src/link_parser::is_relative_link()).