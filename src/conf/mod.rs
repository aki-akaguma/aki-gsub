pub use self::parse::parse_cmdopts;
pub use parse::CmdOptConf;

mod parse;

use regex::Regex;

#[derive(Debug)]
pub struct RegexAndFormat {
    pub regex: Regex,
    pub format: String,
}
