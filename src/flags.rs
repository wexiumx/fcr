use colored_text::Colorize;

pub fn v_flag() {
    use github_release_check::GitHub;

    let github = GitHub::new().unwrap();
    let versions = github.get_all_versions("wexiumx/fcr").unwrap();

    println!("Fcr - A fast code runner");
    println!("{}", versions[0]);
    println!("https://github.com/wexiumx/fcr");
}

pub fn h_flag() {
    println!("fast code runner\n");

    println!("{} {} [OPTIONS] [FILE]...\n", "Usage:".bold().underline(), "fcr".bold());

    println!("{}", "META OPTIONS".bold().underline());
    println!("  {}    Print help", "-h, --help".bold());
    println!("  {}    Print version of program", "-v, --version".bold());
}

pub fn unkown_flag(arg: &str) {
    
    println!("{} unexpectd argument `{}` found", "error:".red().bold(), arg);
    println!("{} {} [OPTIONS] [FILE]...", "Usage:".bold().underline(), "fcr".bold());
    println!("For more information, try `--help`");
}
