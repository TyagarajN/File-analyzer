# File Analyzer (Rust)

A fast, safe, and extensible **CLI file analyzer** written in Rust.  
This tool scans directories recursively and provides insights into files based on size, type, and timestamps.

🚧 **Status:** Work in progress

---

## ✨ Features

### Current
- Recursive directory scanning
- File metadata extraction:
  - File path
  - File size
  - File extension
  - Last modified time

### Planned
- Group files by extension (e.g. `.log`, `.pdf`, `.mp4`)
- Size analysis (largest / smallest files)
- Date-based filtering (files older than X days)
- Safe deletion with dry-run mode
- Export results to CSV / JSON
- Rich CLI options (`--path`, `--min-size`, `--older-than`)

---

## 🦀 Why Rust?

- **Memory safety** without garbage collection
- **High performance** for large directory scans
- Strong type system for reliable tooling
- Excellent ecosystem for CLI applications

---

## 📦 Tech Stack

- **Rust (Edition 2021)**
- [`walkdir`](https://crates.io/crates/walkdir) – recursive directory traversal
- [`chrono`](https://crates.io/crates/chrono) – date & time handling
- [`anyhow`](https://crates.io/crates/anyhow) – ergonomic error handling

---

## 🚀 Getting Started

### Prerequisites
- Rust installed via `rustup`

```bash
rustc --version
cargo --version
