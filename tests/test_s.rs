macro_rules! help_msg {
    () => {
        concat!(
            version_msg!(),
            "\n",
            "Usage:\n",
            "  aki-gsub [options]\n",
            "\n",
            "substitude text command, replace via regex.\n",
            "\n",
            "Options:\n",
            "      --color <when>    use markers to highlight the matching strings\n",
            "  -e, --exp <exp>       regular expression\n",
            "  -f, --format <fmt>    replace format\n",
            "  -n, --quiet           no output unmach lines\n",
            "\n",
            "  -H, --help        display this help and exit\n",
            "  -V, --version     display version information and exit\n",
            "\n",
            "Option Parameters:\n",
            "  <when>    'always', 'never', or 'auto'\n",
            "  <exp>     regular expression can has capture groups\n",
            "  <fmt>     format can has capture group: $0, $1, $2, ...\n",
            "\n",
            "Environments:\n",
            "  AKI_GSUB_COLOR_SEQ_ST     color start sequence specified by ansi\n",
            "  AKI_GSUB_COLOR_SEQ_ED     color end sequence specified by ansi\n",
            "\n",
            "Examples:\n",
            "  Leaving one character between 'a' and 'c', converts 'a' and 'c'\n",
            "  on both sides to '*':\n",
            "    echo \"abcabca\" | aki-gsub -e \"a(.)c\" -f \"*\\$1*\"\n",
            "  result output:\n",
            "    *b**b*a\n",
            "\n",
            "  Converts 'a' to '*' and 'c' to '@':\n",
            "    echo \"abcabca\" | aki-gsub -e \"a\" -f \"*\" -e \"c\" -f \"@\"\n",
            "  result output:\n",
            "    *b@*b@*\n",
            "\n",
        )
    };
}

macro_rules! try_help_msg {
    () => {
        "Try --help for help.\n"
    };
}

macro_rules! program_name {
    () => {
        "aki-gsub"
    };
}

macro_rules! version_msg {
    () => {
        concat!(program_name!(), " ", env!("CARGO_PKG_VERSION"), "\n")
    };
}

/*
macro_rules! fixture_text10k {
    () => {
        "fixtures/text10k.txt"
    };
}
*/

macro_rules! do_execute {
    ($args:expr) => {
        do_execute!($args, "")
    };
    ($args:expr, $sin:expr) => {{
        let sioe = RunnelIoe::new(
            Box::new(StringIn::with_str($sin)),
            Box::new(StringOut::default()),
            Box::new(StringErr::default()),
        );
        let program = env!("CARGO_PKG_NAME");
        let r = execute(&sioe, &program, $args);
        match r {
            Ok(_) => {}
            Err(ref err) => {
                let _ = sioe
                    .perr()
                    .lock()
                    .write_fmt(format_args!("{}: {}\n", program, err));
            }
        };
        (r, sioe)
    }};
    ($env:expr, $args:expr, $sin:expr) => {{
        let sioe = RunnelIoe::new(
            Box::new(StringIn::with_str($sin)),
            Box::new(StringOut::default()),
            Box::new(StringErr::default()),
        );
        let program = env!("CARGO_PKG_NAME");
        let r = execute_env(&sioe, &program, $args, $env);
        match r {
            Ok(_) => {}
            Err(ref err) => {
                let _ = sioe
                    .perr()
                    .lock()
                    .write_fmt(format_args!("{}: {}\n", program, err));
            }
        };
        (r, sioe)
    }};
}

macro_rules! buff {
    ($sioe:expr, serr) => {
        $sioe.perr().lock().buffer_str()
    };
    ($sioe:expr, sout) => {
        $sioe.pout().lock().buffer_str()
    };
}

mod test_s0 {
    use libaki_gsub::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_help() {
        let (r, sioe) = do_execute!(&["-H"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), help_msg!());
        assert_eq!(r.is_ok(), true);
    }
    #[test]
    fn test_help_long() {
        let (r, sioe) = do_execute!(&["--help"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), help_msg!());
        assert_eq!(r.is_ok(), true);
    }
    #[test]
    fn test_version() {
        let (r, sioe) = do_execute!(&["-V"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), version_msg!());
        assert_eq!(r.is_ok(), true);
    }
    #[test]
    fn test_version_long() {
        let (r, sioe) = do_execute!(&["--version"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), version_msg!());
        assert_eq!(r.is_ok(), true);
    }
    #[test]
    fn test_non_option() {
        let (r, sioe) = do_execute!(&[""]);
        #[rustfmt::skip]
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(), ": ",
                "Missing option: e\n",
                "Unexpected argument: \n",
                try_help_msg!()
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert_eq!(r.is_ok(), false);
    }
}

mod test_s1 {
    use libaki_gsub::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_t1() {
        let (r, sioe) = do_execute!(&["-e", "a", "-f", "1"], "abcabca");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "1bc1bc1\n");
        assert_eq!(r.is_ok(), true);
    }
    //
    #[test]
    fn test_t2() {
        let (r, sioe) = do_execute!(&["-e", "a(b)c", "-f", "$1"], "abcabca");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "bba\n");
        assert_eq!(r.is_ok(), true);
    }
    //
    #[test]
    fn test_t3() {
        let (r, sioe) = do_execute!(&["-e", "a(b)c", "-f", "$0"], "abcabca");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "abcabca\n");
        assert_eq!(r.is_ok(), true);
    }
    //
    #[test]
    fn test_t4() {
        let (r, sioe) = do_execute!(&["-e", "a(b)c", "-f", "$2"], "abcabca");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "a\n");
        assert_eq!(r.is_ok(), true);
    }
}

