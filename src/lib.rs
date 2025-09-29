use std::io::{self, Write};

/// A utility for collecting log messages during testing.
///
/// `TestLogCollector` implements the `Write` trait and collects written content into lines.
/// It's designed for testing scenarios where you need to capture and verify log output.
///
/// # Examples
///
/// ```
/// use std::io::Write;
/// use test_log_collector::TestLogCollector;
///
/// let mut collector = TestLogCollector::new();
/// writeln!(collector, "Hello, world!").unwrap();
/// writeln!(collector, "Another line").unwrap();
///
/// assert_eq!(collector.count(), 2);
/// assert_eq!(collector.clone_lines(), vec!["Hello, world!", "Another line"]);
/// ```
pub struct TestLogCollector {
    lines: Vec<String>,
    current_line: String,
}

impl TestLogCollector {
    /// Creates a new empty collector.
    ///
    /// # Examples
    ///
    /// ```
    /// use test_log_collector::TestLogCollector;
    ///
    /// let collector = TestLogCollector::new();
    /// assert_eq!(collector.count(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            current_line: String::new(),
        }
    }

    /// Clears all collected lines and partial content.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::Write;
    /// use test_log_collector::TestLogCollector;
    ///
    /// let mut collector = TestLogCollector::new();
    /// writeln!(collector, "Some content").unwrap();
    ///
    /// collector.clear();
    /// assert_eq!(collector.count(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.lines.clear();
        self.current_line.clear();
    }

    /// Returns the number of complete lines collected.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::Write;
    /// use test_log_collector::TestLogCollector;
    ///
    /// let mut collector = TestLogCollector::new();
    /// writeln!(collector, "Line 1").unwrap();
    /// writeln!(collector, "Line 2").unwrap();
    ///
    /// assert_eq!(collector.count(), 2);
    /// ```
    pub fn count(&self) -> usize {
        self.lines.len()
    }

    /// Returns a reference to the collected lines.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::Write;
    /// use test_log_collector::TestLogCollector;
    ///
    /// let mut collector = TestLogCollector::new();
    /// writeln!(collector, "Test line").unwrap();
    ///
    /// let lines = collector.lines();
    /// assert_eq!(lines[0], "Test line");
    /// ```
    pub fn lines(&self) -> &Vec<String> {
        &self.lines
    }

    /// Returns a clone of all collected lines.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::Write;
    /// use test_log_collector::TestLogCollector;
    ///
    /// let mut collector = TestLogCollector::new();
    /// writeln!(collector, "Hello, world!").unwrap();
    /// writeln!(collector, "Another line").unwrap();
    ///
    /// assert_eq!(collector.clone_lines(), vec!["Hello, world!", "Another line"]);
    /// ```
    pub fn clone_lines(&self) -> Vec<String> {
        self.lines.clone()
    }

    /// Creates a new collector wrapped in `Arc<Mutex<>>` for shared access.
    ///
    /// This is useful for multi-threaded testing scenarios where you need to
    /// pass the collector across thread boundaries.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::Write;
    /// use test_log_collector::TestLogCollector;
    ///
    /// let log_collector = TestLogCollector::new_shared();
    /// let collector_clone = log_collector.clone();
    ///
    /// // Use in a closure or across threads
    /// let logger = Box::new(move |msg: String| {
    ///     let mut collector = collector_clone.lock().unwrap();
    ///     writeln!(collector, "{}", msg).unwrap();
    /// });
    ///
    /// // Later, check the results
    /// let collector = log_collector.lock().unwrap();
    /// assert_eq!(collector.count(), 0); // No messages written in this example
    /// ```
    pub fn new_shared() -> std::sync::Arc<std::sync::Mutex<Self>> {
        std::sync::Arc::new(std::sync::Mutex::new(Self::new()))
    }
}

impl Write for TestLogCollector {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let s = String::from_utf8_lossy(buf);
        for ch in s.chars() {
            if ch == '\n' {
                self.lines.push(self.current_line.clone());
                self.current_line.clear();
            } else {
                self.current_line.push(ch);
            }
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        if !self.current_line.is_empty() {
            self.lines.push(self.current_line.clone());
            self.current_line.clear();
        }
        Ok(())
    }
}
