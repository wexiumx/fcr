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
        let file_extension = path_file.extension()
            .expect("Failed to get file extension")
            .to_string_lossy()
            .to_string();

        let tuple: (String, String, String) = (file, file_without_ex, file_extension);
        new_array.push(tuple);
    };

    new_array
}

fn comparing_extensions(array: Vec<(String, String, String)>) {
    for (file, file_without_ex, extension) in array {

        match extension.as_str() {
            "rs" => println!("RUST is detected!: file: {}, file without extension: {}, file extension: {}", file, file_without_ex, extension),
            "zig" => println!("ZIG is detected!: file: {}, file without extension: {}, file extension: {}", file, file_without_ex, extension),
            "c" => println!("ZIG is detected!: file: {}, file without extension: {}, file extension: {}", file, file_without_ex, extension),
            "c++" | "cpp" => println!("C++ is detected!: file: {}, file without extension: {}, file extension: {}", file, file_without_ex, extension),
            "go" => println!("GO is detected!: file: {}, file without extension: {}, file extension: {}", file, file_without_ex, extension),
            _ => println!("Sorry, we doesn't support that file extension!"),
        }; 
    };
}


fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);

    let file_extension_array = extract_strings_from_file(args);

    comparing_extensions(file_extension_array)
}
