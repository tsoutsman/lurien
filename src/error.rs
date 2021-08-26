pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    FileSystem(ignore::Error),
    IoError(std::io::Error),
    NoMatch,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            Error::FileSystem(_) => "file system error",
            Error::IoError(_) => "io error",
            Error::NoMatch => "no matches found in the given input",
        };

        write!(f, "{}", result)
    }
}

impl std::convert::From<ignore::Error> for Error {
    fn from(e: ignore::Error) -> Self {
        Error::FileSystem(e)
    }
}

impl std::convert::From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IoError(e)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::FileSystem(s) => Some(s),
            Error::IoError(s) => Some(s),
            _ => None,
        }
    }
}
