# Contributing to Gaslighting Compiler

Thank you for considering contributing! This project is a joke, but we take the joke seriously.

## How to Contribute

1. **Fork the repository** and create your branch from `main`.
2. **Make your changes** — add new gaslight messages, improve the tokenizer, or fix bugs.
3. **Ensure CI passes**: `cargo fmt`, `cargo clippy`, and `cargo test` must all succeed.
4. **Write a clear commit message** — no passive-aggressive commit messages, save that for the compiler output.
5. **Open a Pull Request** and describe what you changed and why.

## Code Style

- Run `cargo fmt` before committing.
- Run `cargo clippy -- -D warnings` and fix all issues.
- Keep functions focused and modular.

## Adding New Gaslight Messages

New messages are welcome! Add them to the appropriate vector in `src/main.rs`:

- `random_gaslight_message(..., true)` — warnings (passive-aggressive)
- `random_gaslight_message(..., false)` — errors (harsh)
- `random_praise()` — rare backhanded compliments

Keep them creative, but avoid genuinely harmful or offensive content. The goal is absurd humor, not cruelty.

## Testing

There are no unit tests yet. If you add some, great! Otherwise, manual testing with `cargo run -- compile test.c` is acceptable.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
