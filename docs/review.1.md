# Code Review for aki-gsub

## Overview
`aki-gsub` is a command-line utility for text substitution using regular expressions. It is written in Rust and utilizes the `regex` crate for pattern matching and `flood-tide` for argument parsing. The project structure is clean and follows standard Rust conventions.

## Key Findings

### 1. Critical Bug: Infinite Loop in Format Parsing
In `src/run.rs`, the function `make_replaced_out_one` contains a logic error that leads to an infinite loop when an invalid escape sequence is encountered in the format string.

**Code at fault:**
```rust
        } else {
            cur -= 1;
        }
        //
        st = cur;
```

**Analysis:**
If a `$` character is followed by a character that is not a digit, `$`, or `{` (e.g., `"$x"`), the code enters the `else` branch, decrements `cur` (pointing it back to the `$`), and sets `st = cur`. In the next iteration, `fmt[st..].find('$')` will find the same `$` at the same position, leading to an infinite loop.

**Recommendation:**
Advance `st` past the `$` even if the following character is not a recognized escape sequence, or handle the invalid sequence explicitly.

### 2. Limited Capture Group Support
The manual parsing in `make_replaced_out_one` only supports single-digit capture groups (`$0` through `$9`).

**Code at fault:**
```rust
        let b: u8 = fmt.as_bytes()[cur];
        if b.is_ascii_digit() {
            let i: usize = (b - b'0') as usize;
            if let Some(mat) = caps.get(i) {
                v_out_s.push_str(mat.as_str());
            };
            cur += 1;
```

**Recommendation:**
Support multi-digit capture groups (e.g., `$10`, `$11`) by parsing all consecutive digits following the `$`.

### 3. Handling of Overlapping Matches
When multiple regexes are provided (e.g., `-e "ab" -f "x" -e "bc" -f "y"`), the tool matches all of them against the original line and concatenates the replacements.

**Analysis:**
For the input `"abc"`, the current implementation produces `"xy"`. While this is a deterministic behavior, it may be unexpected for users who expect sequential processing (where the second regex operates on the result of the first) or for the first match to consume the input.

**Recommendation:**
Document this behavior clearly in the README or consider implementing a sequential processing mode.

### 4. Code Refactoring: Use `Captures::expand`
The manual format parsing logic in `make_replaced_out_one` can be simplified and made more robust by using the `regex` crate's built-in expansion capabilities.

**Recommendation:**
Consider using `caps.expand(fmt, &mut v_out_s)`. This would automatically handle multi-digit capture groups, named captures, and literal `$$` escaping, aligning the tool's behavior with standard regex replacement expectations.

## Architecture and Design
- **IO Abstraction**: The use of `runnel::RunnelIoe` for abstracting standard input, output, and error is excellent. it makes the core logic highly testable.
- **Error Handling**: The integration of `anyhow` and the custom `BrokenPipeError` trait shows good attention to CLI usability, especially for handling pipe failures gracefully.
- **Configuration**: The separation of command-line options (`CmdOptConf`) and environment variables (`EnvConf`) is well-implemented.

## Style and Conventions
- The code follows standard Rust naming conventions and formatting.
- Documentation comments are comprehensive and include helpful examples.
- The use of `include!` for generated code is consistent across the project, though modern alternatives like proc-macros or `OUT_DIR` builds are more common in the ecosystem.

## Conclusion
`aki-gsub` is a well-structured tool with clear intent. Addressing the infinite loop bug and improving the capture group parsing (ideally by leveraging the `regex` crate's expansion features) will significantly enhance its reliability and feature completeness.

---
Review Date: 2026-05-17
Reviewer: Gemini CLI Agent
