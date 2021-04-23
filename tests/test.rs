const TARGET_EXE_PATH: &'static str = "target/debug/aki-gsub";

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
            "  -X <x-options>    x options. try -X help\n",
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

macro_rules! fixture_text10k {
    () => {
        "fixtures/text10k.txt"
    };
}

mod helper;

mod test_0 {
    use crate::helper::exec_target;
    //use exec_target::args_from;
    const TARGET_EXE_PATH: &'static str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_help() {
        let oup = exec_target(TARGET_EXE_PATH, &["-H"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, help_msg!());
        assert_eq!(oup.status.success(), true);
    }
    #[test]
    fn test_help_long() {
        let oup = exec_target(TARGET_EXE_PATH, &["--help"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, help_msg!());
        assert_eq!(oup.status.success(), true);
    }
    #[test]
    fn test_version() {
        let oup = exec_target(TARGET_EXE_PATH, &["-V"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, version_msg!());
        assert_eq!(oup.status.success(), true);
    }
    #[test]
    fn test_version_long() {
        let oup = exec_target(TARGET_EXE_PATH, &["--version"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, version_msg!());
        assert_eq!(oup.status.success(), true);
    }
    #[test]
    fn test_non_option() {
        let oup = exec_target(TARGET_EXE_PATH, &[""]);
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": ",
                "Missing option: e\n",
                "Unexpected argument: \n",
                try_help_msg!()
            )
        );
        assert_eq!(oup.stdout, "");
        assert_eq!(oup.status.success(), false);
    }
} // mod test_0

mod test_1 {
    use crate::helper::exec_target_with_in;
    const TARGET_EXE_PATH: &'static str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_t1() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            &["-e", "a", "-f", "1"],
            b"abcabca" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "1bc1bc1\n");
        assert_eq!(oup.status.success(), true);
    }
    //
    #[test]
    fn test_t2() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            &["-e", "a(b)c", "-f", "$1"],
            b"abcabca" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "bba\n");
        assert_eq!(oup.status.success(), true);
    }
    //
    #[test]
    fn test_t3() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            &["-e", "a(b)c", "-f", "$0"],
            b"abcabca" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "abcabca\n");
        assert_eq!(oup.status.success(), true);
    }
    //
    #[test]
    fn test_t4() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            &["-e", "a(b)c", "-f", "$2"],
            b"abcabca" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "a\n");
        assert_eq!(oup.status.success(), true);
    }
}

mod test_1_color {
    use crate::helper::exec_target_with_env_in;
    use std::collections::HashMap;
    const TARGET_EXE_PATH: &'static str = super::TARGET_EXE_PATH;
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
            let mut env: HashMap<String, String> = HashMap::new();
            env.insert(
                "AKI_GSUB_COLOR_SEQ_ST".to_string(),
                color_start!().to_string(),
            );
            env.insert(
                "AKI_GSUB_COLOR_SEQ_ED".to_string(),
                color_end!().to_string(),
            );
            env
        }};
    }
    //
    #[test]
    fn test_t1() {
        let env = env_1!();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            &["-e", "a", "-f", "1", "--color", "always"],
            env,
            b"abcabca" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "<S>1<E>bc<S>1<E>bc<S>1<E>\n");
        assert_eq!(oup.status.success(), true);
    }
    //
    #[test]
    fn test_t2() {
        let env = env_1!();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            &["-e", "a(b)c", "-f", "$1", "--color", "always"],
            env,
            b"abcabca" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "<S>b<E><S>b<E>a\n");
        assert_eq!(oup.status.success(), true);
    }
    //
    #[test]
    fn test_t3() {
        let env = env_1!();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            &["-e", "a(b)c", "-f", "$0", "--color", "always"],
            env,
            b"abcabca" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "<S>abc<E><S>abc<E>a\n");
        assert_eq!(oup.status.success(), true);
    }
    //
    #[test]
    fn test_t4() {
        let env = env_1!();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            &["-e", "a(b)c", "-f", "$2", "--color", "always"],
            env,
            b"abcabca" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "<S><E><S><E>a\n");
        assert_eq!(oup.status.success(), true);
    }
}

