# ducktabe

**Duck**DB **Tab**ular **E**xplorer

> **Pre-alpha. This was built for personal exploration and is not production-ready.**
> Things are expected to break. I'll work on it when I'm free.

A terminal UI for interactively querying tabular files (Parquet, CSV, and anything else DuckDB can read) with a live SQL editor and scrollable results — all in the terminal.

![pre-alpha](https://img.shields.io/badge/status-pre--alpha-red)

---

## What it does

Open a file, write a SQL query against it (the file is exposed as the view `df`), and press `Ctrl+R` to run it. Results are shown in a scrollable table with column names and types in the header.

That's it. Nothing more, nothing less.

---

## What it looks like

![demo](demo.gif)

---

## Keybindings

| Key                 | Action             |
| ------------------- | ------------------ |
| `Ctrl+R`            | Run query          |
| `Ctrl+W`            | Toggle output wrap |
| `Ctrl` + Arrow keys | Scroll results     |
| `Esc`               | Quit               |

---

## Building

You need [Rust](https://rustup.rs/) installed (stable toolchain).

```bash
git clone https://github.com/yourusername/ducktabe
cd ducktabe
cargo build --release
```

The binary will be at `target/release/ducktabe`.

### Linux

```bash
cargo build --release
./target/release/ducktabe --file path/to/your/file.parquet
```

### macOS

```bash
cargo build --release
./target/release/ducktabe --file path/to/your/file.parquet
```

### Windows

```powershell
cargo build --release
.\target\release\ducktabe.exe --file path\to\your\file.parquet
```

> On Windows, make sure you're running inside Windows Terminal for proper terminal rendering.

---

## A note on LLM usage

The SQL test data generation script (`src/tests/generate_data.sql`) and the initial version of this README were created
with the help of Claude. However, all other code in this repository is completely handwritten - the intent was for me
to build a simple, usable tool for myself in Rust, while learning a good amount of it.
