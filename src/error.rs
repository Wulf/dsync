// TODO: change backtrace implementation to be by thiserror, if possible once features become stable
// error_generic_member_access https://github.com/rust-lang/rust/issues/99301
// provide_any https://github.com/rust-lang/rust/issues/96024

#[cfg(feature = "backtrace")]
use std::backtrace::Backtrace;
use std::{io::Error as ioError, path::Path};

pub type Result<T> = std::result::Result<T, Error>;

/// Macro to not repeat having to do multiple implementations of a [ErrorInner] variant with the same string type
macro_rules! fn_string {
    ($fn_name:ident, $fortype:expr) => {
        #[doc = concat!("Create a new [Self] as [", stringify!($fortype), "]")]
        pub fn $fn_name<M>(msg: M) -> Self
        where
            M: Into<String>,
        {
            return Self::new($fortype(msg.into()));
        }
    };
}

/// Error type for libytdlr, contains a backtrace, wrapper around [ErrorInner]
#[derive(Debug)]
pub struct Error {
    /// The actual error
    source: ErrorEnum,
    #[cfg(feature = "backtrace")]
    /// The backtrace for the error
    backtrace: Backtrace,
}

impl Error {
    /// Construct a new [Error] instance based on [ErrorInner]
    pub fn new(source: ErrorEnum) -> Self {
        Self {
            source,
            #[cfg(feature = "backtrace")]
            backtrace: Backtrace::capture(),
        }
    }

    #[cfg(feature = "backtrace")]
    /// Get the backtrace that is stored
    pub fn backtrace(&self) -> &Backtrace {
        &self.backtrace
    }

    fn_string!(other, ErrorEnum::Other);
    fn_string!(
        unsupported_schema_format,
        ErrorEnum::UnsupportedSchemaFormat
    );
    fn_string!(unsupported_type, ErrorEnum::UnsupportedType);
    fn_string!(no_file_signature, ErrorEnum::NoFileSignature);

    /// Create a custom [ioError] with this [Error] wrapped around with a [Path] attached
    pub fn custom_ioerror_path<M, P>(kind: std::io::ErrorKind, msg: M, path: P) -> Self
    where
        M: Into<String>,
        P: AsRef<Path>,
    {
        return Self::new(ErrorEnum::IoError(
            ioError::new(kind, msg.into()),
            format_path(path.as_ref().to_string_lossy().to_string()),
        ));
    }

    pub fn not_a_directory<M, P>(msg: M, path: P) -> Self
    where
        M: Into<String>,
        P: AsRef<Path>,
    {
        return Self::new(ErrorEnum::NotADirectory(
            msg.into(),
            path.as_ref().to_string_lossy().to_string(),
        ));
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.source.fmt(f)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        return self.source.source();
    }
}

// implement all From<> variants that ErrorInner also implements
impl<T> From<T> for Error
where
    T: Into<ErrorEnum>,
{
    fn from(value: T) -> Self {
        Self::new(value.into())
    }
}

/// Error type for "yt-downloader-rust", implements all Error types that could happen in this lib
#[derive(thiserror::Error, Debug)]
pub enum ErrorEnum {
    /// Wrapper Variant for [`std::io::Error`]
    /// Argument 1 (String) is up to the implementation to set, commonly the path
    #[error("IoError: {0}; {1}")]
    IoError(std::io::Error, String),
    /// Variant for when a directory path was expected but did not exist yet or was not a directory
    /// TODO: replace with io::ErrorKind::NotADirectory once stable <https://github.com/rust-lang/rust/issues/86442>
    #[error("NotADirectory: {0}; Path: \"{1}\"")]
    NotADirectory(String, String),
    /// Variant for unsupported diesel schema formats
    #[error("UnsupportedSchemaFormat: {0}")]
    UnsupportedSchemaFormat(String),
    /// Variant for unsupported sql types
    #[error("UnsupportedType: {0}")]
    UnsupportedType(String),
    /// Variant for when "has_file_signature" is `false`
    #[error("NoFileSignature: {0}")]
    NoFileSignature(String),

    /// Invalid generation config
    #[error("InvalidGenerationConfig: {0}")]
    InvalidGenerationConfig(String),

    /// Variant for Other messages
    #[error("Other: {0}")]
    Other(String),
}

/// Helper function to keep consistent formatting
#[inline]
fn format_path(msg: String) -> String {
    format!("Path \"{}\"", msg)
}

/// Trait to map [std::io::Error] into [Error]
pub trait IOErrorToError<T> {
    /// Map a [std::io::Error] to [Error] with a [std::path::Path] attached
    fn attach_path_err<P: AsRef<Path>>(self, path: P) -> Result<T>;

    /// Map a [std::io::Error] to [Error] with a [std::path::Path] and message attached
    fn attach_path_msg<P: AsRef<Path>, M: AsRef<str>>(self, path: P, msg: M) -> Result<T>;
}

impl<T> IOErrorToError<T> for std::result::Result<T, std::io::Error> {
    fn attach_path_err<P: AsRef<Path>>(self, path: P) -> Result<T> {
        return match self {
            Ok(v) => Ok(v),
            Err(e) => Err(crate::Error::new(ErrorEnum::IoError(
                e,
                format_path(path.as_ref().to_string_lossy().to_string()),
            ))),
        };
    }

    fn attach_path_msg<P: AsRef<Path>, M: AsRef<str>>(self, path: P, msg: M) -> Result<T> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => Err(crate::Error::new(ErrorEnum::IoError(
                e,
                format!(
                    "{msg} {path}",
                    msg = msg.as_ref(),
                    path = format_path(path.as_ref().to_string_lossy().to_string())
                ),
            ))),
        }
    }
}
