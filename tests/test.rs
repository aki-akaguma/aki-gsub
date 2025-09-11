const TARGET_EXE_PATH: &str = env!(concat!("CARGO_BIN_EXE_", env!("CARGO_PKG_NAME")));

#[macro_use]
mod helper;

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
        \[build-dependencies\]
        └── rust-version-info-file v\d+\.\d+\.\d+
        \[dev-dependencies\]
        ├── assert-text v\d+\.\d+\.\d+
        (.|\n)*
        ├── exec-target v\d+\.\d+\.\d+
        └── indoc v\d+\.\d+\.\d+ \(proc-macro\)
        
        "#
        )
    };
}

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

mod test_0 {
    use exec_target::exec_target;
    //use exec_target::args_from;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_help() {
        let oup = exec_target(TARGET_EXE_PATH, ["-H"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, help_msg!());
        assert!(oup.status.success());
    }
    #[test]
    fn test_help_long() {
        let oup = exec_target(TARGET_EXE_PATH, ["--help"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, help_msg!());
        assert!(oup.status.success());
    }
    #[test]
    fn test_version() {
        let oup = exec_target(TARGET_EXE_PATH, ["-V"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, version_msg!());
        assert!(oup.status.success());
    }
    #[test]
    fn test_version_long() {
        let oup = exec_target(TARGET_EXE_PATH, ["--version"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, version_msg!());
        assert!(oup.status.success());
    }
    #[test]
    fn test_invalid_opt() {
        let oup = exec_target(TARGET_EXE_PATH, ["-z"]);
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": ",
                "Invalid option: z\n",
                "Missing option: e\n",
                try_help_msg!()
            )
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    #[test]
    fn test_non_option() {
        let oup = exec_target(TARGET_EXE_PATH, [""]);
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
        assert!(!oup.status.success());
    }
    #[test]
    fn test_opt_color() {
        let oup = exec_target(TARGET_EXE_PATH, ["--color"]);
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": ",
                "Missing option argument: color\n",
                "Missing option: e\n",
                try_help_msg!()
            )
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    #[test]
    fn test_opt_color_invalid() {
        let oup = exec_target(TARGET_EXE_PATH, ["--color", "red"]);
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": ",
                "Invalid option argument: color: can not parse 'red'\n",
                "Missing option: e\n",
                try_help_msg!()
            )
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    #[test]
    fn test_opt_color_auto() {
        let oup = exec_target(TARGET_EXE_PATH, ["--color", "auto"]);
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": ",
                "Missing option: e\n",
                try_help_msg!()
            )
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    #[test]
    fn test_no_format() {
        let oup = exec_target(TARGET_EXE_PATH, ["-e", "."]);
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": ",
                "Missing option: e or f\n",
                try_help_msg!()
            )
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    #[test]
    fn test_no_pair_format() {
        let oup = exec_target(TARGET_EXE_PATH, ["-e", "a", "-e", "b", "-f", "X"]);
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": ",
                "Missing option: e or f\n",
                try_help_msg!()
            )
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
}

mod test_0_x_options {
    use exec_target::exec_target;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_x_option() {
        let oup = exec_target(TARGET_EXE_PATH, ["-X"]);
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": ",
                "Missing option argument: X\n",
                "Missing option: e\n",
                try_help_msg!()
            )
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    //
    #[test]
    fn test_x_option_help() {
        let oup = exec_target(TARGET_EXE_PATH, ["-X", "help"]);
        assert_eq!(oup.stderr, "");
        assert!(oup.stdout.contains("Options:"));
        assert!(oup.stdout.contains("-X rust-version-info"));
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_x_option_rust_version_info() {
        use assert_text::assert_text_match;
        let oup = exec_target(TARGET_EXE_PATH, ["-X", "rust-version-info"]);
        assert_eq!(oup.stderr, "");
        assert!(oup.stdout.contains("rustc"));
        assert_text_match!(&oup.stdout, x_rvi_msg!());
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_multiple_x_options() {
        let oup = exec_target(TARGET_EXE_PATH, ["-X", "help", "-X", "rust-version-info"]);
        assert_eq!(oup.stderr, "");
        // The first one should be executed and the program should exit.
        assert!(oup.stdout.contains("Options:"));
        assert!(!oup.stdout.contains("rustc"));
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_x_option_invalid() {
        let oup = exec_target(TARGET_EXE_PATH, ["-X", "red"]);
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": ",
                "Invalid option argument: X: can not parse 'red'\n",
                "Missing option: e\n",
                try_help_msg!()
            )
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
}

mod test_1 {
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_invalid_utf8() {
        let v = std::fs::read(fixture_invalid_utf8!()).unwrap();
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-e", "a", "-f", "x"], &v);
        assert_eq!(
            oup.stderr,
            concat!(program_name!(), ": stream did not contain valid UTF-8\n",)
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    //
    #[test]
    fn test_t1() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-e", "a", "-f", "1"], b"abcabca" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "1bc1bc1\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t2() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-e", "a(b)c", "-f", "$1"],
            b"abcabca" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "bba\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t3() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-e", "a(b)c", "-f", "$0"],
            b"abcabca" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "abcabca\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t4() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-e", "a(b)c", "-f", "$2"],
            b"abcabca" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "a\n");
        assert!(oup.status.success());
    }
}

mod test_1_color {
    use exec_target::exec_target_with_env_in;
    use std::collections::HashMap;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_t1() {
        let env = env_1!();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-e", "a", "-f", "1", "--color", "always"],
            env,
            b"abcabca" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "<S>1<E>bc<S>1<E>bc<S>1<E>\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t2() {
        let env = env_1!();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-e", "a(b)c", "-f", "$1", "--color", "always"],
            env,
            b"abcabca" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "<S>b<E><S>b<E>a\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t3() {
        let env = env_1!();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-e", "a(b)c", "-f", "$0", "--color", "always"],
            env,
            b"abcabca" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "<S>abc<E><S>abc<E>a\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_t4() {
        let env = env_1!();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-e", "a(b)c", "-f", "$2", "--color", "always"],
            env,
            b"abcabca" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "<S><E><S><E>a\n");
        assert!(oup.status.success());
    }
}

mod test_2 {
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_multi_line() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-e", "a", "-f", "1"],
            b"abcabca\noooooo\nabcabca\n" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "1bc1bc1\noooooo\n1bc1bc1\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_multi_line_opt_n() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-e", "a", "-f", "1", "-n"],
            b"abcabca\noooooo\nabcabca\n" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "1bc1bc1\n1bc1bc1\n");
        assert!(oup.status.success());
    }
}

