use std::fmt;
use std::fmt::Formatter;
use std::io::Error;

#[derive(Debug)]
// Contains all possible errors in out tool
pub enum ErrorCode {
    FileError(Error),
    IO(Error),
    ProcessError,
    InvalidTokenKey(char),
    Unknown,
}

impl ErrorCode {
    pub fn get_return_code(&self) -> i32 {
        match &self {
            ErrorCode::InvalidTokenKey(_) => 2,
            ErrorCode::FileError(_) => 10,
            ErrorCode::IO(_) => 11,
            ErrorCode::ProcessError => 65,
            _ => 1, // Everything != 0 will be treated as an error
        }
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            // TODO: Update displayed error message for each error message kind
            _ => write!(f, "{:?}", self), // for any variant not covered
        }
    }
}

impl From<Error> for ErrorCode {
    fn from(e: Error) -> Self {
        ErrorCode::FileError(e)
    }
}

impl PartialEq for ErrorCode {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }

    fn ne(&self, other: &Self) -> bool {
        self != other
    }
}

// Get the result from a function, and exit the process with the correct error code
pub fn exit_with_return_code(res: Result<(), ErrorCode>) {
    match res {
        // if it's a success, return 0
        Ok(_) => {
            log::debug!("Exit without any error, returning 0");
            std::process::exit(0);
        }
        // if there's an error, print an error message and return the return_code
        Err(e) => {
            let return_code = e.get_return_code();
            log::error!("Error on exit:\n\t\n\tReturning {}", e);
            std::process::exit(return_code);
        }
    }
}
