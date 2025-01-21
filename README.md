AnyAttr bug repro for compilation errors with https://github.com/leptos-rs/leptos/pull/3461.

To repro, from main branch, run 
`cargo +stable build --bin demo --features ssr`

For me on macos, compiles for 10ish minutes, uses 50GB of ram, then errors with:
`error: cannot encode offset of relocations; object file too large`

To fix, in the workspace `Cargo.toml` change the leptos branch to `"any-attr-disabled"`, 
see the extra commit on that branch for the minimal change that allows compilation to succeed in about 1m30s.

Leptos fork being depended upon:
https://github.com/zakstucke/leptos