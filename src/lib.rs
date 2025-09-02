/*!
the substitude text program.

# Features

- the substitude text command, replace via regex.
- minimum support rustc 1.65.0 (897e37553 2022-11-02)

# Command help

```text
aki-gsub --help
```

```text
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

# Quick install

1. you can install this into cargo bin path:

```text
cargo install aki-gsub
```

2. you can build debian package:

```text
cargo deb
```

and install **.deb** into your local repository of debian package.

# Examples

## Example 1: simple replacements

Leaving one character between '`a`' and '`c`',
converts '`a`' and '`c`' on both sides to '`*`'.

command line:
```text
echo "abcabca" | aki-gsub -e "a(.)c" -f "*\$1*"
```

result output:
```text
*b**b*a
```

The `\$1` mean 1st capture.


## Example 2: extracting email address

This extracts the email address and prints the name and address in commas.

command line:
```text
echo "From:Red bear<aki.akaguma@example.com>" | aki-gsub -e "From: ?(.*)<([\w\d_.-]+@[\w\d_-]+\.[\w\d._-]+)>" -f "\$1, \$2"
```

result output:
```text
Red bear, aki.akaguma@example.com
```

The `\$1` mean 1st capture.
The `\$2` mean 2nd capture.


## Example 3: multiple format

You can specify multiple formats. See following.

command line:
```text
echo "xxx yyy zzz" | aki-gsub -e "x(x)x" -f "a\$1a" -e "y(y)y" -f "b\$1b"
```

result output:
```text
axa byb zzz
```

The `\$1` mean 1st capture.

# Library example

See [`fn execute()`] for this library examples.

[`fn execute()`]: crate::execute
*/
#[macro_use]
extern crate anyhow;

pub mod conf;
mod run;

#[macro_use]
mod util;

use flood_tide::HelpVersion;
use runnel::RunnelIoe;

const TRY_HELP_MSG: &str = "Try --help for help.";

/// execute gsub
///
/// params:
///   - sioe: stream in/out/err
///   - program: program name. etc. "gsub"
///   - args: parameter arguments.
///
/// return:
///   - ok: ()
///   - err: anyhow
///
/// # Examples
///
/// ## Example 1: simple replacements
///
/// Leaving one character between '`a`' and '`c`',
/// converts '`a`' and '`c`' on both sides to '`*`'.
///
/// ```rust
/// use runnel::RunnelIoeBuilder;
///
/// let r = libaki_gsub::execute(&RunnelIoeBuilder::new().build(),
///     "gsub", &["-e", "a(.)c", "-f", "*$1*"]);
/// ```
///
/// The `$1` mean 1st capture.
///
/// ## Example 2: extracting email address
///
/// This extracts the email address and prints the name and address in commas.
///
/// ```rust
/// use runnel::RunnelIoeBuilder;
///
/// let r = libaki_gsub::execute(&RunnelIoeBuilder::new().build(),
///     "gsub", ["-e", r"From: ?(.*)<([\w\d_.-]+@[\w\d_-]+\.[\w\d._-]+)>", "-f", "$1, $2"]);
/// ```
///
/// The `$1` mean 1st capture.
/// The `$2` mean 2nd capture.
///
pub fn execute<I, S>(sioe: &RunnelIoe, prog_name: &str, args: I) -> anyhow::Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let env = conf::EnvConf::new();
    execute_env(sioe, prog_name, args, &env)
}

pub fn execute_env<I, S>(
    sioe: &RunnelIoe,
    prog_name: &str,
    args: I,
    env: &conf::EnvConf,
) -> anyhow::Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let args: Vec<String> = args
        .into_iter()
        .map(|s| s.as_ref().to_string_lossy().into_owned())
        .collect();
    let args_str: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    //
    match conf::parse_cmdopts(prog_name, &args_str) {
        Ok(conf) => run::run(sioe, &conf, env),
        Err(errs) => {
            if let Some(err) = errs.iter().find(|e| e.is_help() || e.is_version()) {
                sioe.pg_out().write_line(err.to_string())?;
                Ok(())
            } else {
                Err(anyhow!("{errs}\n{TRY_HELP_MSG}"))
            }
        }
    }
}
