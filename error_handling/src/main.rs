use std::{ error::Error, fs::File, io::{ self, ErrorKind, Read } };

fn multiple_handle_file_error() {
    let file = File::open("./src/hello.txt");

    let file_result = match file {
        Ok(file) => file,
        Err(error) =>
            match error.kind() {
                other_error => {
                    panic!("Problem opening the file: {:?}", other_error);
                }
            }
    };

    // Make easier than match
    let file = File::open("./src/hello.txt").unwrap_or_else(|err| { panic!("Err {:?}", err) });

    // Auto call panic!() method for us
    let greeting_file = File::open("./src/hello.txt").unwrap();

    // let greeting_file = File::open("hello.txt")
    File::open("hello.txt").expect("hello.txt should be included in this project");
}

// propagating errors
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_res = File::open("./src/hello.txt");

    let mut username_file = match username_file_res {
        Ok(file) => file,
        Err(e) => {
            println!("Error {:?}", e);
            return Err(e);
        }
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn main() {
    // multiple_handle_file_error();
    dbg!(read_username_from_file());
}
