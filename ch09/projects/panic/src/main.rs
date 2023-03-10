use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    //    let f = File::open("hello.txt");
    //    let f = match f {
    //        Ok(fc) => fc,
    //        Err(ref e) if e.kind() == ErrorKind::NotFound => {
    //            panic!("Tried to create file but there was a problem: {:?}", e);
    //        }
    //        Err(e) => {
    //            panic!("There was a problem opening file: {:?}", e);
    //        }
    //    };
    let name = read_username_from_file_2();
    let name = match name {
        Ok(s) => s,
        Err(e) => {
            panic!("There was a problem opening file: {:?}", e);
        }
    };
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

fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
