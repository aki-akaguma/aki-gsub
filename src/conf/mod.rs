pub use self::parse::parse_cmdopts;
use crate::util::OptUcXParam;
pub use parse::CmdOptConf;

mod parse;

impl CmdOptConf {
    pub fn is_opt_uc_x_help(&self) -> bool {
        for o in self.opt_uc_x.iter() {
            if let OptUcXParam::Help = o {
                return true;
            }
        }
        false
    }
    pub fn is_opt_uc_x_package_version_info(&self) -> bool {
        for o in self.opt_uc_x.iter() {
            if let OptUcXParam::RustVersionInfo = o {
                return true;
            }
        }
        false
    }
}

use regex::Regex;
use std::env;

#[derive(Debug)]
pub struct RegexAndFormat {
    pub regex: Regex,
    pub format: String,
}

//
// ref.) 3-bit and 4-bit color sequence
//   https://en.wikipedia.org/wiki/ANSI_escape_code
// * black letters on white background use: ESC[30;47m
// * red use: ESC[31m
// * bright red use: ESC[1;31m
// * reset colors to their defaults: ESC[39;49m (not supported on some terminals)
// * reset all attributes: ESC[0m
//
static COLOR_START: &str = "\u{1B}[1;31m";
static COLOR_END: &str = "\u{1B}[0m";

pub const ENV_COLOR_SEQ_ST: &str = "AKI_GSUB_COLOR_SEQ_ST";
pub const ENV_COLOR_SEQ_ED: &str = "AKI_GSUB_COLOR_SEQ_ED";

#[derive(Debug)]
pub struct EnvConf {
    pub color_seq_start: String,
    pub color_seq_end: String,
}
impl EnvConf {
    pub fn new() -> Self {
        let mut r = Self::default();
        for (k, v) in env::vars() {
            r.apply_kv(k, v);
        }
        r
    }
    fn apply_kv<K, V>(&mut self, key: K, val: V)
    where
        K: AsRef<std::ffi::OsStr>,
        V: AsRef<std::ffi::OsStr>,
    {
        let key_s = key.as_ref().to_string_lossy();
        match key_s.as_ref() {
            ENV_COLOR_SEQ_ST => {
                self.color_seq_start = val.as_ref().to_string_lossy().into_owned();
            }
            ENV_COLOR_SEQ_ED => {
                self.color_seq_end = val.as_ref().to_string_lossy().into_owned();
            }
            _ => (),
        }
    }
}
impl std::default::Default for EnvConf {
    fn default() -> EnvConf {
        EnvConf {
            color_seq_start: String::from(COLOR_START),
            color_seq_end: String::from(COLOR_END),
        }
    }
}

impl<IKV, K, V> From<IKV> for EnvConf
where
    IKV: IntoIterator<Item = (K, V)>,
    K: AsRef<std::ffi::OsStr>,
    V: AsRef<std::ffi::OsStr>,
{
    fn from(ary: IKV) -> Self {
        // Start with empty strings to allow explicit overrides,
        // but Default should still provide ANSI sequences.
        // Actually, the previous implementation of execute_with_env
        // was calling EnvConf::new() then overriding.
        // Let's match that behavior.
        let mut r = Self::new();
        for a in ary {
            r.apply_kv(a.0, a.1);
        }
        r
    }
}

#[cfg(test)]
mod test {
    use super::*;
    //
    #[test]
    fn test_cmpoptconf_default() {
        let conf = CmdOptConf::default();
        assert!(!conf.is_opt_uc_x_help());
        assert!(!conf.is_opt_uc_x_package_version_info());
    }
    #[test]
    fn test_envconf_default() {
        let env = EnvConf::default();
        assert_eq!(env.color_seq_start, COLOR_START);
        assert_eq!(env.color_seq_end, COLOR_END);
    }
}
