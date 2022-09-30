use std::fmt::Debug;

#[derive(Debug, Default)]
pub struct Reporter {}

impl Reporter {
    pub fn error(line: i32, message: &str) {
        report_error(line, "", message)
    }
}

fn report_error(line: i32, location: &str, message: &str) {
    println!("[line {} Error {}: {}", line, location, message);
}
