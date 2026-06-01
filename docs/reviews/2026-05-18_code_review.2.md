# Code Review for `aki-gsub`

## 1. Overview
`aki-gsub` is a well-engineered command-line tool for regex-based text substitution. The code follows Rust best practices, is highly modular, and exhibits a strong focus on testability and reliability. The separation of concerns between the CLI interface, library logic, and core processing is commendable.

## 2. Architecture and Design
- **Library/Binary Split**: The project is correctly structured with a library crate (`libaki_gsub`) and a thin binary wrapper (`main.rs`). This is an excellent design for CLI tools, facilitating integration testing and potential reuse.
- **I/O Abstraction**: The use of the `runnel` crate for I/O abstraction is a major highlight. It allows the core logic to be tested in memory without actual file or stream access, which is clearly leveraged in the comprehensive test suite.
- **Configuration Management**: The configuration module effectively uses `flood-tide` for robust argument parsing. The use of environment variables for ANSI color sequences is a nice touch for customizability.

## 3. Core Logic Analysis (`src/run.rs`)
- **Multiple Regex Support**: The tool supports multiple `-e` and `-f` pairs. The current implementation finds all matches for each regex independently and then combines them.
- **Overlapping Matches (Observation)**:
    When multiple regex patterns match overlapping regions of the input line, the current logic sorts them by start position and processes them sequentially. If a new match starts within the range of a previously applied match (`prev_ed >= next_st`), the input text is not duplicated, but the replacement string for the overlapping match is still appended.
    *Example*: `echo "abc" | aki-gsub -e "a" -f "X" -e "abc" -f "Y"` results in `XY`.
    This behavior might be unexpected for some users (who might expect either sequential application or a "first-match-wins" strategy). It is recommended to explicitly document this behavior or consider a non-overlapping matching strategy if it's more aligned with user expectations.
- **Performance**: For each line, all matches for all regexes are collected into a `Vec<ReplacedOut>` before the output string is constructed. This is efficient for typical lines but could be a memory consideration for extremely long lines (multi-megabyte single lines) with thousands of matches.

## 4. Error Handling and Robustness
- **Broken Pipe Handling**: The implementation of `BrokenPipeError` to gracefully handle cases where the output is piped to a command that closes early (like `head`) is excellent. This prevents unnecessary panics and noise on the terminal.
- **Input Validation**: The tool correctly validates that every `-e` option has a corresponding `-f` option during the parsing phase.
- **UTF-8 Safety**: The tool handles UTF-8 validation errors by reporting them as errors rather than panicking, which is the correct approach for a robust text filter.

## 5. Code Quality and Idiomatic Rust
- **Macros**: The project uses some custom macros for testing and configuration, which seem appropriate given the repetitive nature of CLI argument matching.
- **Trait Usage**: The use of traits for error detection (`BrokenPipeError`) is a good use of Rust's type system.
- **Formatting and Style**: The code is clean, follows standard Rust naming conventions, and is well-formatted.

## 6. Suggestions for Improvement
- **Clarify Overlapping Match Behavior**: As noted above, the behavior when regexes overlap could be documented in the help message or README to avoid user confusion.
- **`EnvConf` Refactoring**: The `EnvConf` implementation has some slight duplication between `new`, `from_array`, and the `From` trait implementation. Consolidating the logic to retrieve color sequences from environment variables or a provided map into a single internal method could improve maintainability.
- **Capture Group Documentation**: The help message mentions `$0, $1, $2, ...` and also `${1}`. While both are supported by the `regex` crate, it might be clearer to consistently promote the `${n}` syntax to avoid ambiguity with literal dollar signs.

## 7. Conclusion
The `aki-gsub` project is a high-quality implementation of a text substitution tool. It is robust, well-tested, and maintainable. The minor observation regarding overlapping matches is more of a design choice/documentation point than a technical flaw.

---
Review Date: 2026-05-18
Reviewer: Gemini CLI Agent
