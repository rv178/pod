pub const COLOUR_RESET: &str = "\x1b[0m";
pub const COLOUR_GREEN: &str = "\x1b[32m";
pub const COLOUR_YELLOW: &str = "\x1b[33m";
pub const COLOUR_BLUE: &str = "\x1b[34m";
pub const STYLE_BOLD: &str = "\x1b[1m";
pub const STYLE_UNDERLINE: &str = "\x1b[4m";

pub fn help() {
    let help_msg = format!(
        "{green}{bold}pod {reset} {version}
    Piano onset detection

{yellow}USAGE:{reset}
    pod {green}[FILE/OPTIONS]{reset}

{yellow}OPTIONS:{reset}
    {green}-h, --help{reset}
        Show this help message.

{yellow}EXAMPLES:{reset}
    pod {green}recording.wav{reset}

Link: {underline}{blue}https://github.com/rv178/pod{reset}",
        version = env!("CARGO_PKG_VERSION"),
        green = COLOUR_GREEN,
        bold = STYLE_BOLD,
        reset = COLOUR_RESET,
        yellow = COLOUR_YELLOW,
        underline = STYLE_UNDERLINE,
        blue = COLOUR_BLUE
    );
    println!("{}", help_msg);
}
