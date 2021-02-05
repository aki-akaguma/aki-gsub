# rust-gsub

*rust-gsub* is the substitude text program.

## Features

*rust-gsub*  is the substitude text by regex.

* example

command:
```
`rust-gsub` -H
```

* minimum support rustc 1.38.0

## Quick install

-1. you can install this into cargo bin path:

```
cargo install rust-gsub
```

-2. you can build debian package:

```
cargo deb
```
    and install .deb into your local repository of debian package.

## Examples

```
cat text-file | rust-gsub -e "^name: *(.*)$" -f "\$1"
```
