# test_log_collector

A simple utility crate for collecting log messages during testing.

## Overview

`TestLogCollector` is a utility struct that implements the `Write` trait and collects written content into lines. It's designed for testing scenarios where you need to capture and verify log output.

## Features

- Implements `std::io::Write` trait
- Collects content into separate lines
- Provides methods to count and access collected lines
- Thread-safe shared instances via `new_shared()`
- Handles partial lines and flushing

## Usage

### Basic Usage

```rust
use std::io::Write;
use test_log_collector::TestLogCollector;

let mut collector = TestLogCollector::new();
writeln!(collector, "Hello, world!").unwrap();
writeln!(collector, "Another line").unwrap();

assert_eq!(collector.count(), 2);
assert_eq!(collector.clone_lines(), vec!["Hello, world!", "Another line"]);
```

### Shared Instance for Multi-threaded Testing

```rust
use std::io::Write;
use test_log_collector::TestLogCollector;

let log_collector = TestLogCollector::new_shared();
let collector_clone = log_collector.clone();

// Use in a closure or across threads
let logger = Box::new(move |msg: String| {
    let mut collector = collector_clone.lock().unwrap();
    writeln!(collector, "{}", msg).unwrap();
});

// Later, check the results
let collector = log_collector.lock().unwrap();
assert_eq!(collector.count(), 1);
```

## API

### Methods

- `new()` - Creates a new empty collector
- `new_shared()` - Creates a new collector wrapped in `Arc<Mutex<>>`
- `count()` - Returns the number of complete lines collected
- `clone_lines()` - Returns a clone of all collected lines
- `lines()` - Returns a reference to the collected lines
- `clear()` - Clears all collected lines and partial content

### Write Trait

The collector implements `Write` and handles:
- Line breaks (`\n`) to separate lines
- Partial content buffering
- Proper flushing of incomplete lines

## Credits

Created with assistance from Claude Code (claude.ai/code).

## License

Apache or MIT.
