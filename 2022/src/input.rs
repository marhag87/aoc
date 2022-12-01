use std::fs::File;
use std::io::Read;

pub(crate) fn input_as_string(input_file: &str) -> String {
    let mut file =
        File::open(format!("input/{}", input_file)).expect("should be able to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("should be able to read input");
    contents
}