mod test_s1_color {
    use libaki_gsub::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    macro_rules! color_start {
        //() => { "\u{1B}[01;31m" }
        () => {
            "<S>"
        };
    }
    macro_rules! color_end {
        //() => {"\u{1B}[0m"}
        () => {
            "<E>"
        };
    }
    macro_rules! env_1 {
        () => {{
            let mut env = conf::EnvConf::new();
            env.color_seq_start = color_start!().to_string();
            env.color_seq_end = color_end!().to_string();
            env
        }};
    }
    //
    #[test]
    fn test_t1() {
        let env = env_1!();
        let (r, sioe) = do_execute!(
            &env,
            &["-e", "a", "-f", "1", "--color", "always"],
            "abcabca"
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "<S>1<E>bc<S>1<E>bc<S>1<E>\n");
        assert_eq!(r.is_ok(), true);
    }
    //
    #[test]
    fn test_t2() {
        let env = env_1!();
        let (r, sioe) = do_execute!(
            &env,
            &["-e", "a(b)c", "-f", "$1", "--color", "always"],
            "abcabca"
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "<S>b<E><S>b<E>a\n");
        assert_eq!(r.is_ok(), true);
    }
    //
    #[test]
    fn test_t3() {
        let env = env_1!();
        let (r, sioe) = do_execute!(
            &env,
            &["-e", "a(b)c", "-f", "$0", "--color", "always"],
            "abcabca"
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "<S>abc<E><S>abc<E>a\n");
        assert_eq!(r.is_ok(), true);
    }
    //
    #[test]
    fn test_t4() {
        let env = env_1!();
        let (r, sioe) = do_execute!(
            &env,
            &["-e", "a(b)c", "-f", "$2", "--color", "always"],
            "abcabca"
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "<S><E><S><E>a\n");
        assert_eq!(r.is_ok(), true);
    }
}

mod test_s2 {
    use libaki_gsub::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_multi_line() {
        let (r, sioe) = do_execute!(&["-e", "a", "-f", "1"], "abcabca\noooooo\nabcabca\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "1bc1bc1\noooooo\n1bc1bc1\n");
        assert_eq!(r.is_ok(), true);
    }
    //
    #[test]
    fn test_multi_line_opt_n() {
        let (r, sioe) = do_execute!(&["-e", "a", "-f", "1", "-n"], "abcabca\noooooo\nabcabca\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "1bc1bc1\n1bc1bc1\n");
        assert_eq!(r.is_ok(), true);
    }
}

mod test_s2_color {
    use libaki_gsub::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    macro_rules! color_start {
        //() => { "\u{1B}[01;31m" }
        () => {
            "<S>"
        };
    }
    macro_rules! color_end {
        //() => {"\u{1B}[0m"}
        () => {
            "<E>"
        };
    }
    macro_rules! env_1 {
        () => {{
            let mut env = conf::EnvConf::new();
            env.color_seq_start = color_start!().to_string();
            env.color_seq_end = color_end!().to_string();
            env
        }};
    }
    //
    #[test]
    fn test_multi_line() {
        let env = env_1!();
        let (r, sioe) = do_execute!(
            &env,
            &["-e", "a", "-f", "1", "--color", "always"],
            "abcabca\noooooo\nabcabca\n"
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            "<S>1<E>bc<S>1<E>bc<S>1<E>\noooooo\n<S>1<E>bc<S>1<E>bc<S>1<E>\n"
        );
        assert_eq!(r.is_ok(), true);
    }
    //
    #[test]
    fn test_multi_line_opt_n() {
        let env = env_1!();
        let (r, sioe) = do_execute!(
            &env,
            &["-e", "a", "-f", "1", "-n", "--color", "always"],
            "abcabca\noooooo\nabcabca\n"
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            "<S>1<E>bc<S>1<E>bc<S>1<E>\n<S>1<E>bc<S>1<E>bc<S>1<E>\n"
        );
        assert_eq!(r.is_ok(), true);
    }
}

mod test_s3 {
    /*
    use libaki_gsub::*;
    use runnel::RunnelIoe;
    use runnel::medium::stringio::{StringIn, StringOut, StringErr};
    use std::io::Write;
    //
     * can NOT test
    #[test]
    fn test_output_broken_pipe() {
    }
    */
}

mod test_s4 {
    use libaki_gsub::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    //
    // [BUG] thread 'main' panicked at 'begin <= end (4 <= 2) when slicing `$2 :: $0`', /checkout/src/libcore/str/mod.rs:2221:4
    // echo "001cea1eef55.softphone.blizoo.bg" | rust-gsub -e "(.*\\.){0,1}([A-Za-z0-9][A-Za-z0-9\\-]{1,61}(\\.[A-Za-z0-9]{2,}){0,1}(\\.[A-Za-z]{2,}){0,1}\\.[A-Za-z]{2,5})$" -f "\$2 :: \$0"
    //
    #[test]
    fn test_fix_bug_1() {
        let (r, sioe) = do_execute!(&[
                "-e",
                "(.*\\.){0,1}([A-Za-z0-9][A-Za-z0-9\\-]{1,61}(\\.[A-Za-z0-9]{2,}){0,1}(\\.[A-Za-z]{2,}){0,1}\\.[A-Za-z]{2,5})$",
                "-f",
                "$2 :: $0",
            ],
            "001cea1eef55.softphone.blizoo.bg\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            "blizoo.bg :: 001cea1eef55.softphone.blizoo.bg\n"
        );
        assert_eq!(r.is_ok(), true);
    }
    //
    #[test]
    fn test_fix_bug_2() {
        let (r, sioe) = do_execute!(
            &["-e", "ICON=\"[^\"]*\"", "-f", ""],
            "abc ICON=\"ABCDEFG\" defg\n"
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "abc  defg\n");
        assert_eq!(r.is_ok(), true);
    }
}
