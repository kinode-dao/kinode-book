# Kinode Book

"Rust Book"-style introduction and documentation for Kinode OS.

## To build:

Get deps:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install --git https://github.com/nick1udwig/mdBook --branch hf/dont-write-searchindex-json --locked mdbook
cargo install mdbook-linkcheck
cargo install mdbook-webinclude
cargo install --git https://github.com/nick1udwig/mdbook-hide-feature --locked
```

Build and serve book:
```bash
mdbook serve
```

Navigate to http://localhost:3000 to view.

## Conventions

1. Prefer the triple-backtick ("`") codeblocks to single-backtick lines when writing code intended to be copied.
   This is more readable and easy to use for readers following along by copy-pasting commands.
2. Each line in a paragraph should be on a newline.
   When compiled, markdown places these lines into a single paragraph: separate paragraphs must be separated by two newlines.
   Thus, from the reader's perspective, there is no difference.
   From the editor's and reviewer's perspective, though, it is much easier to read diffs of prose that are per-sentence rather than per-paragraph.
3. Do not use "double dashes" in prose (`--`).
   Instead use "em dashes" (`—`).