mod test_2_color {
    use exec_target::exec_target_with_env_in;
    use std::collections::HashMap;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_multi_line() {
        let env = env_1!();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-e", "a", "-f", "1", "--color", "always"],
            env,
            b"abcabca\noooooo\nabcabca\n" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            "<S>1<E>bc<S>1<E>bc<S>1<E>\noooooo\n<S>1<E>bc<S>1<E>bc<S>1<E>\n"
        );
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_multi_line_opt_n() {
        let env = env_1!();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-e", "a", "-f", "1", "-n", "--color", "always"],
            env,
            b"abcabca\noooooo\nabcabca\n" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            "<S>1<E>bc<S>1<E>bc<S>1<E>\n<S>1<E>bc<S>1<E>bc<S>1<E>\n"
        );
        assert!(oup.status.success());
    }
}

mod test_3 {
    use exec_target::exec_target;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_output_broken_pipe() {
        let cmdstr = format!(
            "cat \"{}\" | \"{}\" -e \"A\" -f a | head -n 2",
            fixture_text10k!(),
            TARGET_EXE_PATH
        );
        let oup = exec_target("sh", ["-c", &cmdstr]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "aBCDEFG\nHIJKLMN\n");
        assert!(oup.status.success());
    }
}

mod test_4 {
    use exec_target::exec_target_with_in;
    //use exec_target::args_from;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;

    //
    // [BUG] thread 'main' panicked at 'begin <= end (4 <= 2) when slicing `$2 :: $0`', /checkout/src/libcore/str/mod.rs:2221:4
    // echo "001cea1eef55.softphone.blizoo.bg" | rust-gsub -e "(.*\\.){0,1}([A-Za-z0-9][A-Za-z0-9\\-]{1,61}(\\.[A-Za-z0-9]{2,}){0,1}(\\.[A-Za-z]{2,}){0,1}\\.[A-Za-z]{2,5})$" -f "\$2 :: \$0"
    //
    #[test]
    fn test_fix_bug_1() {
        let oup = exec_target_with_in(TARGET_EXE_PATH,
            [
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
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_fix_bug_2() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-e", "ICON=\"[^\"]*\"", "-f", ""],
            b"abc ICON=\"ABCDEFG\" defg\n" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "abc  defg\n");
        assert!(oup.status.success());
    }
}

mod test_4_more {
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_file_input() {
        let long_string = std::fs::read_to_string("fixtures/text10k.txt").unwrap();
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-e", "A", "-f", "b"],
            long_string.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert!(oup.stdout.contains("bBCDEFG"));
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_long_input() {
        let long_string = "a".repeat(10000);
        let expected_output = "b".repeat(10000) + "\n";
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-e", "a", "-f", "b"],
            long_string.as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, expected_output);
        assert!(oup.status.success());
    }
}

