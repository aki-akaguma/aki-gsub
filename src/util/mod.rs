pub mod err;

mod opt_uc_x_param;
pub use self::opt_uc_x_param::OptUcXParam;
//pub use self::opt_uc_x_param::OptUcXParamParseError;

mod opt_color_when;
pub use self::opt_color_when::OptColorWhen;
//pub use self::opt_color_when::OptColorWhenParseError;

pub fn colorize(s: &str, start: &str, end: &str) -> String {
    let mut res = String::with_capacity(start.len() + s.len() + end.len());
    res.push_str(start);
    res.push_str(s);
    res.push_str(end);
    res
}
