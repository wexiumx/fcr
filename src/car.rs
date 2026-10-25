
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

pub fn python(file: String) {
    let Ok((_, stdout, stderr)) = rashf!("python3 {}", file) else { panic!("failed to print file") };
    println!("{}", stdout);
    println!("{}", stderr);
}

pub fn javascript(file: String) {
    let Ok((_, stdout, stderr)) = rashf!("bun run {}", file) else { panic!("failed to print file") };
    println!("{}", stdout);
    println!("{}", stderr);
}

pub fn java(file: String, file_without_ex: String) {
    let Ok((_, stdout, stderr)) = rashf!("javac {} && java {}", file, file_without_ex) else { panic!("failed to print file") };
    println!("{}", stdout);
    println!("{}", stderr);
}

pub fn c_sharp(file: String) {
    let Ok((_, stdout, stderr)) = rashf!("dotnet run {}", file) else { panic!("failed to print file") };
    println!("{}", stdout);
    println!("{}", stderr);
}

pub fn r(file: String) {
    let Ok((_, stdout, stderr)) = rashf!("Rscript {}", file) else { panic!("failed to print file") };
    println!("{}", stdout);
    println!("{}", stderr);
}

pub fn php(file: String) {
    let Ok((_, stdout, stderr)) = rashf!("php {}", file) else { panic!("failed to print file") };
    println!("{}", stdout);
    println!("{}", stderr);
}

pub fn fortran(file: String, file_without_ex: String) {
    let Ok((_, stdout, stderr)) = rashf!("gfortran {} -o {} && ./{}", file, file_without_ex, file_without_ex) else { panic!("failed to print file") };
    println!("{}", stdout);
    println!("{}", stderr);
}

pub fn swift(file: String) {
    let Ok((_, stdout, stderr)) = rashf!("swift {}", file) else { panic!("failed to print file") };
    println!("{}", stdout);
    println!("{}", stderr);
}

pub fn haskell(file: String) {
    let Ok((_, stdout, stderr)) = rashf!("runghc {}", file) else { panic!("failed to print file") };
    println!("{}", stdout);
    println!("{}", stderr);
}

pub fn elixir(file: String) {
    let Ok((_, stdout, stderr)) = rashf!("elixir {}", file) else { panic!("failed to print file") };
    println!("{}", stdout);
    println!("{}", stderr);
}

pub fn ocaml(file: String) {
    let Ok((_, stdout, stderr)) = rashf!("ocaml {}", file) else { panic!("failed to print file") };
    println!("{}", stdout);
    println!("{}", stderr);
}

pub fn lisp(file: String) {
    let Ok((_, stdout, stderr)) = rashf!("sbcl --script {}", file) else { panic!("failed to print file") };
    println!("{}", stdout);
    println!("{}", stderr);
    
}

pub fn scheme(file: String) {
    let Ok((_, stdout, stderr)) = rashf!("guile {}", file) else { panic!("failed to print file") };
    println!("{}", stdout);
    println!("{}", stderr);
}

pub fn cloujure(file: String) {
    let Ok((_, stdout, stderr)) = rashf!("clj -M {}", file) else { panic!("failed to print file") };
    println!("{}", stdout);
    println!("{}", stderr);
}