mod test_2 {
    use crate::helper::exec_target_with_in;
    const TARGET_EXE_PATH: &'static str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_multi_line() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            &["-e", "a", "-f", "1"],
            b"abcabca\noooooo\nabcabca\n" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "1bc1bc1\noooooo\n1bc1bc1\n");
        assert_eq!(oup.status.success(), true);
    }
    //
    #[test]
    fn test_multi_line_opt_n() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            &["-e", "a", "-f", "1", "-n"],
            b"abcabca\noooooo\nabcabca\n" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "1bc1bc1\n1bc1bc1\n");
        assert_eq!(oup.status.success(), true);
    }
}

mod test_2_color {
    use crate::helper::exec_target_with_env_in;
    use std::collections::HashMap;
    const TARGET_EXE_PATH: &'static str = super::TARGET_EXE_PATH;
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
            let mut env: HashMap<String, String> = HashMap::new();
            env.insert(
                "AKI_GSUB_COLOR_SEQ_ST".to_string(),
                color_start!().to_string(),
            );
            env.insert(
                "AKI_GSUB_COLOR_SEQ_ED".to_string(),
                color_end!().to_string(),
            );
            env
        }};
    }
    //
    #[test]
    fn test_multi_line() {
        let env = env_1!();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            &["-e", "a", "-f", "1", "--color", "always"],
            env,
            b"abcabca\noooooo\nabcabca\n" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            "<S>1<E>bc<S>1<E>bc<S>1<E>\noooooo\n<S>1<E>bc<S>1<E>bc<S>1<E>\n"
        );
        assert_eq!(oup.status.success(), true);
    }
    //
    #[test]
    fn test_multi_line_opt_n() {
        let env = env_1!();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            &["-e", "a", "-f", "1", "-n", "--color", "always"],
            env,
            b"abcabca\noooooo\nabcabca\n" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            "<S>1<E>bc<S>1<E>bc<S>1<E>\n<S>1<E>bc<S>1<E>bc<S>1<E>\n"
        );
        assert_eq!(oup.status.success(), true);
    }
}

mod test_3 {
    use crate::helper::exec_target;
    const TARGET_EXE_PATH: &'static str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_output_broken_pipe() {
        let cmdstr = format!(
            "cat \"{}\" | \"{}\" -e \"A\" -f a | head -n 2",
            fixture_text10k!(),
            TARGET_EXE_PATH
        );
        let oup = exec_target("sh", &["-c", &cmdstr]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "aBCDEFG\nHIJKLMN\n");
        assert_eq!(oup.status.success(), true);
    }
}

mod test_4 {
    use crate::helper::exec_target_with_in;
    //use exec_target::args_from;
    const TARGET_EXE_PATH: &'static str = super::TARGET_EXE_PATH;

    //
    // [BUG] thread 'main' panicked at 'begin <= end (4 <= 2) when slicing `$2 :: $0`', /checkout/src/libcore/str/mod.rs:2221:4
    // echo "001cea1eef55.softphone.blizoo.bg" | rust-gsub -e "(.*\\.){0,1}([A-Za-z0-9][A-Za-z0-9\\-]{1,61}(\\.[A-Za-z0-9]{2,}){0,1}(\\.[A-Za-z]{2,}){0,1}\\.[A-Za-z]{2,5})$" -f "\$2 :: \$0"
    //
    #[test]
    fn test_fix_bug_1() {
        let oup = exec_target_with_in(TARGET_EXE_PATH,
            &[
                "-e",
                "(.*\\.){0,1}([A-Za-z0-9][A-Za-z0-9\\-]{1,61}(\\.[A-Za-z0-9]{2,}){0,1}(\\.[A-Za-z]{2,}){0,1}\\.[A-Za-z]{2,5})$",
                "-f",
                "$2 :: $0",
            ],
            b"001cea1eef55.softphone.blizoo.bg\n" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            "blizoo.bg :: 001cea1eef55.softphone.blizoo.bg\n"
        );
        assert_eq!(oup.status.success(), true);
    }
    //
    #[test]
    fn test_fix_bug_2() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            &["-e", "ICON=\"[^\"]*\"", "-f", ""],
            b"abc ICON=\"ABCDEFG\" defg\n" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "abc  defg\n");
        assert_eq!(oup.status.success(), true);
    }
}
