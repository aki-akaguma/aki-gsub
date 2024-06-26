use crate::conf::{CmdOptConf, EnvConf, RegexAndFormat};
use crate::util::err::BrokenPipeError;
use crate::util::OptColorWhen;
use regex::{Captures, Regex};
use runnel::RunnelIoe;
use std::io::{BufRead, Write};

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

struct ReplacedOut {
    st: usize,
    ed: usize,
    out_s: String,
}

fn format(line_offset: usize, caps: &Captures<'_>, fmt: &str) -> ReplacedOut {
    let mut v_out_s = String::new();
    let mat = match caps.get(0) {
        Some(mat) => mat,
        None => {
            return ReplacedOut {
                st: line_offset,
                ed: line_offset,
                out_s: v_out_s,
            }
        }
    };
    //
    let mut st: usize = 0;
    let fmt_len = fmt.len();
    loop {
        let mut cur = match (fmt[st..]).find('$') {
            Some(found) => {
                v_out_s.push_str(&fmt[st..(st + found)]);
                st + found
            }
            None => {
                v_out_s.push_str(&fmt[st..]);
                break;
            }
        };
        //
        cur += 1;
        if cur >= fmt_len {
            break;
        }
        //
        let b: u8 = fmt.as_bytes()[cur];
        if b.is_ascii_digit() {
            let i: usize = (b - b'0') as usize;
            if let Some(mat) = caps.get(i) {
                v_out_s.push_str(mat.as_str());
            };
            cur += 1;
            if cur >= fmt_len {
                break;
            }
        } else {
            cur -= 1;
        }
        //
        st = cur;
    }
    //
    ReplacedOut {
        st: line_offset + mat.start(),
        ed: line_offset + mat.end(),
        out_s: v_out_s,
    }
}

fn do_match_proc(
    sioe: &RunnelIoe,
    conf: &CmdOptConf,
    env: &EnvConf,
    regfmts: &[RegexAndFormat],
) -> anyhow::Result<()> {
    let color_start_s = env.color_seq_start.as_str();
    let color_end_s = env.color_seq_end.as_str();
    let color_is_alyways = matches!(conf.opt_color, OptColorWhen::Always);
    //
    for line in sioe.pin().lock().lines() {
        let line_s = line?;
        let line_ss = line_s.as_str();
        let line_len: usize = line_ss.len();
        //
        //
        let mut routs: Vec<ReplacedOut> = Vec::new();
        //
        for regfmt in regfmts {
            let fmt = &regfmt.format;
            let re = &regfmt.regex;
            let mut st: usize = 0;
            while let Some(caps) = re.captures(&line_ss[st..]) {
                let mat = match caps.get(0) {
                    Some(mat) => mat,
                    None => break,
                };
                let rep_out = format(st, &caps, fmt);
                routs.push(rep_out);
                st += mat.end();
                if st >= line_len {
                    break;
                }
            }
        }
        if !routs.is_empty() {
            //
            routs.sort_unstable_by(|a, b| a.st.cmp(&b.st));
            //
            let mut out_s: String = String::new();
            let mut prev_ed: usize = 0;
            for rout in &routs {
                let next_st = rout.st;
                if prev_ed < next_st {
                    out_s.push_str(&line_ss[prev_ed..next_st]);
                }
                if next_st >= line_len {
                    prev_ed = rout.ed;
                    break;
                }
                if color_is_alyways {
                    out_s.push_str(color_start_s);
                }
                out_s.push_str(rout.out_s.as_str());
                if color_is_alyways {
                    out_s.push_str(color_end_s);
                }
                //
                prev_ed = rout.ed;
            }
            if prev_ed < line_len {
                out_s.push_str(&line_ss[prev_ed..]);
            }
            //
            #[rustfmt::skip]
            sioe.pout().lock().write_fmt(format_args!("{out_s}\n"))?;
        } else if !conf.flg_quiet {
            #[rustfmt::skip]
            sioe.pout().lock().write_fmt(format_args!("{line_ss}\n"))?;
        }
    }
    //
    sioe.pout().lock().flush()?;
    //
    Ok(())
}
