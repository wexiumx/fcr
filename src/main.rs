use std::path::Path;
use rsbash::rashf;


enum FileExtension {
    Rust,
    Zig,
    C,
    Cpp,
    Go,
}

impl FileExtension {
    // car = compile and run;
    fn car_rust(file: &str, file_without_ex: &str) {
        rashf!("rustc {} && ./{}", file, file_without_ex);
    }

    fn run_zig(file: &str) {
        rashf!("zig run {}", file);
    }

    fn car_c(file: &str, file_without_ex: &str) {
        rashf!("gcc {} -o {} && ./{}", file, file, file_without_ex);
    }

    fn car_cpp(file: &str, file_without_ex: &str) {
        rashf!("");
    }

    fn run_go(file: &str) {
        rashf!("go run {}", file);
    }
}




fn get_file_extension(array: Vec<String>) -> Vec<&str> {
    let mut new_array: Vec<String> = Vec::new();    

    for file in array {
        let extension = Path::new(&file).extension();

        // I will add error handeling later
        match extension {
            Some(extension) => new_array.push(extension.to_string_lossy().to_string()),
            _ => println!("failed to get file extension!"),
        }
    };

    new_array
}

fn comparing_extensions(array: Vec<&str>) {
    for extension in array {

        let extension = match extension {
            "rs" => FileExtension::Rust,
            "zig" => FileExtension::Zig,
            "c" => FileExtension::C,
            "c++" | "cpp" => FileExtension::Cpp,
            "go" => FileExtension::Go,
            _ => println!("Sorry, we doesn't support that file extension!"),         } 


        
        match extension {
            FileExtension::Rust => ,
            FileExtension::Zig => ,
            FileExtension::C => ,
            FileExtension::Cpp => ,
            FileExtension::Go => , 
        }
    }
}


fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_extension_array = get_file_extension(args);

    comparing_extensions(file_extension_array)


    for arg in file_extension_array {
        println!(" extensions: {}", arg);
    }
}
