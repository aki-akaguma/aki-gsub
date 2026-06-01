use crate::conf::{CmdOptConf, EnvConf, RegexAndFormat};
use crate::util::err::BrokenPipeError;
use crate::util::OptColorWhen;
use regex::Regex;
use runnel::RunnelIoe;

pub fn run(sioe: &RunnelIoe, conf: &CmdOptConf, env: &EnvConf) -> anyhow::Result<()> {
    let mut regfmts: Vec<RegexAndFormat> = Vec::new();
    for i in 0..conf.opt_exp.len() {
        let pat = &conf.opt_exp[i];
        let fmt = &conf.opt_format[i];
        let re = Regex::new(pat)?;
        regfmts.push(RegexAndFormat {
            format: fmt.clone(),
            regex: re,
        });
    }
    let r = do_match_proc(sioe, conf, env, &regfmts);
    if r.is_broken_pipe() {
        return Ok(());
    }
    r
}

fn do_match_proc(
    sioe: &RunnelIoe,
    conf: &CmdOptConf,
    env: &EnvConf,
    regfmts: &[RegexAndFormat],
) -> anyhow::Result<()> {
    let color_start_s = env.color_seq_start.as_str();
    let color_end_s = env.color_seq_end.as_str();
    let color_is_always = matches!(conf.opt_color, OptColorWhen::Always);

    for line in sioe.pg_in().lines() {
        let line_s = line?;
        let mut current_line = line_s.clone();
        let mut any_matched = false;

        for regfmt in regfmts {
            let re = &regfmt.regex;
            let fmt = &regfmt.format;

            let replaced = re.replace_all(&current_line, |caps: &regex::Captures| {
                let mut expanded = String::new();
                caps.expand(fmt, &mut expanded);
                if color_is_always {
                    crate::util::colorize(&expanded, color_start_s, color_end_s)
                } else {
                    expanded
                }
            });

            if let std::borrow::Cow::Owned(s) = replaced {
                any_matched = true;
                current_line = s;
            }
        }

        if any_matched {
            sioe.pg_out().write_line(current_line)?;
        } else if !conf.flg_quiet {
            sioe.pg_out().write_line(line_s)?;
        }
    }
    sioe.pg_out().flush_line()?;
    //
    Ok(())
}
