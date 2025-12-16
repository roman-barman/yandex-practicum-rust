use std::io::Write;

/// A helper trait for serializing a message into an output writer.
///
/// Types implementing this trait can write their representation into any
/// [`std::io::Write`] implementor (file, buffer, network stream, etc.).
pub trait MessageWriter {
    /// The error type returned if serialization fails.
    type Error: std::error::Error;
    /// Writes this message into the provided writer.
    ///
    /// Parameters:
    /// - `writer`: destination to write the serialized message to.
    ///
    /// Returns `Ok(())` on success or an error if writing/serialization fails.
    fn write_to<T: Write>(&self, writer: &mut T) -> Result<(), Self::Error>;
}
