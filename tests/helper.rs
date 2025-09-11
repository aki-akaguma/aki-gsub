#[allow(unused_macros)]
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

#[allow(unused_macros)]
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

#[allow(unused_macros)]
macro_rules! try_help_msg {
    () => {
        "Try --help for help.\n"
    };
}

#[allow(unused_macros)]
macro_rules! program_name {
    () => {
        "aki-gsub"
    };
}

#[allow(unused_macros)]
macro_rules! version_msg {
    () => {
        concat!(program_name!(), " ", env!("CARGO_PKG_VERSION"), "\n")
    };
}

#[allow(unused_macros)]
macro_rules! fixture_invalid_utf8 {
    () => {
        "fixtures/invalid_utf8.txt"
    };
}

#[allow(unused_macros)]
macro_rules! fixture_text10k {
    () => {
        "fixtures/text10k.txt"
    };
}
