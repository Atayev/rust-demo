use std::env::current_dir;
use std::fs::create_dir_all;
use std::io::Error as IOError;
use std::io::ErrorKind;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    // input output result needs Ok(()) , Box<dyn std::error::Error>> any error that we dont know

    let a = vec![1, 2];

    // a[3]; // panic - error

    // panic!("fail") // for throw error

    // let cur_path = current_dir(); // type with error Res<buf,ERROR>

    // let mut target = match cur_path {
    //     Ok(path) => path,
    //     Err(e) => panic!("Cant get current path {e:?}"),
    // };
    let mut target = get_current_path()?;
    target.push("nomr");

    match create_dir_in(&target) {
        Ok(path) => print!("Created : {path:?}"),
        Err(e) => print!("Fail {e:?}"),
    }

    Ok(())
    // create_dir_all(&target).unwrap(); //throws panic if error
    // create_dir_all(&target).unwrap_or_else(|e| panic!("Error {e:?}")); with or_else version we provide callback to handle error

    // create_dir_all(&target).expect("Cant create directory") // throws error with message

    // match create_dir_all(&target) {
    //     Ok(()) => print!("Created path {target:?}"),
    //     Err(e) => match e.kind() {
    //         ErrorKind::InvalidData => {
    //             //
    //         }
    //         unknown_e => panic!("Erro : {unknown_e}"),
    //     },
    // } // catching errors with match and ErrorKind
}

fn create_dir_in(target: &PathBuf) -> Result<(), IOError> {
    // match output -Result<String, IOError> , without match Result <(),IOError>
    // match create_dir_all(target) {
    //     Ok(_) => Ok(format!("{}", target.to_string_lossy())),
    //     Err(e) => Err(e),
    // }

    create_dir_all(target)?; // ? for ignore error
    Ok(())
}
fn get_current_path() -> Result<PathBuf, IOError> {
    let path = current_dir()?; // ? if ok give Result

    Ok(path)
}
