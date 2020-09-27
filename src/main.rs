use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::ops::Mul;
use std::mem::swap;

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
fn mul(a:i32,b:i32) -> Result<i32,String> {
    return if a < b {
        mul(b,a)
    }else{
        if 0x3fffffff/b < a {
            Err("数字太大了".to_string())
        } else{
            Ok(a*b)
        }
    }
}
fn file_double_4<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
    let mut file = File::open(file_path).map_err(|e| e.to_string())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| e.to_string())?;
    let n = contents.trim().parse::<i32>().map_err(|e|e.to_string())?;
    mul(n,2)
}
fn main() {
    match file_double_4(r"E:\rust_actual\myFirstRust\src\foobar") {
        Ok(n) => println!("{}",n),
        Err(err) => println!("Error {:?}",err),
    }
}