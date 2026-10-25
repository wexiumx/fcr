pub mod car;

use std::path::Path;


fn extract_strings_from_file(array: Vec<String>) -> Vec<(String, String, String)> {
    let mut new_array: Vec<(String, String, String)> = Vec::new();    

    for file in array {
        let path_file = Path::new(&file);

        let file = path_file.to_string_lossy().to_string();
        let file_without_ex = path_file.file_stem()
            .expect("Failed to get file without extension")
            .to_string_lossy()
            .to_string();
        let file_extension = match path_file.extension() {
           Some(extension) => extension.to_string_lossy().to_string(),
           None => String::from(""), 
        };

        let tuple: (String, String, String) = (file, file_without_ex, file_extension);
        new_array.push(tuple);
    };

    new_array
}

fn comparing_extensions(array: Vec<(String, String, String)>) {
    for (file, file_without_ex, extension) in array {

        match extension.as_str() {
            "" => car::binary(file),
            "rs" => car::rust(file, file_without_ex),
            "zig" => car::zig(file),
            "c" => car::c(file, file_without_ex),
            "c++" | "cpp" => car::cpp(file, file_without_ex),
            "go" => car::go(file),
            "py" => car::python(file),
            "js" => car::javascript(file),
            "java" => car::java(file, file_without_ex),
            "cs" => car::c_sharp(file),
            "R" | "r" => car::r(file),
            "php" => car::php(file),
            "f90" | "f95" | "f03" | "f08" => car::fortran(file, file_without_ex),
            "swift" => car::swift(file),
            "hs" => car::haskell(file),
            "exs" => car::elixir(file),
            "ml" => car::ocaml(file),
            "lisp" | "lsp" => car::lisp(file),
            "scm" | "ss" => car::scheme(file),
            "clj"  => car::cloujure(file),
            _ => println!("Sorry, this file extension: {} we don't support", file),
        }; 
    };
}


fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);

    let file_extension_array = extract_strings_from_file(args);

    comparing_extensions(file_extension_array)
}
