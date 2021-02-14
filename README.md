# aki-gsub

*aki-gsub* is the substitude text program.

## Features

*aki-gsub*  is the substitude text by regex.

* example

command:
```
`aki-gsub` -H
```

* minimum support rustc 1.38.0

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

command line:
```
cat text-file | aki-gsub -e "^name: *(.*)$" -f "\$1"
```

command line:
```text
echo "abcabca" | aki-gsub -e "a(b)c" -f "*\$1*"
```

result output:
```text
*b**b*a
```
