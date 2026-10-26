pub fn v_flag() {
    use github_release_check::GitHub;

    let github = GitHub::new().unwrap();
    let versions = github.get_all_versions("wexiumx/fcr").unwrap();

    println!("Fcr - A fast code runner");
    println!("{}", versions[0]);
    println!("https://github.com/wexiumx/fcr");
}

pub fn unkown_flag(arg: &str) {
    println!("error: unexpectd argument `{}` found", arg);
    println!("Usage: fcr [OPTIONS] [FILE]...");
    println!("For more information, try `--help`");
}
