pub trait BrokenPipeError {
    fn is_broken_pipe(&self) -> bool;
}

impl BrokenPipeError for anyhow::Error {
    fn is_broken_pipe(&self) -> bool {
        matches!(self.downcast_ref::<std::io::Error>(),
            Some(ioe) if ioe.kind() == std::io::ErrorKind::BrokenPipe)
    }
}

impl<T> BrokenPipeError for anyhow::Result<T> {
    fn is_broken_pipe(&self) -> bool {
        match self {
            Err(ref err) => err.is_broken_pipe(),
            _ => false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    //
    fn func_bail_broken_pipe() -> anyhow::Result<()> {
        let io_error = std::io::Error::from(std::io::ErrorKind::BrokenPipe);
        bail!(io_error);
    }
    fn func_bail_unexpected_eof() -> anyhow::Result<()> {
        let io_error = std::io::Error::from(std::io::ErrorKind::UnexpectedEof);
        bail!(io_error);
    }
    //
    #[test]
    fn test_anyhow_is_broken_pipe() {
        let err = func_bail_broken_pipe();
        assert!(err.is_broken_pipe());
        let err = func_bail_unexpected_eof();
        assert!(!err.is_broken_pipe());
    }
}
