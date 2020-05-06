use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::str::from_utf8;

pub fn read_file(path: &str) -> String {
    let mut file = match File::open(path) {
        Err(err) => panic!("Couldn't open: {}", err.description()),
        Ok(value) => value,
    };

    let stat = match file.metadata() {
        Err(err) => panic!("Couldn't get stat: {}", err.description()),
        Ok(value) => value,
    };

    let mut buffer = vec![0; stat.len() as usize];

    match file.read(&mut buffer) {
        Err(err) => panic!("Couldn't read: {}", err.description()),
        Ok(_) => (),
    };

    match from_utf8(&buffer) {
        Err(err) => panic!("Couldn't convert buffer to string: {}", err.description()),
        Ok(value) => value.to_string(),
    }
}
