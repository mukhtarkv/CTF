// This is the root level error/result file

// This way we can force all application errors to be
// our custom error type.

// This will allow us to map errors to HTTP responses
// and custom error messages exposed to the client.

use std::fmt::Formatter;

// Application wide result with root level error
pub type Result<T> = core::result::Result<T, Error>;

// Root level error
#[derive(Debug)]
pub enum Error {
    // This is how we will wrap other error types from our
    // application.

    // For example this is how we can wrap errors from our
    // error module.

    // Model(model::Error)
}

// Implementing the From trait allows errors to be seamlessly
// recast as our main error type.

// For example by implementing this trait on Error,
// we can convert a model::Error to our root Error

//impl From<model::Error> for Error {
//    fn from(val: model::Error) -> {
//        Self::Model(val)
//    }
//}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut Formatter) -> std::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
