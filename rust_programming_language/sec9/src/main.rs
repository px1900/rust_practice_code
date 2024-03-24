use std::fs::File;
use std::io::{self, Read, ErrorKind};


fn read_username_from_file1() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("username.txt")?.read_to_string(&mut username)?;
    
    Ok(username)
}


fn main() {
    //panic!("crash and burn");

    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fd) => fd,
                Err(err) => panic!("Error on creating the file, error is {:?}", err),
            }
            other_error => {
                panic!("Error on opening the file, {:?}", other_error);
            }
        },
    };


    let data_file = File::open("data.txt").unwrap_or_else( |error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("data.txt").unwrap_or_else( |err| {
                panic!("failed on creating the file, error is {:?}", err);
            })
        } else {
            panic!("failed to open the file, error is {:?}", error);
        }
    });

    // These two methods are similar, the only difference is that .unwrap() will panic!
    // with the default error message, but .expect() will panic! with given message.
    //let test_file_1 = File::open("test1.txt").unwrap();
    //let test_file_2 = File::open("test2.txt").expect("failed on creating testfile2");


    let uname = read_username_from_file1().unwrap();
    println!("the username is {uname}");
    

}
