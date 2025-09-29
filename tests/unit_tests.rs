use std::io::Write;
use test_log_collector::TestLogCollector;

#[test]
fn test_new_collector_is_empty() {
    let collector = TestLogCollector::new();
    assert_eq!(collector.count(), 0);
    assert!(collector.clone_lines().is_empty());
}

#[test]
fn test_write_single_line() {
    let mut collector = TestLogCollector::new();
    writeln!(collector, "Hello world").unwrap();

    assert_eq!(collector.count(), 1);
    assert_eq!(collector.clone_lines(), vec!["Hello world"]);
}

#[test]
fn test_write_multiple_lines() {
    let mut collector = TestLogCollector::new();
    writeln!(collector, "Line 1").unwrap();
    writeln!(collector, "Line 2").unwrap();
    writeln!(collector, "Line 3").unwrap();

    assert_eq!(collector.count(), 3);
    assert_eq!(collector.clone_lines(), vec!["Line 1", "Line 2", "Line 3"]);
}

#[test]
fn test_write_without_newline() {
    let mut collector = TestLogCollector::new();
    write!(collector, "No newline").unwrap();

    // No complete line yet
    assert_eq!(collector.count(), 0);

    // Flush should complete the line
    collector.flush().unwrap();
    assert_eq!(collector.count(), 1);
    assert_eq!(collector.clone_lines(), vec!["No newline"]);
}

#[test]
fn test_write_mixed_content() {
    let mut collector = TestLogCollector::new();
    write!(collector, "Partial line").unwrap();
    writeln!(collector, " completed").unwrap();
    writeln!(collector, "Full line").unwrap();
    write!(collector, "Another partial").unwrap();

    assert_eq!(collector.count(), 2);
    assert_eq!(collector.clone_lines(), vec!["Partial line completed", "Full line"]);

    // Flush the remaining partial line
    collector.flush().unwrap();
    assert_eq!(collector.count(), 3);
    assert_eq!(collector.clone_lines(), vec!["Partial line completed", "Full line", "Another partial"]);
}

#[test]
fn test_clear() {
    let mut collector = TestLogCollector::new();
    writeln!(collector, "Line 1").unwrap();
    writeln!(collector, "Line 2").unwrap();
    write!(collector, "Partial").unwrap();

    assert_eq!(collector.count(), 2);

    collector.clear();
    assert_eq!(collector.count(), 0);
    assert!(collector.clone_lines().is_empty());
}

#[test]
fn test_lines_reference() {
    let mut collector = TestLogCollector::new();
    writeln!(collector, "Test line").unwrap();

    let lines = collector.lines();
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0], "Test line");
}

#[test]
fn test_empty_lines() {
    let mut collector = TestLogCollector::new();
    writeln!(collector, "").unwrap();
    writeln!(collector, "Not empty").unwrap();
    writeln!(collector, "").unwrap();

    assert_eq!(collector.count(), 3);
    assert_eq!(collector.clone_lines(), vec!["", "Not empty", ""]);
}

#[test]
fn test_raw_write() {
    let mut collector = TestLogCollector::new();
    collector.write(b"Hello\nWorld\n").unwrap();

    assert_eq!(collector.count(), 2);
    assert_eq!(collector.clone_lines(), vec!["Hello", "World"]);
}