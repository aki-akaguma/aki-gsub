# aki-gsub

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![Rust Version][rustc-image]
![Apache2/MIT licensed][license-image]
[![Test ubu][test-ubuntu-image]][test-ubuntu-link]
[![Test mac][test-windows-image]][test-windows-link]
[![Test win][test-macos-image]][test-macos-link]

the substitude text program.

## Features

- the substitude text command, replace via regex.
- minimum support rustc 1.56.1 (59eed8a2a 2021-11-01)

## Command help

```
aki-gsub --help
```

```
Usage:
  aki-gsub [options]

substitude text command, replace via regex.

Options:
      --color <when>    use markers to highlight the matching strings
  -e, --exp <exp>       regular expression
  -f, --format <fmt>    replace format
  -n, --quiet           no output unmach lines

  -H, --help        display this help and exit
  -V, --version     display version information and exit

Option Parameters:
  <when>    'always', 'never', or 'auto'
  <exp>     regular expression can has capture groups
  <fmt>     format can has capture group: $0, $1, $2, ...

Environments:
  AKI_GSUB_COLOR_SEQ_ST     color start sequence specified by ansi
  AKI_GSUB_COLOR_SEQ_ED     color end sequence specified by ansi

Examples:
  Leaving one character between 'a' and 'c', converts 'a' and 'c'
  on both sides to '*':
    echo "abcabca" | aki-gsub -e "a(.)c" -f "*\$1*"
  result output:
    *b**b*a

  Converts 'a' to '*' and 'c' to '@':
    echo "abcabca" | aki-gsub -e "a" -f "*" -e "c" -f "@"
  result output:
    *b@*b@*
```

## Quick install

1. you can install this into cargo bin path:

```
cargo install aki-gsub
```

2. you can build debian package:

```
cargo deb
```

and install **.deb** into your local repository of debian package.

## Examples

### Example 1: simple replacements

Leaving one character between '`a`' and '`c`',
converts '`a`' and '`c`' on both sides to '`*`'.

command line:
```
echo "abcabca" | aki-gsub -e "a(.)c" -f "*\$1*"
```

result output:
```
*b**b*a
```

The `\$1` mean 1st capture.


### Example 2: extracting email address

This extracts the email address and prints the name and address in commas.

command line:
```
echo "From:Red bear<aki.akaguma@example.com>" | aki-gsub -e "From: ?(.*)<([\w\d_.-]+@[\w\d_-]+\.[\w\d._-]+)>" -f "\$1, \$2"
```

result output:
```
Red bear, aki.akaguma@example.com
```

The `\$1` mean 1st capture.
The `\$2` mean 2nd capture.


### Example 3: multiple format

You can specify multiple formats. See following.

command line:
```
echo "xxx yyy zzz" | aki-gsub -e "x(x)x" -f "a\$1a" -e "y(y)y" -f "b\$1b"
```

result output:
```
axa byb zzz
```

The `\$1` mean 1st capture.

## Library example

See [`fn execute()`] for this library examples.

[`fn execute()`]: crate::execute

# Changelogs

[This crate's changelog here.](https://github.com/aki-akaguma/aki-gsub/blob/main/CHANGELOG.md)

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)

at your option.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/aki-gsub.svg
[crate-link]: https://crates.io/crates/aki-gsub
[docs-image]: https://docs.rs/aki-gsub/badge.svg
[docs-link]: https://docs.rs/aki-gsub/
[rustc-image]: https://img.shields.io/badge/rustc-1.58+-blue.svg
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[test-ubuntu-image]: https://github.com/aki-akaguma/aki-gsub/actions/workflows/test-ubuntu.yml/badge.svg
[test-ubuntu-link]: https://github.com/aki-akaguma/aki-gsub/actions/workflows/test-ubuntu.yml
[test-macos-image]: https://github.com/aki-akaguma/aki-gsub/actions/workflows/test-macos.yml/badge.svg
[test-macos-link]: https://github.com/aki-akaguma/aki-gsub/actions/workflows/test-macos.yml
[test-windows-image]: https://github.com/aki-akaguma/aki-gsub/actions/workflows/test-windows.yml/badge.svg
[test-windows-link]: https://github.com/aki-akaguma/aki-gsub/actions/workflows/test-windows.yml
