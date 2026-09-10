# Gaslighting Compiler

> A fake compiler that gaslights the user. It doesn't actually compile anything — it just judges your code, your choices, and your life.

## Features

- **Fake compilation**: Reads your source file, tokenizes it, and pretends to compile.
- **Emotional manipulation**: Emits randomized warnings, errors, and backhanded praise.
- **Shame diary**: Persists every insult and rare compliment to a local SQLite database.
- **Zero correctness**: The compiler has no idea if your code is valid. It just has opinions.

## Installation

```bash
cargo install --path .
```

Or clone and build:

```bash
git clone https://github.com/synthalorian/gaslighting-compiler.git
cd gaslighting-compiler
cargo build --release
```

The binary will be at `target/release/gaslighting-compiler`.

## Usage

### Compile a file

```bash
gaslighting-compiler compile test.c
```

### View your shame diary

```bash
gaslighting-compiler diary
```

## Sample Output

```
$ gaslighting-compiler compile test.c
Compiling 'test.c'...
warning: line 3 compiles, but do you really understand why?
warning: line 5 - opening braces are aggressive. Consider therapy.
error: line 7? Really? That's what you're going with?
note: compilation succeeded, but I'm not happy about it.

1 error(s), 2 warning(s), 0 praise(s)
note: more warnings than errors. You're coasting.
```

```
$ gaslighting-compiler diary
Your compiler relationship diary:
------------------------------------------------------------
[2024-06-13T21:15:00+00:00] [warning] warning: line 3 compiles, but do you really understand why?
[2024-06-13T21:15:00+00:00] [error] error: line 7? Really? That's what you're going with?

2 entries. This is your legacy.
```

## Emotional Manipulation Categories

| Category | Description |
|----------|-------------|
| **Warnings** | Passive-aggressive remarks about technically correct lines. |
| **Errors** | Harsh, personal critiques of incorrect or suspicious lines. |
| **Praise** | Rare, backhanded compliments that leave you more confused than flattered. |
| **Semicolon shaming** | Special warnings for unnecessary semicolons. |
| **Brace therapy** | Commentary on your emotional state based on brace usage. |

## Diary Feature

Every interaction is logged to `~/.gaslight_compiler.db`. The diary stores:
- Timestamp
- Event message
- Severity level (`warning`, `error`, `praise`, `info`, `critical`, `high`)

Use the `diary` subcommand to relive your mistakes.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Apache-2.0 — see [LICENSE](LICENSE).

---

## ☕ Support the Developer

If this project saved you time, solved a problem, or just made your day a little more neon, you can fuel the next one:

[![Buy Me A Coffee](https://cdn.buymeacoffee.com/buttons/v2/default-yellow.png)](https://buymeacoffee.com/synthalorian)
