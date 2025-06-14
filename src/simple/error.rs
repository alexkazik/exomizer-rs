/// Errors returned by the decrunch routines.
#[derive(Copy, Clone, Debug, thiserror::Error)]
pub enum DecrunchError {
    #[error("End of input")]
    EndOfInput,
    #[error("Unused input")]
    UnusedInput,
    #[error("Buffer overflow")]
    BufferOverflow,
    #[error("Decoding error")]
    DecodingError,
    #[error("Buffer too big")]
    BufferTooBig,
}
