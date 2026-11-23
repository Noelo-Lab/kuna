# Known Rust function addresses

This small, repository-authored C program gives its callbacks Rust-style linker names. The `main` wrapper passes the distinct `r17_probe::main` function to `std::rt::lang_start`; using the bare name in emitted Rust instead resolves to the wrapper itself. The dispatcher checks the exact callback, arguments and one invocation. Two functions return callbacks in the `type` and `const` namespaces to exercise Rust raw identifiers. A function inside `r17_probe` returns a root-level callback to check ancestor lookup.

Regenerate with `python3 build.py` and a system C compiler. The builder never executes the output and leaves it non-executable. No upstream binary or code is included.
