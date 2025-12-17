use std::num::{ParseFloatError, ParseIntError};
use std::string::FromUtf8Error;
use crate::error::error_kind::{FLOAT_PARSE_ERROR, INT_PARSE_ERROR, INVALID_UTF8_STR, STD_IO_ERROR};

/// Macro to define error kinds with codes and messages.
///
/// This macro generates a module containing error kind constants, each with a unique code and descriptive message.
///
/// # Arguments
///
/// * `$id` - The identifier for the error kind
/// * `$code` - The numeric error code
/// * `$message` - The descriptive error message
macro_rules! error_kind {
    ($(($id:ident,$code:literal,$message:literal)),+$(,)?) => {
        /// Module containing error kind constants.
        pub(crate) mod error_kind{
        $(
            /// Error kind constant with code and message.
            pub(crate) const $id: super::Kind = ($code, $message);
        )+
    }
    };
}

/// Type alias for error kind, consisting of a code and message.
pub(crate) type Kind = (u16, &'static str);

/// Type alias for results that may contain errors.
pub type Result<T> = std::result::Result<T, Error>;

/// Enumeration of all error kinds used in the PDF parser.
error_kind!(
    (INVALID_PDF_VERSION, 1000, "Invalid PDF version"),
    (STD_IO_ERROR, 1001, "Std IO Error"),
    (INVALID_PDF_FILE, 1002, "Invalid PDF file"),
    (TRAILER_NOT_FOUND, 1003, "Trailer not found"),
    (EOF, 1004, "End of file"),
    (INVALID_UTF8_STR,1005, "Invalid UTF8 string"),
    (INT_PARSE_ERROR,1006, "Int parse error"),
    (INVALID_CROSS_TABLE_ENTRY,1007, "Invalid cross table entry"),
    (TRAILER_EXCEPT_A_DICT,1008, "Trailer except a dict"),
    (INVALID_NUMBER,1009, "Invalid number"),
    (FLOAT_PARSE_ERROR,1010, "Float parse error"),
    (EXCEPT_TOKEN,1011, "Except a token"),
    (STR_NOT_ENCODED,1012, "String not encoded"),
    (ILLEGAL_TOKEN,1013, "Illegal token"),
    (INVALID_REAL_NUMBER,1014, "Invalid real number"),
    (PARSE_UNSIGNED_VALUE_ERR,1015, "Parse unsigned value error"),
    (SEEK_EXEED_MAX_SIZE,1016, "Seek exceed max size"),
    (NO_XREF_TABLE_FOUND,1017, "No xref table found"),
    (ILLEGAL_STREAM,1018, "Illegal stream"),
    (EXCEPT_TRAILER,1019, "Except trailer"),
    (CANT_FIND_ROOT,1020, "Can't find root"),
    (PAGE_PARSE_ERROR,1021, "Page parse error"),
);

/// Inner structure holding error details.
#[derive(Debug)]
struct Inner {
    /// Numeric error code.
    pub code: u16,
    /// Descriptive error message.
    pub message: String,
}

/// Custom error type for PDF parsing operations.
#[derive(Debug)]
pub struct Error {
    /// Inner error details.
    inner: Inner,
}

impl Error {
    /// Creates a new error with the specified kind and message.
    ///
    /// # Arguments
    ///
    /// * `kind` - The error kind containing code and base message
    /// * `message` - The specific error message
    ///
    /// # Returns
    ///
    /// A new Error instance
    pub(crate) fn new<T>(kind: Kind, message: T) -> Self where T: Into<String>{
        let message = message.into();
        Self {
            inner: Inner {
                code: kind.0,
                message,
            },
        }
    }
}

impl From<Kind> for Error {
    /// Converts an error kind to an Error instance.
    ///
    /// # Arguments
    ///
    /// * `kind` - The error kind to convert
    ///
    /// # Returns
    ///
    /// An Error instance with the kind's code and message
    fn from(kind: Kind) -> Self {
        Self {
            inner: Inner {
                code: kind.0,
                message: kind.1.to_string(),
            },
        }
    }
}

impl From<std::io::Error> for Error {
    /// Converts a standard IO error to a custom Error instance.
    ///
    /// # Arguments
    ///
    /// * `e` - The IO error to convert
    ///
    /// # Returns
    ///
    /// A custom Error instance with STD_IO_ERROR kind
    fn from(e: std::io::Error) -> Self {
        Self {
            inner: Inner {
                code: STD_IO_ERROR.0,
                message: e.to_string(),
            },
        }
    }
}

impl From<FromUtf8Error> for Error {
    /// Converts a UTF-8 conversion error to a custom Error instance.
    ///
    /// # Arguments
    ///
    /// * `e` - The UTF-8 conversion error to convert
    ///
    /// # Returns
    ///
    /// A custom Error instance with INVALID_UTF8_STR kind
    fn from(e: FromUtf8Error) -> Self {
        Self {
            inner: Inner {
                code: INVALID_UTF8_STR.0,
                message: e.to_string(),
            },
        }
    }
}

impl From<ParseIntError> for Error{
    /// Converts an integer parsing error to a custom Error instance.
    ///
    /// # Arguments
    ///
    /// * `e` - The integer parsing error to convert
    ///
    /// # Returns
    ///
    /// A custom Error instance with INT_PARSE_ERROR kind
    fn from(e: ParseIntError) -> Self {
        Self {
            inner: Inner {
                code: INT_PARSE_ERROR.0,
                message: e.to_string(),
            },
        }
    }
}

impl From<ParseFloatError> for Error {
    /// Converts a float parsing error to a custom Error instance.
    ///
    /// # Arguments
    ///
    /// * `e` - The float parsing error to convert
    ///
    /// # Returns
    ///
    /// A custom Error instance with FLOAT_PARSE_ERROR kind
    fn from(e: ParseFloatError) -> Self {

        Self {
            inner: Inner {
                code: FLOAT_PARSE_ERROR.0,
                message: e.to_string(),
            },
        }
    }
}