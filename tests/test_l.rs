#[macro_use]
mod helper;

#[macro_use]
mod helper_l;

mod test_0_l {
    use libaki_gsub::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_help() {
        let (r, sioe) = do_execute!(["-H"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), help_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_help_long() {
        let (r, sioe) = do_execute!(["--help"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), help_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_version() {
        let (r, sioe) = do_execute!(["-V"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), version_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_version_long() {
        let (r, sioe) = do_execute!(["--version"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), version_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_invalid_opt() {
        let (r, sioe) = do_execute!(["-z"]);
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(),
                ": ",
                "Invalid option: z\n",
                "Missing option: e\n",
                try_help_msg!()
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    #[test]
    fn test_non_option() {
        let (r, sioe) = do_execute!([""]);
        #[rustfmt::skip]
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(),
                ": ",
                "Missing option: e\n",
                "Unexpected argument: \n",
                try_help_msg!()
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    #[test]
    fn test_opt_color() {
        let (r, sioe) = do_execute!(["--color"]);
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
        let (r, sioe) = do_execute!(["--color", "red"]);
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
        let (r, sioe) = do_execute!(["--color", "auto"]);
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
        let (r, sioe) = do_execute!(["-e", "."]);
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
    #[test]
    fn test_no_pair_format() {
        let (r, sioe) = do_execute!(["-e", "a", "-e", "b", "-f", "X"]);
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

mod test_0_x_options_l {
    use libaki_gsub::*;
    use runnel::medium::stringio::*;
    use runnel::*;
    //
    #[test]
    fn test_x_option() {
        let (r, sioe) = do_execute!(["-X"]);
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
    //
    #[test]
    fn test_x_option_help() {
        let (r, sioe) = do_execute!(["-X", "help"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), x_help_msg!());
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_x_option_rust_version_info() {
        use assert_text::assert_text_match;
        let (r, sioe) = do_execute!(["-X", "rust-version-info"]);
        assert_eq!(buff!(sioe, serr), "");
        assert!(buff!(sioe, sout).contains("rustc"));
        assert_text_match!(buff!(sioe, sout), x_rvi_msg!());
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_multiple_x_options() {
        let (r, sioe) = do_execute!(["-X", "help", "-X", "rust-version-info"]);
        assert_eq!(buff!(sioe, serr), "");
        // The first one should be executed and the program should exit.
        assert!(buff!(sioe, sout).contains("Options:"));
        assert!(!buff!(sioe, sout).contains("rustc"));
        assert!(r.is_ok());
    }
    #[test]
    fn test_x_option_invalid() {
        let (r, sioe) = do_execute!(["-X", "red"]);
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
}

mod test_1_l {
    use libaki_gsub::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_invalid_utf8() {
        let v = std::fs::read(fixture_invalid_utf8!()).unwrap();
        let s = unsafe { String::from_utf8_unchecked(v) };
        let (r, sioe) = do_execute!(["-e", "a", "-f", "x"], &s);
        assert_eq!(
            buff!(sioe, serr),
            concat!(program_name!(), ": stream did not contain valid UTF-8\n",)
        );
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    //
    #[test]
    fn test_t1() {
        let (r, sioe) = do_execute!(["-e", "a", "-f", "1"], "abcabca");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "1bc1bc1\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t2() {
        let (r, sioe) = do_execute!(["-e", "a(b)c", "-f", "$1"], "abcabca");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "bba\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t3() {
        let (r, sioe) = do_execute!(["-e", "a(b)c", "-f", "$0"], "abcabca");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "abcabca\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t4() {
        let (r, sioe) = do_execute!(["-e", "a(b)c", "-f", "$2"], "abcabca");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "a\n");
        assert!(r.is_ok());
    }
}

mod test_1_color_l {
    use libaki_gsub::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_t1() {
        let (r, sioe) = do_execute!(
            env_1!(),
            ["-e", "a", "-f", "1", "--color", "always"],
            "abcabca"
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "<S>1<E>bc<S>1<E>bc<S>1<E>\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t2() {
        let (r, sioe) = do_execute!(
            env_1!(),
            ["-e", "a(b)c", "-f", "$1", "--color", "always"],
            "abcabca"
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "<S>b<E><S>b<E>a\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t3() {
        let (r, sioe) = do_execute!(
            env_1!(),
            ["-e", "a(b)c", "-f", "$0", "--color", "always"],
            "abcabca"
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "<S>abc<E><S>abc<E>a\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_t4() {
        let (r, sioe) = do_execute!(
            env_1!(),
            ["-e", "a(b)c", "-f", "$2", "--color", "always"],
            "abcabca"
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "<S><E><S><E>a\n");
        assert!(r.is_ok());
    }
}

mod test_2_l {
    use libaki_gsub::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_multi_line() {
        let (r, sioe) = do_execute!(["-e", "a", "-f", "1"], "abcabca\noooooo\nabcabca\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "1bc1bc1\noooooo\n1bc1bc1\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_multi_line_opt_n() {
        let (r, sioe) = do_execute!(["-e", "a", "-f", "1", "-n"], "abcabca\noooooo\nabcabca\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "1bc1bc1\n1bc1bc1\n");
        assert!(r.is_ok());
    }
}

mod test_2_color_l {
    use libaki_gsub::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_multi_line() {
        let (r, sioe) = do_execute!(
            env_1!(),
            ["-e", "a", "-f", "1", "--color", "always"],
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
        let (r, sioe) = do_execute!(
            env_1!(),
            ["-e", "a", "-f", "1", "-n", "--color", "always"],
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

mod test_3_l {
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

mod test_4_l {
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
        let (r, sioe) = do_execute!([
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
            ["-e", "ICON=\"[^\"]*\"", "-f", ""],
            "abc ICON=\"ABCDEFG\" defg\n"
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "abc  defg\n");
        assert!(r.is_ok());
    }
}

mod test_4_more_l {
    use libaki_gsub::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_file_input() {
        let long_string = std::fs::read_to_string("fixtures/text10k.txt").unwrap();
        let (r, sioe) = do_execute!(["-e", "A", "-f", "b"], &long_string);
        assert_eq!(buff!(sioe, serr), "");
        assert!(buff!(sioe, sout).contains("bBCDEFG"));
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_long_input() {
        let long_string = "a".repeat(10000);
        let expected_output = "b".repeat(10000) + "\n";
        let (r, sioe) = do_execute!(["-e", "a", "-f", "b"], &long_string);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), expected_output);
        assert!(r.is_ok());
    }
}

mod test_5_replace_l {
    use libaki_gsub::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_replace_with_newline() {
        let (r, sioe) = do_execute!(["-e", "a", "-f", "b\nc"], "daded\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "db\ncded\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_replace_with_tab() {
        let (r, sioe) = do_execute!(["-e", "a", "-f", "b\tc"], "daded\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "db\tcded\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_replace_with_dollar() {
        let (r, sioe) = do_execute!(["-e", "a", "-f", "$$"], "daded\n");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "d$ded\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_unicode_input() {
        let (r, sioe) = do_execute!(["-e", "ü", "-f", "ue"], "fübar");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "fuebar\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_unicode_replacement() {
        let (r, sioe) = do_execute!(["-e", "u", "-f", "ü"], "fubar");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "fübar\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_quiet_no_match() {
        let (r, sioe) = do_execute!(["-n", "-e", "z", "-f", "y"], "abc");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_multiple_replacements() {
        let (r, sioe) = do_execute!(["-e", "a", "-f", "b", "-e", "c", "-f", "d"], "abc");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "bbd\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_literal_backslash() {
        let (r, sioe) = do_execute!(["-e", "a", "-f", "\\"], "bac");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "b\\c\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_complex_regex() {
        let (r, sioe) = do_execute!(
            ["-e", "([a-z]+)-([0-9]+)", "-f", "$2-$1"],
            "abc-123 xyz-456"
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "123-abc 456-xyz\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_backreference() {
        let (r, sioe) = do_execute!(["-e", "(a)b(c)", "-f", "$2$1"], "abc");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ca\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_replacement_with_original_string_part() {
        let (r, sioe) = do_execute!(["-e", "a(b)c", "-f", "x$1y"], "abcde");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "xbyde\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_multiple_capture_groups_replacement() {
        let (r, sioe) = do_execute!(["-e", "(a)(b)(c)", "-f", "$3$2$1"], "abc");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "cba\n");
        assert!(r.is_ok());
    }
}

mod test_6_regex_l {
    use libaki_gsub::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_overlapping_matches() {
        let (r, sioe) = do_execute!(["-e", "aba", "-f", "x"], "ababa");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "xba\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_case_insensitive() {
        let (r, sioe) = do_execute!(["-e", "(?i)a", "-f", "b"], "AbC");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "bbC\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_empty_match_begin() {
        let (r, sioe) = do_execute!(["-e", "^", "-f", "x"], "abc");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "xabc\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_empty_match_end() {
        let (r, sioe) = do_execute!(["-e", "$", "-f", "x"], "abc");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "abcx\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_quantifier_regex() {
        let (r, sioe) = do_execute!(["-e", "a{2,3}", "-f", "x"], "aaabaaba");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "xbxba\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_alternation_regex() {
        let (r, sioe) = do_execute!(["-e", "a|b", "-f", "x"], "abc");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "xxc\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_character_class_regex() {
        let (r, sioe) = do_execute!(["-e", "[ab]", "-f", "x"], "abc");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "xxc\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_word_boundary_regex() {
        let (r, sioe) = do_execute!(["-e", "\\ba\\b", "-f", "x"], "a ab a");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "x ab x\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_non_capturing_group_regex() {
        let (r, sioe) = do_execute!(["-e", "(?:a)b", "-f", "x"], "ab");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "x\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_greedy_quantifier_regex() {
        let (r, sioe) = do_execute!(["-e", "a.*b", "-f", "x"], "acbacb");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "x\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_lazy_quantifier_regex() {
        let (r, sioe) = do_execute!(["-e", "a.*?b", "-f", "x"], "acbacd");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "xacd\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_possessive_quantifier_regex() {
        let (r, sioe) = do_execute!(["-e", "a*+", "-f", "x"], "aa");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "x\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_named_capture_groups() {
        let (r, sioe) = do_execute!(
            [
                "-e",
                "(?P<first>[a-z]+)-(?P<second>[0-9]+)",
                "-f",
                "${second}-${first}",
            ],
            "abc-123"
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "123-abc\n");
        assert!(r.is_ok());
    }
}

mod test_6_regex_unsupport_l {
    use libaki_gsub::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_look_ahead() {
        let (r, sioe) = do_execute!(["-e", "a(?=b)", "-f", "x"], "abac");
        // aki-gsub: regex parse error:
        //     a(?=b)
        //      ^^^
        // error: look-around, including look-ahead and look-behind, is not supported
        assert!(buff!(sioe, serr).contains("regex parse error"));
        assert!(buff!(sioe, serr).contains("look-ahead"));
        assert!(buff!(sioe, serr).contains("is not supported"));
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    //
    #[test]
    fn test_look_behind() {
        let (r, sioe) = do_execute!(["-e", "(?<=b)a", "-f", "x"], "baca");
        // aki-gsub: regex parse error:
        //     (?<=b)a
        //     ^^^^
        // error: look-around, including look-ahead and look-behind, is not supported
        assert!(buff!(sioe, serr).contains("regex parse error"));
        assert!(buff!(sioe, serr).contains("look-behind"));
        assert!(buff!(sioe, serr).contains("is not supported"));
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    //
    #[test]
    fn test_atomic_group_regex() {
        let (r, sioe) = do_execute!(["-e", "(?>ab|a)c", "-f", "x"], "abc");
        // aki-gsub: regex parse error:
        //     (?>ab|a)c
        //       ^
        // error: unrecognized flag
        assert!(buff!(sioe, serr).contains("regex parse error"));
        assert!(buff!(sioe, serr).contains("unrecognized flag"));
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    //
    #[test]
    fn test_backreference_to_named_group_regex() {
        let (r, sioe) = do_execute!(["-e", "(?P<name>a)b\\k<name>", "-f", "x"], "aba");
        // aki-gsub: regex parse error:
        //     (?P<name>a)b\k<name>
        //                 ^^
        // error: unrecognized escape sequence
        assert!(buff!(sioe, serr).contains("regex parse error"));
        assert!(buff!(sioe, serr).contains("unrecognized escape sequence"));
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
}
