use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;
use std::error::Error;


pub fn make_panic(){
    panic!("crash and burn");
}

pub fn out_of_index_panic(){
    let v = vec![1, 2, 3];
    v[99];
}

pub fn make_res_error(){
    let f = File::open("Hello.txt");

    let f = match f{
        Ok(file)=>file,
        Err(error)=>match error.kind(){
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            other_error => panic!("There was a problem opening the file: {:?}", other_error),
        }
    };
}

pub fn make_map_error(){
    let f = File::open("hello.txt").map_err(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Tried to create file but there was a problem: {:?}", error);
            })
        } else {
            panic!("There was a problem opening the file: {:?}", error);
        }
    });
}

pub fn make_unwrap_error(){
    //unwrap automatically returns the value inside the Ok result or panics if it catches an error
    let f = File::open("hello.txt").unwrap();
}

pub fn make_expect_error(){
    //expect, instead of unwrap, lets us choose the message to print for the error.
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}



fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_simplified() -> Result<String, io::Error>{
    //the ? let the code run as in the previous, non-simplified function:
    //if the Result value is an Ok it will return the value and the code keeps
    //running, if it is an Err it will return the error out of the function.
    //it can only be used in functions that return Result<T,E>
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_shortened_version() -> Result<String, io::Error>{
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

pub fn propagating_error(){
    let res = read_username_from_file();

    let res = match res{
        Ok(s)=>s,
        Err(error)=>{panic!("error : {}",error);},
    };

    let second_result = read_username_from_file_simplified();

    let second_result = match second_result{
        Ok(s)=>s,
        Err(error)=>{panic!("error : {}",error);},
    };

    let third_result = read_username_shortened_version();

    let third_result = match third_result {
        Ok(s)=>s,
        Err(error)=>panic!("error: {}", error),
    };
}

pub fn _make_box_error()-> Result<(),Box<dyn Error>>{
    let f = File::open("hello.txt")?;

    Ok(())

}