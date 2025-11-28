# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`test_log_collector` is a minimal Rust utility crate that provides a `TestLogCollector` struct for capturing log output during tests. It implements `std::io::Write` to collect written content into lines.

## Commands

- **Build**: `cargo build`
- **Test**: `cargo test`
- **Run single test**: `cargo test <test_name>`
- **Generate docs**: `cargo doc --open`

## Architecture

This is a single-module library crate with a simple architecture:

- **src/lib.rs**: Contains the `TestLogCollector` struct with:
  - `lines: Vec<String>` - stores complete lines
  - `current_line: String` - buffers partial line content until a newline
  - `Write` trait implementation that splits on `\n` characters
  - `new_shared()` method returns `Arc<Mutex<TestLogCollector>>` for thread-safe usage

- **tests/unit_tests.rs**: Comprehensive unit tests covering basic writes, partial lines, flushing, and clearing

The crate has no dependencies and focuses on a single, well-defined purpose.