mod test_5_replace {
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_replace_with_newline() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-e", "a", "-f", "b\nc"],
            b"daded" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "db\ncded\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_replace_with_tab() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-e", "a", "-f", "b\tc"],
            b"daded" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "db\tcded\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_replace_with_dollar() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-e", "a", "-f", "$$"], b"daded" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "d$ded\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_unicode_input() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-e", "ü", "-f", "ue"], "fübar".as_bytes());
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "fuebar\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_unicode_replacement() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-e", "u", "-f", "ü"], b"fubar" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "fübar\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_quiet_no_match() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-n", "-e", "z", "-f", "y"],
            b"abc" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_multiple_replacements() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-e", "a", "-f", "b", "-e", "c", "-f", "d"],
            b"abc" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "bbd\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_literal_backslash() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-e", "a", "-f", "\\"], b"bac" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "b\\c\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_complex_regex() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-e", "([a-z]+)-([0-9]+)", "-f", "$2-$1"],
            b"abc-123 xyz-456" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "123-abc 456-xyz\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_backreference() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-e", "(a)b(c)", "-f", "$2$1"],
            b"abc" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ca\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_replacement_with_original_string_part() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-e", "a(b)c", "-f", "x$1y"],
            b"abcde" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "xbyde\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_multiple_capture_groups_replacement() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-e", "(a)(b)(c)", "-f", "$3$2$1"],
            b"abc" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "cba\n");
        assert!(oup.status.success());
    }
}

mod test_6_regex {
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_overlapping_matches() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-e", "aba", "-f", "x"], b"ababa" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "xba\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_case_insensitive() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-e", "(?i)a", "-f", "b"], b"AbC" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "bbC\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_empty_match_begin() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-e", "^", "-f", "x"], b"abc" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "xabc\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_empty_match_end() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-e", "$", "-f", "x"], b"abc" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "abcx\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_quantifier_regex() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-e", "a{2,3}", "-f", "x"],
            b"aaabaaba" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "xbxba\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_alternation_regex() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-e", "a|b", "-f", "x"], b"abc" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "xxc\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_character_class_regex() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-e", "[ab]", "-f", "x"], b"abc" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "xxc\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_word_boundary_regex() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-e", "\\ba\\b", "-f", "x"],
            b"a ab a" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "x ab x\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_non_capturing_group_regex() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-e", "(?:a)b", "-f", "x"], b"ab" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "x\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_greedy_quantifier_regex() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-e", "a.*b", "-f", "x"],
            b"acbacb" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "x\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_lazy_quantifier_regex() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-e", "a.*?b", "-f", "x"],
            b"acbacd" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "xacd\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_possessive_quantifier_regex() {
        let oup = exec_target_with_in(TARGET_EXE_PATH, ["-e", "a*+", "-f", "x"], b"aa" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "x\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_named_capture_groups() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            [
                "-e",
                "(?P<first>[a-z]+)-(?P<second>[0-9]+)",
                "-f",
                "${second}-${first}",
            ],
            b"abc-123" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "123-abc\n");
        assert!(oup.status.success());
    }
}

mod test_6_regex_unsupport {
    use exec_target::exec_target_with_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_look_ahead() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-e", "a(?=b)", "-f", "x"],
            b"abac" as &[u8],
        );
        // aki-gsub: regex parse error:
        //     a(?=b)
        //      ^^^
        // error: look-around, including look-ahead and look-behind, is not supported
        assert!(oup.stderr.contains("regex parse error"));
        assert!(oup.stderr.contains("look-ahead"));
        assert!(oup.stderr.contains("is not supported"));
        assert!(!oup.status.success());
    }
    //
    #[test]
    fn test_look_behind() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-e", "(?<=b)a", "-f", "x"],
            b"baca" as &[u8],
        );
        // aki-gsub: regex parse error:
        //     (?<=b)a
        //     ^^^^
        // error: look-around, including look-ahead and look-behind, is not supported
        assert!(oup.stderr.contains("regex parse error"));
        assert!(oup.stderr.contains("look-behind"));
        assert!(oup.stderr.contains("is not supported"));
        assert!(!oup.status.success());
    }
    //
    #[test]
    fn test_atomic_group_regex() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-e", "(?>ab|a)c", "-f", "x"],
            b"abc" as &[u8],
        );
        // aki-gsub: regex parse error:
        //     (?>ab|a)c
        //       ^
        // error: unrecognized flag
        assert!(oup.stderr.contains("regex parse error"));
        assert!(oup.stderr.contains("unrecognized flag"));
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    //
    #[test]
    fn test_backreference_to_named_group_regex() {
        let oup = exec_target_with_in(
            TARGET_EXE_PATH,
            ["-e", "(?P<name>a)b\\k<name>", "-f", "x"],
            b"aba" as &[u8],
        );
        // aki-gsub: regex parse error:
        //     (?P<name>a)b\k<name>
        //                 ^^
        // error: unrecognized escape sequence
        assert!(oup.stderr.contains("regex parse error"));
        assert!(oup.stderr.contains("unrecognized escape sequence"));
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
}
