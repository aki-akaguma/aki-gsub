macro_rules! help_msg {
    () => {
        concat!(
            version_msg!(),
            "\n",
            indoc::indoc!(
                r#"
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
              -X <x-options>    x options. try -X help

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
            "#
            ),
            "\n",
        )
    };
}

macro_rules! x_help_msg {
    () => {
        concat!(
            indoc::indoc!(
                r#"
            Options:
              -X rust-version-info     display rust version info and exit
              -X base_dir=<path>       set <path> is base directory
            "#
            ),
            "\n",
        )
    };
}

macro_rules! x_rvi_msg {
    () => {
        indoc::indoc!(
            r#"
        rustc \d+\.\d+\.\d+(-(beta\.\d+|nightly))? \(.* \d+-\d+-\d+\)
        aki-gsub v\d+\.\d+\.\d+
        (.|\n)*
        ├── regex v\d+\.\d+\.\d+
        (.|\n)*
        └── runnel v\d+\.\d+\.\d+
        (.|\n)*
        \[build-dependencies\]
        ├── rust-version-info-file v\d+\.\d+\.\d+
        └── rustc_version v\d+\.\d+\.\d+ \(\*\)
        \[dev-dependencies\]
        ├── assert-text v\d+\.\d+\.\d+
        (.|\n)*
        ├── exec-target v\d+\.\d+\.\d+
        └── indoc v\d+\.\d+\.\d+ \(proc-macro\)
        
        "#
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
            #[allow(clippy::box_default)]
            Box::new(StringOut::default()),
            #[allow(clippy::box_default)]
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
            #[allow(clippy::box_default)]
            Box::new(StringOut::default()),
            #[allow(clippy::box_default)]
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
        assert!(r.is_ok());
    }
    #[test]
    fn test_help_long() {
        let (r, sioe) = do_execute!(&["--help"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), help_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_version() {
        let (r, sioe) = do_execute!(&["-V"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), version_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_version_long() {
        let (r, sioe) = do_execute!(&["--version"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), version_msg!());
        assert!(r.is_ok());
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
        assert!(r.is_err());
    }
    #[test]
    fn test_x_option() {
        let (r, sioe) = do_execute!(&["-X"]);
        #[rustfmt::skip]
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(),
                ": ",
                "Missing option argument: X\n",
                "Missing option: e\n",
                try_help_msg!()
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    #[test]
    fn test_x_option_help() {
        let (r, sioe) = do_execute!(&["-X", "help"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), x_help_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_x_option_rvi() {
        use assert_text::assert_text_match;
        //
        let (r, sioe) = do_execute!(&["-X", "rust-version-info"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_text_match!(buff!(sioe, sout), x_rvi_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_x_option_invalid() {
        let (r, sioe) = do_execute!(&["-X", "red"]);
        #[rustfmt::skip]
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(),
                ": ",
                "Invalid option argument: X: can not parse 'red'\n",
                "Missing option: e\n",
                try_help_msg!()
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    #[test]
    fn test_opt_color() {
        let (r, sioe) = do_execute!(&["--color"]);
        #[rustfmt::skip]
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(),
                ": ",
                "Missing option argument: color\n",
                "Missing option: e\n",
                try_help_msg!()
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    #[test]
    fn test_opt_color_invalid() {
        let (r, sioe) = do_execute!(&["--color", "red"]);
        #[rustfmt::skip]
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(),
                ": ",
                "Invalid option argument: color: can not parse 'red'\n",
                "Missing option: e\n",
                try_help_msg!()
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    #[test]
    fn test_opt_color_auto() {
        let (r, sioe) = do_execute!(&["--color", "auto"]);
        #[rustfmt::skip]
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(),
                ": ",
                "Missing option: e\n",
                try_help_msg!()
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    #[test]
    fn test_no_format() {
        let (r, sioe) = do_execute!(&["-e", "."]);
        #[rustfmt::skip]
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(),
                ": ",
                "Missing option: e or f\n",
                try_help_msg!()
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
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
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t2() {
        let (r, sioe) = do_execute!(&["-e", "a(b)c", "-f", "$1"], "abcabca");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "bba\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t3() {
        let (r, sioe) = do_execute!(&["-e", "a(b)c", "-f", "$0"], "abcabca");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "abcabca\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t4() {
        let (r, sioe) = do_execute!(&["-e", "a(b)c", "-f", "$2"], "abcabca");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "a\n");
        assert!(r.is_ok());
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
        assert!(r.is_ok());
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
        assert!(r.is_ok());
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
        assert!(r.is_ok());
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
        assert!(r.is_ok());
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
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_multi_line_opt_n() {
        let (r, sioe) = do_execute!(&["-e", "a", "-f", "1", "-n"], "abcabca\noooooo\nabcabca\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "1bc1bc1\n1bc1bc1\n");
        assert!(r.is_ok());
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
        assert!(r.is_ok());
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
        assert!(r.is_ok());
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
        assert!(r.is_ok());
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
        assert!(r.is_ok());
    }
}
