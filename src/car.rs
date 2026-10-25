
// car -> Compile And Run
use rsbash::rashf;

pub fn rust(file: String, file_without_ex: String) {
    let Ok((_, stdout, stderr)) = rashf!("rustc {} && ./{}", file, file_without_ex) else { panic!("failed to print file") };
    println!("{}", stdout);
    println!("{}", stderr);
}

pub fn zig(file: String) {
    let Ok((_, stdout, stderr)) = rashf!("zig run {}", file) else { panic!("failed to print file") };
    println!("{}", stdout);
    println!("{}", stderr);
}

pub fn c(file: String, file_without_ex: String) {
    let Ok((_, stdout, stderr)) = rashf!("gcc {} -o {} && ./{}", file, file_without_ex, file_without_ex) else { panic!("failed to print file") };
    println!("{}", stdout);
    println!("{}", stderr);
}

pub fn cpp(file: String, file_without_ex: String) {
    let Ok((_, stdout, stderr)) = rashf!("g++ {} -o {} && ./{}", file, file_without_ex, file_without_ex) else { panic!("failed to print file") };
    println!("{}", stdout);
    println!("{}", stderr);
}

pub fn go(file: String) {
    let Ok((_, stdout, stderr)) = rashf!("go run {}", file) else { panic!("failed to print file") };
    println!("{}", stdout);
    println!("{}", stderr);
}
