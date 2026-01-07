# HARV

**Personal Cognitive Operating System**

HARV is a personal ETL (Extract, Transform, Load) pipeline for thoughts and ideas. It's designed to make the human input → output process maximally efficient by capturing thoughts with minimal friction and routing them into appropriate systems.

## Current Status

**Version:** 0.1.0 (CLI foundation)

Currently implements:
- Basic CLI structure with subcommands
- `harv init` - Initialize system
- `harv ingest` - Ingest thoughts (coming soon)
- `harv process` - Process thoughts (coming soon)

## Philosophy

HARV is not a chatbot or AI assistant. It's a software system that augments human cognition through:
- Capturing thoughts with zero friction
- Externalizing memory and decision-making
- Routing ideas into appropriate systems
- Enabling automation without overengineering

## Installation

Requires Rust 1.70+
```bash
cargo build --release
cargo run -- --help
```

## Roadmap

- [x] CLI foundation
- [ ] SQLite storage layer
- [ ] Apple Reminders ingestion
- [ ] Transform pipeline
- [ ] Obsidian export
- [ ] Weekly review system

## Built With

- Rust
- clap (CLI)
- More to come...