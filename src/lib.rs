//!
//! the substitude text program.
//!
//! ```text
//! Usage:
//!   aki-gsub \[options\]
//!
//! replace string by rust lang.
//!
//! Options:
//!   -e, --expression <exp>   regular expression
//!   -f, --format <fmt>       replace format
//!   -n, --quiet              no output unmach lines
//!
//!   -H, --help     display this help and exit
//!   -V, --version  display version information and exit
//! ```
//!
//! # Examples
//!
//! command line:
//! ```text
//! echo "abcabca" | aki-gsub -e "a(b)c" -f "*\$1*"
//! ```
//!
//! result output:
//! ```text
//! *b**b*a
//! ```
//!
//! See [`fn execute()`] for this library examples.
//!
//! [`fn execute()`]: crate::execute
//!

#[macro_use]
extern crate anyhow;

mod conf;
mod run;
mod util;

use flood_tide::HelpVersion;
use runnel::StreamIoe;
use std::io::Write;

const TRY_HELP_MSG: &str = "Try --help for help.";

///
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
/// example:
///
/// ```rust
/// use runnel::StreamIoe;
/// use runnel::medium::stdio::{StdErr, StdIn, StdOut};
///
/// let r = libaki_gsub::execute(&StreamIoe {
///     pin: Box::new(StdIn::default()),
///     pout: Box::new(StdOut::default()),
///     perr: Box::new(StdErr::default()),
/// }, "gsub", &["-e", "a(b)c", "-f", "$1"]);
/// ```
///
pub fn execute(sioe: &StreamIoe, prog_name: &str, args: &[&str]) -> anyhow::Result<()> {
    let conf = match conf::parse_cmdopts(prog_name, args) {
        Ok(conf) => conf,
        Err(errs) => {
            for err in errs.iter().take(1) {
                if err.is_help() || err.is_version() {
                    let _r = sioe.pout.lock().write_fmt(format_args!("{}\n", err));
                    return Ok(());
                }
            }
            return Err(anyhow!("{}\n{}", errs, TRY_HELP_MSG));
        }
    };
    run::run(sioe, &conf)
}
