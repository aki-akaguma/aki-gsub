# aki-gsub

*aki-gsub* is the substitude text command, replace via regex.

## Features

*aki-gsub*  is the substitude text command, replace via regex.

* command help

```text
aki-gsub --help
```

```text
Usage:
  aki-gsub [options]

substitude text command, replace via regex.

Options:
  -e, --exp <exp>       regular expression
  -f, --format <fmt>    replace format
  -n, --quiet           no output unmach lines

  -H, --help        display this help and exit
  -V, --version     display version information and exit

Examples:
  Leaving one character between 'a' and 'c', converts 'a' and 'c'
  on both sides to '*':
    echo "abcabca" | aki-gsub -e "a(.)c" -f "*\$1*"
  result output:
    *b**b*a

  Converts 'a' to '*' and 'c' to '@'.
    echo "abcabca" | aki-gsub -e "a" -f "*" -e "c" -f "@"
  result output:
    *b@*b@*
```

* minimum support rustc 1.38.0

## Quick install

1. you can install this into cargo bin path:

```text
cargo install aki-gsub
```

2. you can build debian package:

```text
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

## Library example

See [`fn execute()`] for this library examples.

[`fn execute()`]: crate::execute
