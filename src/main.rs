
use std::env;
use std::fs::File;
use std::io::{Read,self};
use std::path::Path;
use std::ops::Mul;
use std::mem::swap;
use std::num;
use std::fmt;
use std::error;
use std::io::{Error, ErrorKind};




fn double_arg(mut argv: env::Args) -> Result<i32, String> {
    argv.nth(1)
        .ok_or("Please give at least one argument".to_owned())
        .and_then(|arg| arg.parse::<i32>().map_err(|err| err.to_string()))
        .map(|i| i * 2)
}
//without error control
fn file_double_1<P: AsRef<Path>>(file_path: P) -> i32 {
    let mut file = File::open(file_path).unwrap(); // error 1
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap(); // error 2
    let n: i32 = contents.trim().parse().unwrap(); // error 3
    2 * n
}
// without early return
// This code looks a bit hairy.
fn file_double_2<P: AsRef<Path>>(file_path: P) -> Result<i32, String>{
    File::open(file_path)
        .map_err(|err| err.to_string())
        .and_then(|mut file| {
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .map_err(|err| err.to_string())
                .map(|_| contents)
        })
        .and_then(|contents|{
            contents.trim().parse::<i32>()
                .map_err(|err| err.to_string())
        })
        .map(|n| 2*n)
}
// without macro
fn file_double_3<P: AsRef<Path>>(file_path: P) -> Result<i32,String>{
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => return Err(err.to_string()),
    };
    let mut contents = String::new();
    if let Err(err) = file.read_to_string(&mut contents){
        return Err(err.to_string());
    };
    let n: i32 = match contents.trim().parse() {
        Ok(n) => n,
        Err(err) => return Err(err.to_string()),
    };
    Ok(2*n)
}
fn mul1(a:i32,b:i32) -> Result<i32,String> {
    return if a < b {
        mul1(b,a)
    }else{
        if 0x3fffffff/b < a {
            Err("数字太大了".to_string())
        } else{
            Ok(a*b)
        }
    }
}

// we should't use string to solve all err
fn file_double_4<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
    let mut file = File::open(file_path).map_err(|e| e.to_string())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| e.to_string())?;
    let n = contents.trim().parse::<i32>().map_err(|e|e.to_string())?;
    mul1(n,2)
}
// we can define our error type

enum CliError{
    Io(io::Error),
    Parse(num::ParseIntError),
    Int(io::Error),
}
impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Both underlying errors already impl `Display`, so we defer to
            // their implementations.
            CliError::Io(ref err) => write!(f, "IO error: {}", err),
            CliError::Parse(ref err) => write!(f, "Parse error: {}", err),
            CliError::Int(ref err) => write!(f, "IO error: {}", err),
        }
    }
}
impl fmt::Debug for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Both underlying errors already impl `Display`, so we defer to
            // their implementations.
            CliError::Io(ref err) => write!(f, "IO error: {}", err),
            CliError::Parse(ref err) => write!(f, "Parse error: {}", err),
            CliError::Int(ref err) => write!(f, "IO error: {}", err),
        }
    }
}
impl error::Error for CliError {
    fn description(&self) -> &str {
        // Both underlying errors already impl `Error`, so we defer to their
        // implementations.
        match *self {
            CliError::Io(ref err) => err.description(),
            // Normally we can just write `err.description()`, but the error
            // type has a concrete method called `description`, which conflicts
            // with the trait method. For now, we must explicitly call
            // `description` through the `Error` trait.
            CliError::Parse(ref err) => err.description(),
            CliError::Int(ref err) => err.description(),
        }
    }
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            // N.B. Both of these implicitly cast `err` from their concrete
            // types (either `&io::Error` or `&num::ParseIntError`)
            // to a trait object `&Error`. This works because both error types
            // implement `Error`.
            CliError::Io(ref err) => Some(err),
            CliError::Parse(ref err) => Some(err),
            CliError::Int(ref err) => Some(err),
        }
    }
}
impl From<io::Error> for CliError{
    fn from(err: io::Error) -> CliError{
        CliError::Io(err)
    }
}
impl From<num::ParseIntError> for CliError{
    fn from(err: num::ParseIntError) -> CliError{
        CliError::Parse(err)
    }
}

fn mul2(a:i32,b:i32) -> Result<i32,CliError> {
    return if a < b {
        mul2(b,a)
    }else{
        if 0x3fffffff/b < a {
            Err(Error::new(ErrorKind::Other,"123"))?
        } else{
            Ok(a*b)
        }
    }
}
fn file_double_5<P:AsRef<Path>>(file_path:P) -> Result<i32,CliError>{
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let n: i32 = contents.trim().parse::<i32>()?;
    mul2(n,2)
}
fn main() {
    match file_double_5(r"E:\rust_actual\myFirstRust\src\foobar") {
        Ok(n) => println!("{}",n),
        Err(err) => println!("Error {:?}",err),
    }
}