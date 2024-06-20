//{{{ OptUcXParam
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum OptUcXParam {
    #[default]
    Void,
    Help,
    RustVersionInfo,
    BaseDir(String),
}

impl ::std::str::FromStr for OptUcXParam {
    type Err = OptUcXParamParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let oc = match s {
            "void" => OptUcXParam::Void,
            "help" => OptUcXParam::Help,
            "rust-version-info" => OptUcXParam::RustVersionInfo,
            _ => {
                let bs = "base_dir=";
                if let Some(stripped) = s.strip_prefix(bs) {
                    OptUcXParam::BaseDir(stripped.to_string())
                } else {
                    let s = format!("can not parse '{s}'");
                    return Err(OptUcXParamParseError::new(s));
                }
            }
        };
        Ok(oc)
    }
}

impl ::std::fmt::Display for OptUcXParam {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let s = match self {
            OptUcXParam::Void => "void".into(),
            OptUcXParam::Help => "help".into(),
            OptUcXParam::RustVersionInfo => "rust-version-info".into(),
            OptUcXParam::BaseDir(p) => format!("base_dir=\"{p}\""),
        };
        write!(f, "{s}")
    }
}
//}}} OptUcXParam

//{{{ OptUcXParamParseError
#[derive(Debug)]
pub struct OptUcXParamParseError {
    desc: String,
}

impl OptUcXParamParseError {
    fn new(s: String) -> OptUcXParamParseError {
        OptUcXParamParseError { desc: s }
    }
}

impl ::std::fmt::Display for OptUcXParamParseError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        self.desc.fmt(f)
    }
}

impl ::std::error::Error for OptUcXParamParseError {}
//}}} OptUcXParamParseError

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_ucx_default() {
        let ucx = OptUcXParam::default();
        assert_eq!(format!("{ucx}"), "void");
    }
    #[test]
    fn test_ucx_void() {
        let ucx = OptUcXParam::Void;
        assert_eq!(format!("{ucx}"), "void");
    }
    #[test]
    fn test_ucx_help() {
        let ucx = OptUcXParam::Help;
        assert_eq!(format!("{ucx}"), "help");
    }
    #[test]
    fn test_ucx_rvi() {
        let ucx = OptUcXParam::RustVersionInfo;
        assert_eq!(format!("{ucx}"), "rust-version-info");
    }
    #[test]
    fn test_ucx_base_dir() {
        let ucx = OptUcXParam::BaseDir("abc/defg".to_string());
        assert_eq!(format!("{ucx}"), "base_dir=\"abc/defg\"");
    }
    #[test]
    fn test_from_str_void() {
        let ucx: OptUcXParam = match FromStr::from_str("void") {
            Ok(c) => c,
            Err(_) => {
                unreachable!();
            }
        };
        assert_eq!(ucx, OptUcXParam::Void);
    }
    #[test]
    fn test_from_str_help() {
        let ucx: OptUcXParam = match FromStr::from_str("help") {
            Ok(c) => c,
            Err(_) => {
                unreachable!();
            }
        };
        assert_eq!(ucx, OptUcXParam::Help);
    }
    #[test]
    fn test_from_str_rvi() {
        let ucx: OptUcXParam = match FromStr::from_str("rust-version-info") {
            Ok(c) => c,
            Err(_) => {
                unreachable!();
            }
        };
        assert_eq!(ucx, OptUcXParam::RustVersionInfo);
    }
    #[test]
    fn test_from_str_base_dir() {
        let ucx: OptUcXParam = match FromStr::from_str("base_dir=abc/defg") {
            Ok(c) => c,
            Err(_) => {
                unreachable!();
            }
        };
        assert_eq!(ucx, OptUcXParam::BaseDir("abc/defg".to_string()));
    }
    #[test]
    fn test_from_str_invalid() {
        let _ucx: OptUcXParam = match FromStr::from_str("other") {
            Ok(_c) => _c,
            Err(e) => {
                assert_eq!(e.to_string(), "can not parse \'other\'");
                return;
            }
        };
        unreachable!();
    }
}
