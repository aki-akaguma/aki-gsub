//! the substitude text program.
//!
//! ```text
//! Usage:
//!   aki-gsub [options]
//!
//! substitude text command, replace via regex.
//!
//! Options:
//!       --color <when>    use markers to highlight the matching strings
//!   -e, --exp <exp>       regular expression
//!   -f, --format <fmt>    replace format
//!   -n, --quiet           no output unmach lines
//!
//!   -H, --help        display this help and exit
//!   -V, --version     display version information and exit
//!
//! Option Parameters:
//!   <when>    'always', 'never', or 'auto'
//!   <exp>     regular expression can has capture groups
//!   <fmt>     format can has capture group: $0, $1, $2, ...
//!
//! Environments:
//!   AKI_GSUB_COLOR_SEQ_ST     color start sequence specified by ansi
//!   AKI_GSUB_COLOR_SEQ_ED     color end sequence specified by ansi
//!
//! Examples:
//!   Leaving one character between 'a' and 'c', converts 'a' and 'c'
//!   on both sides to '*':
//!     echo "abcabca" | aki-gsub -e "a(.)c" -f "*\$1*"
//!   result output:
//!     *b**b*a
//!
//!   Converts 'a' to '*' and 'c' to '@':
//!     echo "abcabca" | aki-gsub -e "a" -f "*" -e "c" -f "@"
//!   result output:
//!     *b@*b@*
//! ```
//!
//! # Examples
//!
//! ## Example 1: simple replacements
//!
//! Leaving one character between '`a`' and '`c`',
//! converts '`a`' and '`c`' on both sides to '`*`'.
//!
//! command line:
//! ```text
//! echo "abcabca" | aki-gsub -e "a(.)c" -f "*\$1*"
//! ```
//!
//! result output:
//! ```text
//! *b**b*a
//! ```
//!
//! The `\$1` mean 1st capture.
//!
//!
//! ## Example 2: extracting email address
//!
//! This extracts the email address and prints the name and address in commas.
//!
//! command line:
//! ```text
//! echo "From:Red bear<aki.akaguma@example.com>" | aki-gsub -e "From: ?(.*)<([\w\d_.-]+@[\w\d_-]+\.[\w\d._-]+)>" -f "\$1, \$2"
//! ```
//!
//! result output:
//! ```text
//! Red bear, aki.akaguma@example.com
//! ```
//!
//! The `\$1` mean 1st capture.
//! The `\$2` mean 2nd capture.
//!
//!
//! ## Example 3: multiple format
//!
//! You can specify multiple formats. See following.
//!
//! command line:
//! ```text
//! echo "xxx yyy zzz" | aki-gsub -e "x(x)x" -f "a\$1a" -e "y(y)y" -f "b\$1b"
//! ```
//!
//! result output:
//! ```text
//! axa byb zzz
//! ```
//!
//! The `\$1` mean 1st capture.
//!
//! # Library example
//!
//! See [`fn execute()`] for this library examples.
//!
//! [`fn execute()`]: crate::execute

#[macro_use]
extern crate anyhow;

pub mod conf;
mod run;
mod util;

use flood_tide::HelpVersion;
use runnel::RunnelIoe;
use std::io::Write;

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
///     "gsub", &["-e", r"From: ?(.*)<([\w\d_.-]+@[\w\d_-]+\.[\w\d._-]+)>", "-f", "$1, $2"]);
/// ```
///
/// The `$1` mean 1st capture.
/// The `$2` mean 2nd capture.
///
pub fn execute(sioe: &RunnelIoe, prog_name: &str, args: &[&str]) -> anyhow::Result<()> {
    let env = conf::EnvConf::new();
    execute_env(sioe, prog_name, args, &env)
}

pub fn execute_env(
    sioe: &RunnelIoe,
    prog_name: &str,
    args: &[&str],
    env: &conf::EnvConf,
) -> anyhow::Result<()> {
    let conf = match conf::parse_cmdopts(prog_name, args) {
        Ok(conf) => conf,
        Err(errs) => {
            for err in errs.iter().take(1) {
                if err.is_help() || err.is_version() {
                    let _r = sioe.pout().lock().write_fmt(format_args!("{}\n", err));
                    return Ok(());
                }
            }
            return Err(anyhow!("{}\n{}", errs, TRY_HELP_MSG));
        }
    };
    run::run(sioe, &conf, &env)
}
