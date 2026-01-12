# MysticRhythm

> *"We sometimes catch a window, a glimpse of what's beyond"* — Rush, Mystic Rhythms

**Personal Cognitive Operating System**

MysticRhythm is a personal ETL (Extract, Transform, Load) pipeline for thoughts and ideas. It's designed to make the human input → output process maximally efficient by capturing thoughts with minimal friction and routing them into appropriate systems.

## Current Status

**Version:** 0.1.0 (Database foundation)

Currently implements:
- Basic CLI structure with subcommands
- SQLite database layer for thought storage
- Schema initialization

## Philosophy

MysticRhythm is not a chatbot or AI assistant. It's a software system that augments human cognition through:
- Capturing thoughts with zero friction
- Externalizing memory and decision-making
- Routing ideas into appropriate systems
- Enabling automation without overengineering

## Installation

Requires Rust 1.70+
```bash
cargo build --release
cargo run -- init
cargo run -- --help
```

## Roadmap

- [x] CLI foundation
- [x] SQLite storage layer
- [ ] Thought insertion and retrieval
- [ ] Apple Reminders ingestion
- [ ] Transform pipeline
- [ ] Obsidian export
- [ ] Weekly review system

## Built With

- Rust
- clap (CLI)
- rusqlite (Database)
- chrono, uuid (Utilities)