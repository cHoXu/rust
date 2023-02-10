use std::fs::File;
use std::io::{self, ErrorKind, Read};

pub fn get_error_curse() {
    //panic!("crash and burn");
    //let v = vec![1, 2, 3];
    //v[99];
    //try_open_file();
    //try_open_file_unwrap_or_else();
    //try_open_file_unwrap();
    //try_open_file_expect();
    //read_username_from_file().expect("Failed to read username");
    read_username_from_file_shortcut().expect("Failed to read username");
}
fn try_open_file() {
    let path_file = "D:\\restaurant\\upload\\hello.txt";
    let f_result = File::open(path_file);
    let f = match f_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(path_file) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => { panic!("Problem opening the file: {:?}", other_error); },
        },
    };
}
fn try_open_file_unwrap_or_else() {
    let path_file = "D:\\restaurant\\upload\\hell.txt";
    let f = File::open(path_file).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(path_file).unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else { panic!("Problem opening the file: {:?}", error); }
    });
}
fn try_open_file_unwrap() {
    let path_file = "D:\\restaurant\\upload\\hell.txt";
    let f = File::open(path_file).unwrap();
}
//Is most usefully when you want to call panic! and use the error value to create the panic message
fn try_open_file_expect() {
    let path_file = "x:\\restaurant\\upload\\hell.txt";
    let f = File::open(path_file).expect("Custom MSG : Failed to open hell.txt");
}
//error propagation
fn read_username_from_file() -> Result<String, io::Error> {
    let path_file = "x:\\restaurant\\upload\\hell.txt";
    let username_file_result = File::open(path_file);
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
//error propagation with ? shortcut
fn read_username_from_file_shortcut() -> Result<String, io::Error> {
    let path_file = "x:\\restaurant\\upload\\hell.txt";
    let mut username_file = File::open(path_file)?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
