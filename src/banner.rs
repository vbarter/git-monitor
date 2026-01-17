/// ANSI colored banner for git-monitor
pub fn print_banner() {
    // ANSI color codes
    const RESET: &str = "\x1b[0m";
    const BOLD: &str = "\x1b[1m";
    const GREEN: &str = "\x1b[38;2;166;227;161m";    // #A6E3A1
    const BLUE: &str = "\x1b[38;2;137;180;250m";     // #89B4FA
    const YELLOW: &str = "\x1b[38;2;249;226;175m";   // #F9E2AF
    const TEXT: &str = "\x1b[38;2;205;214;244m";     // #CDD6F4
    const DIM: &str = "\x1b[38;2;108;112;134m";      // #6C7086

    let banner = format!(
        r#"
{BLUE}     ●────●────●{RESET}
{BLUE}          │    ╲{RESET}
{BLUE}          ●     ●{RESET}  {GREEN}{BOLD}>{RESET} {TEXT}{BOLD}git-monitor{RESET} {YELLOW}_{RESET}
{DIM}
     Real-time Git file change monitoring
     with beautiful animations{RESET}
"#
    );

    eprintln!("{}", banner);
}

/// Print version info
pub fn print_version() {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    const DIM: &str = "\x1b[38;2;108;112;134m";
    const RESET: &str = "\x1b[0m";

    eprintln!("{DIM}v{VERSION}{RESET}\n");
}
