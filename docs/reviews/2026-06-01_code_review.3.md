# Code Review for aki-gsub

## Overview
`aki-gsub` is a Rust-based command-line utility for performing text substitution using regular expressions. It supports multiple regex patterns and replacement formats, optional color highlighting for matches, and handles standard I/O streams efficiently.

## Logic and Implementation

### 1. Sequential Replacement Behavior
The tool applies multiple regex replacements sequentially to each line.
- **Observation:** If multiple `-e`/`-f` pairs are provided, the result of one replacement can be the input for the next.
- **Example:** `-e a -f b -e b -f c` on input `a` results in `c`.
- **Recommendation:** This is standard behavior for many filters, but ensure it is clearly documented as it differs from tools that perform simultaneous replacements.

### 2. Redundant Regex Search
In `src/run.rs`, the code performs an `is_match` check before calling `replace_all`.
- **Code:**
  ```rust
  if re.is_match(&current_line) {
      any_matched = true;
      let replaced = re.replace_all(&current_line, ...);
      current_line = replaced.into_owned();
  }
  ```
- **Issue:** `replace_all` internally performs a search. Calling `is_match` beforehand doubles the search effort for every matching line.
- **Recommendation:** Use `replace_all` directly and check the returned `Cow<str>`. If it is `Cow::Owned`, it means at least one replacement occurred.

### 3. Highlighting (Coloring) Logic
The color highlighting is implemented within the `replace_all` closure.
- **Observation:** When coloring is enabled, every match is wrapped in ANSI escape sequences.
- **Performance:** For large files with many matches, this approach is efficient as it avoids a separate pass for highlighting.

### 4. Error Handling
- **Observation:** The project implements a `BrokenPipeError` trait to handle `EPIPE` errors gracefully. This is an excellent practice for CLI tools intended to be used in pipelines (e.g., `aki-gsub ... | head`).

## Performance and Design

### 1. Line-by-Line Processing
The tool processes input line-by-line using `sioe.pg_in().lines()`.
- **Pros:** Low memory footprint even for very large files.
- **Cons:** Cannot match patterns that span across multiple lines. This is a common design choice for line-oriented filters.

### 2. Regex Compilation
Regexes are compiled once at the start of the `run` function.
- **Observation:** This is appropriate for a CLI tool where the overhead of compilation is negligible compared to the total execution time.

### 3. Dependency Management
- **Observation:** The project uses `flood-tide` for argument parsing and `runnel` for I/O abstraction. These appear to be specialized libraries that provide good control over stream management.

## Testing
- **Observation:** The integration tests in `tests/` are very comprehensive. They cover:
    - Basic and complex regex features (capture groups, named captures).
    - Edge cases (empty matches, overlapping matches).
    - Error conditions (invalid UTF-8, regex parse errors).
    - Pipeline behavior (broken pipes).
    - Performance (large input).
- **Quality:** The test suite is robust and provides high confidence in the tool's correctness.

## Suggestions for Improvement

### Optimization of `do_match_proc` in `src/run.rs`
Refactor the loop to avoid redundant searches:

```rust
for regfmt in regfmts {
    let re = &regfmt.regex;
    let fmt = &regfmt.format;

    let replaced = re.replace_all(&current_line, |caps: &regex::Captures| {
        let mut expanded = String::new();
        caps.expand(fmt, &mut expanded);
        if color_is_always {
            let mut res = String::with_capacity(
                color_start_s.len() + expanded.len() + color_end_s.len(),
            );
            res.push_str(color_start_s);
            res.push_str(&expanded);
            res.push_str(color_end_s);
            res
        } else {
            expanded
        }
    });

    if let std::borrow::Cow::Owned(s) = replaced {
        any_matched = true;
        current_line = s;
    }
}
```

---
Review Date: 2026-06-01
Reviewer: Gemini CLI Agent
