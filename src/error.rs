use std::{error, fmt};

pub(crate) type AppResult<T> = Result<T, AppError>;

#[derive(Debug, PartialEq)]
pub(crate) struct AppError {
    kind: String,
    message: String
}

impl std::fmt::Display for AppError {

    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        write!(f, "An Error Occurred, Please Try Again!") // user-facing output
    }
}

impl AppError {
    pub fn new(message: &str) -> Self {
        AppError {
            kind: "app".to_string(),
            message: message.to_string()
        }
    }
}

impl<T> From<T> for AppError 
    where
        T: Into<Box<dyn error::Error + Sync + Send>> {
    fn from(err: T) -> Self {

        AppError {
            kind: "app".to_string(),
            message: format!("app error: {}", err.into())
        }
        
    }
}

// impl From<io::Error> for AppError {
//     fn from(error: io::Error) -> Self {
//         AppError {
//             kind: String::from("io"),
//             message: error.to_string()
//         }
//     }
// }

// impl From<reqwest::Error> for AppError {
//     fn from(error: reqwest::Error) -> Self {
//         AppError {
//             kind: String::from("http"),
//             message: error.to_string()
//         }
//     }
// }

// impl From<SetLoggerError> for AppError {
//     fn from(error: SetLoggerError) -> Self {
//         AppError {
//             kind: String::from("logger"),
//             message: error.to_string()
//         }
//     }
// }

#[cfg(test)]
mod tests{
    use std::io::ErrorKind;

    use super::*;

    fn produce_error() -> Result<(), AppError> {
        Err(AppError{
            kind: "io".to_string(),
            message: "error message".into()
        })
    }

    #[test]
    fn test_app_error() {

        match produce_error() {
            Err(e) => {
                eprintln!("{:#?}", e)
            }
            _ => println!("ok")
        }
    }

    #[test]
    fn test_from_io_error() {
        let _err = AppError::from(std::io::Error::from(ErrorKind::NotFound));
        eprintln!("{:#?}", _err)
    }
}
