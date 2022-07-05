# Broken Link Finder
This project seaches broken hrefs (hypertext references) in HTML \<a\> tags.

# Features
- Parsing a .html document or website via URL.

# Planned Features
- Recursive searches
- Parse HTML documents

# Todo
- Fix website::Website::new() so it allows URLs without the 'https?://' prefix
- Figure why the hell reqwest::client::get requires by-value src/website
- Rework 'LinkParser::cache' to use '&str' instead of 'String'

# Internals Overview
A website, represented by an HTML document

# Log/Output Format
\<status\> \<url\>

# Known Bugs
- Hyperlinks that do not use the HTTP(S) protocol are considered relative links (src/link_parser::is_relative_link()).
- Subdomains (e.g. 'https://sports.yahoo.com') always yield HTTP status 404 (???)